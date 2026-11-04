#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

use std::sync::atomic::{AtomicI32, Ordering};

pub const VERBOSE: i32 = 2;
pub const DEBUG: i32 = 3;
pub const INFO: i32 = 4;
pub const WARN: i32 = 5;
pub const ERROR: i32 = 6;
pub const ASSERT: i32 = 7;

static MAX_ENABLED_LOG_LEVEL: AtomicI32 = AtomicI32::new(0);

// Java: public class Log
#[derive(Default)]
pub struct Log {}

impl Log {
    pub fn new() -> Log { Self {} }

    pub fn setMaxEnabledLogLevel(level: i32) {
        MAX_ENABLED_LOG_LEVEL.store(level, Ordering::Relaxed);
    }

    fn isLoggable(tag: &str, priority: i32) -> bool {
        if !tag.is_empty() {
            return true;
        }
        MAX_ENABLED_LOG_LEVEL.load(Ordering::Relaxed) <= priority
    }

    pub fn v(tag: String, msg: String) {}

    pub fn v__string_string_throwable(tag: String, msg: String, tr: String) {}

    pub fn d(tag: String, msg: String) {}

    pub fn d__string_string_throwable(tag: String, msg: String, tr: String) {}

    pub fn i(tag: String, msg: String) {}

    pub fn i__string_string_throwable(tag: String, msg: String, tr: String) {}

    pub fn w(tag: String, msg: String) {
        Self::printLog(WARN, tag, msg);
    }

    pub fn w__string_string_throwable(tag: String, msg: String, tr: String) {
        Self::printLog(WARN, tag, format!("{}\n{}", msg, tr));
    }

    pub fn e(tag: String, msg: String) {
        Self::printLog(ERROR, tag, msg);
    }

    pub fn e__string_string_throwable(tag: String, msg: String, tr: String) {
        Self::printLog(ERROR, tag, format!("{}\n{}", msg, tr));
    }

    pub fn getStackTraceString(tr: String) -> String {
        tr
    }

    fn printLog(priority: i32, tag: String, msg: String) -> bool {
        if !Self::isLoggable(&tag, priority) {
            return false;
        }
        eprintln!("{}: [{}]: {}", priority, tag, msg);
        true
    }

    pub fn waitForDebugger() {}
}
