#[cfg(test)]
mod tests {
    use eyre::{Ok, Result};

    use veilid_storage_piper::processors::brotli::Brotli;

    use std::{
        fs::File,
        io::{BufReader, BufWriter},
    };

    #[cfg(target_arch = "wasm32")]
    use async_std::test;
    #[cfg(not(target_arch = "wasm32"))]
    use tokio::test;

    #[test]
    async fn test_video_compression() -> Result<()> {
        compress("./tests/av1-test-video.webm").await?;
        Ok(())
    }

    #[test]
    async fn test_text_compression() -> Result<()> {
        compress("./tests/text-file-random.txt").await?;
        Ok(())
    }

    async fn compress(file_name: &str) -> Result<()> {
        // Create a bufstream from an input file
        let input_file = File::open(file_name)?;
        let input_buf = BufReader::new(input_file);
        let output_file = File::create(format!("{}.br", file_name))?;
        let output_buf = BufWriter::new(output_file);

        // Initialize the stream brotli compressor
        let mut compressor_brotli = Brotli::new(input_buf, output_buf, 11);

        // Start processing
        compressor_brotli.compress().await?;

        Ok(())
    }
}
