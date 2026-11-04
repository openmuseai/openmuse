// java public class IOThreadSign
#[derive(Default)]
pub struct IOThreadSign {
    // java private boolean mIsFinish = false;
    mIsFinish: bool,
}

// java public class IOThreadSign
impl IOThreadSign {
    // java public void setFinish(boolean isFinish)
    pub fn setFinish(&mut self, isFinish: bool) {
      // java this.mIsFinish = isFinish;
      self.mIsFinish = isFinish;
    }

    // java public boolean isFinish()
    pub fn isFinish(&self) -> bool {
      // java return this.mIsFinish;
      return self.mIsFinish;
    }

}
