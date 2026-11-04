// Java: import cn.wps.moffice.property.MutablePropertySet
// use util::cn::wps::moffice::property::MutablePropertySet;

// Java: public class RubyContent
pub struct RubyContent {
    // Java field: MutablePropertySet mProp
    pub(crate) mProp: Option<Box<dyn std::any::Any>>,
    // Java field: String mTmpText
    pub(crate) mTmpText: Option<String>,
}

// Java: public class RubyContent
impl RubyContent {
    // Java: RubyContent()
    pub fn new() -> Self {
      // Java: super();
      RubyContent { mProp: None, mTmpText: None }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: this.mProp = null;
      self.mProp = None;
      // Java: this.mTmpText = null;
      self.mTmpText = None;
    }
}
