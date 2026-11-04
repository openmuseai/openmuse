use super::CLSID::CLSID;
use super::OLETIME::OLETIME;

// Java field: int OLEENTRY_EMPTY
pub(crate) const OLEENTRY_EMPTY: i32 = 0;
// Java field: int OLEENTRY_STORAGE
pub(crate) const OLEENTRY_STORAGE: i32 = 1;
// Java field: int OLEENTRY_STREAM
pub(crate) const OLEENTRY_STREAM: i32 = 2;
// Java field: int OLEENTRY_LOCKBYTES
pub(crate) const OLEENTRY_LOCKBYTES: i32 = 3;
// Java field: int OLEENTRY_PROPERTY
pub(crate) const OLEENTRY_PROPERTY: i32 = 4;
// Java field: int OLEENTRY_ROOTSTORAGE
pub(crate) const OLEENTRY_ROOTSTORAGE: i32 = 5;

// Java: public class OLEEntry
pub struct OLEEntry {
    // Java field: String szName
    pub(crate) szName: String,
    // Java field: int type
    pub(crate) r#type: i32,
    // Java field: int size
    pub(crate) size: i32,
    // Java field: CLSID clsid
    pub(crate) clsid: CLSID,
    // Java field: OLETIME ctime
    pub(crate) ctime: OLETIME,
    // Java field: OLETIME mtime
    pub(crate) mtime: OLETIME,
}

// Java: public class OLEEntry
impl OLEEntry {
    // Java: OLEEntry()
    pub fn new() -> Self {
      // Java: super();
      OLEEntry {
          szName: String::new(),
          r#type: 0,
          size: 0,
          clsid: CLSID::new(),
          ctime: OLETIME::new(),
          mtime: OLETIME::new(),
      }
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this.szName;
      &self.szName
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
      // Java: return this.type;
      self.r#type
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this.size;
      self.size
    }

    // Java: CLSID getCLSID()
    pub fn getCLSID(&self) -> &CLSID {
      // Java: return this.clsid;
      &self.clsid
    }

    // Java: OLETIME getCreationTime()
    pub fn getCreationTime(&self) -> &OLETIME {
      // Java: return this.ctime;
      &self.ctime
    }

    // Java: OLETIME getModificationTime()
    pub fn getModificationTime(&self) -> &OLETIME {
      // Java: return this.mtime;
      &self.mtime
    }

}
