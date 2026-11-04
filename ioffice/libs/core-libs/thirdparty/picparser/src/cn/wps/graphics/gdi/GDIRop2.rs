#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java field: int BLACK
pub(crate) const BLACK: i32 = 1;
// Java field: int NOTMERGEPEN
pub(crate) const NOTMERGEPEN: i32 = 2;
// Java field: int MASKNOTPEN
pub(crate) const MASKNOTPEN: i32 = 3;
// Java field: int NOTCOPYPEN
pub(crate) const NOTCOPYPEN: i32 = 4;
// Java field: int MASKPENNOT
pub(crate) const MASKPENNOT: i32 = 5;
// Java field: int NOT
pub(crate) const NOT: i32 = 6;
// Java field: int XORPEN
pub(crate) const XORPEN: i32 = 7;
// Java field: int NOTMASKPEN
pub(crate) const NOTMASKPEN: i32 = 8;
// Java field: int MASKPEN
pub(crate) const MASKPEN: i32 = 9;
// Java field: int NOTXORPEN
pub(crate) const NOTXORPEN: i32 = 10;
// Java field: int NOP
pub(crate) const NOP: i32 = 11;
// Java field: int MERGENOTPEN
pub(crate) const MERGENOTPEN: i32 = 12;
// Java field: int COPYPEN
pub(crate) const COPYPEN: i32 = 13;
// Java field: int MERGEPENNOT
pub(crate) const MERGEPENNOT: i32 = 14;
// Java field: int MERGEPEN
pub(crate) const MERGEPEN: i32 = 15;
// Java field: int WHITE
pub(crate) const WHITE: i32 = 16;

// Java: public class GDIRop2
pub struct GDIRop2 {
}

// Java: public class GDIRop2
impl GDIRop2 {
    // Java: GDIRop2()
    pub fn new() -> Self {
      // Java: super();
      GDIRop2 {}
    }

    // Java: static IGdiPainter getPaintByRop(IGdiPainter paint, int rop2)
    pub fn getPaintByRop(paint: i32, rop2: i32) -> i32 {
      // Java: switch (rop2)
      match rop2
      {
         // Java: case 1:
         1 =>
         {
            // Java: return ColorPaint.BLACK;
            return 0x000000;
         }
         // Java: case 16:
         16 =>
         {
            // Java: return ColorPaint.WHITE;
            return 0xFFFFFF;
         }
         // Java: default:
         _ =>
         {
            // Java: return paint;
            return paint;
         }
      }
    }

}
