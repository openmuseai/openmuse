// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName

use super::BaseData::BaseData;

// Java: public class CustomModule extends BaseData
pub struct CustomModule {
    pub(crate) base: BaseData,
    // Java field: String mName
    pub(crate) mName: String,
    // Java field: String mContent
    pub(crate) mContent: String,
}

// Java: public class CustomModule extends BaseData
impl CustomModule {
    // Java: CustomModule()
    pub fn new() -> Self {
      // Java: super();
      CustomModule {
          base: BaseData::new(),
          mName: String::new(),
          mContent: String::new(),
      }
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this.mName;
      &self.mName
    }

    // Java: String getContent()
    pub fn getContent(&self) -> &str {
      // Java: return this.mContent;
      &self.mContent
    }

    // Java: void setContent(String content)
    pub fn setContent(&mut self, _content: &str) {
      // Java: this.mContent = content;
      self.mContent = _content.to_string();
    }

}
