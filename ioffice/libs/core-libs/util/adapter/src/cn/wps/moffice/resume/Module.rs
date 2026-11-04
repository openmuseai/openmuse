// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName

// Java: public class Module
pub struct Module {
    // Java field: String mName
    pub(crate) mName: String,
    // Java field: boolean mIsHide
    pub(crate) mIsHide: bool,
}

// Java: public class Module
impl Module {
    // Java: Module()
    pub fn new() -> Self {
      // Java: super();
      Module {
          mName: String::new(),
          mIsHide: false,
      }
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this.mName;
      &self.mName
    }

    // Java: boolean isHide()
    pub fn isHide(&self) -> bool {
      // Java: return this.mIsHide;
      self.mIsHide
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, _name: &str) {
      // Java: this.mName = name;
      self.mName = _name.to_string();
    }

    // Java: void setHide(boolean hide)
    pub fn setHide(&mut self, _hide: bool) {
      // Java: this.mIsHide = hide;
      self.mIsHide = _hide;
    }

}
