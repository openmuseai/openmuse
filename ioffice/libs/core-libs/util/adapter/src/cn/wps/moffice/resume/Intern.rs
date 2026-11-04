// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName

use super::BaseData::BaseData;
use super::Period::Period;

// Java: public class Intern extends BaseData
pub struct Intern {
    pub(crate) base: BaseData,
    // Java field: String mId
    pub(crate) mId: String,
    // Java field: String mName
    pub(crate) mName: String,
    // Java field: String mPosition
    pub(crate) mPosition: String,
    // Java field: String mContent
    pub(crate) mContent: String,
    // Java field: Period mPeriod
    pub(crate) mPeriod: Option<Period>,
}

// Java: public class Intern extends BaseData
impl Intern {
    // Java: Intern()
    pub fn new() -> Self {
      // Java: super();
      Intern {
          base: BaseData::new(),
          mId: String::new(),
          mName: String::new(),
          mPosition: String::new(),
          mContent: String::new(),
          mPeriod: None,
      }
    }

    // Java: String getId()
    pub fn getId(&self) -> &str {
      // Java: return this.mId;
      &self.mId
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this.mName;
      &self.mName
    }

    // Java: String getPosition()
    pub fn getPosition(&self) -> &str {
      // Java: return this.mPosition;
      &self.mPosition
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

    // Java: Period getPeriod()
    pub fn getPeriod(&self) -> &Option<Period> {
      // Java: return this.mPeriod;
      &self.mPeriod
    }

}
