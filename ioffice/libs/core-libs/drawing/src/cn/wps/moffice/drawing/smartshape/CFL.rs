#![allow(non_snake_case)]
#![allow(dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.math.MathUtil
use core_libs_util::math::MathUtil::MathUtil;
// Java: import cn.wps.base.assertion.Assert
use core_libs_util::base::assertion::Assert::Assert;

// Java field: int CONSTANT_UPPER_BOUND
pub(crate) const CONSTANT_UPPER_BOUND: i32 = i32::MIN;
// Java field: int FORMULA_RANGE
pub(crate) const FORMULA_RANGE: i32 = 127;
// Java field: int FORMULA_LOWER_BOUND
pub(crate) const FORMULA_LOWER_BOUND: i32 = i32::MIN;
// Java field: int FORMULA_UPPER_BOUND
pub(crate) const FORMULA_UPPER_BOUND: i32 = -2147483521;
// Java field: int ADJUST_RANGE
pub(crate) const ADJUST_RANGE: i32 = 10;
// Java field: int ADJUST_LOWER_BOUND
pub(crate) const ADJUST_LOWER_BOUND: i32 = -2147483521;
// Java field: int ADJUST_UPPER_BOUND
pub(crate) const ADJUST_UPPER_BOUND: i32 = -2147483511;
// Java field: int SYMBOLIC_VAR_RANGE
pub(crate) const SYMBOLIC_VAR_RANGE: i32 = 48;
// Java field: int SYMBOLIC_VAR_LOWER_BOUND
pub(crate) const SYMBOLIC_VAR_LOWER_BOUND: i32 = -2147483511;
// Java field: int SYMBOLIC_VAR_UPPER_BOUND
pub(crate) const SYMBOLIC_VAR_UPPER_BOUND: i32 = -2147483463;
// Java field: int UNITS_PER_DEGREE_FOR_VML
pub(crate) const UNITS_PER_DEGREE_FOR_VML: i32 = 65535;
// Java field: int UNITS_PER_DEGREE_FOR_DML
pub(crate) const UNITS_PER_DEGREE_FOR_DML: i32 = 60000;
// Java field: double DEGREE_2_RADIAN
pub(crate) const DEGREE_2_RADIAN: f64 = std::f64::consts::PI / 180.0_f64;
// Java field: int S_WIDTH
pub(crate) const S_WIDTH: i32 = 0;
// Java field: int S_HEIGHT
pub(crate) const S_HEIGHT: i32 = 1;
// Java field: int S_XCENTER
pub(crate) const S_XCENTER: i32 = 2;
// Java field: int S_YCENTER
pub(crate) const S_YCENTER: i32 = 3;
// Java field: int S_XLIMO
pub(crate) const S_XLIMO: i32 = 4;
// Java field: int S_YLIMO
pub(crate) const S_YLIMO: i32 = 5;
// Java field: int S_HASSTROKE
pub(crate) const S_HASSTROKE: i32 = 6;
// Java field: int S_HASFILL
pub(crate) const S_HASFILL: i32 = 7;
// Java field: int S_PIXELLINEWIDTH
pub(crate) const S_PIXELLINEWIDTH: i32 = 8;
// Java field: int S_PIXELWIDTH
pub(crate) const S_PIXELWIDTH: i32 = 9;
// Java field: int S_PIXELHEIGHT
pub(crate) const S_PIXELHEIGHT: i32 = 10;
// Java field: int S_EMUWIDTH
pub(crate) const S_EMUWIDTH: i32 = 11;
// Java field: int S_EMUHEIGHT
pub(crate) const S_EMUHEIGHT: i32 = 12;
// Java field: int S_EMUWIDTH2
pub(crate) const S_EMUWIDTH2: i32 = 13;
// Java field: int S_EMUHEIGHT2
pub(crate) const S_EMUHEIGHT2: i32 = 14;
// Java field: int S_3CD4
pub(crate) const S_3CD4: i32 = 15;
// Java field: int S_3CD8
pub(crate) const S_3CD8: i32 = 16;
// Java field: int S_5CD8
pub(crate) const S_5CD8: i32 = 17;
// Java field: int S_7CD8
pub(crate) const S_7CD8: i32 = 18;
// Java field: int S_CD2
pub(crate) const S_CD2: i32 = 19;
// Java field: int S_CD4
pub(crate) const S_CD4: i32 = 20;
// Java field: int S_CD8
pub(crate) const S_CD8: i32 = 21;
// Java field: int S_HD4
pub(crate) const S_HD4: i32 = 22;
// Java field: int S_HD5
pub(crate) const S_HD5: i32 = 23;
// Java field: int S_HD6
pub(crate) const S_HD6: i32 = 24;
// Java field: int S_HD8
pub(crate) const S_HD8: i32 = 25;
// Java field: int S_LEFT
pub(crate) const S_LEFT: i32 = 26;
// Java field: int S_TOP
pub(crate) const S_TOP: i32 = 26;
// Java field: int S_LS
pub(crate) const S_LS: i32 = 27;
// Java field: int S_SS
pub(crate) const S_SS: i32 = 28;
// Java field: int S_SSD2
pub(crate) const S_SSD2: i32 = 29;
// Java field: int S_SSD4
pub(crate) const S_SSD4: i32 = 30;
// Java field: int S_SSD6
pub(crate) const S_SSD6: i32 = 31;
// Java field: int S_SSD8
pub(crate) const S_SSD8: i32 = 32;
// Java field: int S_WD4
pub(crate) const S_WD4: i32 = 33;
// Java field: int S_WD5
pub(crate) const S_WD5: i32 = 34;
// Java field: int S_WD6
pub(crate) const S_WD6: i32 = 35;
// Java field: int S_WD8
pub(crate) const S_WD8: i32 = 36;
// Java field: int S_WD10
pub(crate) const S_WD10: i32 = 37;
// Java field: int S_HD3
pub(crate) const S_HD3: i32 = 38;
// Java field: int S_WD3
pub(crate) const S_WD3: i32 = 39;
// Java field: int S_SSD16
pub(crate) const S_SSD16: i32 = 40;
// Java field: int S_SSD32
pub(crate) const S_SSD32: i32 = 41;
// Java field: int S_HD7
pub(crate) const S_HD7: i32 = 42;
// Java field: int S_WD32
pub(crate) const S_WD32: i32 = 43;
// Java field: int S_CD3
pub(crate) const S_CD3: i32 = 44;
// Java field: int S_HD10
pub(crate) const S_HD10: i32 = 45;
// Java field: int S_WD12
pub(crate) const S_WD12: i32 = 46;
// Java field: int S_LINE_DRAWN
pub(crate) const S_LINE_DRAWN: i32 = 47;
// Java field: int F_ADDSUB
pub(crate) const F_ADDSUB: i32 = 0;
// Java field: int F_MULDIV
pub(crate) const F_MULDIV: i32 = 1;
// Java field: int F_ADDDIV
pub(crate) const F_ADDDIV: i32 = 2;
// Java field: int F_ABS
pub(crate) const F_ABS: i32 = 3;
// Java field: int F_MIN
pub(crate) const F_MIN: i32 = 4;
// Java field: int F_MAX
pub(crate) const F_MAX: i32 = 5;
// Java field: int F_IFELSE
pub(crate) const F_IFELSE: i32 = 6;
// Java field: int F_MOD
pub(crate) const F_MOD: i32 = 7;
// Java field: int F_ATAN2_DML
pub(crate) const F_ATAN2_DML: i32 = 19;
// Java field: int F_SIN_DML
pub(crate) const F_SIN_DML: i32 = 20;
// Java field: int F_COS_DML
pub(crate) const F_COS_DML: i32 = 21;
// Java field: int F_COSATAN2
pub(crate) const F_COSATAN2: i32 = 11;
// Java field: int F_SINATAN2
pub(crate) const F_SINATAN2: i32 = 12;
// Java field: int F_SQRT
pub(crate) const F_SQRT: i32 = 13;
// Java field: int F_SUMANGLE
pub(crate) const F_SUMANGLE: i32 = 14;
// Java field: int F_ELLIPSE
pub(crate) const F_ELLIPSE: i32 = 15;
// Java field: int F_TAN_DML
pub(crate) const F_TAN_DML: i32 = 22;
// Java field: int F_PIN
pub(crate) const F_PIN: i32 = 17;
// Java field: int F_VAL
pub(crate) const F_VAL: i32 = 18;
// Java field: int F_ATAN2
pub(crate) const F_ATAN2: i32 = 8;
// Java field: int F_SIN
pub(crate) const F_SIN: i32 = 9;
// Java field: int F_COS
pub(crate) const F_COS: i32 = 10;
// Java field: int F_TAN
pub(crate) const F_TAN: i32 = 16;
// Java field: long CONSTANT_UPPER_BOUND_LONG
pub(crate) const CONSTANT_UPPER_BOUND_LONG: i64 = i32::MIN as i64;

