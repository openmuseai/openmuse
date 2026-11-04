use std::future::Future;
use std::pin::Pin;

// Java: public class ZipEntryFutureTask extends FutureTask
pub struct ZipEntryFutureTask {
    pub(crate) task: Pin<Box<dyn Future<Output = ()> + Send>>,
}

// Java: public class ZipEntryFutureTask extends FutureTask
impl ZipEntryFutureTask {
    // Java: public static final int TIMEOUT = 5000;
    pub const TIMEOUT: i32 = 5000;

    // Java: ZipEntryFutureTask(Callable<Void> callable)
    pub fn new(callable: Pin<Box<dyn Future<Output = ()> + Send>>) -> Self {
        // Java: super(callable);
        ZipEntryFutureTask { task: callable }
    }
}
