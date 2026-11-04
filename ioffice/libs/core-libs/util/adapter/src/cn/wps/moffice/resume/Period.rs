// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName

use super::BaseData::BaseData;

// Java: public class Period extends BaseData
pub struct Period {
    pub(crate) base: BaseData,
    // Java field: String mStart
    pub(crate) mStart: String,
    // Java field: String mEnd
    pub(crate) mEnd: String,
}

// Java: public class Period extends BaseData
impl Period {
    // Java: Period()
    pub fn new() -> Self {
      // Java: super();
      Period {
          base: BaseData::new(),
          mStart: String::new(),
          mEnd: String::new(),
      }
    }

    // Java: String getStart()
    pub fn getStart(&self) -> &str {
      // Java: return this.mStart;
      &self.mStart
    }

    // Java: String getEnd()
    pub fn getEnd(&self) -> &str {
      // Java: return this.mEnd;
      &self.mEnd
    }

}