struct Math;
impl Math {
    fn max(a: f32, b: f32) -> f32 { f32::max(a, b) }
    fn min(a: f32, b: f32) -> f32 { f32::min(a, b) }
}


// Java: public class CFL
pub struct CFL {
}

// Java: public class CFL
impl CFL {
    // Java: CFL()
    pub fn new() -> Self  {
      // Java: super();
      CFL {}
    }

    // Java: static int getAvcGdIndex(int pcArg)
    pub fn getAvcGdIndex(pcArg: i32) -> i32 {
      // Java: return pcArg - 48;
      pcArg - 48
    }

    // Java: static int getArgCnt(int avcGdIndex)
    pub fn getArgCnt(avcGdIndex: i32) -> i32 {
      // Java: return avcGdIndex >> 16;
      avcGdIndex >> 16
    }

    // Java: static int getFormulaIndex(int avcGdIndex)
    pub fn getFormulaIndex(avcGdIndex: i32) -> i32 {
      // Java: return avcGdIndex & '\uffff';
      avcGdIndex & 0xFFFF
    }

    // Java: static float getSymbolicValue(int val, RectF rect)
    pub fn getSymbolicValue(val: i32, rect: &RectF) -> f32 {
        // Java: switch (val)
        match val {
            // Java: case 0:
            0 => {
                // Java: return _width(rect);
                Self::_width(rect)
            },
            // Java: case 1:
            1 => {
                // Java: return _height(rect);
                Self::_height(rect)
            },
            // Java: case 2:
            2 => {
                // Java: return _xCenter(rect);
                Self::_xCenter(rect)
            },
            // Java: case 3:
            3 => {
                // Java: return _yCenter(rect);
                Self::_yCenter(rect)
            },
            4 | // Java: case 4:
            5 | // Java: case 5:
            6 | // Java: case 6:
            7 | // Java: case 7:
            8 | // Java: case 8:
            9 | // Java: case 9:
            10 | // Java: case 10:
            11 | // Java: case 11:
            12 | // Java: case 12:
            13 | // Java: case 13:
            14 => { // Java: case 14:
                // Java: Assert.logFail("Unrecognized symbolic variable is met: " + val);
                Assert::logFail_msg(&format!("Unrecognized symbolic variable is met: {}", val));
                // Java: return 0.0F;
                0.0_f32
            },
            // Java: case 15:
            15 => {
                // Java: return 1.62E7F;
                1.62E7_f32
            },
            // Java: case 16:
            16 => {
                // Java: return 8100000.0F;
                8100000.0_f32
            },
            // Java: case 17:
            17 => {
                // Java: return 1350000.0F;
                1350000.0_f32
            },
            // Java: case 18:
            18 => {
                // Java: return 1.89E7F;
                1.89E7_f32
            },
            // Java: case 19:
            19 => {
                // Java: return 1.08E7F;
                1.08E7_f32
            },
            // Java: case 20:
            20 => {
                // Java: return 5400000.0F;
                5400000.0_f32
            },
            // Java: case 21:
            21 => {
                // Java: return 2700000.0F;
                2700000.0_f32
            },
            // Java: case 22:
            22 => {
                // Java: return _hd4(rect);
                Self::_hd4(rect)
            },
            // Java: case 23:
            23 => {
                // Java: return _hd5(rect);
                Self::_hd5(rect)
            },
            // Java: case 24:
            24 => {
                // Java: return _hd6(rect);
                Self::_hd6(rect)
            },
            // Java: case 25:
            25 => {
                // Java: return _hd8(rect);
                Self::_hd8(rect)
            },
            // Java: case 26:
            26 => {
                // Java: return 0.0F;
                0.0_f32
            },
            // Java: case 27:
            27 => {
                // Java: return _ls(rect);
                Self::_ls(rect)
            },
            // Java: case 28:
            28 => {
                // Java: return _ss(rect);
                Self::_ss(rect)
            },
            // Java: case 29:
            29 => {
                // Java: return _ssd2(rect);
                Self::_ssd2(rect)
            },
            // Java: case 30:
            30 => {
                // Java: return _ssd4(rect);
                Self::_ssd4(rect)
            },
            // Java: case 31:
            31 => {
                // Java: return _ssd6(rect);
                Self::_ssd6(rect)
            },
            // Java: case 32:
            32 => {
                // Java: return _ssd8(rect);
                Self::_ssd8(rect)
            },
            // Java: case 33:
            33 => {
                // Java: return _wd4(rect);
                Self::_wd4(rect)
            },
            // Java: case 34:
            34 => {
                // Java: return _wd5(rect);
                Self::_wd5(rect)
            },
            // Java: case 35:
            35 => {
                // Java: return _wd6(rect);
                Self::_wd6(rect)
            },
            // Java: case 36:
            36 => {
                // Java: return _wd8(rect);
                Self::_wd8(rect)
            },
            // Java: case 37:
            37 => {
                // Java: return _wd10(rect);
                Self::_wd10(rect)
            },
            // Java: case 38:
            38 => {
                // Java: return _hd3(rect);
                Self::_hd3(rect)
            },
            // Java: case 39:
            39 => {
                // Java: return _wd3(rect);
                Self::_wd3(rect)
            },
            // Java: case 40:
            40 => {
                // Java: return _ssd16(rect);
                Self::_ssd16(rect)
            },
            // Java: case 41:
            41 => {
                // Java: return _ssd32(rect);
                Self::_ssd32(rect)
            },
            // Java: case 42:
            42 => {
                // Java: return _hd7(rect);
                Self::_hd7(rect)
            },
            // Java: case 43:
            43 => {
                // Java: return _wd32(rect);
                Self::_wd32(rect)
            },
            // Java: case 44:
            44 => {
                // Java: return 7200000.0F;
                7200000.0_f32
            },
            // Java: case 45:
            45 => {
                // Java: return _hd10(rect);
                Self::_hd10(rect)
            },
            // Java: case 46:
            46 => {
                // Java: return _wd12(rect);
                Self::_wd12(rect)
            },
            // Java: default:
            _ => {
                // Java: Assert.logFail("Unrecognized symbolic variable is met: " + val);
                Assert::logFail_msg(&format!("Unrecognized symbolic variable is met: {}", val));
                // Java: return 0.0F;
                0.0_f32
            },
        }
    }

