use brotli::{enc::BrotliEncoderParams, CompressorWriter};
use std::io::{Read, Write};

pub struct Brotli<R: Read, W: Write> {
    pub input: R,
    pub output: CompressorWriter<W>,
}

impl<R: Read, W: Write> Brotli<R, W> {
    pub fn new(input: R, output: W) -> Self {
        // Modify params to fit the application needs
        let mut brotli_encoder_params = BrotliEncoderParams::default();
        // Level is between 0-11, we always set it to 11, as it's minimal overhead for us
        brotli_encoder_params.quality = 11;

        // Create a compression output writer for streaming - 4096 bytes buffer
        let comp_out_writer = CompressorWriter::with_params(output, 4096, &brotli_encoder_params);

        // Return the comp_out_writer as output
        Self {
            input,
            output: comp_out_writer,
        }
    }
}
