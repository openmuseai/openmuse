// Java: import org.xml.sax.SAXException

// Java: public class DocumentEndOfParseException extends SAXException
pub struct DocumentEndOfParseException {
    message: Option<String>,
}

// Java: public class DocumentEndOfParseException extends SAXException
impl DocumentEndOfParseException {
    // Java: DocumentEndOfParseException()
    pub fn new() -> Self {
      // Java: super();
      DocumentEndOfParseException { message: None }
    }

    // Java: DocumentEndOfParseException(String message)
    pub fn new_message(message: &str) -> Self {
      // Java: super(message);
      DocumentEndOfParseException { message: Some(message.to_string()) }
    }

    // Java: DocumentEndOfParseException(Exception e)
    pub fn new_exception(_e: &str) -> Self {
      // Java: super();
      DocumentEndOfParseException { message: None }
    }

    // Java: DocumentEndOfParseException(String message, Exception e)
    pub fn new_message_exception(message: &str, _e: &str) -> Self {
      // Java: super(message, e);
      DocumentEndOfParseException { message: Some(message.to_string()) }
    }

}