    // Java: static float getFormulasValue(int index, float[] args)
    pub fn getFormulasValue(index: i32, args: &[f32]) -> f32 {
        // Java: switch (index)
        match index {
            // Java: case 0:
            0 => {
                // Java: return _sum(args[0], args[1], args[2]);
                Self::_sum(args[0], args[1], args[2])
            },
            // Java: case 1:
            1 => {
                // Java: return _product(args[0], args[1], args[2]);
                Self::_product(args[0], args[1], args[2])
            },
            // Java: case 2:
            2 => {
                // Java: if (args.length == 2)
                if args.len() == 2 {
                    // Java: return _mid(args[0], args[1], 2.0F);
                    Self::_mid(args[0], args[1], 2.0_f32)
                } else {
                    // Java: return _mid(args[0], args[1], args[2]);
                    Self::_mid(args[0], args[1], args[2])
                }
            },
            // Java: case 3:
            3 => {
                // Java: return _abs(args[0]);
                Self::_abs(args[0])
            },
            // Java: case 4:
            4 => {
                // Java: return _min(args[0], args[1]);
                Self::_min(args[0], args[1])
            },
            // Java: case 5:
            5 => {
                // Java: return _max(args[0], args[1]);
                Self::_max(args[0], args[1])
            },
            // Java: case 6:
            6 => {
                // Java: return _ifElse(args[0], args[1], args[2]);
                Self::_ifElse(args[0], args[1], args[2])
            },
            // Java: case 7:
            7 => {
                // Java: return _mod(args[0], args[1], args[2]);
                Self::_mod(args[0], args[1], args[2])
            },
            // Java: case 8:
            8 => {
                // Java: return _atan2(args[0], args[1]);
                Self::_atan2(args[0], args[1])
            },
            // Java: case 9:
            9 => {
                // Java: return _sin(args[0], args[1]);
                Self::_sin(args[0], args[1])
            },
            // Java: case 10:
            10 => {
                // Java: return _cos(args[0], args[1]);
                Self::_cos(args[0], args[1])
            },
            // Java: case 11:
            11 => {
                // Java: return _cosatan2(args[0], args[1], args[2]);
                Self::_cosatan2(args[0], args[1], args[2])
            },
            // Java: case 12:
            12 => {
                // Java: return _sinatan2(args[0], args[1], args[2]);
                Self::_sinatan2(args[0], args[1], args[2])
            },
            // Java: case 13:
            13 => {
                // Java: return _sqrt(args[0]);
                Self::_sqrt(args[0])
            },
            // Java: case 14:
            14 => {
                // Java: return _sumangle(args[0], args[1], args[2]);
                Self::_sumangle(args[0], args[1], args[2])
            },
            // Java: case 15:
            15 => {
                // Java: return _ellipse(args[0], args[1], args[2]);
                Self::_ellipse(args[0], args[1], args[2])
            },
            // Java: case 16:
            16 => {
                // Java: return _tan(args[0], args[1]);
                Self::_tan(args[0], args[1])
            },
            // Java: case 17:
            17 => {
                // Java: return _pin(args[0], args[1], args[2]);
                Self::_pin(args[0], args[1], args[2])
            },
            // Java: case 18:
            18 => {
                // Java: return _val(args[0]);
                Self::_val(args[0])
            },
            // Java: case 19:
            19 => {
                // Java: return _atan2_dml(args[0], args[1]);
                Self::_atan2_dml(args[0], args[1])
            },
            // Java: case 20:
            20 => {
                // Java: return _sin_dml(args[0], args[1]);
                Self::_sin_dml(args[0], args[1])
            },
            // Java: case 21:
            21 => {
                // Java: return _cos_dml(args[0], args[1]);
                Self::_cos_dml(args[0], args[1])
            },
            // Java: case 22:
            22 => {
                // Java: return _tan_dml(args[0], args[1]);
                Self::_tan_dml(args[0], args[1])
            },
            // Java: default:
            _ => {
                // Java: Assert.logFail("Unrecognized formula operation is met: " + index);
                Assert::logFail();
                // Java: return 0.0F;
                0.0_f32
            },
        }
    }

