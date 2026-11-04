// Java: import java.io.PrintStream
// Java: import java.io.PrintWriter

// Java field: long serialVersionUID
// Java field: double javaVersion

// Java: public class SAXPathException extends Exception
#[derive(Debug)]
pub struct SAXPathException {
    // Java field: String message
    pub(crate) message: String,
    // Java field: Throwable cause
    pub(crate) cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    // Java field: boolean causeSet
    cause_set: bool,
}

// Java: public class SAXPathException extends Exception
impl SAXPathException {
    // Java: SAXPathException(String message)
    pub fn new(message: String) -> Self {
        // Java: super(message);
        SAXPathException {
            message,
            cause: None,
            cause_set: false,
        }
    }

    // Java: SAXPathException(Throwable cause)
    pub fn from_cause(cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(cause.getMessage());
        let message = cause.to_string();
        // Java: this.initCause(cause);
        SAXPathException {
            message,
            cause: Some(cause),
            cause_set: true,
        }
    }

    // Java: SAXPathException(String message, Throwable cause)
    pub fn new_with_cause(message: String, cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(message);
        // Java: this.initCause(cause);
        SAXPathException {
            message,
            cause: Some(cause),
            cause_set: true,
        }
    }

    // Java: Throwable getCause()
    pub fn getCause(&self) -> Option<&(dyn std::error::Error + Send + Sync)> {
        // Java: return this.cause;
        self.cause.as_deref()
    }

    // Java: Throwable initCause(Throwable cause)
    pub fn initCause(&mut self, cause: Box<dyn std::error::Error + Send + Sync>) {
        // Java: if (this.causeSet)
        if self.cause_set {
            // Java: throw new IllegalStateException("Cause cannot be reset");
            panic!("Cause cannot be reset");
        }
        // Java: else if (cause == this)
        // Java: throw new IllegalArgumentException("Exception cannot be its own cause");
        // Java: else
        // Java: this.causeSet = true;
        self.cause_set = true;
        // Java: this.cause = cause;
        self.cause = Some(cause);
        // Java: return this;
    }

    // Java: void printStackTrace(PrintStream s)
    pub fn printStackTrace_stream(&self) {
        // Java: super.printStackTrace(s);
        // Java: if (javaVersion < 1.4 && this.getCause() != null)
        // Java: s.print("Caused by: ");
        // Java: this.getCause().printStackTrace(s);
        eprintln!("{}", self.message);
    }

    // Java: void printStackTrace(PrintWriter s)
    pub fn printStackTrace_writer(&self) {
        // Java: super.printStackTrace(s);
        // Java: if (javaVersion < 1.4 && this.getCause() != null)
        // Java: s.print("Caused by: ");
        // Java: this.getCause().printStackTrace(s);
        eprintln!("{}", self.message);
    }
}

impl std::fmt::Display for SAXPathException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SAXPathException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}
