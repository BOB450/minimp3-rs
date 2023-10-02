//! # minimp3
//!
//! Provides a simple wrapper and bindinings to the [minimp3](https://github.com/lieff/minimp3) C library.
//!
//! ## Tokio
//!
//! By enabling the feature flag `async_tokio` you can decode frames using async
//! IO and tokio.
//!
//! [See the README for example usages.](https://github.com/germangb/minimp3-rs/tree/async)
pub use minimp3_sys as ffi;

// use std::mem;
use std::io::{Read, Seek};
// use std::marker::Send;
use std::os::raw::{c_int, c_void};

pub use error::Error;
use error::from_mini_error;
use slice_ring_buffer::SliceRingBuffer;
use std::{io, marker::Send, mem};

mod error;

/// Maximum number of samples present in a MP3 frame.
pub const MAX_SAMPLES_PER_FRAME: usize = ffi::MINIMP3_MAX_SAMPLES_PER_FRAME as usize;

const BUFFER_SIZE: usize = MAX_SAMPLES_PER_FRAME * 15;
const REFILL_TRIGGER: usize = MAX_SAMPLES_PER_FRAME * 8;

/// A MP3 decoder which consumes a reader and produces [`Frame`]s.
///
/// [`Frame`]: ./struct.Frame.html
pub struct Decoder<R> {
    reader: R,
    buffer: SliceRingBuffer<u8>,
    buffer_refill: Box<[u8; MAX_SAMPLES_PER_FRAME * 5]>,
    decoder: Box<ffi::mp3dec_t>,
}

// Explicitly impl [Send] for [Decoder]s. This isn't a great idea and should
// probably be removed in the future. The only reason it's here is that
// [SliceRingBuffer] doesn't implement [Send] (since it uses raw pointers
// internally), even though it's safe to send it across thread boundaries.
unsafe impl<R: Send> Send for Decoder<R> {}

/// A MP3 frame, owning the decoded audio of that frame.
#[derive(Debug, Clone)]
pub struct Frame {
    /// The decoded audio held by this frame. Channels are interleaved.
    pub data: Vec<i16>,
    /// This frame's sample rate in hertz.
    pub sample_rate: i32,
    /// The number of channels in this frame.
    pub channels: usize,
    /// MPEG layer used by this file.
    pub layer: usize,
    /// Current bitrate as of this frame, in kb/s.
    pub bitrate: i32,
}

impl<R> Decoder<R> {
    /// Creates a new decoder, consuming the `reader`.
    pub fn new(reader: R) -> Self {
        let mut minidec = unsafe { Box::new(mem::zeroed()) };
        unsafe { ffi::mp3dec_init(&mut *minidec) }

        Self {
            reader,
            buffer: SliceRingBuffer::with_capacity(BUFFER_SIZE),
            buffer_refill: Box::new([0; MAX_SAMPLES_PER_FRAME * 5]),
            decoder: minidec,
        }
    }

    /// Return a reference to the underlying reader.
    pub fn reader(&self) -> &R {
        &self.reader
    }

    /// Return a mutable reference to the underlying reader (reading from it is
    /// not recommended).
    pub fn reader_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Destroy the decoder and return the inner reader
    pub fn into_inner(self) -> R {
        self.reader
    }

    fn decode_frame(&mut self) -> Result<Frame, Error> {
        let mut frame_info = unsafe { mem::zeroed() };
        let mut pcm = Vec::with_capacity(MAX_SAMPLES_PER_FRAME);
        let samples: usize = unsafe {
            ffi::mp3dec_decode_frame(
                &mut *self.decoder,
                self.buffer.as_ptr(),
                self.buffer.len() as _,
                pcm.as_mut_ptr(),
                &mut frame_info,
            ) as _
        };

        if samples > 0 {
            unsafe {
                pcm.set_len(samples * frame_info.channels as usize);
            }
        }

        let frame = Frame {
            data: pcm,
            sample_rate: frame_info.hz,
            channels: frame_info.channels as usize,
            layer: frame_info.layer as usize,
            bitrate: frame_info.bitrate_kbps,
        };

        let current_len = self.buffer.len();
        self.buffer
            .truncate_front(current_len - frame_info.frame_bytes as usize);

        if samples == 0 {
            if frame_info.frame_bytes > 0 {
                Err(Error::SkippedData)
            } else {
                Err(Error::InsufficientData)
            }
        } else {
            Ok(frame)
        }
    }
}

