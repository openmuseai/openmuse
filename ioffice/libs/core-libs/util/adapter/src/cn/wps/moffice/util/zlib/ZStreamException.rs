// Java: import java.io.IOException

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 1;

// Java: public class ZStreamException extends IOException
#[derive(Debug)]
pub struct ZStreamException {
    pub(crate) message: Option<String>,
}

// Java: public class ZStreamException extends IOException
impl ZStreamException {
    // Java: ZStreamException()
    pub fn new() -> Self {
      // Java: super();
      ZStreamException { message: None }
    }

    // Java: ZStreamException(String s)
    pub fn new_with_message(_s: &str) -> Self {
      // Java: super(s);
      ZStreamException { message: Some(_s.to_string()) }
    }

}

impl std::fmt::Display for ZStreamException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "ZStreamException: {}", msg),
            None => write!(f, "ZStreamException"),
        }
    }
}

impl std::error::Error for ZStreamException {}

