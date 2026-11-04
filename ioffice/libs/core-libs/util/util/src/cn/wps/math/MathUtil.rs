#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java field: _b: &[i32]
const B: [i32; 5] = [2, 12, 240, 65280, -65536];
// Java field: _S: &[i32]
const S: [i32; 5] = [1, 2, 4, 8, 16];
// Java field: float FLOAT_ACCURACY
pub(crate) const FLOAT_ACCURACY: f32 = 1.0e-6;
// Java field: double DOUBLE_ACCURACY
pub(crate) const DOUBLE_ACCURACY: f64 = 1.0e-6;

// Java: public class MathUtil
pub struct MathUtil;

// Java: public class MathUtil
impl MathUtil {
    // Java: MathUtil()
    pub fn new() -> Self {
        // Java: super();
        MathUtil
    }

    // Java: static long int2long(int val)
    pub fn int2long(val: i32) -> i64 {
        // Java: return (long)val & 4294967295L;
        (val as i64) & 4294967295i64
    }

    // Java: static int log2(int v)
    pub fn log2(mut v: i32) -> i32 {
        // Java: int r = 0;
        let mut r: i32 = 0;
        // Java: for(int i = 4; i >= 0; --i)
        for i in (0..=4).rev() {
            // Java: if ((v & b[i]) != 0)
            if (v & B[i as usize]) != 0 {
                // Java: v >>= S[i];
                v >>= S[i as usize];
                // Java: r |= S[i];
                r |= S[i as usize];
            }
        }
        // Java: return r;
        r
    }

    // Java: static int nextPower2(int x)
    pub fn nextPower2(mut x: i32) -> i32 {
        // Java: --x;
        x -= 1;
        // Java: x |= x >> 1;
        x |= x >> 1;
        // Java: x |= x >> 2;
        x |= x >> 2;
        // Java: x |= x >> 4;
        x |= x >> 4;
        // Java: x |= x >> 8;
        x |= x >> 8;
        // Java: x |= x >> 16;
        x |= x >> 16;
        // Java: ++x;
        x += 1;
        // Java: return x;
        x
    }

    // Java: static int gcd(int a, int b)
    pub fn gcd(mut a: i32, mut b: i32) -> i32 {
        // Java: int mod;
        let mut modv: i32;
        // Java: do
        loop {
            // Java: mod = a % b;
            modv = a % b;
            // Java: a = b;
            a = b;
            // Java: b = mod;
            b = modv;
            // Java: while(mod != 0)
            if modv == 0 { break; }
        }
        // Java: return a;
        a
    }

    // Java: static int lcm(int a, int b)
    pub fn lcm(a: i32, b: i32) -> i32 {
        // Java: return a / gcd(a, b) * b;
        a / Self::gcd(a, b) * b
    }

    // Java: static boolean floatEqual(float a, float b)
    pub fn floatEqual(a: f32, b: f32) -> bool {
        // Java: return Math.abs(a - b) <= 1.0E-6F;
        (a - b).abs() <= 1.0e-6f32
    }

    // Java: static boolean doubleEqual(double a, double b)
    pub fn doubleEqual(a: f64, b: f64) -> bool {
        // Java: return Math.abs(a - b) <= 1.0E-6;
        (a - b).abs() <= 1.0e-6f64
    }
}
