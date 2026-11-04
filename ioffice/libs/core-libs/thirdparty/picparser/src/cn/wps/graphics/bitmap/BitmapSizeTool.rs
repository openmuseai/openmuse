#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.graphics.Size

pub struct Size {
    pub width: i32,
    pub height: i32,
}

// Java field: int RANK_MIN
pub(crate) const RANK_MIN: i32 = 3;
// Java field: int RANK_MAX
pub(crate) const RANK_MAX: i32 = 10;
// Java field: int RANK_COUNT
pub(crate) const RANK_COUNT: i32 = 8;
// Java field: int MIN_SIZE
pub(crate) const MIN_SIZE: i32 = 16;
// Java field: int MAX_SIZE
pub(crate) const MAX_SIZE: i32 = 800;
// Java field: int MAX_WIDTH
pub(crate) const MAX_WIDTH: i32 = 800;
// Java field: int MAX_HEIGHT
pub(crate) const MAX_HEIGHT: i32 = 800;
// Java field: int MAX_AREA
pub(crate) const MAX_AREA: i32 = 640000;

// Java: public class BitmapSizeTool
pub struct BitmapSizeTool {
}

// Java: public class BitmapSizeTool
impl BitmapSizeTool {
    // Java: BitmapSizeTool()
    pub fn new() -> Self {
      // Java: super();
      BitmapSizeTool {}
    }

    // Java: static int rankSize(int srcSize)
    pub fn rankSize_int(srcSize: i32) -> i32 {
      // Java: return getSize(rank(srcSize));
      Self::getSize(Self::rank(srcSize))
    }

    // Java: static int getSize(int rank)
    pub fn getSize(rank: i32) -> i32 {
      // Java: if (rank <= 3)
      if rank <= 3
      {
         // Java: return 16;
         return 16;
      }
      // Java: else
      else
      {
         // Java: return rank >= 10 ? 800 : 1 << rank;
         return if rank >= 10 { 800 } else { 1 << rank };
      }
    }

    // Java: static int rank(int value)
    pub fn rank(value: i32) -> i32 {
      // Java: int v = value;
      let mut v = value;
      // Java: if (value < 0)
      if value < 0
      {
         // Java: v = Math.abs(value);
         v = value.abs();
      }
      // Java: int rank;
      let mut rank: i32 = 0;
      // Java: for(rank = 0; v > 1; v >>= 1)
      while v > 1
      {
         // Java: ++rank;
         rank += 1;
         v >>= 1;
      }
      // Java: if (value > 1 << rank)
      if value > 1 << rank
      {
         // Java: ++rank;
         rank += 1;
      }
      // Java: return rank;
      rank
    }

    // Java: static void rankSize(Size expectedSize)
    pub fn rankSize_size(expectedSize: &mut Size) {
      // Java: int width = expectedSize.width;
      let width = expectedSize.width;
      // Java: int height = expectedSize.height;
      let height = expectedSize.height;
      // Java: float size = (float)(width * height);
      let size = (width * height) as f32;
      // Java: if (!(size < 640000.0F))
      if !(size < 640000.0_f32)
      {
         // Java: float ratio = (float)Math.sqrt((double)(640000.0F / size));
         let ratio = (640000.0_f32 / size).sqrt();
         // Java: expectedSize.width = (int)((float)width * ratio);
         expectedSize.width = (width as f32 * ratio) as i32;
         // Java: expectedSize.height = (int)((float)height * ratio);
         expectedSize.height = (height as f32 * ratio) as i32;
      }
    }

}
