pub mod brotli;

// Create a global variable to store the processor object
// static PROCESSORS: Lazy<Arc<AsyncMutex<Option<None>>>> = Lazy::new(|| Arc::new(AsyncMutex::new(None)));

pub trait Processor<R, W> {
    fn new(input: R, output: W) -> Self;
    fn modulate(&mut self);
    fn demodulate(&mut self);
}
