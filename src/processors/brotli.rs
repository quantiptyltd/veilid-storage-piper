use brotli::{enc::BrotliEncoderParams, BrotliCompress, BrotliDecompress};
use std::io::{Read, Write};

pub struct Brotli<'a, R: Read, W: Write> {
    pub input: &'a mut R,
    pub output: &'a mut W,
}

impl<'a, R: Read, W: Write> Brotli<'a, R, W> {
    pub fn compress(input: &'a mut R, output: &'a mut W) -> Self {
        // Modify params to fit the application needs
        let mut brotli_encoder_params = BrotliEncoderParams::default();
        // Level is between 0-11, we always set it to 11, as it's minimal overhead for us
        brotli_encoder_params.quality = 11;

        // Mutate the streams with compression
        if let Err(e) = BrotliCompress(input, output, &brotli_encoder_params) {
            eprintln!("Error during brotli compression: {:?}", e);
        }

        // Return self
        Self { input, output }
    }
    pub fn decompress(input: &'a mut R, output: &'a mut W) -> Self {
        // Mutate the streams with compression
        if let Err(e) = BrotliDecompress(input, output) {
            eprintln!("Error during brotli decompression: {:?}", e);
        }

        // Return self
        Self { input, output }
    }
}
