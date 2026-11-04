#![allow(non_snake_case, dead_code, unused_variables)]

// Java field: int C1
pub(crate) const C1: i32 = -862048943;
// Java field: int C2
pub(crate) const C2: i32 = 461845907;

// Java: public class Hashing
pub struct Hashing;

// Java: public class Hashing
impl Hashing {
    // Java: Hashing()
    // Java: super();

    // Java: static int smear(int hashCode)
    pub fn smear(hash_code: i32) -> i32 {
        // Java: return 461845907 * Integer.rotateLeft(hashCode * -862048943, 15);
        C2.wrapping_mul((hash_code.wrapping_mul(C1)).rotate_left(15))
    }
}
