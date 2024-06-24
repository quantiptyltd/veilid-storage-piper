use crate::Processor;
use eyre::{Ok, Result};
use std::io::{BufReader, Read, Write};
use fast_smaz::{compress,decompress};

pub struct Smaz<'a, R, W> {
    input: &'a mut R,
    output: &'a mut W,
}

// Implement a public method for Smaz
pub fn should_process<R: Read>(input: &mut R) -> Result<bool> {
    // Read the contents of the input into a buffer
    let mut buffer = Vec::new();
    input.take(1024).read_to_end(&mut buffer)?;
    // Smaz if it's a small string less than 1024 bytes
    if buffer.len() < 1024 {
        return Ok(true);
    }
    // Otherwise, return false
    Ok(false)
}

// Implement the Processor trait for Brotli
impl<'a, R: Read, W: Write> Processor<'a, R, W> for Smaz<'a, R, W> {
    fn new(input: &'a mut R, output: &'a mut W) -> Self {
        Self { input, output }
    }

    fn modulate(&mut self) -> Result<()> {
        
        Ok(())
    }

    fn demodulate(&mut self) -> Result<()> {
        
        Ok(())
    }
}
