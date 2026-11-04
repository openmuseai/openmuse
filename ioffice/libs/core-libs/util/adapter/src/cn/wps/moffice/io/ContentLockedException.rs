// Java: import java.io.IOException

// Java: public class ContentLockedException extends IOException
pub struct ContentLockedException {
}

// Java: public class ContentLockedException extends IOException
impl ContentLockedException {
    // Java: ContentLockedException(Throwable cause)
    pub fn new(cause: Option<Box<dyn std::any::Any + Send>>) -> Self {
        // Java: super(cause);
        ContentLockedException {}
    }

}
