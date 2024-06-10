struct compress_brotli<S> {
    inner: S,
    compression_level: u8,
}

impl<S> compress_brotli<S> {
    pub fn new(inner: S, compression_level: u8) -> Self {
        compress_brotli {
            inner,
            compression_level,
        }
    }
}
