#![allow(non_snake_case)]
// Java: public class FileVersion
#[derive(Default)]
pub struct FileVersion {
    mAppName: Option<String>,
    mLastEdited: Option<String>,
    mLowestEdited: Option<String>,
    mRupBuild: Option<String>,
    mCodeName: Option<String>,
}

impl FileVersion {
    // Java: public String appName()
    pub fn appName(&self) -> Option<&str> { self.mAppName.as_deref() }
    // Java: public void setAppName(String appName)
    pub fn setAppName(&mut self, appName: String) { self.mAppName = Some(appName); }
    // Java: public String lastEdited()
    pub fn lastEdited(&self) -> Option<&str> { self.mLastEdited.as_deref() }
    // Java: public void setLastEdited(String lastEdited)
    pub fn setLastEdited(&mut self, lastEdited: String) { self.mLastEdited = Some(lastEdited); }
    // Java: public String lowestEdited()
    pub fn lowestEdited(&self) -> Option<&str> { self.mLowestEdited.as_deref() }
    // Java: public void setLowestEdited(String lowestEdited)
    pub fn setLowestEdited(&mut self, lowestEdited: String) { self.mLowestEdited = Some(lowestEdited); }
    // Java: public String rupBuild()
    pub fn rupBuild(&self) -> Option<&str> { self.mRupBuild.as_deref() }
    // Java: public void setRupBuild(String rupBuild)
    pub fn setRupBuild(&mut self, rupBuild: String) { self.mRupBuild = Some(rupBuild); }
    // Java: public String codeName()
    pub fn codeName(&self) -> Option<&str> { self.mCodeName.as_deref() }
    // Java: public void setCodeName(String codeName)
    pub fn setCodeName(&mut self, codeName: String) { self.mCodeName = Some(codeName); }
}
