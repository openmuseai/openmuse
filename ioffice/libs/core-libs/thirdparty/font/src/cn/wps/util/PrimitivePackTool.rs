// Java field: int OFFSET_32_BITS
const OFFSET_32_BITS: i32 = 32;
// Java field: long MASK_LOWER_32_BITS
const MASK_LOWER_32_BITS: i64 = 4294967295;
// Java field: int INT_BOOL_TRUE
const INT_BOOL_TRUE: i32 = 1;
// Java field: int INT_BOOL_FALSE
const INT_BOOL_FALSE: i32 = 0;

// Java: public class PrimitivePackTool
pub struct PrimitivePackTool {
}

// Java: public class PrimitivePackTool
impl PrimitivePackTool {
    // Java: PrimitivePackTool()
    pub fn new() -> Self {
        // Java: super();
        PrimitivePackTool {}
    }

    // Java: static int unpackFirstInt(long res)
    pub fn unpackFirstInt(res: i64) -> i32 {
        // Java: int a = (int)(res >> 32);
        let a = (res >> 32) as i32;
        // Java: return a;
        a
    }

    // Java: static int unpackSecondInt(long res)
    pub fn unpackSecondInt(res: i64) -> i32 {
        // Java: int b = (int)(res & 4294967295L);
        let b = (res & 4294967295i64) as i32;
        // Java: return b;
        b
    }

    // Java: static float unpackFirstFloat(long res)
    pub fn unpackFirstFloat(res: i64) -> f32 {
        // Java: return Float.intBitsToFloat(unpackFirstInt(res));
        f32::from_bits(Self::unpackFirstInt(res) as u32)
    }

    // Java: static float unpackSecondFloat(long res)
    pub fn unpackSecondFloat(res: i64) -> f32 {
        // Java: return Float.intBitsToFloat(unpackSecondInt(res));
        f32::from_bits(Self::unpackSecondInt(res) as u32)
    }

    // Java: static boolean unpackFirstBoolean(long res)
    pub fn unpackFirstBoolean(res: i64) -> bool {
        // Java: return 1 == unpackFirstInt(res);
        1 == Self::unpackFirstInt(res)
    }

    // Java: static long packIntAndInt(int a, int b)
    pub fn packIntAndInt(a: i32, b: i32) -> i64 {
        // Java: long high = (long)a << 32;
        let high = (a as i64) << 32;
        // Java: long low = (long)b & 4294967295L;
        let low = (b as i64) & 4294967295i64;
        // Java: return high | low;
        high | low
    }

    // Java: static long packIntAndFloat(int a, float b)
    pub fn packIntAndFloat(a: i32, b: f32) -> i64 {
        // Java: return packIntAndInt(a, Float.floatToRawIntBits(b));
        Self::packIntAndInt(a, b.to_bits() as i32)
    }

    // Java: static long packFloatAndFloat(float a, float b)
    pub fn packFloatAndFloat(a: f32, b: f32) -> i64 {
        // Java: return packIntAndInt(Float.floatToRawIntBits(a), Float.floatToRawIntBits(b));
        Self::packIntAndInt(a.to_bits() as i32, b.to_bits() as i32)
    }

    // Java: static long packBooleanAndInt(boolean a, int b)
    pub fn packBooleanAndInt(a: bool, b: i32) -> i64 {
        // Java: int val = a ? 1 : 0;
        let val = if a { 1 } else { 0 };
        // Java: return packIntAndInt(val, b);
        Self::packIntAndInt(val, b)
    }
}