    // Java: static float _width(RectF rc)
    pub fn _width(rc: &RectF) -> f32 {
      // Java: return rc.width();
      rc.width()
    }

    // Java: static float _height(RectF rc)
    pub fn _height(rc: &RectF) -> f32 {
      // Java: return rc.height();
      rc.height()
    }

    // Java: static float _xCenter(RectF rc)
    pub fn _xCenter(rc: &RectF) -> f32 {
      // Java: return rc.width() / 2.0F;
      rc.width() / 2.0_f32
    }

    // Java: static float _yCenter(RectF rc)
    pub fn _yCenter(rc: &RectF) -> f32 {
      // Java: return rc.height() / 2.0F;
      rc.height() / 2.0_f32
    }

    // Java: static float _hd4(RectF rc)
    pub fn _hd4(rc: &RectF) -> f32 {
      // Java: return rc.height() / 4.0F;
      rc.height() / 4.0_f32
    }

    // Java: static float _hd5(RectF rc)
    pub fn _hd5(rc: &RectF) -> f32 {
      // Java: return rc.height() / 5.0F;
      rc.height() / 5.0_f32
    }

    // Java: static float _hd6(RectF rc)
    pub fn _hd6(rc: &RectF) -> f32 {
      // Java: return rc.height() / 6.0F;
      rc.height() / 6.0_f32
    }

