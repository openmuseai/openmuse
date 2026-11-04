#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import cn.wps.graphics.PictureInfo
// Java: import cn.wps.graphics.Size
// Java: import cn.wps.moffice.kfs.File

use std::path::Path;
use std::fs;

use crate::cn::wps::graphics::bitmap::NormalPicture::{NormalPicture, PictureInfo, Size};
use crate::cn::wps::graphics::bitmap::BitmapSizeTool::BitmapSizeTool;

// Java field: int FAST_DRAW_MAX_SIZE
pub(crate) const FAST_DRAW_MAX_SIZE: i32 = 16384;

// Java: public class MetaPicture extends NormalPicture
pub struct MetaPicture {
    pub base: NormalPicture,
    // Java field: NormalPicture[] mPictures
    pub(crate) mPictures: Option<Vec<Option<NormalPicture>>>,
    // Java field: boolean mIsScaleByWidth
    mIsScaleByWidth: bool,
}

// Java: public class MetaPicture extends NormalPicture
impl MetaPicture {
    // Java: MetaPicture(PictureInfo info)
    pub fn new(info: PictureInfo) -> Self {
      // Java: super(info);
      let is_scale_by_width = info.width >= info.height;
      // Java: this.mIsScaleByWidth = info.width >= info.height;
      MetaPicture {
          base: NormalPicture::new(info),
          mPictures: None,
          mIsScaleByWidth: is_scale_by_width,
      }
    }

    // Java: int getIndex(int value)
    fn getIndex(&self, value: i32) -> i32 {
      // Java: int min = 3;
      let _min = 3;
      // Java: int max = 10;
      let _max = 10;
      // Java: int rank = BitmapSizeTool.rank(value);
      let rank = BitmapSizeTool::rank(value);
      // Java: int index = Math.max(Math.min(rank, 10), 3) - 3;
      let index = rank.min(10).max(3) - 3;
      // Java: return index;
      index
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: super.dispose();
      self.base.dispose();
      // Java: if (this.mPictures != null)
      if self.mPictures.is_some()
      {
         // Java: NormalPicture[] locPictures = this.mPictures;
         let mut pictures = self.mPictures.take().unwrap();
         // Java: int size = locPictures.length;
         let size = pictures.len();
         // Java: NormalPicture locPicture = null;
         // Java: String path = null;
         let mut path: Option<String> = None;
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size
         {
            // Java: locPicture = locPictures[i];
            let loc_picture = &mut pictures[i];
            // Java: if (null != locPicture)
            if loc_picture.is_some()
            {
               // Java: path = locPicture.getPath();
               path = Some(loc_picture.as_ref().unwrap().getPath().to_string());
               // Java: if (null != path && path.length() > 0)
               if path.as_ref().map_or(false, |p| !p.is_empty())
               {
                  // Java: File file = new File(path);
                  let file_path = path.as_ref().unwrap();
                  // Java: if (file.exists())
                  if Path::new(file_path).exists()
                  {
                     // Java: file.delete();
                     let _ = fs::remove_file(file_path);
                  }
               }
               // Java: locPicture.dispose();
               if let Some(ref mut pic) = loc_picture {
                   pic.dispose();
               }
            }
         }
         // Java: this.mPictures = null;
         self.mPictures = None;
      }
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
      // Java: return 1;
      1
    }

    // Java: void setDiskCache(IPicture pic)
    pub fn setDiskCache(&mut self, pic: &NormalPicture) {
      // Java: int value = this.mIsScaleByWidth ? pic.getWidth() : pic.getHeight();
      let value = if self.mIsScaleByWidth { pic.getWidth() } else { pic.getHeight() };
      // Java: int index = this.getIndex(value);
      let index = self.getIndex(value) as usize;
      // Java: if (this.mPictures == null)
      if self.mPictures.is_none()
      {
         // Java: this.mPictures = new NormalPicture[8];
         self.mPictures = Some(vec![None, None, None, None, None, None, None, None]);
      }
      // Java: this.mPictures[index] = (NormalPicture)pic;
    }

    // Java: NormalPicture getDiskCache(int width, int height)
    pub fn getDiskCache(&self, width: i32, height: i32) -> Option<&NormalPicture> {
      // Java: if (this.mPictures == null)
      if self.mPictures.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: int value = this.mIsScaleByWidth ? width : height;
         let value = if self.mIsScaleByWidth { width } else { height };
         // Java: int index = this.getIndex(value);
         let index = self.getIndex(value) as usize;
         // Java: return this.mPictures[index];
         self.mPictures.as_ref().and_then(|pics| pics.get(index).and_then(|p| p.as_ref()))
      }
    }

    // Java: boolean isSmall()
    pub fn isSmall(&self) -> bool {
      // Java: return this.mInfo.byteLen <= 16384;
      self.base.mInfo.as_ref().map_or(false, |i| i.byteLen <= 16384)
    }

    // Java: boolean isFixedSize()
    pub fn isFixedSize(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: Size calBitmapSize(Size expectedSize, boolean bRankSize)
    pub fn calBitmapSize(&self, expectedSize: Option<Size>, bRankSize: bool) -> Size {
      // Java: expectedSize = this.pretreatSize(expectedSize);
      let mut expectedSize = self.base.pretreatSize(expectedSize);
      // Java: if (this.mInfo.width != 0 && this.mInfo.height != 0)
      let info_width = self.base.mInfo.as_ref().map_or(0, |i| i.width);
      let info_height = self.base.mInfo.as_ref().map_or(0, |i| i.height);
      if info_width != 0 && info_height != 0
      {
         // Java: if (bRankSize)
         if bRankSize
         {
            // Java: int width = expectedSize.width;
            let width = expectedSize.width;
            // Java: int height = expectedSize.height;
            let height = expectedSize.height;
            // Java: if (this.mIsScaleByWidth)
            if self.mIsScaleByWidth
            {
               // Java: expectedSize.width = BitmapSizeTool.rankSize(width);
               expectedSize.width = BitmapSizeTool::rankSize_int(width);
               // Java: expectedSize.height = expectedSize.width * this.mInfo.height / this.mInfo.width;
               expectedSize.height = expectedSize.width * info_height / info_width;
            }
            // Java: else
            else
            {
               // Java: expectedSize.height = BitmapSizeTool.rankSize(height);
               expectedSize.height = BitmapSizeTool::rankSize_int(height);
               // Java: expectedSize.width = expectedSize.height * this.mInfo.width / this.mInfo.height;
               expectedSize.width = expectedSize.height * info_width / info_height;
            }
         }
         // Java: return expectedSize;
         return expectedSize;
      }
      // Java: else
      else
      {
         // Java: return expectedSize;
         return expectedSize;
      }
    }

    // Java: boolean isFastDraw()
    pub fn isFastDraw(&self) -> bool {
      // Java: if (this.mInfo.byteLen <= 16384)
      let byte_len = self.base.mInfo.as_ref().map_or(0, |i| i.byteLen);
      let info_width = self.base.mInfo.as_ref().map_or(1, |i| i.width);
      let info_height = self.base.mInfo.as_ref().map_or(1, |i| i.height);
      if byte_len <= 16384
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: float ratio = (float)this.mInfo.byteLen / (float)(this.mInfo.width * this.mInfo.height);
         let ratio = byte_len as f32 / (info_width * info_height) as f32;
         // Java: return ratio < 0.01F;
         return ratio < 0.01_f32;
      }
    }

}
