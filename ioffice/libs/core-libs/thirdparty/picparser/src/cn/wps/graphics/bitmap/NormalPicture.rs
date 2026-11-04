#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import cn.wps.graphics.BitmapCreator
// Java: import cn.wps.graphics.PictureInfo
// Java: import cn.wps.graphics.Size

use crate::cn::wps::graphics::bitmap::BitmapSizeTool;

pub struct PictureInfo {
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub pic_type: i32,
    pub byteLen: i32,
}

pub struct Size {
    pub width: i32,
    pub height: i32,
}
impl Size {
    pub fn new() -> Self { Size { width: 0, height: 0 } }
    pub fn isValid(&self) -> bool { self.width > 0 && self.height > 0 }
}

pub trait IPicture {
    fn getPath(&self) -> &str;
    fn getWidth(&self) -> i32;
    fn getHeight(&self) -> i32;
    fn getType(&self) -> i32;
    fn isSmall(&self) -> bool;
    fn isFixedSize(&self) -> bool;
    fn dispose(&mut self);
    fn setDiskCache(&mut self, pic: i32);
    fn getDiskCache(&self, width: i32, height: i32) -> Option<i32>;
    fn isTooSmall(&self, width: i32, height: i32, expectedWidth: i32, expectedHeight: i32) -> bool;
    fn isFastDraw(&self) -> bool;
}

// Java: public class NormalPicture implements IPicture
pub struct NormalPicture {
    // Java field: PictureInfo mInfo
    pub(crate) mInfo: Option<PictureInfo>,
}

// Java: public class NormalPicture implements IPicture
impl NormalPicture {
    // Java: NormalPicture(PictureInfo info)
    pub fn new(info: PictureInfo) -> Self {
      // Java: super();
      // Java: this.mInfo = info;
      NormalPicture { mInfo: Some(info) }
    }

    // Java: int getWidth()
    pub fn getWidth(&self) -> i32 {
      // Java: return this.mInfo.width;
      self.mInfo.as_ref().map_or(0, |i| i.width)
    }

    // Java: int getHeight()
    pub fn getHeight(&self) -> i32 {
      // Java: return this.mInfo.height;
      self.mInfo.as_ref().map_or(0, |i| i.height)
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: this.mInfo = null;
      self.mInfo = None;
    }

    // Java: String getPath()
    pub fn getPath(&self) -> &str {
      // Java: return this.mInfo.path;
      self.mInfo.as_ref().map_or("", |i| i.path.as_str())
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
      // Java: return this.mInfo.type == 0 ? -1 : 0;
      if self.mInfo.as_ref().map_or(0, |i| i.pic_type) == 0 { -1 } else { 0 }
    }

    // Java: boolean isSmall()
    pub fn isSmall(&self) -> bool {
      // Java: return this.mInfo.byteLen <= 16384;
      self.mInfo.as_ref().map_or(false, |i| i.byteLen <= 16384)
    }