    // Java: static float _hd8(RectF rc)
    pub fn _hd8(rc: &RectF) -> f32 {
      // Java: return rc.height() / 8.0F;
      rc.height() / 8.0_f32
    }

    // Java: static float _ls(RectF rc)
    pub fn _ls(rc: &RectF) -> f32 {
      // Java: return Math.max(rc.width(), rc.height());
      Math::max(rc.width(), rc.height())
    }

    // Java: static float _ss(RectF rc)
    pub fn _ss(rc: &RectF) -> f32 {
      // Java: return Math.min(rc.width(), rc.height());
      Math::min(rc.width(), rc.height())
    }

    // Java: static float _ssd2(RectF rc)
    pub fn _ssd2(rc: &RectF) -> f32 {
      // Java: return Math.min(rc.width(), rc.height()) / 2.0F;
      Math::min(rc.width(), rc.height()) / 2.0_f32
    }

    // Java: static float _ssd4(RectF rc)
    pub fn _ssd4(rc: &RectF) -> f32 {
      // Java: return Math.min(rc.width(), rc.height()) / 4.0F;
      Math::min(rc.width(), rc.height()) / 4.0_f32
    }

    // Java: static float _ssd6(RectF rc)
    pub fn _ssd6(rc: &RectF) -> f32 {
      // Java: return Math.min(rc.width(), rc.height()) / 6.0F;
      Math::min(rc.width(), rc.height()) / 6.0_f32
    }

    // Java: static float _ssd8(RectF rc)
    pub fn _ssd8(rc: &RectF) -> f32 {
      // Java: return Math.min(rc.width(), rc.height()) / 8.0F;
      Math::min(rc.width(), rc.height()) / 8.0_f32
    }

    // Java: static float _wd4(RectF rc)
    pub fn _wd4(rc: &RectF) -> f32 {
      // Java: return rc.width() / 4.0F;
      rc.width() / 4.0_f32
    }

    // Java: static float _wd5(RectF rc)
    pub fn _wd5(rc: &RectF) -> f32 {
      // Java: return rc.width() / 5.0F;
      rc.width() / 5.0_f32
    }

    // Java: static float _wd6(RectF rc)
    pub fn _wd6(rc: &RectF) -> f32 {
      // Java: return rc.width() / 6.0F;
      rc.width() / 6.0_f32
    }

    // Java: static float _wd8(RectF rc)
    pub fn _wd8(rc: &RectF) -> f32 {
      // Java: return rc.width() / 8.0F;
      rc.width() / 8.0_f32
    }

    // Java: static float _wd10(RectF rc)
    pub fn _wd10(rc: &RectF) -> f32 {
      // Java: return rc.width() / 10.0F;
      rc.width() / 10.0_f32
    }

    // Java: static float _hd3(RectF rc)
    pub fn _hd3(rc: &RectF) -> f32 {
      // Java: return rc.height() / 3.0F;
      rc.height() / 3.0_f32
    }

    // Java: static float _wd3(RectF rc)
    pub fn _wd3(rc: &RectF) -> f32 {
      // Java: return rc.width() / 3.0F;
      rc.width() / 3.0_f32
    }

    // Java: static float _ssd16(RectF rc)
    pub fn _ssd16(rc: &RectF) -> f32 {
      // Java: return Math.min(rc.width(), rc.height()) / 16.0F;
      Math::min(rc.width(), rc.height()) / 16.0_f32
    }

    // Java: static float _ssd32(RectF rc)
    pub fn _ssd32(rc: &RectF) -> f32 {
      // Java: return Math.min(rc.width(), rc.height()) / 32.0F;
      Math::min(rc.width(), rc.height()) / 32.0_f32
    }

    // Java: static float _hd7(RectF rc)
    pub fn _hd7(rc: &RectF) -> f32 {
      // Java: return rc.height() / 7.0F;
      rc.height() / 7.0_f32
    }

    // Java: static float _wd32(RectF rc)
    pub fn _wd32(rc: &RectF) -> f32 {
      // Java: return rc.width() / 32.0F;
      rc.width() / 32.0_f32
    }

    // Java: static float _hd10(RectF rc)
    pub fn _hd10(rc: &RectF) -> f32 {
      // Java: return rc.height() / 10.0F;
      rc.height() / 10.0_f32
    }

