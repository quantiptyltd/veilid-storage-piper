struct blob<B> {
    inner: B,
    blob_name: String,
}

impl<B> blob<B> {
    pub fn new(inner: B, blob_name: String) -> Self {
        blob {
            inner,
            blob_name,
        }
    }
}

#[cfg(test)]
mod blob_master {
    use super::*;

    #[test]
    fn create_stream_from_file() {
        
    }
}
