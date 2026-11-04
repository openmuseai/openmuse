// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName

use super::BaseData::BaseData;

// Java: public class JobIntention extends BaseData
pub struct JobIntention {
    pub(crate) base: BaseData,
    // Java field: String mObjective
    pub(crate) mObjective: String,
    // Java field: String mType
    pub(crate) mType: String,
    // Java field: String mCity
    pub(crate) mCity: String,
    // Java field: String mSalary
    pub(crate) mSalary: String,
    // Java field: String mIndustry
    pub(crate) mIndustry: String,
}

// Java: public class JobIntention extends BaseData
impl JobIntention {
    // Java: JobIntention()
    pub fn new() -> Self {
      // Java: super();
      JobIntention {
          base: BaseData::new(),
          mObjective: String::new(),
          mType: String::new(),
          mCity: String::new(),
          mSalary: String::new(),
          mIndustry: String::new(),
      }
    }

    // Java: String getObjective()
    pub fn getObjective(&self) -> &str {
      // Java: return this.mObjective;
      &self.mObjective
    }

    // Java: String getIndustry()
    pub fn getIndustry(&self) -> &str {
      // Java: return this.mIndustry;
      &self.mIndustry
    }

    // Java: String getType()
    pub fn getType(&self) -> &str {
      // Java: return this.mType;
      &self.mType
    }

    // Java: String getCity()
    pub fn getCity(&self) -> &str {
      // Java: return this.mCity;
      &self.mCity
    }

    // Java: String getSalary()
    pub fn getSalary(&self) -> &str {
      // Java: return this.mSalary;
      &self.mSalary
    }

}
