#[cfg(test)]
mod tests {
    use eyre::{Ok, Result};

    use veilid_storage_piper::processors::brotli::Brotli;

    use std::{
        fs::File,
        io::{copy, BufReader, BufWriter},
    };

    #[cfg(target_arch = "wasm32")]
    use async_std::test;
    #[cfg(not(target_arch = "wasm32"))]
    use tokio::test;

    static VIDEO_FILE: &str = "./tests/av1-test-video.webm";
    static TEXT_FILE: &str = "./tests/text-file-random.txt";

    #[test]
    async fn test_video() -> Result<()> {
        compress(VIDEO_FILE).await?; // Run the compression test first
        decompress(&format!("{}.br", VIDEO_FILE)).await?;
        // TODO: Compare the hashes of the input and .br.decomp files
        Ok(())
    }

    #[test]
    async fn test_text_decompression() -> Result<()> {
        compress(TEXT_FILE).await?; // Run the compression test first
        decompress(&format!("{}.br", TEXT_FILE)).await?;
        Ok(())
    }

    async fn compress(file_name: &str) -> Result<()> {
        // Create a bufstream from an input file
        let input_file = File::open(file_name)?;
        let mut input_buf = BufReader::new(input_file);
        let output_file = File::create(format!("{}.br", file_name))?;
        let mut output_buf = BufWriter::new(output_file);

        // Initialize the stream brotli compressor - maximum compression
        let mut compressor_brotli = Brotli::compress(&mut input_buf, &mut output_buf);

        // Start processing - copy the streams
        copy(&mut compressor_brotli.input, &mut compressor_brotli.output)?;

        Ok(())
    }

    async fn decompress(file_name: &str) -> Result<()> {
        // Create a bufstream from an input file
        let input_file = File::open(file_name)?;
        let mut input_buf = BufReader::new(input_file);
        let output_file = File::create(format!("{}.decomp", file_name))?;
        let mut output_buf = BufWriter::new(output_file);

        // Initialize the stream brotli decompressor
        let mut compressor_brotli = Brotli::decompress(&mut input_buf, &mut output_buf);

        // Start processing - copy the streams
        copy(&mut compressor_brotli.input, &mut compressor_brotli.output)?;

        Ok(())
    }
}
