#![allow(dead_code, non_snake_case)]

// Java: public class UnitUtil
pub struct UnitUtil;

// Java: public class UnitUtil
impl UnitUtil {
    // Java: UnitUtil()
    // Java: super();

    // Java: static final int FLOAT2FIX(float value)
    pub fn FLOAT2FIX(value: f32) -> i32 {
        // Java: return (int)(value * 65536.0F);
        (value * 65536.0_f32) as i32
    }

    // Java: static final float FIX2FLOAT(int value)
    pub fn FIX2FLOAT(value: i32) -> f32 {
        // Java: int fractional = value & '\uffff';
        let fractional = value & 0xFFFF;
        // Java: int integral = value >> 16;
        let integral = value >> 16;
        // Java: return (float)integral + (float)fractional / 65536.0F;
        integral as f32 + fractional as f32 / 65536.0_f32
    }

    // Java: static final int PT2TWIPS(float pt)
    pub fn PT2TWIPS(pt: f32) -> i32 {
        // Java: return pt >= 0.0F ? (int)(pt * 20.0F + 0.5F) : (int)(pt * 20.0F - 0.5F);
        if pt >= 0.0_f32 {
            (pt * 20.0_f32 + 0.5_f32) as i32
        } else {
            (pt * 20.0_f32 - 0.5_f32) as i32
        }
    }

    // Java: static final float TWIPS2PT(int twips)
    pub fn TWIPS2PT(twips: i32) -> f32 {
        // Java: return (float)twips / 20.0F;
        twips as f32 / 20.0_f32
    }

    // Java: static final float TWIPS2PT(float twips)
    pub fn TWIPS2PT_f32(twips: f32) -> f32 {
        // Java: return twips / 20.0F;
        twips / 20.0_f32
    }

    // Java: static final int PT2EMU(float pt)
    pub fn PT2EMU(pt: f32) -> i32 {
        // Java: return pt >= 0.0F ? (int)(pt * 12700.0F + 0.5F) : (int)(pt * 12700.0F - 0.5F);
        if pt >= 0.0_f32 {
            (pt * 12700.0_f32 + 0.5_f32) as i32
        } else {
            (pt * 12700.0_f32 - 0.5_f32) as i32
        }
    }

    // Java: static final float EMU2PT(int emu)
    pub fn EMU2PT(emu: i32) -> f32 {
        // Java: return (float)emu / 12700.0F;
        emu as f32 / 12700.0_f32
    }

    // Java: static final float EMU2TWIPS(int emu)
    pub fn EMU2TWIPS(emu: i32) -> f32 {
        // Java: return (float)emu / 635.0F;
        emu as f32 / 635.0_f32
    }

    // Java: static final float TWIPS2EMU(int twips)
    pub fn TWIPS2EMU(twips: i32) -> f32 {
        // Java: return (float)twips * 635.0F;
        twips as f32 * 635.0_f32
    }

    // Java: static final float HalfPT2PT(int half)
    pub fn HalfPT2PT(half: i32) -> f32 {
        // Java: return (float)half / 2.0F;
        half as f32 / 2.0_f32
    }

    // Java: static final int PT2HalfPT(float pt)
    pub fn PT2HalfPT(pt: f32) -> i32 {
        // Java: return (int)(pt * 2.0F);
        (pt * 2.0_f32) as i32
    }

    // Java: static final int MsoYear2CoreYear(int year)
    pub fn MsoYear2CoreYear(year: i32) -> i32 {
        // Java: return year + 1900;
        year + 1900
    }

    // Java: static final int CoreYear2MsoYear(int year)
    pub fn CoreYear2MsoYear(year: i32) -> i32 {
        // Java: return year - 1900;
        year - 1900
    }

    // Java: static final float Percent2Float(int percent)
    pub fn Percent2Float(percent: i32) -> f32 {
        // Java: return (float)percent / 100.0F;
        percent as f32 / 100.0_f32
    }

    // Java: static final int Float2Percent(float value)
    pub fn Float2Percent(value: f32) -> i32 {
        // Java: return Math.round(value * 100.0F);
        (value * 100.0_f32).round() as i32
    }
}
