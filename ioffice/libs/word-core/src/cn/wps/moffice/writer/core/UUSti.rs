// java public class UUSti
#[derive(Default, Clone)]
pub struct UUSti {
    // java private int mStiCustom = 4096;
    mStiCustom: i32,
}

// java public class UUSti
impl UUSti {
    // java   UUSti()
    pub fn UUSti() -> Self {
      Self { mStiCustom: 4096 }
    }

    // java public int getUUSti()
    pub fn getUUSti(&mut self) -> i32 {
      // java return ++this.mStiCustom;
      self.mStiCustom += 1;
      return self.mStiCustom;
    }

}
