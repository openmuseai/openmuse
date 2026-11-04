// java public class CompatSetting
#[derive(Default, Clone)]
pub struct CompatSetting {
    // java private String mName = null;
    mName: Option<String>,
    // java private String mURI = null;
    mURI: Option<String>,
    // java private String mVal = null;
    mVal: Option<String>,
}

// java public class CompatSetting
impl CompatSetting {
    // java public void setName(String name)
    pub fn setName(&mut self, name: &str) {
      // java this.mName = name;
      self.mName = Some(name.to_string());
    }

    // java public void setURI(String uri)
    pub fn setURI(&mut self, uri: &str) {
      // java this.mURI = uri;
      self.mURI = Some(uri.to_string());
    }

    // java public void setVal(String val)
    pub fn setVal(&mut self, val: &str) {
      // java this.mVal = val;
      self.mVal = Some(val.to_string());
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
      // java return this.mName;
      return self.mName.clone();
    }

    // java public String getURI()
    pub fn getURI(&self) -> Option<String> {
      // java return this.mURI;
      return self.mURI.clone();
    }

    // java public String getVal()
    pub fn getVal(&self) -> Option<String> {
      // java return this.mVal;
      return self.mVal.clone();
    }

}
