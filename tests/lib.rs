#[cfg(test)]
mod tests {
    use eyre::{Ok, Result};
    use veilid_storage_piper::upload;
    use std::{
        fs::File,
        io::{BufReader, BufWriter},
    };

    #[cfg(target_arch = "wasm32")]
    use async_std::test;
    #[cfg(not(target_arch = "wasm32"))]
    use tokio::test;

    static VIDEO_FILE: &str = "./tests/av1-test-video.webm";
    static TEXT_FILE: &str = "./tests/text-file-random.txt";
    static SMALL_TEXT_FILE: &str = "./tests/text-file-small.txt";

    #[test]
    async fn test_video() -> Result<()> {
        compress(VIDEO_FILE).await?; // Run the compression test first
        // decompress(&format!("{}.br", VIDEO_FILE)).await?;
        // TODO: Compare the hashes of the input and .br.decomp files
        Ok(())
    }

    #[test]
    async fn test_text() -> Result<()> {
        compress(TEXT_FILE).await?; // Run the compression test first
        // decompress(&format!("{}.br", TEXT_FILE)).await?;
        // TODO: Compare the hashes of the input and .br.decomp files
        Ok(())
    }

    #[test]
    async fn test_small_text() -> Result<()> {
        compress(SMALL_TEXT_FILE).await?; // Run the compression test first
        // decompress(&format!("{}.br", TEXT_FILE)).await?;
        // TODO: Compare the hashes of the input and .br.decomp files
        Ok(())
    }

    async fn compress(file_name: &str) -> Result<()> {
        // Create a bufstream from an input file
        let input_file = File::open(file_name)?;
        let mut input_buf = BufReader::new(input_file);
        let output_file = File::create(format!("{}.comp", file_name))?;
        let mut output_buf = BufWriter::new(output_file);

        // Initialize the stream brotli compressor - maximum compression level of 11
        upload(&mut input_buf, &mut output_buf)?;

        Ok(())
    }

    async fn decompress(file_name: &str) -> Result<()> {
        // // Create a bufstream from an input file
        // let input_file = File::open(file_name)?;
        // let mut input_buf = BufReader::new(input_file);
        // let output_file = File::create(format!("{}.decomp", file_name))?;
        // let mut output_buf = BufWriter::new(output_file);

        // // Initialize the stream brotli decompressor
        // let mut processor = Brotli::new(&mut input_buf, &mut output_buf);
        // processor.demodulate();

        // // Start processing - copy the streams
        // copy(&mut input_buf, &mut output_buf)?;

        Ok(())
    }
}
