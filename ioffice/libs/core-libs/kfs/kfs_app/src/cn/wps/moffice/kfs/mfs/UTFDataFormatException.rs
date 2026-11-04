#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: public class UTFDataFormatException extends IOException
#[derive(Default, Debug, Clone)]
pub struct UTFDataFormatException {
    pub message: String,
}

// Java: public class UTFDataFormatException extends IOException
impl UTFDataFormatException {
    // Java: public UTFDataFormatException()
    pub fn new() -> UTFDataFormatException {
        Self { message: String::new() }
    }

    // Java: public UTFDataFormatException(String detailMessage)
    pub fn new__string(detail_message: String) -> UTFDataFormatException {
        Self { message: detail_message }
    }
}

impl std::fmt::Display for UTFDataFormatException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UTFDataFormatException: {}", self.message)
    }
}
