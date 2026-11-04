// Java: import org.apache.poi.hwpf.ole.OleInfo
use crate::org::apache::poi::hwpf::ole::OleInfo::*;

// java public class UotOleInfo extends OleInfo
#[derive(Default)]
pub struct UotOleInfo {
    // java OleInfo super_
    pub(crate) super_: OleInfo,
    // java private String partName;
    partName: Option<String>,
}

// java public class UotOleInfo extends OleInfo
impl UotOleInfo {
    // java public  UotOleInfo(String partName)
    pub fn UotOleInfo(partName: Option<String>) -> Self {
      // java super();
      // java this.partName = partName;
      Self { super_: OleInfo::new(), partName: partName }
    }

    // java public String getPartName()
    pub fn getPartName(&self) -> Option<String> {
      // java return this.partName;
      return self.partName.clone();
    }

    // java public void setPartName(String partName)
    pub fn setPartName(&mut self, partName: Option<String>) {
      // java this.partName = partName;
      self.partName = partName;
    }

}

impl std::ops::Deref for UotOleInfo {
    type Target = OleInfo;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for UotOleInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
