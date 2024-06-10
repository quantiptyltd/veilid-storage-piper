use eyre::{Ok, Result};
use tokio::io::{BufReader, BufStream};

struct blob<B> {
    inner: B,
    input_stream: BufStream<BufReader<B>>,
}

impl<B> blob<B> {
    pub fn new(inner: B, input_stream: BufStream<BufReader<B>>) -> Self {
        blob {
            inner,
            input_stream,
        }
    }

    pub fn process_stream() -> Result<()> {
        Ok(())
    }
}

// Unit tests for this module

#[cfg(test)]
mod blob_master {
    use super::*;

    #[test]
    fn create_stream_from_file() {}
}
