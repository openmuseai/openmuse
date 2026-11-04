// Java: import java.util.ArrayList

// Java: public class SlimContext
pub struct SlimContext {
    // Java field: boolean mFindSlimItem
    pub(crate) mFindSlimItem: bool,
    // Java field: ArrayList<SlimResult> mSlimResults
    pub(crate) mSlimResults: Vec<Box<dyn std::any::Any>>,
}

// Java: public class SlimContext
impl SlimContext {
    // Java: SlimContext()
    pub fn new() -> Self {
        // Java: super();
        SlimContext {
            mFindSlimItem: false,
            mSlimResults: Vec::new(),
        }
    }

    // Java: boolean isFindSlimItem()
    pub fn isFindSlimItem(&self) -> bool {
        // Java: return this.mFindSlimItem;
        self.mFindSlimItem
    }

    // Java: void setFindSlimItem(boolean val)
    pub fn setFindSlimItem(&mut self, val: bool) {
        // Java: this.mFindSlimItem = val;
        self.mFindSlimItem = val;
    }

    // Java: ArrayList<SlimResult> getSlimResults()
    pub fn getSlimResults(&self) -> &Vec<Box<dyn std::any::Any>> {
        // Java: return this.mSlimResults;
        &self.mSlimResults
    }

    // Java: void addSlimResult(SlimResult result)
    pub fn addSlimResult(&mut self, result: Box<dyn std::any::Any>) {
        // Java: this.mSlimResults.add(result);
        self.mSlimResults.push(result);
    }

    // Java: void clearSlimResult()
    pub fn clearSlimResult(&mut self) {
        // Java: this.mSlimResults.clear();
        self.mSlimResults.clear();
    }

}
