use std::io::{Read, Write};

pub struct Smaz<'a, 'b, R: Read, W: Write> {
    pub input: &'a mut R,
    pub output: &'b mut W,
}

impl<'a, 'b, R: Read, W: Write> Smaz<'a, 'b, R, W> {
    pub fn compress(input: &'a mut R, output: &'b mut W) -> Self {
        // Mutate the streams with SMAZ compression
        // Callback for every buffer read
        // output.by_ref().write_fmt(fmt)
        Self { input, output }
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