    // Java: static float _wd12(RectF rc)
    pub fn _wd12(rc: &RectF) -> f32 {
      // Java: return rc.width() / 12.0F;
      rc.width() / 12.0_f32
    }

    // Java: static float _sum(float x, float y, float z)
    pub fn _sum(x: f32, y: f32, z: f32) -> f32 {
      // Java: return x + y - z;
      x + y - z
    }

    // Java: static float _product(float x, float y, float z)
    pub fn _product(x: f32, y: f32, z: f32) -> f32 {
      // Java: return z == 0.0F ? 0.0F : x * y / z;
      let z = z;
      if z == 0.0_f32 { 0.0_f32 } else { x * y / z }
    }

    // Java: static float _mid(float x, float y, float z)
    pub fn _mid(x: f32, y: f32, z: f32) -> f32 {
      // Java: return z == 0.0F ? 0.0F : (x + y) / z;
      let z = z;
      if z == 0.0_f32 { 0.0_f32 } else { (x + y) / z }
    }

    // Java: static float _abs(float x)
    pub fn _abs(x: f32) -> f32 {
      // Java: return x < 0.0F ? -1.0F * x : x;
      if x < 0.0_f32 { -1.0_f32 * x } else { x }
    }

    // Java: static float _min(float x, float y)
    pub fn _min(x: f32, y: f32) -> f32 {
      // Java: return Math.min(x, y);
      Math::min(x, y)
    }

    // Java: static float _max(float x, float y)
    pub fn _max(x: f32, y: f32) -> f32 {
      // Java: return Math.max(x, y);
      Math::max(x, y)
    }

    // Java: static float _ifElse(float x, float y, float z)
    pub fn _ifElse(x: f32, y: f32, z: f32) -> f32 {
      // Java: return x > 0.0F ? y : z;
      if x > 0.0_f32 { y } else { z }
    }

    // Java: static float _mod(float x, float y, float z)
    pub fn _mod(x: f32, y: f32, z: f32) -> f32 {
      // Java: return (float)Math.sqrt((double)(x * x + y * y + z * z));
      ((x * x + y * y + z * z) as f64).sqrt() as f32
    }

    // Java: static float _atan2_dml(float x, float y)
    pub fn _atan2_dml(x: f32, y: f32) -> f32 {
      // Java: return (float)(Math.atan2((double)y, (double)x) * (double)60000.0F / (Math.PI / 180D));
      ((y as f64).atan2(x as f64) * 60000.0_f64 / (std::f64::consts::PI / 180.0_f64)) as f32
    }

    // Java: static float _sin_dml(float x, float y)
    pub fn _sin_dml(x: f32, y: f32) -> f32 {
      // Java: return (float)((double)x * Math.sin((Math.PI / 180D) * (double)y / (double)60000.0F));
      ((x as f64) * ((std::f64::consts::PI / 180.0_f64) * (y as f64) / 60000.0_f64).sin()) as f32
    }

    // Java: static float _cos_dml(float x, float y)
    pub fn _cos_dml(x: f32, y: f32) -> f32 {
      // Java: return (float)((double)x * Math.cos(Math.PI * (double)y / (double)1.08E7F));
      ((x as f64) * (std::f64::consts::PI * (y as f64) / 1.08E7_f64).cos()) as f32
    }

    // Java: static float _cosatan2(float x, float y, float z)
    pub fn _cosatan2(x: f32, y: f32, z: f32) -> f32 {
      // Java: return (float)((double)x * Math.cos(Math.atan2((double)z, (double)y)));
      ((x as f64) * (z as f64).atan2(y as f64).cos()) as f32
    }

    // Java: static float _sinatan2(float x, float y, float z)
    pub fn _sinatan2(x: f32, y: f32, z: f32) -> f32 {
      // Java: return (float)((double)x * Math.sin(Math.atan2((double)z, (double)y)));
      ((x as f64) * (z as f64).atan2(y as f64).sin()) as f32
    }

    // Java: static float _sqrt(float x)
    pub fn _sqrt(x: f32) -> f32 {
      // Java: return (float)Math.sqrt((double)x);
      (x as f64).sqrt() as f32
    }

    // Java: static float _sumangle(float x, float y, float z)
    pub fn _sumangle(x: f32, y: f32, z: f32) -> f32 {
      // Java: return x + (y - z) * (float)Math.pow((double)2.0F, (double)16.0F);
      x + (y - z) * (2.0_f64).powf(16.0_f64) as f32
    }

    // Java: static float _ellipse(float x, float y, float z)
    pub fn _ellipse(x: f32, y: f32, z: f32) -> f32 {
      // Java: return z * (float)Math.sqrt((double)1.0F - Math.pow((double)(x / y), (double)2.0F));
      z * (1.0_f64 - ((x / y) as f64).powf(2.0_f64)).sqrt() as f32
    }

