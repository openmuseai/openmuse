// Java: public class HashCode

pub struct HashCode;

// Java: HashCode()
impl HashCode {
    // Java: super();
    pub fn new() -> Self {
        HashCode
    }
}

impl HashCode {
    // Java: static int hash(double value)
    pub fn hash_double(value: f64) -> i32 {
        // Java: assert !Double.isNaN(value) : "Values of NaN are not supported.";
        debug_assert!(!value.is_nan(), "Values of NaN are not supported.");
        // Java: long bits = Double.doubleToLongBits(value);
        let bits = value.to_bits() as i64;
        // Java: return (int)(bits ^ bits >>> 32);
        (bits ^ (bits >> 32)) as i32
    }

    // Java: static int hash(float value)
    pub fn hash_float(value: f32) -> i32 {
        // Java: assert !Float.isNaN(value) : "Values of NaN are not supported.";
        debug_assert!(!value.is_nan(), "Values of NaN are not supported.");
        // Java: return Float.floatToIntBits(value * 6.6360896E8F);
        // LLM Rust Pattern: Float.floatToIntBits -> f32::to_bits()
        (value * 663_608_960.0_f32).to_bits() as i32
    }

    // Java: static int hash(int value)
    pub fn hash_int(value: i32) -> i32 {
        // Java: return value;
        value
    }

    // Java: static int hash(long value)
    pub fn hash_long(value: i64) -> i32 {
        // Java: return (int)(value ^ value >>> 32);
        (value ^ (value >> 32)) as i32
    }

    // Java: static int hash(Object object)
    pub fn hash_object<T: std::hash::Hash>(object: Option<&T>) -> i32 {
        // Java: return object == null ? 0 : object.hashCode();
        // LLM Rust Pattern: ternary null check -> match Option
        match object {
            None => 0,
            Some(obj) => {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                obj.hash(&mut hasher);
                std::hash::Hasher::finish(&hasher) as i32
            }
        }
    }

    // Java: static int fastCeil(float v)
    pub fn fastCeil(v: f32) -> i32 {
        // Java: int possible_result = (int)v;
        let mut possible_result = v as i32;
        // Java: if (v - (float)possible_result > 0.0F)
        if v - (possible_result as f32) > 0.0 {
            // Java: ++possible_result;
            possible_result += 1;
        }
        // Java: return possible_result;
        possible_result
    }
}
