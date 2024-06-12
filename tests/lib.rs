#[cfg(test)]
mod tests {
    use eyre::{Ok, Result};
    use std::{
        fs::File,
        io::{BufReader, BufWriter},
    };
    
    use veilid_storage_piper::processors::compressor_zstd::CompressorZstd;
    
    #[cfg(not(target_arch = "wasm32"))]
    use tokio;
    #[cfg(target_arch = "wasm32")]
    use tokio_with_wasm::tokio;

    #[tokio::test]
    async fn test_video_compression() -> Result<()> {
        compress("./tests/av1-test-video.webm").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_text_compression() -> Result<()> {
        compress("./tests/text-file-random.txt").await?;
        Ok(())
    }

    async fn compress(file_name: &str) -> Result<()> {
        // Create a bufstream from an input file
        let input_file = File::open(file_name)?;
        let input_buf = BufReader::new(input_file);
        let output_file = File::create(format!("{}.lz4", file_name))?;
        let output_buf = BufWriter::new(output_file);

        // Initialize the stream zstd compressor
        let mut compressor_zstd = CompressorZstd::new(input_buf, output_buf, 22);

        // Start processing
        compressor_zstd.process().await?;

        Ok(())
    }
}
