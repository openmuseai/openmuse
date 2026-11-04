#![allow(non_snake_case, dead_code, unused_variables)]

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

// Java: public class ThaiDictJNI
pub struct ThaiDictJNI;

// Java field: String TAG
const TAG: &str = "";
// Java field: String LIBRARY_NAME
const LIBRARY_NAME: &str = "thaiDict";
// Java field: boolean isLoaded
static IS_LOADED: AtomicBool = AtomicBool::new(false);
// Java field: int MAX_WORDLEN
static MAX_WORDLEN: AtomicI32 = AtomicI32::new(1);

// Java: public class ThaiDictJNI
impl ThaiDictJNI {
    // Java: ThaiDictJNI()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static boolean isLoaded()
    pub fn isLoaded() -> bool {
        // Java: return isLoaded;
        IS_LOADED.load(Ordering::Relaxed)
    }

    // Java: static public static native int getWordLen(_var0: &[char], int var1, int var2)
    pub fn getWordLen(_var0: &[char], _var1: i32, _var2: i32) -> i32 {
        todo!()
    }

    // Java: static private static native int getMaxWordLen()
    fn getMaxWordLen() -> i32 {
        todo!()
    }
}
