// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName

use super::BaseData::BaseData;

// Java: public class Qualifications extends BaseData
pub struct Qualifications {
    pub(crate) base: BaseData,
    // Java field: String mContent
    pub(crate) mContent: String,
}

// Java: public class Qualifications extends BaseData
impl Qualifications {
    // Java: Qualifications()
    pub fn new() -> Self {
      // Java: super();
      Qualifications {
          base: BaseData::new(),
          mContent: String::new(),
      }
    }

    // Java: void setContent(String content)
    pub fn setContent(&mut self, _content: &str) {
      // Java: this.mContent = content;
      self.mContent = _content.to_string();
    }

    // Java: String getContent()
    pub fn getContent(&self) -> &str {
      // Java: return this.mContent;
      &self.mContent
    }

}
