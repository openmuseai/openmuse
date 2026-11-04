// Java: import java.util.ArrayList
use std::collections::HashSet;
// Java: import java.util.HashSet

// Java field: int TYPE_FIND_SLIM_ITEM
pub(crate) const TYPE_FIND_SLIM_ITEM: i32 = 0;
// Java field: int TYPE_SLIM_CHECK_FINISH
pub(crate) const TYPE_SLIM_CHECK_FINISH: i32 = 1;
// Java field: int TYPE_STOP_FINISH
pub(crate) const TYPE_STOP_FINISH: i32 = 2;
// Java field: int TYPE_SLIM_FINISH
pub(crate) const TYPE_SLIM_FINISH: i32 = 3;
// Java field: int TYPE_SLIM_ITEM_FINISH
pub(crate) const TYPE_SLIM_ITEM_FINISH: i32 = 4;

// Java: public class InnerSlimListener implements ISlimListener
pub struct InnerSlimListener {
    // Java field: HashSet<Integer> mExcludeSet
    pub(crate) mExcludeSet: HashSet<i32>,
    // Java field: ISlimListener mRealListener
    mRealListener: Option<Box<dyn std::any::Any>>,
}

// Java: public class InnerSlimListener implements ISlimListener
impl InnerSlimListener {
    // Java: InnerSlimListener(ISlimListener listener)
    pub fn new(listener: Option<Box<dyn std::any::Any>>) -> Self {
      // Java: super();
      // Java: this.mRealListener = listener;
      InnerSlimListener {
          mExcludeSet: HashSet::new(),
          mRealListener: listener,
      }
    }

    // Java: void exclude(int type)
    pub fn exclude(&mut self, r#type: i32) {
      // Java: this.mExcludeSet.add(type);
      self.mExcludeSet.insert(r#type);
    }

    // Java: void include(int type)
    pub fn include(&mut self, r#type: i32) {
      // Java: this.mExcludeSet.remove(type);
      self.mExcludeSet.remove(&r#type);
    }

    // Java: boolean isInExcludeSet(int type)
    pub fn isInExcludeSet(&self, r#type: i32) -> bool {
      // Java: return this.mExcludeSet.contains(type);
      self.mExcludeSet.contains(&r#type)
    }

    // Java: void onFindSlimItem()
    pub fn onFindSlimItem(&self) {
      // Java: if (!this.mExcludeSet.contains(0))
      if !self.mExcludeSet.contains(&0) {
         // Java: this.mRealListener.onFindSlimItem();
         todo!()
      }
    }

    // Java: void onSlimCheckFinish(ArrayList<SlimResult> result)
    pub fn onSlimCheckFinish(&self, _result: Vec<Box<dyn std::any::Any>>) {
      // Java: if (!this.mExcludeSet.contains(1))
      if !self.mExcludeSet.contains(&1) {
         // Java: this.mRealListener.onSlimCheckFinish(result);
         todo!()
      }
    }

    // Java: void onStopFinish()
    pub fn onStopFinish(&self) {
      // Java: if (!this.mExcludeSet.contains(2))
      if !self.mExcludeSet.contains(&2) {
         // Java: this.mRealListener.onStopFinish();
         todo!()
      }
    }

    // Java: void onSlimFinish()
    pub fn onSlimFinish(&self) {
      // Java: if (!this.mExcludeSet.contains(3))
      if !self.mExcludeSet.contains(&3) {
         // Java: this.mRealListener.onSlimFinish();
         todo!()
      }
    }

    // Java: void onSlimItemFinish(int type, long slimSize)
    pub fn onSlimItemFinish(&self, r#type: i32, _slimSize: i64) {
      // Java: if (!this.mExcludeSet.contains(4))
      if !self.mExcludeSet.contains(&4) {
         // Java: this.mRealListener.onSlimItemFinish(type, slimSize);
         todo!()
      }
    }

}

impl super::ISlimListener::ISlimListener for InnerSlimListener {
    fn onFindSlimItem(&self) {
        InnerSlimListener::onFindSlimItem(self);
    }
    fn onSlimCheckFinish(&self, var1: Vec<Box<dyn std::any::Any>>) {
        InnerSlimListener::onSlimCheckFinish(self, var1);
    }
    fn onStopFinish(&self) {
        InnerSlimListener::onStopFinish(self);
    }
    fn onSlimFinish(&self) {
        InnerSlimListener::onSlimFinish(self);
    }
    fn onSlimItemFinish(&self, var1: i32, var2: i64) {
        InnerSlimListener::onSlimItemFinish(self, var1, var2);
    }
}
