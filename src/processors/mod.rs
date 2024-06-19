use std::io::{Read, Write};
use brotli::Brotli;

pub mod brotli;

// Create a global variable to store the processor object
// static PROCESSORS: Lazy<Arc<AsyncMutex<Option<None>>>> = Lazy::new(|| Arc::new(AsyncMutex::new(None)));

pub trait Processor<R, W> {
    fn new(input: R, output: W) -> Self;
    fn modulate(&mut self);
    fn demodulate(&mut self);
}

// Public function that generates a Vector of processors
pub fn build_processors<R: Read, W: Write>(input: &mut R, output: &mut W) {
    let mut processors = Vec::new();
    // Check if the input should be processed by brotli
    if Brotli::<R, W>::should_process(input) {
        processors.push(Brotli::new(input, output));
    }
}