// Java: import java.io.PrintStream
use std::fmt;
// Java: import java.io.PrintWriter
use std::error::Error;

// Java: public class DocumentException extends Exception
pub struct DocumentException {
    // Java field: Throwable nestedException
    pub(crate) message: String,
    pub(crate) nestedException: Option<Box<dyn Error + Send + Sync>>,
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
    pub fn new_with_message(message: &str) -> Self {
        // Java: super(message);
        DocumentException {
            message: message.to_string(),
            nestedException: None,
        }
    }

    // Java: DocumentException(Throwable nestedException)
    pub fn new_with_cause(nested_exception: Box<dyn Error + Send + Sync>) -> Self {
        // Java: super(nestedException.getMessage());
        let message = nested_exception.to_string();
        // Java: this.nestedException = nestedException;
        DocumentException {
            message,
            nestedException: Some(nested_exception),
        }
    }

    // Java: DocumentException(String message, Throwable nestedException)
    pub fn new_with_message_and_cause(message: &str, nested_exception: Box<dyn Error + Send + Sync>) -> Self {
        // Java: super(message);
        // Java: this.nestedException = nestedException;
        DocumentException {
            message: message.to_string(),
            nestedException: Some(nested_exception),
        }
    }

    // Java: Throwable getNestedException()
    pub fn getNestedException(&self) -> Option<&(dyn Error + Send + Sync)> {
        // Java: return this.nestedException;
        self.nestedException.as_deref()
    }

    // Java: String getMessage()
    pub fn getMessage(&self) -> String {
        // Java: return this.nestedException != null ? super.getMessage() + " Nested exception: " + this.nestedException.getMessage() : super.getMessage();
        match &self.nestedException {
            Some(nested) => format!("{} Nested exception: {}", self.message, nested),
            None => self.message.clone(),
        }
    }

    // Java: void printStackTrace()
    pub fn printStackTrace(&self) {
        // Java: super.printStackTrace();
        eprintln!("{}", self);
        // Java: if (this.nestedException != null)
        if let Some(nested) = &self.nestedException {
            // Java: System.err.print("Nested exception: ");
            eprint!("Nested exception: ");
            // Java: this.nestedException.printStackTrace();
            eprintln!("{}", nested);
        }
    }

    // Java: void printStackTrace(PrintStream out)
    pub fn printStackTrace_to_stream(&self, out: &mut dyn std::io::Write) {
        // Java: super.printStackTrace(out);
        let _ = writeln!(out, "{}", self);
        // Java: if (this.nestedException != null)
        if let Some(nested) = &self.nestedException {
            // Java: out.println("Nested exception: ");
            let _ = writeln!(out, "Nested exception: ");
            // Java: this.nestedException.printStackTrace(out);
            let _ = writeln!(out, "{}", nested);
        }
    }

    // Java: void printStackTrace(PrintWriter writer)
    pub fn printStackTrace_to_writer(&self, writer: &mut dyn fmt::Write) {
        // Java: super.printStackTrace(writer);
        let _ = writeln!(writer, "{}", self);
        // Java: if (this.nestedException != null)
        if let Some(nested) = &self.nestedException {
            // Java: writer.println("Nested exception: ");
            let _ = writeln!(writer, "Nested exception: ");
            // Java: this.nestedException.printStackTrace(writer);
            let _ = writeln!(writer, "{}", nested);
        }
    }
}

impl fmt::Display for DocumentException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.getMessage())
    }
}

impl fmt::Debug for DocumentException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DocumentException: {}", self.getMessage())
    }
}

impl Error for DocumentException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.nestedException.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}
