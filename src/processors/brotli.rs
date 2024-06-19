use brotli::{enc::BrotliEncoderParams, BrotliCompress, BrotliDecompress};
use std::io::{BufReader, Read, Write};

use super::Processor;

pub struct Brotli<R, W> {
    input: R,
    output: W,
}

// Implement a public method for Brotli
impl<R: Read, W: Write> Brotli<R, W> {
    pub fn should_process(input: &mut R) -> bool {
        // Check using infer crate if the file type is video
        // If yes, then return false, as we don't want to process video files
        let kind = infer::get(BufReader::new(input).buffer()).expect("file type is known");
        if kind.mime_type().contains("video") {
            return false;
        }
        // Otherwise, return true
        true
    }
}

// Implement the Processor trait for Brotli
impl<R: Read, W: Write> Processor<R, W> for Brotli<R, W> {
    fn new(input: R, output: W) -> Self {
        Self { input, output }
    }

    fn modulate(&mut self) {
        // Modify params to fit the application needs
        let mut brotli_encoder_params = BrotliEncoderParams::default();
        // Level is between 0-11, we always set it to 11, as it's minimal overhead for us
        brotli_encoder_params.quality = 11;

        // Mutate the streams with compression
        if let Err(e) = BrotliCompress(&mut self.input, &mut self.output, &brotli_encoder_params) {
            eprintln!("Error during brotli compression: {:?}", e);
        }
    }

    fn demodulate(&mut self) {
        // Mutate the streams with compression
        if let Err(e) = BrotliDecompress(&mut self.input, &mut self.output) {
            eprintln!("Error during brotli decompression: {:?}", e);
        }
    }
}
