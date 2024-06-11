use std::{io::{self, BufReader, BufWriter, Read, Write}, thread::available_parallelism};

use eyre::{Ok, Result};

#[derive(Debug)]
pub struct CompressorZstd<R: Read, W : Write> {
    level: i32,
    input_buf: BufReader<R>,
    output_buf: BufWriter<W>,
}

impl<R: Read, W : Write> CompressorZstd<R, W> {
    pub fn new(input_buf: BufReader<R>, output_buf: BufWriter<W>, level: i32) -> Self {
        Self {
            level,
            input_buf,
            output_buf,
        }
    }

    pub fn process(&mut self) -> Result<()> {
        // Get the no of threads, for us to parallelize
        let n_workers = available_parallelism()?.get().try_into()?;
        let mut encoder = zstd::stream::Encoder::new(&mut self.output_buf, self.level)?;
        // Enable multithreading
        encoder.multithread(n_workers)?;
        // Start the compression
        io::copy(&mut self.input_buf, &mut encoder.auto_finish())?;
        Ok(())
    }
}