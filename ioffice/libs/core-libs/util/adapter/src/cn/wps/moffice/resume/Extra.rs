// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName

use super::CustomModule::CustomModule;

// Java: public class Extra
pub struct Extra {
    // Java field: CustomModule mCustomModule
    pub(crate) mCustomModule: Option<CustomModule>,
    // Java field: int fontSize
    pub(crate) fontSize: i32,
    // Java field: int lineSpace
    pub(crate) lineSpace: i32,
    // Java field: int moduleSpace
    pub(crate) moduleSpace: i32,
}

// Java: public class Extra
impl Extra {
    // Java: Extra()
    pub fn new() -> Self {
      // Java: super();
      Extra {
          mCustomModule: None,
          fontSize: 0,
          lineSpace: 0,
          moduleSpace: 0,
      }
    }

    // Java: CustomModule getCustomModule()
    pub fn getCustomModule(&self) -> &Option<CustomModule> {
      // Java: return this.mCustomModule;
      &self.mCustomModule
    }

}
