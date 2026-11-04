// Java: import java.util.ArrayList

use super::ISlimListener::ISlimListener;

// Java: public class SlimListenerNull implements ISlimListener
pub struct SlimListenerNull {
}

// Java: public class SlimListenerNull implements ISlimListener
impl SlimListenerNull {
    // Java: SlimListenerNull()
    pub fn new() -> Self {
      // Java: super();
      SlimListenerNull {}
    }

    // Java: void onFindSlimItem()
    // Java: void onSlimCheckFinish(ArrayList<SlimResult> result)
    // Java: void onStopFinish()
    // Java: void onSlimFinish()
    // Java: void onSlimItemFinish(int type, long slimSize)
}

impl ISlimListener for SlimListenerNull {
    fn onFindSlimItem(&self) {
    }

    fn onSlimCheckFinish(&self, _result: Vec<Box<dyn std::any::Any>>) {
    }

    fn onStopFinish(&self) {
    }

    fn onSlimFinish(&self) {
    }

    fn onSlimItemFinish(&self, _type: i32, _slimSize: i64) {
    }
}