    // Java: static float _tan_dml(float x, float y)
    pub fn _tan_dml(x: f32, y: f32) -> f32 {
      // Java: return (float)((double)x * Math.tan((Math.PI / 180D) * (double)y / (double)60000.0F));
      ((x as f64) * ((std::f64::consts::PI / 180.0_f64) * (y as f64) / 60000.0_f64).tan()) as f32
    }

    // Java: static float _pin(float x, float y, float z)
    pub fn _pin(x: f32, y: f32, z: f32) -> f32 {
      // Java: if (y < x)
        if y < x {
         // Java: return x;
         x
      }
      // Java: else
        else {
         // Java: return y > z ? z : y;
         if y > z { z } else { y }
      }
    }

    // Java: static float _val(float x)
    pub fn _val(x: f32) -> f32 {
      // Java: return x;
      x
    }

    // Java: static float _atan2(float x, float y)
    pub fn _atan2(x: f32, y: f32) -> f32 {
      // Java: return (float)(Math.atan2((double)y, (double)x) * (double)65535.0F / (Math.PI / 180D));
      ((y as f64).atan2(x as f64) * 65535.0_f64 / (std::f64::consts::PI / 180.0_f64)) as f32
    }

    // Java: static float _sin(float x, float y)
    pub fn _sin(x: f32, y: f32) -> f32 {
      // Java: return (float)((double)x * Math.sin((Math.PI / 180D) * (double)y / (double)65535.0F));
      ((x as f64) * ((std::f64::consts::PI / 180.0_f64) * (y as f64) / 65535.0_f64).sin()) as f32
    }

    // Java: static float _cos(float x, float y)
    pub fn _cos(x: f32, y: f32) -> f32 {
      // Java: return (float)((double)x * Math.cos(Math.PI * (double)y / (double)1.17963E7F));
      ((x as f64) * (std::f64::consts::PI * (y as f64) / 1.17963E7_f64).cos()) as f32
    }

    // Java: static float _tan(float x, float y)
    pub fn _tan(x: f32, y: f32) -> f32 {
      // Java: return (float)((double)x * Math.tan((Math.PI / 180D) * (double)y / (double)65535.0F));
      ((x as f64) * ((std::f64::consts::PI / 180.0_f64) * (y as f64) / 65535.0_f64).tan()) as f32
    }

    // Java: static boolean isFormulaIndex(int pcArg)
    pub fn isFormulaIndex(pcArg: i32) -> bool {
      // Java: return pcArg < 48;
      pcArg < 48
    }

    // Java: static int getFormulaParamsCount(int formulaIndex)
    pub fn getFormulaParamsCount(formulaIndex: i32) -> i32 {
        // Java: switch (formulaIndex)
        match formulaIndex {
            2 | // Java: case 2:
            4 | // Java: case 4:
            5 | // Java: case 5:
            8 | // Java: case 8:
            9 | // Java: case 9:
            10 | // Java: case 10:
            16 | // Java: case 16:
            19 | // Java: case 19:
            20 | // Java: case 20:
            21 | // Java: case 21:
            22 => { // Java: case 22:
                // Java: return 2;
                2
            },
            3 | // Java: case 3:
            13 | // Java: case 13:
            18 => { // Java: case 18:
                // Java: return 1;
                1
            },
            6 | // Java: case 6:
            7 | // Java: case 7:
            11 | // Java: case 11:
            12 | // Java: case 12:
            14 | // Java: case 14:
            15 | // Java: case 15:
            17 | // Java: case 17:
            _ => { // Java: default:
                // Java: return 3;
                3
            },
        }
    }

