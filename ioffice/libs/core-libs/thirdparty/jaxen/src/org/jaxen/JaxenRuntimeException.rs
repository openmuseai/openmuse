use std::io::Write;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -5765625826164407899;

// Java: public class JaxenRuntimeException extends RuntimeException
#[derive(Debug)]
pub struct JaxenRuntimeException {
    // Java field: Throwable cause
    pub(crate) cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    // Java field: boolean causeSet
    cause_set: bool,
    pub(crate) message: String,
}

// Java: public class JaxenRuntimeException extends RuntimeException
impl JaxenRuntimeException {
    // Java: JaxenRuntimeException(Throwable cause)
    pub fn from_cause(cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(cause.getMessage());
        let message = cause.to_string();
        // Java: this.initCause(cause);
        JaxenRuntimeException {
            message,
            cause: Some(cause),
            cause_set: true,
        }
    }

    // Java: JaxenRuntimeException(String message)
    pub fn new(message: String) -> Self {
        // Java: super(message);
        JaxenRuntimeException {
            message,
            cause: None,
            cause_set: false,
        }
    }

    // Java: Throwable getCause()
    pub fn getCause(&self) -> Option<&(dyn std::error::Error + Send + Sync)> {
        // Java: return this.cause;
        self.cause.as_deref()
    }

    // Java: Throwable initCause(Throwable cause)
    pub fn initCause(&mut self, cause: Box<dyn std::error::Error + Send + Sync>) -> Result<&Self, String> {
        // Java: if (this.causeSet)
        if self.cause_set {
            // Java: throw new IllegalStateException("Cause cannot be reset");
            return Err("Cause cannot be reset".to_string());
        }
        // Java: else if (cause == this)
        // Note: In Rust, we cannot compare error references to self, skip this check
        // Java: throw new IllegalArgumentException("Exception cannot be its own cause");
        // Java: else
        // Java: this.causeSet = true;
        self.cause_set = true;
        // Java: this.cause = cause;
        self.cause = Some(cause);
        // Java: return this;
        Ok(self)
    }

    // Java: void printStackTrace(PrintStream s)
    pub fn printStackTrace_stream(&self, s: &mut dyn Write) {
        // Java: super.printStackTrace(s);
        let _ = writeln!(s, "{}", self.message);
        // Java: if (JaxenException.javaVersion < 1.4 && this.getCause() != null)
        if let Some(ref cause) = self.cause {
            // Java: s.print("Caused by: ");
            let _ = write!(s, "Caused by: ");
            // Java: this.getCause().printStackTrace(s);
            let _ = writeln!(s, "{}", cause);
        }
    }

    // Java: void printStackTrace(PrintWriter s)
    pub fn printStackTrace_writer(&self, s: &mut dyn Write) {
        // Java: super.printStackTrace(s);
        let _ = writeln!(s, "{}", self.message);
        // Java: if (JaxenException.javaVersion < 1.4 && this.getCause() != null)
        if let Some(ref cause) = self.cause {
            // Java: s.print("Caused by: ");
            let _ = write!(s, "Caused by: ");
            // Java: this.getCause().printStackTrace(s);
            let _ = writeln!(s, "{}", cause);
        }
    }
}

impl std::fmt::Display for JaxenRuntimeException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(ref cause) = self.cause {
            write!(f, "\nCaused by: {}", cause)?;
        }
        Ok(())
    }
}

impl std::error::Error for JaxenRuntimeException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}
