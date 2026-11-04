// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName

// Java: public class BaseData
pub struct BaseData {
    // Java field: boolean mIsHideInTemplate
    pub(crate) mIsHideInTemplate: bool,
}

// Java: public class BaseData
impl BaseData {
    // Java: BaseData()
    pub fn new() -> Self {
      // Java: super();
      BaseData {
          mIsHideInTemplate: false,
      }
    }

    // Java: boolean isHideInTemplate()
    pub fn isHideInTemplate(&self) -> bool {
      // Java: return this.mIsHideInTemplate;
      self.mIsHideInTemplate
    }

}
