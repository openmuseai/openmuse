// java public class ModifyFlag
impl ModifyFlag {
    // java public static final ModifyFlag _true = new ModifyFlag(true);
    pub const _true: ModifyFlag = ModifyFlag { mFlag: true, mAutoSave: false };
    // java public static final ModifyFlag _false = new ModifyFlag(false);
    pub const _false: ModifyFlag = ModifyFlag { mFlag: false, mAutoSave: false };
}

// java public class ModifyFlag
#[derive(Default)]
pub struct ModifyFlag {
    // java private boolean mFlag;
    mFlag: bool,
    // java private boolean mAutoSave;
    mAutoSave: bool,
}

// java public class ModifyFlag
impl ModifyFlag {
    // java private  ModifyFlag(boolean flag)
    pub fn ModifyFlag(flag: bool) -> Self {
      // java this.mFlag = flag;
      Self { mFlag: flag, mAutoSave: Default::default() }
    }

    // java public boolean isModified()
    pub fn isModified(&self) -> bool {
      // java return this.mFlag;
      return self.mFlag;
    }

    // java public void setAutoSave(boolean autoSave)
    pub fn setAutoSave(&mut self, autoSave: bool) {
      // java this.mAutoSave = autoSave;
      self.mAutoSave = autoSave;
    }

    // java public boolean isAutoSave()
    pub fn isAutoSave(&self) -> bool {
      // java return this.mAutoSave;
      return self.mAutoSave;
    }

}
