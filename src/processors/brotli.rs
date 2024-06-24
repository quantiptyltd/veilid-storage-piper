use crate::Processor;
use brotli::{enc::BrotliEncoderParams, BrotliCompress, BrotliDecompress};
use eyre::{Ok, Result};
use std::io::{Read, Seek, Write};

pub struct Brotli<'a, R, W> {
    input: &'a mut R,
    output: &'a mut W,
}

// Implement a public method for Brotli
pub fn should_process<R: Read + Seek>(input: &mut R) -> Result<bool> {
    let mut buffer = Vec::new();
    // Read 1024 bytes off the file
    input.take(1024).read_to_end(&mut buffer)?;
    // Rewind to be consumed for later
    input.rewind()?;
    // If the input is a video, we don't want to process it
    if infer::is_video(&buffer) {
        return Ok(false);
    }
    Ok(true)
}

// Implement the Processor trait for Brotli
impl<'a, R: Read, W: Write> Processor<'a, R, W> for Brotli<'a, R, W> {
    fn new(input: &'a mut R, output: &'a mut W) -> Self {
        Self { input, output }
    }

    fn modulate(&mut self) -> Result<()> {
        // Modify params to fit the application needs
        let mut brotli_encoder_params = BrotliEncoderParams::default();
        // Level is between 0-11, we always set it to 11, as it's minimal overhead for us
        brotli_encoder_params.quality = 11;

        // Mutate the streams with compression
        BrotliCompress(&mut self.input, &mut self.output, &brotli_encoder_params)?;
        Ok(())
    }

    fn demodulate(&mut self) -> Result<()> {
        // Mutate the streams with compression
        BrotliDecompress(&mut self.input, &mut self.output)?;
        Ok(())
    }
}
