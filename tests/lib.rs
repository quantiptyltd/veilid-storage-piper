#[cfg(test)]
mod tests {
    use eyre::{Ok, Result};
    use std::{
        fs::File,
        io::{BufReader, BufWriter},
    };

    use veilid_storage_piper::processors::compressor_zstd::CompressorZstd;

    #[test]
    fn test_video_compression() -> Result<()> {
        compress("./tests/av1-test-video.webm")?;
        Ok(())
    }

    #[test]
    fn test_text_compression() -> Result<()> {
        compress("./tests/text-file-random.txt")?;
        Ok(())
    }

    fn compress(file_name: &str) -> Result<()> {
        // Create a bufstream from an input file
        let input_file = File::open(file_name)?;
        let input_buf = BufReader::new(input_file);
        let output_file = File::create(format!("{}.zstd", file_name))?;
        let output_buf = BufWriter::new(output_file);

        // Initialize the stream zstd compressor
        let mut compressor_zstd = CompressorZstd::new(input_buf, output_buf, 22);

        // Start processing
        compressor_zstd.process()?;

        Ok(())
    }
}
