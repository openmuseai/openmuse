#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

use super::Log::Log;

pub static ON_IO_LOG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// Java: public class KfsLog
#[derive(Default)]
pub struct KfsLog {}

impl KfsLog {
    pub fn new() -> KfsLog { Self {} }

    pub fn v(tag: String, msg: String) {
        if ON_IO_LOG.load(std::sync::atomic::Ordering::Relaxed) {
            Log::v(tag, msg);
        }
    }

    pub fn v__string_string_throwable(tag: String, msg: String, tr: String) {
        if ON_IO_LOG.load(std::sync::atomic::Ordering::Relaxed) {
            Log::v__string_string_throwable(tag, msg, tr);
        }
    }

    pub fn d(tag: String, msg: String) {
        if ON_IO_LOG.load(std::sync::atomic::Ordering::Relaxed) {
            Log::d(tag, msg);
        }
    }

    pub fn d__string_string_throwable(tag: String, msg: String, tr: String) {
        if ON_IO_LOG.load(std::sync::atomic::Ordering::Relaxed) {
            Log::d__string_string_throwable(tag, msg, tr);
        }
    }

    pub fn i(tag: String, msg: String) {
        if ON_IO_LOG.load(std::sync::atomic::Ordering::Relaxed) {
            Log::i(tag, msg);
        }
    }

    pub fn i__string_string_throwable(tag: String, msg: String, tr: String) {
        if ON_IO_LOG.load(std::sync::atomic::Ordering::Relaxed) {
            Log::i__string_string_throwable(tag, msg, tr);
        }
    }

    pub fn w(tag: String, msg: String) {
        Log::w(tag, msg);
    }

    pub fn w__string_string_throwable(tag: String, msg: String, tr: String) {
        Log::w__string_string_throwable(tag, msg, tr);
    }

    pub fn e(tag: String, msg: String) {
        Log::e(tag, msg);
    }

    pub fn e__string_string_throwable(tag: String, msg: String, tr: String) {
        Log::e__string_string_throwable(tag, msg, tr);
    }
}
