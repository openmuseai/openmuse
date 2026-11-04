#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! Soft WriterAutoWriteException (IOException stub lacks Default/new).

// java private static final long serialVersionUID = 2L;
const serialVersionUID: i64 = 2;

// java public class WriterAutoWriteException extends IOException
#[derive(Default, Debug)]
pub struct WriterAutoWriteException {
    pub(crate) message: Option<String>,
}

// java public class WriterAutoWriteException extends IOException
impl WriterAutoWriteException {
    // java public  WriterAutoWriteException()
    pub fn new() -> Self {
        Self::default()
    }

    // java public  WriterAutoWriteException(String message)
    pub fn new__string(message: &str) -> Self {
        // java super(message);
        Self {
            message: Some(message.to_string()),
        }
    }
}
