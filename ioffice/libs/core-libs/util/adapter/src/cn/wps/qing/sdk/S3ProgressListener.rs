// Java: public class S3ProgressListener
pub struct S3ProgressListener {
}

// Java: public class S3ProgressListener
impl S3ProgressListener {
    // Java: S3ProgressListener()
    pub fn new() -> Self {
      // Java: super();
      S3ProgressListener {}
    }

    // Java: abstract boolean onProgress(long var1, long var3)
    // Java: long getProgressInterval()
    pub fn getProgressInterval(&self) -> i64 {
      // Java: return 500L;
      500i64
    }

}

pub trait S3ProgressListenerTrait {
    // Java: abstract boolean onProgress(long var1, long var3)
    fn onProgress(&self, _var1: i64, _var3: i64) -> bool;
}
