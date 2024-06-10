
#[cfg(test)]
mod tests {
    use tokio::{fs::File, io::{BufReader, BufWriter}};
    use eyre::{Ok,Result};
    use veilid_storage_piper::processors::compressor_zstd::CompressorZstd;

    #[tokio::test]
    async fn create_compress_zstd() -> Result<()>{
        let file_name = "./tests/av1-test-video.webm";
        // Create a bufstream from an input file
        let input_file = File::open(file_name).await?;
        let input_buf = BufReader::new(input_file);
        let output_file = File::create(format!("{}.zstd", file_name)).await?;
        let output_buf = BufWriter::new(output_file);

        // Initialize the stream zstd compressor
        let mut compressor_zstd = CompressorZstd::new(22, input_buf, output_buf);

        // Start processing
        compressor_zstd.process().await?;

        Ok(())
    }
}