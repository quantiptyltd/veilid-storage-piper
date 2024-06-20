use brotli::{enc::BrotliEncoderParams, BrotliCompress, BrotliDecompress};
use eyre::{Ok, Result};
use std::io::{BufReader, Read, Write};

use super::Processor;

pub struct Smaz<R, W> {
    input: R,
    output: W,
}

// Implement a public method for Smaz
impl<R: Read, W: Write> Smaz<R, W> {
    pub fn should_process(input: &mut R) -> Result<bool> {
        // Read the contents of the input into a buffer
        let mut buffer = String::new();
        input.read_to_string(&mut buffer)?;
        // If it's a large string no need to Smaz
        if buffer.len() > 1024 {
            return Ok(false);
        }
        // Otherwise, return true
        Ok(true)
    }
}

// Implement the Processor trait for Brotli
impl<R: Read, W: Write> Processor<R, W> for Smaz<R, W> {
    fn new(input: R, output: W) -> Self {
        Self { input, output }
    }

    fn modulate(&mut self) {
        unimplemented!()
    }

    fn demodulate(&mut self) {
        unimplemented!()
    }
}