    // Java: static int reverseFormulaParamCore2Doc(int value)
    pub fn reverseFormulaParamCore2Doc(value: i32) -> i32 {
      // Java: if (MathUtil.int2long(value) < CONSTANT_UPPER_BOUND_LONG)
        if MathUtil::int2long(value) < CONSTANT_UPPER_BOUND_LONG {
         // Java: return value;
         value
      }
      // Java: else if (value < -2147483521)
        else if value < -2147483521 {
         // Java: return 1024 + (value - Integer.MIN_VALUE);
         1024 + (value - i32::MIN)
      }
      // Java: else if (value < -2147483511)
        else if value < -2147483511 {
         // Java: return 327 + (value - -2147483521);
         327 + (value - -2147483521)
      }
      // Java: else if (value < -2147483463)
        else if value < -2147483463 {
         // Java: int symbolic = value - -2147483511;
         let symbolic = value - -2147483511;
         // Java: switch (symbolic)
         match symbolic {
             // Java: case 0:
             0 => {
                 // Java: return 322;
                 322
             },
             // Java: case 1:
             1 => {
                 // Java: return 323;
                 323
             },
             // Java: case 2:
             2 => {
                 // Java: return 320;
                 320
             },
             // Java: case 3:
             3 => {
                 // Java: return 321;
                 321
             },
             // Java: case 4:
             4 => {
                 // Java: return 339;
                 339
             },
             // Java: case 5:
             5 => {
                 // Java: return 340;
                 340
             },
             6 | // Java: case 6:
             47 => { // Java: case 47:
                 // Java: return 508;
                 508
             },
             // Java: case 8:
             8 => {
                 // Java: return 1271;
                 1271
             },
             // Java: case 9:
             9 => {
                 // Java: return 1272;
                 1272
             },
             // Java: case 10:
             10 => {
                 // Java: return 1273;
                 1273
             },
             // Java: case 11:
             11 => {
                 // Java: return 1276;
                 1276
             },
             // Java: case 12:
             12 => {
                 // Java: return 1277;
                 1277
             },
             // Java: case 13:
             13 => {
                 // Java: return 1278;
                 1278
             },
             // Java: case 14:
             14 => {
                 // Java: return 1279;
                 1279
             },
             7 | // Java: case 7:
             15 | // Java: case 15:
             16 | // Java: case 16:
             17 | // Java: case 17:
             18 | // Java: case 18:
             19 | // Java: case 19:
             20 | // Java: case 20:
             21 | // Java: case 21:
             22 | // Java: case 22:
             23 | // Java: case 23:
             24 | // Java: case 24:
             25 | // Java: case 25:
             26 | // Java: case 26:
             27 | // Java: case 27:
             28 | // Java: case 28:
             29 | // Java: case 29:
             30 | // Java: case 30:
             31 | // Java: case 31:
             32 | // Java: case 32:
             33 | // Java: case 33:
             34 | // Java: case 34:
             35 | // Java: case 35:
             36 | // Java: case 36:
             37 | // Java: case 37:
             38 | // Java: case 38:
             39 | // Java: case 39:
             40 | // Java: case 40:
             41 | // Java: case 41:
             42 | // Java: case 42:
             43 | // Java: case 43:
             44 | // Java: case 44:
             45 | // Java: case 45:
             46 | // Java: case 46:
             _ => { // Java: default:
                 // Java: return value;
                 value
             },
         }
      }
      // Java: else
        else {
         // Java: return value;
         value
      }
    }

    // Java: static int reverseFormulaParamDoc2Core(int value)
    pub fn reverseFormulaParamDoc2Core(value: i32) -> i32 {
        // Java: if (value >= 1024 && value < 1151)
        if value >= 1024 && value < 1151 {
            // Java: return value - 1024 + Integer.MIN_VALUE;
            value - 1024 + i32::MIN
        }
        // Java: else
        else {
            // Java: switch (value)
            match value {
                // Java: case 320:
                320 => {
                    // Java: return -2147483509;
                    -2147483509
                },
                // Java: case 321:
                321 => {
                    // Java: return -2147483508;
                    -2147483508
                },
                // Java: case 322:
                322 => {
                    // Java: return -2147483511;
                    -2147483511
                },
                // Java: case 323:
                323 => {
                    // Java: return -2147483510;
                    -2147483510
                },
                // Java: case 327:
                327 => {
                    // Java: return -2147483521;
                    -2147483521
                },
                // Java: case 328:
                328 => {
                    // Java: return -2147483520;
                    -2147483520
                },
                // Java: case 329:
                329 => {
                    // Java: return -2147483519;
                    -2147483519
                },
                // Java: case 330:
                330 => {
                    // Java: return -2147483518;
                    -2147483518
                },
                // Java: case 331:
                331 => {
                    // Java: return -2147483517;
                    -2147483517
                },
                // Java: case 332:
                332 => {
                    // Java: return -2147483516;
                    -2147483516
                },
                // Java: case 333:
                333 => {
                    // Java: return -2147483515;
                    -2147483515
                },
                // Java: case 334:
                334 => {
                    // Java: return -2147483514;
                    -2147483514
                },
                // Java: case 339:
                339 => {
                    // Java: return -2147483507;
                    -2147483507
                },
                // Java: case 340:
                340 => {
                    // Java: return -2147483506;
                    -2147483506
                },
                // Java: case 508:
                508 => {
                    // Java: return -2147483505;
                    -2147483505
                },
                // Java: case 1271:
                1271 => {
                    // Java: return -2147483503;
                    -2147483503
                },
                // Java: case 1272:
                1272 => {
                    // Java: return -2147483502;
                    -2147483502
                },
                // Java: case 1273:
                1273 => {
                    // Java: return -2147483501;
                    -2147483501
                },
                // Java: case 1276:
                1276 => {
                    // Java: return -2147483500;
                    -2147483500
                },
                // Java: case 1277:
                1277 => {
                    // Java: return -2147483499;
                    -2147483499
                },
                // Java: case 1278:
                1278 => {
                    // Java: return -2147483498;
                    -2147483498
                },
                // Java: case 1279:
                1279 => {
                    // Java: return -2147483497;
                    -2147483497
                },
                // Java: default:
                _ => {
                    // Java: return value;
                    value
                },
            }
        }
    }

    // Java: static boolean isCalculated(int val)
    pub fn isCalculated(val: i32) -> bool {
      // Java: return MathUtil.int2long(val) >= CONSTANT_UPPER_BOUND_LONG;
      MathUtil::int2long(val) >= CONSTANT_UPPER_BOUND_LONG
    }

}
