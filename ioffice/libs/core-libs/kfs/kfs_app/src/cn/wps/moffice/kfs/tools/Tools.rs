#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Java: public class Tools
#[derive(Default)]
pub struct Tools {}

// Java: public class Tools
impl Tools {
    // Java: public Tools()
    pub fn new() -> Tools { Self {} }

    // Java: public static byte[] string2bytes(String str)
    pub fn string2bytes(s: String) -> Vec<u8> {
        // Java: byte[] bytes = str.getBytes("utf-8");
        // Java: return bytes;
        s.into_bytes()
    }
}