    // Java: boolean isFixedSize()
    pub fn isFixedSize(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: void setDiskCache(IPicture pic)
    pub fn setDiskCache(&mut self, _pic: i32) {
    }

    // Java: IPicture getDiskCache(int width, int height)
    pub fn getDiskCache(&self, width: i32, height: i32) -> Option<i32> {
      // Java: return null;
      None
    }

    // Java: PictureInfo getPictureInfo()
    pub fn getPictureInfo(&self) -> Option<&PictureInfo> {
      // Java: return this.mInfo;
      self.mInfo.as_ref()
    }

    // Java: Size pretreatSize(Size size)
    pub fn pretreatSize(&self, size: Option<Size>) -> Size {
      // Java: if (null == size)
      let mut size = if size.is_none()
      {
         // Java: size = new Size();
         Size::new()
      } else {
         size.unwrap()
      };
      // Java: if (!size.isValid())
      if !size.isValid()
      {
         // Java: size.width = this.mInfo.width;
         size.width = self.mInfo.as_ref().map_or(0, |i| i.width);
         // Java: size.height = this.mInfo.height;
         size.height = self.mInfo.as_ref().map_or(0, |i| i.height);
      }
      // Java: return size;
      size
    }

    // Java: Size calBitmapSize(Size expectedSize, boolean bRankSize)
    pub fn calBitmapSize(&self, expectedSize: Option<Size>, bRankSize: bool) -> Size {
      // Java: expectedSize = this.pretreatSize(expectedSize);
      let mut expectedSize = self.pretreatSize(expectedSize);
      // Java: if (bRankSize)
      if bRankSize
      {
         // Java: BitmapSizeTool.rankSize(expectedSize);
         let area = (expectedSize.width * expectedSize.height) as f32;
         if !(area < 640000.0_f32) {
             let ratio = (640000.0_f32 / area).sqrt();
             expectedSize.width = (expectedSize.width as f32 * ratio) as i32;
             expectedSize.height = (expectedSize.height as f32 * ratio) as i32;
         }
      }
      // Java: int width = expectedSize.width;
      let width = expectedSize.width;
      // Java: int height = expectedSize.height;
      let height = expectedSize.height;
      // Java: float scaleX = 1.0F * (float)width / (float)this.mInfo.width;
      let info_width = self.mInfo.as_ref().map_or(1, |i| i.width);
      let info_height = self.mInfo.as_ref().map_or(1, |i| i.height);
      let scaleX = 1.0_f32 * width as f32 / info_width as f32;
      // Java: float scaleY = 1.0F * (float)height / (float)this.mInfo.height;
      let scaleY = 1.0_f32 * height as f32 / info_height as f32;
      // Java: if (scaleX > scaleY)
      if scaleX > scaleY
      {
         // Java: expectedSize.width = Math.min(this.mInfo.width, width);
         expectedSize.width = info_width.min(width);
         // Java: expectedSize.height = expectedSize.width * this.mInfo.height / this.mInfo.width;
         expectedSize.height = expectedSize.width * info_height / info_width;
      }
      // Java: else
      else
      {
         // Java: expectedSize.height = Math.min(this.mInfo.height, height);
         expectedSize.height = info_height.min(height);
         // Java: expectedSize.width = expectedSize.height * this.mInfo.width / this.mInfo.height;
         expectedSize.width = expectedSize.height * info_width / info_height;
      }
      // Java: return expectedSize;
      expectedSize
    }

    // Java: boolean isTooSmall(int width, int height, int expectedWidth, int expectedHeight)
    pub fn isTooSmall(&self, width: i32, height: i32, expectedWidth: i32, expectedHeight: i32) -> bool {
      // Java: return BitmapCreator.isTooSmall(expectedWidth, expectedHeight, width, height);
      let rate_width = expectedWidth as f32 / width as f32;
      let rate_height = expectedHeight as f32 / height as f32;
      rate_width > 1.5 || rate_height > 1.5
    }

    // Java: boolean isFastDraw()
    pub fn isFastDraw(&self) -> bool {
      // Java: return false;
      false
    }

}

impl IPicture for NormalPicture {
    fn getPath(&self) -> &str { NormalPicture::getPath(self) }
    fn getWidth(&self) -> i32 { NormalPicture::getWidth(self) }
    fn getHeight(&self) -> i32 { NormalPicture::getHeight(self) }
    fn getType(&self) -> i32 { NormalPicture::getType(self) }
    fn isSmall(&self) -> bool { NormalPicture::isSmall(self) }
    fn isFixedSize(&self) -> bool { NormalPicture::isFixedSize(self) }
    fn dispose(&mut self) { NormalPicture::dispose(self) }
    fn setDiskCache(&mut self, pic: i32) { NormalPicture::setDiskCache(self, pic) }
    fn getDiskCache(&self, width: i32, height: i32) -> Option<i32> { NormalPicture::getDiskCache(self, width, height) }
    fn isTooSmall(&self, width: i32, height: i32, expectedWidth: i32, expectedHeight: i32) -> bool { NormalPicture::isTooSmall(self, width, height, expectedWidth, expectedHeight) }
    fn isFastDraw(&self) -> bool { NormalPicture::isFastDraw(self) }
}

