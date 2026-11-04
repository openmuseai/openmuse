// Java: import cn.wps.moffice.writer.layout.base.line.charborder.BdrFlag
use crate::cn::wps::moffice::writer::layout::base::line::charborder::BdrFlag::BdrFlag;

// java public class BdrInfo extends BdrFlag
#[derive(Default, Clone)]
pub struct BdrInfo {
    // java BdrFlag super_
    pub(crate) super_: BdrFlag,
    // java int mFlag = 0;
    pub(crate) mFlag: i32,
}

// java public class BdrInfo extends BdrFlag
impl BdrInfo {
    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mFlag = 0;
      self.mFlag = 0;
    }

    // java public boolean hasBdr()
    pub fn hasBdr(&self) -> bool {
      // java return hasBdr(this.mFlag);
      return BdrFlag::hasBdr(self.mFlag);
    }

    // java public void clearWidth()
    pub fn clearWidth(&mut self) {
      // java this.mFlag = 0;
      self.mFlag = 0;
    }

    // java public boolean hasBegin()
    pub fn hasBegin(&self) -> bool {
      // java return hasBegin(this.mFlag);
      return BdrFlag::hasBegin(self.mFlag);
    }

    // java public void setBegin(int width)
    pub fn setBegin(&mut self, width: i32) {
      // java this.setWidth(width);
      self.setWidth(width);
      // java this.mFlag = setBegin(this.mFlag, true);
      self.mFlag = BdrFlag::setBegin(self.mFlag, true);
    }

    // java public boolean hasEnd()
    pub fn hasEnd(&self) -> bool {
      // java return hasEnd(this.mFlag);
      return BdrFlag::hasEnd(self.mFlag);
    }

    // java public void setEnd(int width)
    pub fn setEnd(&mut self, width: i32) {
      // java this.setWidth(width);
      self.setWidth(width);
      // java this.mFlag = setEnd(this.mFlag, true);
      self.mFlag = BdrFlag::setEnd(self.mFlag, true);
    }

    // java public void setWidth(int width)
    pub fn setWidth(&mut self, width: i32) {
      // java this.mFlag = setWidth(this.mFlag, width);
      self.mFlag = BdrFlag::setWidth(self.mFlag, width);
    }

    // java public int getWidth()
    pub fn getWidth(&self) -> i32 {
      // java return getWidth(this.mFlag);
      return BdrFlag::getWidth(self.mFlag);
    }

    // java public void setRtl(boolean rtl)
    pub fn setRtl(&mut self, rtl: bool) {
      // java this.mFlag = setRtl(this.mFlag, rtl);
      self.mFlag = BdrFlag::setRtl(self.mFlag, rtl);
    }

    // java public void setLeftMerged(boolean leftMerged)
    pub fn setLeftMerged(&mut self, leftMerged: bool) {
      // java this.mFlag = setLeftMerged(this.mFlag, leftMerged);
      self.mFlag = BdrFlag::setLeftMerged(self.mFlag, leftMerged);
    }

    // java public boolean isLeftMerged()
    pub fn isLeftMerged(&self) -> bool {
      // java return isLeftMerged(this.mFlag);
      return BdrFlag::isLeftMerged(self.mFlag);
    }

    // java public void setClipVer(boolean clipVer)
    pub fn setClipVer(&mut self, clipVer: bool) {
      // java this.mFlag = setClipVer(this.mFlag, clipVer);
      self.mFlag = BdrFlag::setClipVer(self.mFlag, clipVer);
    }

    // java public boolean isClipVer()
    pub fn isClipVer(&self) -> bool {
      // java return isClipVer(this.mFlag);
      return BdrFlag::isClipVer(self.mFlag);
    }

    // java public boolean isRtl()
    pub fn isRtl(&self) -> bool {
      // java return isRtl(this.mFlag);
      return BdrFlag::isRtl(self.mFlag);
    }

    // java public int getFlag()
    pub fn getFlag(&self) -> i32 {
      // java return this.mFlag;
      return self.mFlag;
    }

    // java public void setFlag(int flag)
    pub fn setFlag(&mut self, flag: i32) {
      // java this.mFlag = flag;
      self.mFlag = flag;
    }

    // java public void copyTo(BdrInfo dest)
    pub fn copyTo(&self, dest: Option<&mut BdrInfo>) {
      // java if (dest != null)
      if let Some(dest) = dest {
        // java dest.setFlag(this.mFlag);
        dest.setFlag(self.mFlag);
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java return "BDR" + BdrFlag.toString(this.mFlag);
      return format!("BDR{}", BdrFlag::toString(self.mFlag));
    }

}

impl std::ops::Deref for BdrInfo {
    type Target = BdrFlag;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for BdrInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
