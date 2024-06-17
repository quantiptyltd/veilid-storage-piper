use std::io::{Read, Write};
use eyre::{Result, Ok};

pub struct Smaz<'a, 'b, R: Read, W: Write> {
    pub input: &'a mut R,
    pub output: &'b mut W,
}

impl<'a, 'b, R: Read, W: Write> Smaz<'a, 'b, R, W> {
    pub fn compress(input: &'a mut R, output: &'b mut W) -> Result<()> {
        // Mutate the streams with SMAZ compression
        let mut smazzed_output = Vec::new();
        input.read_to_end(&mut smazzed_output)?;
        smazzed_output = fast_smaz::compress(&smazzed_output);
        output.write_all(&smazzed_output)?;
        Ok(())
    }

    pub fn decompress(input: &'a mut R, output: &'b mut W) -> Self {
        // Mutate the streams with SMAZ compression
        // if let Err(e) = smaz::decompress(input, output) {
        //     eprintln!("Error during smaz decompression: {:?}", e);
        // }

        // Return self
        Self { input, output }
    }
}
