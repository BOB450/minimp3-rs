use minimp3_fixed::{Error, Frame, SeekDecoder};

use std::fs::File;
use std::io::{Read, Seek};

fn count_frames(decoder: &mut SeekDecoder<impl Read + Seek>) -> usize {
    let mut samples = 0;
    loop {
        match decoder.decode_frame() {
            Ok(Frame { data, channels, .. }) => {
                samples += data.len() / channels;
            }
            Err(Error::Eof) => break,
            Err(Error::InsufficientData) => break,
            Err(e) => panic!("{:?}", e),
        }
    }
    samples
}

fn main() {
    let mut decoder = SeekDecoder::new(
        File::open("minimp3-sys/minimp3/vectors/M2L3_bitrate_24_all.bit").unwrap(),
    )
    .unwrap();

    let before_seek = count_frames(&mut decoder);

    decoder.seek_samples(100).unwrap();
    let after_seek = count_frames(&mut decoder);

    assert_eq!(after_seek, before_seek - 100);
}
