use std::io::{BufReader, BufWriter};

use eyre::{Ok, Result};
use zstd::stream::copy_encode;

#[derive(Debug)]
pub struct CompressorZstd<R> {
    level: i32,
    input_buf: BufReader<R>,
    output_buf: BufWriter<W>,
}

impl<R, W, D> CompressorZstd<R, W, D> {
    pub fn new(level: i32, input_buf: Reader<R, D>, output_buf: Writer<W, D>) -> Self {
        Self {
            level,
            input_buf,
            output_buf,
        }
    }
    pub async fn process(&mut self) -> Result<()> {
        copy_encode(self.input_buf.reader().into(), self.output_buf, self.level);
        Ok(())
    }
}
