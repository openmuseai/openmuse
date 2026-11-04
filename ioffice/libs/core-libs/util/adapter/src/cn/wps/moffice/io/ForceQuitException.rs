// Java: import java.io.IOException

// Java: public class ForceQuitException extends IOException
pub struct ForceQuitException {
    // Java field: int errorCode
    pub(crate) errorCode: i32,
}

// Java: public class ForceQuitException extends IOException
impl ForceQuitException {
    // Java: ForceQuitException()
    pub fn new() -> Self {
        // Java: this(-1);
        ForceQuitException::new_with_error_code(-1)
    }

    // Java: ForceQuitException(int errorCode)
    pub fn new_with_error_code(errorCode: i32) -> Self {
        // Java: super();
        // Java: this.errorCode = errorCode;
        ForceQuitException { errorCode }
    }

    // Java: int getErrorCode()
    pub fn getErrorCode(&self) -> i32 {
        // Java: return this.errorCode;
        self.errorCode
    }

}
