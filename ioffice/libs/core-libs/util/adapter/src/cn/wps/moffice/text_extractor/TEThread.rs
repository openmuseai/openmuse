// Java: public class TEThread implements Runnable
pub struct TEThread {
    // Java field: TextExtractor mTe
    mTe: Option<Box<dyn std::any::Any>>,
}

// Java: public class TEThread implements Runnable
impl TEThread {
    // Java: TEThread(TextExtractor te)
    pub fn new(_te: Option<Box<dyn std::any::Any>>) -> Self {
      // Java: super();
      // Java: this.mTe = te;
      TEThread {
          mTe: _te,
      }
    }

    // Java: void run()
    pub fn run(&self) {
      // Java: String result = this.mTe.result();
      // Java: if (null != result)
      // Java: this.mTe.getListener().onExtractFinish(result);
      todo!()
    }

}
