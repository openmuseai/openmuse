// Java: import cn.wps.moffice.writer.io.reader.xml07.helper.Util
use crate::cn::wps::moffice::writer::io::reader::xml07::helper::Util::*;

// java public class PartInfo
#[derive(Default)]
pub struct PartInfo {
    // java private String mEntryDir;
    mEntryDir: Option<String>,
    // java private String mPartName;
    mPartName: Option<String>,
    // java private PartInfo mNext;
    mNext: Option<Box<Self>>,
    // java private boolean mIsRels;
    mIsRels: bool,
    // java private String mOwnerPart;
    mOwnerPart: Option<String>,
    // java private String mRelsPart;
    mRelsPart: Option<String>,
}

// java public class PartInfo
impl PartInfo {
    // java public  PartInfo(String partName, String contentType)
    pub fn PartInfo(partName: &str, contentType: Option<&str>) -> Self {
      // java this.mIsRels = true;
      this.mIsRels = true;
      // java this.mPartName = partName;
      let mut this = Self { mEntryDir: Util::getEntryDir(Some(&mut self.mPartName)), mPartName: partName, mNext: Default::default(), mIsRels: Default::default(), mOwnerPart: None, mRelsPart: None };
      // java this.mEntryDir = Util.getEntryDir(this.mPartName);
      // java this.mOwnerPart = null;
      // java this.mRelsPart = null;
      this
    }

    // java public void setNext(PartInfo next)
    pub fn setNext(&mut self, next: &PartInfo) {
      // java this.mNext = next;
      self.mNext = next;
    }

    // java public PartInfo getNext()
    pub fn getNext(&self) -> PartInfo {
      // java return this.mNext;
      return self.mNext;
    }

    // java public String getPartName()
    pub fn getPartName(&self) -> String {
      // java return this.mPartName;
      return self.mPartName;
    }

    // java public String getEnetryDir()
    pub fn getEnetryDir(&self) -> String {
      // java return this.mEntryDir;
      return self.mEntryDir;
    }

    // java public String getOwnerPart()
    pub fn getOwnerPart(&mut self) -> Option<String> {
      // java return null != this.mOwnerPart ? this.mOwnerPart : (this.mOwnerPart = Util.getOwnerPart(this.mPartName));
      return (if self.mOwnerPart.is_some() { Some(self.mOwnerPart) } else { Some({ self.mOwnerPart = Util::getOwnerPart(Some(&mut self.mPartName)); self.mOwnerPart }) });
    }

    // java public String getRelsPart()
    pub fn getRelsPart(&mut self) -> Option<String> {
      // java if (null != this.mRelsPart)
      if let Some(mRelsPart) = self.mRelsPart.as_mut() {
        // java return this.mRelsPart;
        return mRelsPart;
      }
      // java else
      else {
        // java return this.mIsRels ? null : (this.mRelsPart = Util.getRelsFrom(this.mEntryDir, this.mPartName));
        return (if self.mIsRels { None } else { Some({ self.mRelsPart = Util::getRelsFrom(Some(&self.mEntryDir), Some(&mut self.mPartName)); self.mRelsPart }) });
      }
    }

}
