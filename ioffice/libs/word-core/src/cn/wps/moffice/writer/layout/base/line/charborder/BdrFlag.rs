// Java: import cn.wps.moffice.util.BoolBitSet
use crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet;
// Java: import cn.wps.moffice.util.IntBitSet
use crate::cn::wps::moffice::util::IntBitSet::IntBitSet;

// java public static final IntBitSet WIDTH = new IntBitSet(0, 15);
pub(crate) static WIDTH: std::sync::LazyLock<IntBitSet> =
                    std::sync::LazyLock::new(|| IntBitSet::new(0, 15));
// java public static final BoolBitSet BEGIN = new BoolBitSet(16);
pub(crate) static BEGIN: std::sync::LazyLock<BoolBitSet> =
                    std::sync::LazyLock::new(|| BoolBitSet::new(16));
// java public static final BoolBitSet END = new BoolBitSet(17);
pub(crate) static END: std::sync::LazyLock<BoolBitSet> =
                    std::sync::LazyLock::new(|| BoolBitSet::new(17));
// java public static final BoolBitSet RTL = new BoolBitSet(18);
pub(crate) static RTL: std::sync::LazyLock<BoolBitSet> =
                    std::sync::LazyLock::new(|| BoolBitSet::new(18));
// java public static final BoolBitSet LEFT_MERGED = new BoolBitSet(19);
pub(crate) static LEFT_MERGED: std::sync::LazyLock<BoolBitSet> =
                    std::sync::LazyLock::new(|| BoolBitSet::new(19));
// java public static final BoolBitSet CLIP_VER = new BoolBitSet(20);
pub(crate) static CLIP_VER: std::sync::LazyLock<BoolBitSet> =
                    std::sync::LazyLock::new(|| BoolBitSet::new(20));

// java public class BdrFlag
#[derive(Default, Clone)]
pub struct BdrFlag {
}

// java public class BdrFlag
impl BdrFlag {
    // java public static final boolean hasBdr(int flag)
    pub fn hasBdr(flag: i32) -> bool {
      // java return getWidth(flag) != 0;
      return Self::getWidth(flag) != 0;
    }

    // java public static boolean hasBegin(int flag)
    pub fn hasBegin(flag: i32) -> bool {
      // java return BEGIN.get(flag);
      return BEGIN.get(flag);
    }

    // java public static boolean hasEnd(int flag)
    pub fn hasEnd(flag: i32) -> bool {
      // java return END.get(flag);
      return END.get(flag);
    }

    // java public static int getWidth(int flag)
    pub fn getWidth(flag: i32) -> i32 {
      // java return WIDTH.get(flag);
      return WIDTH.get(flag);
    }

    // java public static int setWidth(int flag, int width)
    pub fn setWidth(flag: i32, width: i32) -> i32 {
      // java return WIDTH.set(flag, width);
      return WIDTH.set(flag, width);
    }

    // java public static int setBegin(int flag, boolean v)
    pub fn setBegin(flag: i32, v: bool) -> i32 {
      // java return BEGIN.set(flag, v);
      return BEGIN.set(flag, v);
    }

    // java public static int setEnd(int flag, boolean v)
    pub fn setEnd(flag: i32, v: bool) -> i32 {
      // java return END.set(flag, v);
      return END.set(flag, v);
    }

    // java public static int setRtl(int flag, boolean rtl)
    pub fn setRtl(flag: i32, rtl: bool) -> i32 {
      // java return RTL.set(flag, rtl);
      return RTL.set(flag, rtl);
    }

    // java public static boolean isRtl(int flag)
    pub fn isRtl(flag: i32) -> bool {
      // java return RTL.get(flag);
      return RTL.get(flag);
    }

    // java public static int setLeftMerged(int flag, boolean leftMerged)
    pub fn setLeftMerged(flag: i32, leftMerged: bool) -> i32 {
      // java return LEFT_MERGED.set(flag, leftMerged);
      return LEFT_MERGED.set(flag, leftMerged);
    }

    // java public static int setClipVer(int flag, boolean clipVer)
    pub fn setClipVer(flag: i32, clipVer: bool) -> i32 {
      // java return CLIP_VER.set(flag, clipVer);
      return CLIP_VER.set(flag, clipVer);
    }

    // java public static boolean isClipVer(int flag)
    pub fn isClipVer(flag: i32) -> bool {
      // java return CLIP_VER.get(flag);
      return CLIP_VER.get(flag);
    }

    // java public static boolean isLeftMerged(int flag)
    pub fn isLeftMerged(flag: i32) -> bool {
      // java return LEFT_MERGED.get(flag);
      return LEFT_MERGED.get(flag);
    }

    // java public static String toString(int flag)
    pub fn toString(flag: i32) -> String {
      // java StringBuffer sb = new StringBuffer();
      let mut sb: String = String::new();
      // java if (flag == 0)
      if flag == 0 {
        // java sb.append("()");
        sb.push_str("()");
      }
      // java else
      else {
        // java sb.append(hasBegin(flag) ? "[" : "(");
        sb.push_str(if Self::hasBegin(flag) { "[" } else { "(" });
        // java sb.append(getWidth(flag));
        sb.push_str(&Self::getWidth(flag).to_string());
        // java sb.append(hasEnd(flag) ? "]" : ")");
        sb.push_str(if Self::hasEnd(flag) { "]" } else { ")" });
      }
      // java return sb.toString();
      return sb;
    }

}