#[cfg(feature = "async_tokio")]
impl<R: tokio::io::AsyncRead + std::marker::Unpin> Decoder<R> {
    /// Reads a new frame from the internal reader. Returns a [`Frame`](Frame)
    /// if one was found, or, otherwise, an `Err` explaining why not.
    pub async fn next_frame_future(&mut self) -> Result<Frame, Error> {
        loop {
            // Keep our buffers full
            let bytes_read = if self.buffer.len() < REFILL_TRIGGER {
                Some(self.refill_future().await?)
            } else {
                None
            };

            match self.decode_frame() {
                Ok(frame) => return Ok(frame),
                // Don't do anything if we didn't have enough data or we skipped data,
                // just let the loop spin around another time.
                Err(Error::InsufficientData) | Err(Error::SkippedData) => {
                    // If there are no more bytes to be read from the file, return EOF
                    if let Some(0) = bytes_read {
                        return Err(Error::Eof);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    async fn refill_future(&mut self) -> Result<usize, io::Error> {
        use tokio::io::AsyncReadExt;

        let read_bytes = self.reader.read(&mut self.buffer_refill[..]).await?;
        self.buffer.extend(self.buffer_refill[..read_bytes].iter());

        Ok(read_bytes)
    }
}

// TODO FIXME do something about the code repetition. The only difference is the
//  use of .await after IO reads...

impl<R: io::Read> Decoder<R> {
    /// Reads a new frame from the internal reader. Returns a [`Frame`](Frame)
    /// if one was found, or, otherwise, an `Err` explaining why not.
    pub fn next_frame(&mut self) -> Result<Frame, Error> {
        loop {
            // Keep our buffers full
            let bytes_read = if self.buffer.len() < REFILL_TRIGGER {
                Some(self.refill()?)
            } else {
                None
            };

            match self.decode_frame() {
                Ok(frame) => return Ok(frame),
                // Don't do anything if we didn't have enough data or we skipped data,
                // just let the loop spin around another time.
                Err(Error::InsufficientData) | Err(Error::SkippedData) => {
                    // If there are no more bytes to be read from the file, return EOF
                    if let Some(0) = bytes_read {
                        return Err(Error::Eof);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn refill(&mut self) -> Result<usize, io::Error> {
        let read_bytes = self.reader.read(&mut self.buffer_refill[..])?;
        self.buffer.extend(self.buffer_refill[..read_bytes].iter());

        Ok(read_bytes)
    }
}

unsafe extern "C" fn read_callback<R>(buf: *mut c_void, size: u64, user_data: *mut c_void) -> u64
where
    R: Read,
{
    // Not sure how to safely panic from within callback
    let reader = &mut *(user_data as *mut R);
    let buf = std::slice::from_raw_parts_mut(buf as *mut u8, size as usize);
    let mut position = 0;
    // Mimic fread call where we return
    // -1 for error
    // size for not end of stream/file
    // 0 or less than size for end of stream/file
    while position < size as usize {
        match reader.read(&mut buf[position..]) {
            Ok(n) if n == 0 => return position as u64,
            Ok(n) => position += n,
            // -1
            Err(_) => return std::u64::MAX,
        }
    }
    position as u64
}

unsafe extern "C" fn seek_callback<S>(position: u64, user_data: *mut c_void) -> c_int
where
    S: Seek,
{
    use std::io::SeekFrom;
    let seeker = &mut *(user_data as *mut S);
    match seeker.seek(SeekFrom::Start(position)) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

// Need to box this to avoid pointers being invalidated due to movement
struct Mp3dec<R> {
    reader: R,
    io: ffi::mp3dec_io_t,
    ex: ffi::mp3dec_ex_t,
}

/// A sample level Seekable MP3 decoder which consumes a reader and produces samples.
///
/// Unlike `Decoder` this requires `Seek` + `Read`. Also when possible, depending on the mp3 encoder this will trim of samples that were added as part of the encoding process.
pub struct SeekDecoder<R> {
    decoder: Box<Mp3dec<R>>,
}

// Explicitly impl [Send] for [SeekDecoder]. This isn't a great idea and should
// probably be removed in the future. However we need raw pointers
unsafe impl<R: Send> Send for SeekDecoder<R> {}

impl<R> SeekDecoder<R>
where
    R: Read + Seek,
{
    /// Creates a new `SeekDecoder`, consuming the `reader`.
    pub fn new(reader: R) -> Result<SeekDecoder<R>, Error> {
        let mut minidec = Box::new(Mp3dec {
            reader,
            io: unsafe { mem::zeroed() },
            ex: unsafe { mem::zeroed() },
        });
        
        // can only set the io fields here as the memory location of the 
        // reader must stay constant (which the Box::new takes care of)
        minidec.io.read = Some(read_callback::<R>); 
        minidec.io.seek = Some(seek_callback::<R>);
        // data needed by the callbacks set above, passed as C void pointer
        minidec.io.read_data = &mut minidec.reader as * mut _ as *mut c_void;
        minidec.io.seek_data = &mut minidec.reader as * mut _ as *mut c_void;
        
        // open the reader
        let res = unsafe {
            ffi::mp3dec_ex_open_cb(
                &mut minidec.ex,
                &mut minidec.io,
                ffi::MP3D_SEEK_TO_SAMPLE as i32,
            )
        };
        from_mini_error(res)?;
        
        Ok(SeekDecoder {
            decoder: minidec,
        })
    }

    pub fn decode_frame(&mut self) -> Result<Frame, Error> {
        let mut frame_info = unsafe { mem::zeroed() };
        let mut buffer = std::ptr::null_mut();
        let samples: u64 = unsafe {
            ffi::mp3dec_ex_read_frame(
                &mut self.decoder.ex,
                &mut buffer, // seems to allocate its own memory.....
                &mut frame_info,
                MAX_SAMPLES_PER_FRAME as u64,
            )
        };

        let len = samples as usize;
        let buffer = unsafe { std::slice::from_raw_parts(buffer, len)};
        let buffer = buffer.to_owned();

        let frame = Frame {
            data: buffer,
            sample_rate: frame_info.hz,
            channels: frame_info.channels as usize,
            layer: frame_info.layer as usize,
            bitrate: frame_info.bitrate_kbps,
        };

        if samples == 0 {
            if frame_info.frame_bytes > 0 {
                Err(Error::SkippedData)
            } else {
                Err(Error::InsufficientData)
            }
        } else {
            Ok(frame)
        }
    }

    /// This mp3s sample rate in hertz, when using read_samples or read_sample_slice this can
    /// for every sample returned
    pub fn current_sample_rate(&self) -> i32 {
        self.decoder.ex.info.hz
    }
    /// The number of channels in this mp3, when using read_samples or read_sample_slice this can
    /// for every sample returned
    pub fn _current_channels(&self) -> usize {
        self.decoder.ex.info.channels as usize
    }

    /// Returns the number of samples that were set
    /// Will be zero at end of stream
    pub fn read_samples(&mut self, buf: &mut [i16]) -> Result<usize, Error> {
        let len = unsafe {
            ffi::mp3dec_ex_read(&mut self.decoder.ex, buf.as_mut_ptr(), buf.len() as u64) as usize
        };

        if len == buf.len() {
            Ok(len)
        } else if len < buf.len() {
            // Check for error
            from_mini_error(self.decoder.ex.last_error)?;
            // Must be end of stream
            Ok(len)
        } else {
            panic!("Minimp3 returned invalid read result. Likely corrupt memory")
        }
    }

    /// Convenience wrapper around `read_samples` to use with a while let loop
    /// Returns None when out of samples
    /// Returns the slice of newly assigned samples otherwise
    pub fn read_sample_slice<'a>(
        &mut self,
        buf: &'a mut [i16],
    ) -> Result<Option<&'a mut [i16]>, Error> {
        let len = self.read_samples(buf)?;
        Ok(if len == 0 {
            None
        } else {
            Some(&mut buf[..len])
        })
    }

    /// Seek to the given sample index
    pub fn seek_samples(&mut self, sample: u64) -> Result<(), Error> {
        let res = unsafe { ffi::mp3dec_ex_seek(&mut self.decoder.ex, sample) };
        from_mini_error(res)
    }

    /// Destroy the decoder and return the inner reader
    pub fn into_inner(self) -> R {
        self.decoder.reader
    }
}
