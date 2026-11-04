#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: import android.graphics.Paint
// Java: import android.graphics.Paint.Align

// Java field: int TA_NOUPDATECP
pub(crate) const TA_NOUPDATECP: i32 = 0;
// Java field: int TA_ALIGN_LEFT
pub(crate) const TA_ALIGN_LEFT: i32 = 0;
// Java field: int TA_ALIGN_TOP
pub(crate) const TA_ALIGN_TOP: i32 = 0;
// Java field: int TA_UPDATECP
pub(crate) const TA_UPDATECP: i32 = 1;
// Java field: int TA_RIGHT
pub(crate) const TA_RIGHT: i32 = 2;
// Java field: int TA_CENTER
pub(crate) const TA_CENTER: i32 = 6;
// Java field: int TA_BOTTOM
pub(crate) const TA_BOTTOM: i32 = 8;
// Java field: int TA_BASELINE
pub(crate) const TA_BASELINE: i32 = 24;
// Java field: int TA_RTLREADING
pub(crate) const TA_RTLREADING: i32 = 256;

// Java: public class TextAlign
pub struct TextAlign {
}

// Java: public class TextAlign
impl TextAlign {
    // Java: TextAlign()
    pub fn new() -> Self {
      // Java: super();
      TextAlign {}
    }

    // Java: static boolean isUpdateCp(int align)
    pub fn isUpdateCp(align: i32) -> bool {
      // Java: return (align & 1) == 1;
      (align & 1) == 1
    }

    // Java: static boolean isAlignLeft(int align)
    pub fn isAlignLeft(align: i32) -> bool {
      // Java: return !isAlignCenter(align) && !isAlignRight(align);
      !Self::isAlignCenter(align) && !Self::isAlignRight(align)
    }

    // Java: static boolean isAlignRight(int align)
    pub fn isAlignRight(align: i32) -> bool {
      // Java: return !isAlignCenter(align) && (align & 2) == 2;
      !Self::isAlignCenter(align) && (align & 2) == 2
    }

    // Java: static boolean isAlignCenter(int align)
    pub fn isAlignCenter(align: i32) -> bool {
      // Java: return (align & 6) == 6;
      (align & 6) == 6
    }

    // Java: static boolean isAlignBottom(int align)
    pub fn isAlignBottom(align: i32) -> bool {
      // Java: return !isAlignBaseLine(align) && (align & 8) == 8;
      !Self::isAlignBaseLine(align) && (align & 8) == 8
    }

    // Java: static boolean isAlignBaseLine(int align)
    pub fn isAlignBaseLine(align: i32) -> bool {
      // Java: return (align & 24) == 24;
      (align & 24) == 24
    }

}
