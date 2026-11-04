// Java: public enum Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Uof,
}

// Java: public enum Mode
impl Mode {
    // Java: Mode()
    pub fn new() -> Self {
        Mode::Normal
    }
}
