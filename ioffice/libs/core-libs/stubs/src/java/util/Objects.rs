// Java: package java.util;
use crate::java::lang::Object::Object;

// Java: public class Objects
pub struct Objects;

// Java: public class Objects
impl Objects {
    // Java: public static void hash(Object arg0) { }
    pub fn hash(arg0: &dyn Object) {
        let _ = arg0;
    }

    // Java: public static int hashCode(Object arg0) { }
    pub fn hashCode(arg0: Option<&dyn Object>) -> i32 {
        arg0.map_or(0, Object::hashCode)
    }

    // Java: public static boolean equals(Object arg0, Object arg1) { }
    pub fn equals(arg0: Option<&dyn Object>, arg1: Option<&dyn Object>) -> bool {
        match (arg0, arg1) {
            (None, None) => true,
            (Some(left), Some(right)) => left.equals(right),
            _ => false,
        }
    }

    // Java: public static void requireNonNull(Object arg0) { }
    pub fn requireNonNull(arg0: &dyn Object) {
        let _ = arg0;
    }
}
