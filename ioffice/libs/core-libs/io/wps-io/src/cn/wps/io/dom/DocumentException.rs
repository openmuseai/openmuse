use std::fmt;
use std::io::Write;

// Java: public class DocumentException extends Exception
#[derive(Debug)]
pub struct DocumentException {
    pub(crate) message: String,
    // Java field: Throwable nestedException
    pub(crate) nestedException: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl fmt::Display for DocumentException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.getMessage())
    }
}

impl std::error::Error for DocumentException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.nestedException.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

// Java: public class DocumentException extends Exception
impl DocumentException {
    // Java: DocumentException()
    pub fn new() -> Self {
        // Java: super("Error occurred in DOM4J application.");
        DocumentException {
            message: "Error occurred in DOM4J application.".to_string(),
            nestedException: None,
        }
    }

    // Java: DocumentException(String message)
    pub fn new_with_message(message: String) -> Self {
        // Java: super(message);
        DocumentException {
            message,
            nestedException: None,
        }
    }

    // Java: DocumentException(Throwable nestedException)
    pub fn new_with_cause(nested_exception: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(nestedException.getMessage());
        let message = nested_exception.to_string();
        DocumentException {
            message,
            // Java: this.nestedException = nestedException;
            nestedException: Some(nested_exception),
        }
    }

    // Java: DocumentException(String message, Throwable nestedException)
    pub fn new_with_message_cause(message: String, nested_exception: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(message);
        DocumentException {
            message,
            // Java: this.nestedException = nestedException;
            nestedException: Some(nested_exception),
        }
    }

    // Java: Throwable getNestedException()
    pub fn getNestedException(&self) -> Option<&(dyn std::error::Error + Send + Sync)> {
        // Java: return this.nestedException;
        self.nestedException.as_deref()
    }

    // Java: String getMessage()
    pub fn getMessage(&self) -> String {
        // Java: return this.nestedException != null ? super.getMessage() + " Nested exception: " + this.nestedException.getMessage() : super.getMessage();
        if let Some(ref nested) = self.nestedException {
            format!("{} Nested exception: {}", self.message, nested)
        } else {
            self.message.clone()
        }
    }

    // Java: void printStackTrace()
    pub fn printStackTrace(&self) {
        // Java: super.printStackTrace();
        eprintln!("{:?}", self);
        // Java: if (this.nestedException != null)
        if let Some(ref nested) = self.nestedException {
            // Java: System.err.print("Nested exception: ");
            eprint!("Nested exception: ");
            // Java: this.nestedException.printStackTrace();
            eprintln!("{:?}", nested);
        }
    }

    // Java: void printStackTrace(PrintStream out)
    pub fn printStackTrace_to_stream(&self, out: &mut dyn Write) {
        // Java: super.printStackTrace(out);
        let _ = writeln!(out, "{:?}", self);
        // Java: if (this.nestedException != null)
        if let Some(ref nested) = self.nestedException {
            // Java: out.println("Nested exception: ");
            let _ = writeln!(out, "Nested exception: ");
            // Java: this.nestedException.printStackTrace(out);
            let _ = writeln!(out, "{:?}", nested);
        }
    }

    // Java: void printStackTrace(PrintWriter writer)
    pub fn printStackTrace_to_writer(&self, writer: &mut dyn Write) {
        // Java: super.printStackTrace(writer);
        let _ = writeln!(writer, "{:?}", self);
        // Java: if (this.nestedException != null)
        if let Some(ref nested) = self.nestedException {
            // Java: writer.println("Nested exception: ");
            let _ = writeln!(writer, "Nested exception: ");
            // Java: this.nestedException.printStackTrace(writer);
            let _ = writeln!(writer, "{:?}", nested);
        }
    }
}
