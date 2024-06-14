use brotli::enc::BrotliEncoderParams;
use brotli::BrotliCompress;
use eyre::{Ok, Result};

use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Debug)]
pub struct Brotli<R: Read, W: Write> {
    level: u8,
    input_buf: BufReader<R>,
    output_buf: BufWriter<W>,
}

impl<R: Read, W: Write> Brotli<R, W> {
    pub fn new(input_buf: BufReader<R>, output_buf: BufWriter<W>, level: u8) -> Self {
        Self {
            level,
            input_buf,
            output_buf,
        }
    }

    pub async fn compress(&mut self) -> Result<()> {
        // Modify params to fit the application needs
        let mut brotli_encoder_params = BrotliEncoderParams::default();
        // Level is between 0-11
        brotli_encoder_params.quality = self.level as i32;

        // Use Stream Copy abstraction to compress the input stream and write to the output stream
        BrotliCompress(
            &mut self.input_buf,
            &mut self.output_buf,
            &brotli_encoder_params,
        )?;

        Ok(())
    }
}
