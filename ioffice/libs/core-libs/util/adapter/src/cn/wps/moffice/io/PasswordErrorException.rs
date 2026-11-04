// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 7905984615137262154;

// Java: public class PasswordErrorException extends Exception
pub struct PasswordErrorException {
    // Java field: Throwable ex
    pub(crate) ex: Option<Box<dyn std::any::Any + Send>>,
}

// Java: public class PasswordErrorException extends Exception
impl PasswordErrorException {
    // Java: PasswordErrorException()
    pub fn new() -> Self {
        // Java: super((Throwable)null);
        PasswordErrorException { ex: None }
    }

    // Java: PasswordErrorException(String s)
    pub fn new_with_msg(_s: &str) -> Self {
        // Java: super(s, (Throwable)null);
        PasswordErrorException { ex: None }
    }

    // Java: PasswordErrorException(String s, Throwable ex)
    pub fn new_with_msg_and_ex(_s: &str, ex: Box<dyn std::any::Any + Send>) -> Self {
        // Java: super(s, (Throwable)null);
        // Java: this.ex = ex;
        PasswordErrorException { ex: Some(ex) }
    }

    // Java: Throwable getException()
    pub fn getException(&self) -> Option<&(dyn std::any::Any + Send)> {
        // Java: return this.ex;
        self.ex.as_deref()
    }

    // Java: Throwable getCause()
    pub fn getCause(&self) -> Option<&(dyn std::any::Any + Send)> {
        // Java: return this.ex;
        self.ex.as_deref()
    }

}
