// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName

use super::BaseData::BaseData;
use super::Period::Period;

// Java: public class Education extends BaseData
pub struct Education {
    pub(crate) base: BaseData,
    // Java field: String mName
    pub(crate) mName: String,
    // Java field: String mMajor
    pub(crate) mMajor: String,
    // Java field: String mDegree
    pub(crate) mDegree: String,
    // Java field: Period mPeriod
    pub(crate) mPeriod: Option<Period>,
    // Java field: String mType
    pub(crate) mType: String,
    // Java field: String mContent
    pub(crate) mContent: String,
    // Java field: String mCourse
    pub(crate) mCourse: String,
}

// Java: public class Education extends BaseData
impl Education {
    // Java: Education()
    pub fn new() -> Self {
      // Java: super();
      Education {
          base: BaseData::new(),
          mName: String::new(),
          mMajor: String::new(),
          mDegree: String::new(),
          mPeriod: None,
          mType: String::new(),
          mContent: String::new(),
          mCourse: String::new(),
      }
    }

    // Java: void setCourse(String course)
    pub fn setCourse(&mut self, _course: &str) {
      // Java: this.mCourse = course;
      self.mCourse = _course.to_string();
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this.mName;
      &self.mName
    }

    // Java: String getMajor()
    pub fn getMajor(&self) -> &str {
      // Java: return this.mMajor;
      &self.mMajor
    }

    // Java: String getDegree()
    pub fn getDegree(&self) -> &str {
      // Java: return this.mDegree;
      &self.mDegree
    }

    // Java: Period getPeriod()
    pub fn getPeriod(&self) -> &Option<Period> {
      // Java: return this.mPeriod;
      &self.mPeriod
    }

    // Java: String getType()
    pub fn getType(&self) -> &str {
      // Java: return this.mType;
      &self.mType
    }

    // Java: String getContent()
    pub fn getContent(&self) -> &str {
      // Java: return this.mContent;
      &self.mContent
    }

    // Java: String getCourse()
    pub fn getCourse(&self) -> &str {
      // Java: return this.mCourse;
      &self.mCourse
    }

}
