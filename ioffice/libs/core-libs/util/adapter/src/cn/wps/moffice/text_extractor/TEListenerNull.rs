use super::ITEListener::ITEListener;

// Java: public class TEListenerNull implements ITEListener
pub struct TEListenerNull {
}

// Java: public class TEListenerNull implements ITEListener
impl TEListenerNull {
    // Java: TEListenerNull()
    pub fn new() -> Self {
      // Java: super();
      TEListenerNull {}
    }

    // Java: void onExtractFinish(String result)
    // Java: void onNeedPassword()
}

impl ITEListener for TEListenerNull {
    fn onExtractFinish(&self, _result: &str) {
    }

    fn onNeedPassword(&self) {
    }
}
