//! Native transport for Java new RuntimeException(IOException): retain the cause.
#[derive(Debug)]
pub struct RuntimeException(pub std::io::Error);
impl RuntimeException {
    pub fn throw(cause: std::io::Error) -> ! { std::panic::panic_any(Self(cause)); }
}
