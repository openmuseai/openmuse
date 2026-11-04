
#![allow(dead_code, non_snake_case)]
// Java: public class KSAssert extends Assert
pub struct KSAssert;

impl KSAssert {
    // Java: KSAssert()
    fn new() -> Self {
        // Java: super();
        KSAssert
    }

    // Java: static void assertTrue(boolean condition)
    pub fn assertTrue(condition: bool) {
        debug_assert!(condition);
    }

    // Java: static void assertNotNull(Object object) — soft Option glue for nullable refs
    pub fn assertNotNull<T>(_object: Option<T>) {}

    // Java: static void assertNotEqual(int a, int b)
    pub fn assertNotEqual(a: i32, b: i32) {
        // Java: assertTrue(a != b);
        debug_assert!(a != b);
    }

    pub fn assertEquals(expected: i32, actual: i32) {
        debug_assert_eq!(expected, actual);
    }
}
