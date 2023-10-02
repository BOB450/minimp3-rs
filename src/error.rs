/// Errors encountered by the MP3 decoder.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error caused by some IO operation required during decoding.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// The decoder tried to parse a frame from its internal buffer, but there
    /// was not enough.
    #[error("Insufficient data")]
    InsufficientData,
    /// The decoder encountered data which was not a frame (ie, ID3 data), and
    /// skipped it.
    #[error("Skipped data")]
    SkippedData,
    /// The decoder has reached the end of the provided reader.
    #[error("End of reader")]
    Eof,
    /// Minimp3 had a memory error, likely allocation
    #[error("Minimp3 memory error")]
    MiniMemory,
    /// Minimp3 had an IO error
    #[error("Minimp3 io error")]
    MiniIo,
    /// Minimp3 had a parameter error
    #[error("Minimp3 parameter error")]
    MiniParam,
    /// Minimp3 had an user error
    #[error("Minimp3 user error")]
    MiniUser,
    /// Minimp3 had an decoder error, likely the sampling rate/channels/layer changed mid stream
    #[error("Minimp3 decode error")]
    MiniDecode,
    /// Minimp3 returned an unknown error code
    #[error("Minimp3 unknown error")]
    MiniUnknown,
}

pub fn from_mini_error(ec: i32) -> Result<(), Error> {
    match ec {
        0 => Ok(()),
        crate::ffi::MP3D_E_MEMORY => Err(Error::MiniMemory),
        crate::ffi::MP3D_E_IOERROR => Err(Error::MiniIo),
        crate::ffi::MP3D_E_PARAM => Err(Error::MiniParam),
        crate::ffi::MP3D_E_USER => Err(Error::MiniUser),
        crate::ffi::MP3D_E_DECODE => Err(Error::MiniDecode),
        _ => Err(Error::MiniUnknown),
    }
}
