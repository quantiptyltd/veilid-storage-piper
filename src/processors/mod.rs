use std::io::{Read, Write};
use brotli::Brotli;
use eyre::{Ok, Result};
use smaz::Smaz;
use once_cell::sync::Lazy;
use std::sync::Arc;

pub mod brotli;
pub mod smaz;

#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::Mutex as AsyncMutex;
#[cfg(target_arch = "wasm32")]
use tokio_with_wasm::tokio::sync::Mutex as AsyncMutex;

// Create a global variable to store the processor object
// static PROCESSORS: Lazy<Arc<AsyncMutex<Vec<dyn Processor>>>> = Lazy::new(|| Lazy::new);

pub trait Processor<R, W> {
    fn new(input: R, output: W) -> Self;
    fn modulate(&mut self);
    fn demodulate(&mut self);
}

// Public function that generates a Vector of processors
pub fn build_processors<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    let mut processors : Vec<Processor> = Vec::new();
    // Check if the input is to be processed by smaz
    if Smaz::<R, W>::should_process(input)? {
        processors.push(Smaz::new(input, output));
    }
    // Check if the input should be processed by brotli
    if Brotli::<R, W>::should_process(input) {
        processors.push(Brotli::new(input, output));
    }

    Ok(())
}