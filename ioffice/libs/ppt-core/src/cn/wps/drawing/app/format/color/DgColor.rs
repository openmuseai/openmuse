// Java: import cn.wps.kfc.util.KcBitsOp
// LLM No Rust equivalent: using local stub

struct Math;
impl Math {
    fn max(a: i32, b: i32) -> i32 { a.max(b) }
}

pub(crate) struct KcBitsOp {
    _mask: i32,
    _shift_count: i32,
}

impl KcBitsOp {
    pub const fn new(mask: i32) -> Self {
        let mut count = 0;
        let mut bit_pattern = mask;
        if mask != 0 {
            while (bit_pattern & 1) == 0 {
                count += 1;
                bit_pattern >>= 1;
            }
        }
        KcBitsOp { _mask: mask, _shift_count: count }
    }
    pub const fn getValue(&self, holder: i32) -> i32 {
        ((holder & self._mask) as u32 >> self._shift_count as u32) as i32
    }
    pub const fn setValue(&self, holder: i32, value: i32) -> i32 {
        (holder & !self._mask) | ((value << self._shift_count) & self._mask)
    }
}

static T_OP: KcBitsOp = KcBitsOp::new(-16777216);
static R_OP: KcBitsOp = KcBitsOp::new(16711680);
static G_OP: KcBitsOp = KcBitsOp::new(65280);
static B_OP: KcBitsOp = KcBitsOp::new(255);
static I_OP: KcBitsOp = KcBitsOp::new(16777215);

// Java field: int RGB
pub(crate) const RGB: i32 = 1;
// Java field: int SCHEME
pub(crate) const SCHEME: i32 = 4;
// Java field: int PRESET
pub(crate) const PRESET: i32 = 5;
// Java field: int SYSTEM
pub(crate) const SYSTEM: i32 = 3;
// Java field: int BLACK
pub(crate) const BLACK: i32 = 0x01000000;
// Java field: int WHITE
pub(crate) const WHITE: i32 = 0x01FFFFFF;
// Java field: int RED
pub(crate) const RED: i32 = 0x01FF0000;
// Java field: int GREEN
pub(crate) const GREEN: i32 = 0x0100FF00;
// Java field: int BLUE
pub(crate) const BLUE: i32 = 0x010000FF;
// Java field: int YELLOW
pub(crate) const YELLOW: i32 = 0x01FFFF00;

// Java: public class DgColor
pub struct DgColor {
}

// Java: public class Helper
pub struct Helper {
}

// Java: public class DgColor
impl DgColor {
    // Java: static int rgb(int rgb)
    pub fn rgb(rgb: i32) -> i32 {
      // Java: return $t.setValue(rgb, 1);
      T_OP.setValue(rgb, 1)
    }

    // Java: static int rgb(int r, int g, int b)
    pub fn rgb_components(r: i32, g: i32, b: i32) -> i32 {
      // Java: int clr = 0;
      let mut clr = 0;
      // Java: clr = $t.setValue(clr, 1);
      clr = T_OP.setValue(clr, 1);
      // Java: clr = $r.setValue(clr, DgColor.Helper.round(r));
      clr = R_OP.setValue(clr, Helper::round(r));
      // Java: clr = $g.setValue(clr, DgColor.Helper.round(g));
      clr = G_OP.setValue(clr, Helper::round(g));
      // Java: clr = $b.setValue(clr, DgColor.Helper.round(b));
      clr = B_OP.setValue(clr, Helper::round(b));
      // Java: return clr;
      clr
    }

    // Java: static int rgbPercentage(double rPercent, double gPercent, double bPercent)
    pub fn rgbPercentage(rPercent: f64, gPercent: f64, bPercent: f64) -> i32 {
      // Java: return rgb(DgColor.Helper.d2i(rPercent), DgColor.Helper.d2i(gPercent), DgColor.Helper.d2i(bPercent));
      DgColor::rgb_components(Helper::d2i(rPercent), Helper::d2i(gPercent), Helper::d2i(bPercent))
    }

    // Java: static int scheme(int index)
    pub fn scheme(index: i32) -> i32 {
      // Java: int clr = 0;
      let mut clr = 0;
      // Java: clr = $t.setValue(clr, 4);
      clr = T_OP.setValue(clr, 4);
      // Java: clr = $i.setValue(clr, index);
      clr = I_OP.setValue(clr, index);
      // Java: return clr;
      clr
    }

    // Java: static int system(int index)
    pub fn system(index: i32) -> i32 {
      // Java: int clr = 0;
      let mut clr = 0;
      // Java: clr = $t.setValue(clr, 3);
      clr = T_OP.setValue(clr, 3);
      // Java: clr = $i.setValue(clr, index);
      clr = I_OP.setValue(clr, index);
      // Java: return clr;
      clr
    }

    // Java: static int preset(int index)
    pub fn preset(index: i32) -> i32 {
      // Java: int clr = 0;
      let mut clr = 0;
      // Java: clr = $t.setValue(clr, 5);
      clr = T_OP.setValue(clr, 5);
      // Java: clr = $i.setValue(clr, index);
      clr = I_OP.setValue(clr, index);
      // Java: return clr;
      clr
    }

    // Java: static int type(int color)
    pub fn r#type(color: i32) -> i32 {
      // Java: return DgColor.Helper.typeRegulator($t.getValue(color));
      Helper::typeRegulator(T_OP.getValue(color))
    }

    // Java: static int r(int color)
    pub fn r(color: i32) -> i32 {
      // Java: return DgColor.Helper.round($r.getValue(color));
      Helper::round(R_OP.getValue(color))
    }

    // Java: static int g(int color)
    pub fn g(color: i32) -> i32 {
      // Java: return DgColor.Helper.round($g.getValue(color));
      Helper::round(G_OP.getValue(color))
    }

    // Java: static int b(int color)
    pub fn b(color: i32) -> i32 {
      // Java: return DgColor.Helper.round($b.getValue(color));
      Helper::round(B_OP.getValue(color))
    }

    // Java: static int indexValue(int color)
    pub fn indexValue(color: i32) -> i32 {
      // Java: return Math.max(0, $i.getValue(color));
      Math::max(0, I_OP.getValue(color))
    }

}

// Java: public class Helper
impl Helper {
    // Java: static int round(int clr)
    pub fn round(clr: i32) -> i32 {
         // Java: clr = Math.min(clr, 255);
         let clr = clr.min(255);
         // Java: clr = Math.max(clr, 0);
         let clr = clr.max(0);
         // Java: return clr;
         clr
    }

    // Java: static int d2i(double percent)
    pub fn d2i(percent: f64) -> i32 {
         // Java: percent = Math.min((double)1.0F, percent);
         let percent = percent.min(1.0);
         // Java: percent = Math.max((double)0.0F, percent);
         let percent = percent.max(0.0);
         // Java: return (int)(percent * (double)255.0F);
         (percent * 255.0) as i32
    }

    // Java: static int typeRegulator(int type)
    pub fn typeRegulator(type_: i32) -> i32 {
         // Java: switch (type)
         match type_ {
            // Java: case 1:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: case 3:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: case 4:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: case 5:
            // LLM JavaLine Aggregation: merged into the next Rust statement
               // Java: return type;
            1 | 3 | 4 | 5 => type_,
            // Java: case 2:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: default:
            // LLM JavaLine Aggregation: merged into the next Rust statement
               // Java: return 1;
            _ => 1,
         }
    }

}
