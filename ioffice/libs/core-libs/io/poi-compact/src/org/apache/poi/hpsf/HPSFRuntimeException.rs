use std::fmt;
use std::ops::Deref;

pub struct RuntimeException {
    pub(crate) message: String,
}

impl RuntimeException {
    pub fn new() -> Self {
        RuntimeException { message: String::new() }
    }
    pub fn new_with_msg(msg: &str) -> Self {
        RuntimeException { message: msg.to_string() }
    }
    pub fn getMessage(&self) -> &str {
        &self.message
    }
}

// Java: public class HPSFRuntimeException extends RuntimeException
pub struct HPSFRuntimeException {
    // Java field: Throwable reason
    pub(crate) reason: Option<Box<dyn std::error::Error + Send + Sync>>,
    parent: RuntimeException,
}

impl Deref for HPSFRuntimeException {
    type Target = RuntimeException;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

// Java: public class HPSFRuntimeException extends RuntimeException
impl HPSFRuntimeException {
    // Java: HPSFRuntimeException()
    pub fn new() -> Self {
        // Java: super();
        HPSFRuntimeException {
            reason: None,
            parent: RuntimeException::new(),
        }
    }

    // Java: HPSFRuntimeException(String msg)
    pub fn new_with_msg(msg: &str) -> Self {
        // Java: super(msg);
        HPSFRuntimeException {
            reason: None,
            parent: RuntimeException::new_with_msg(msg),
        }
    }

    // Java: HPSFRuntimeException(Throwable reason)
    pub fn new_with_reason(reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super();
        // Java: this.reason = reason;
        HPSFRuntimeException {
            reason: Some(reason),
            parent: RuntimeException::new(),
        }
    }

    // Java: HPSFRuntimeException(String msg, Throwable reason)
    pub fn new_with_msg_and_reason(msg: &str, reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(msg);
        // Java: this.reason = reason;
        HPSFRuntimeException {
            reason: Some(reason),
            parent: RuntimeException::new_with_msg(msg),
        }
    }

    // Java: Throwable getReason()
    pub fn getReason(&self) -> &Option<Box<dyn std::error::Error + Send + Sync>> {
        // Java: return this.reason;
        &self.reason
    }

    // Java: void printStackTrace()
    pub fn printStackTrace_default(&self) {
        // Java: this.printStackTrace(System.err);
        eprintln!("{:?}", self);
        if let Some(reason) = &self.reason {
            eprintln!("Caused by:");
            eprintln!("{}", reason);
        }
    }

    // Java: void printStackTrace(PrintStream p)
    pub fn printStackTrace_stream(&self, out: &mut dyn std::io::Write) {
        // Java: Throwable reason = this.getReason();
        let reason = self.getReason();
        // Java: super.printStackTrace(p);
        let _ = writeln!(out, "{:?}", self);
        // Java: if (reason != null)
        if let Some(r) = reason {
            // Java: p.println("Caused by:");
            let _ = writeln!(out, "Caused by:");
            // Java: reason.printStackTrace(p);
            let _ = writeln!(out, "{}", r);
        }
    }

    // Java: void printStackTrace(PrintWriter p)
    pub fn printStackTrace_writer(&self, out: &mut dyn std::io::Write) {
        // Java: Throwable reason = this.getReason();
        let reason = self.getReason();
        // Java: super.printStackTrace(p);
        let _ = writeln!(out, "{:?}", self);
        // Java: if (reason != null)
        if let Some(r) = reason {
            // Java: p.println("Caused by:");
            let _ = writeln!(out, "Caused by:");
            // Java: reason.printStackTrace(p);
            let _ = writeln!(out, "{}", r);
        }
    }
}

impl fmt::Display for HPSFRuntimeException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parent.message)
    }
}

impl fmt::Debug for HPSFRuntimeException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HPSFRuntimeException: {}", self.parent.message)
    }
}

impl std::error::Error for HPSFRuntimeException {}
