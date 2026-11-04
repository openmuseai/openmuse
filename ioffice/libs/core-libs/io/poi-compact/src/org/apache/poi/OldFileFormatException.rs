// Java: public class OldFileFormatException extends IllegalArgumentException
pub struct OldFileFormatException {
    pub message: String,
}

// Java: public class OldFileFormatException extends IllegalArgumentException
impl OldFileFormatException {
    // Java: OldFileFormatException(String s)
    pub fn new(s: &str) -> Self {
        // Java: super(s);
        OldFileFormatException { message: s.to_string() }
    }
}

impl std::fmt::Display for OldFileFormatException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
