use crate::Processor;
use eyre::{Ok, Result};
use fast_smaz::{compress, decompress};
use std::io::{Read, Seek, Write};

pub struct Smaz<'a, R, W> {
    input: &'a mut R,
    output: &'a mut W,
}

// Implement a public method for Smaz
pub fn should_process<R: Read + Seek>(input: &mut R) -> Result<bool> {
    // Read the contents of the input into a buffer
    let mut buffer = Vec::new();
    input.take(1024).read_to_end(&mut buffer)?;
    // Rewind to be consumed for later
    input.rewind()?;
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
        // Read the contents of the input into a buffer
        let mut buffer_string = String::new();
        self.input.read_to_string(&mut buffer_string)?;
        // Compress the buffer
        let compressed = compress(&buffer_string);
        // Write the compressed buffer to the output stream
        self.output.write_all(&compressed)?;

        Ok(())
    }

    fn demodulate(&mut self) -> Result<()> {
        // Read the contents of the input into a buffer
        let mut buffer_string = String::new();
        self.input.read_to_string(&mut buffer_string)?;
        // Compress the buffer
        let decompressed = decompress(&buffer_string)?;
        // Write the compressed buffer to the output stream
        self.output.write_all(&decompressed)?;

        Ok(())
    }
}
