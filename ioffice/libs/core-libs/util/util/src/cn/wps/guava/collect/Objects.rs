#![allow(non_snake_case, dead_code, unused_variables)]

use std::any::Any;

// Java: public class Objects
pub struct Objects;

// Java: public class Objects
impl Objects {
    // Java: Objects()
    // Java: super();

    // Java: static boolean equal(Object a, Object b)
    pub fn equal(a: Option<&dyn Any>, b: Option<&dyn Any>) -> bool {
        // Java: return a == b || a != null && a.equals(b);
        match (a, b) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(_a), Some(_b)) => false,
        }
    }

    // Java: static int hashCode(Object... objects)
    pub fn hashCode(objects: &[Option<Box<dyn Any>>]) -> i32 {
        // Java: return Arrays.hashCode(objects);
        let mut result = 1i32;
        for obj in objects {
            let element_hash = if obj.is_none() { 0 } else { 0 };
            result = result.wrapping_mul(31).wrapping_add(element_hash);
        }
        result
    }
}
