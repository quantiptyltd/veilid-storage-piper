pub mod processors;

use eyre::Result;
use processors::brotli::{self, Brotli};
use processors::smaz::{self, Smaz};
use std::io::{copy, Read, Seek, Write};

pub trait Processor<'a, R, W> {
    fn new(input: &'a mut R, output: &'a mut W) -> Self;
    fn modulate(&mut self) -> Result<()>;
    fn demodulate(&mut self) -> Result<()>;
}

pub enum ProcessorType {
    Smaz,
    Brotli,
}

// Private function that generates a Vector of processors
fn build_processors<R: Read + Seek>(input: &mut R) -> Result<Vec<ProcessorType>> {
    let mut processors: Vec<ProcessorType> = Vec::new();
    // Check if the input is to be processed by smaz
    if smaz::should_process(input)? {
        processors.push(ProcessorType::Smaz);
    }
    // Check if the input should be processed by brotli
    if brotli::should_process(input)? {
        processors.push(ProcessorType::Brotli);
    }

    Ok(processors)
}

pub fn upload<R: Read + Seek, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    let processors = build_processors(input)?;

    for processor in processors {
        match processor {
            ProcessorType::Smaz => {
                Smaz::new(input, output).modulate()?;
            }
            ProcessorType::Brotli => {
                Brotli::new(input, output).modulate()?;
            }
        }
    }

    // Without copy brotli compression wouldn't be triggered
    copy(input, output)?;

    Ok(())
}
