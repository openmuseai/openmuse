use std::sync::atomic::{AtomicBool, Ordering};
use super::Log::Log;

// Java: public class IOLog
pub struct IOLog;

// Java: public class IOLog
impl IOLog {
    // Java field: boolean ON_IO_LOG
    pub fn on_io_log() -> bool {
        static ON_IO_LOG: AtomicBool = AtomicBool::new(false);
        ON_IO_LOG.load(Ordering::Relaxed)
    }

    // Java: IOLog()
    pub fn new() -> Self {
        // Java: super();
        IOLog
    }

    // Java: static void v(String tag, String msg)
    pub fn v(tag: &str, msg: &str) {
        // Java: if (ON_IO_LOG)
        if Self::on_io_log() {
            // Java: Log.v(tag, msg);
            Log::v(tag, msg);
        }
    }

    // Java: static void v(String tag, String msg, Throwable tr)
    pub fn v_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (ON_IO_LOG)
        if Self::on_io_log() {
            // Java: Log.v(tag, msg, tr);
            Log::v(tag, &format!("{} {:?}", msg, tr));
        }
    }

    // Java: static void d(String tag, String msg)
    pub fn d(tag: &str, msg: &str) {
        // Java: if (ON_IO_LOG)
        if Self::on_io_log() {
            // Java: Log.d(tag, msg);
            Log::d(tag, msg);
        }
    }

    // Java: static void d(String tag, String msg, Throwable tr)
    pub fn d_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (ON_IO_LOG)
        if Self::on_io_log() {
            // Java: Log.d(tag, msg, tr);
            Log::d(tag, &format!("{} {:?}", msg, tr));
        }
    }

    // Java: static void i(String tag, String msg)
    pub fn i(tag: &str, msg: &str) {
        // Java: if (ON_IO_LOG)
        if Self::on_io_log() {
            // Java: Log.i(tag, msg);
            Log::i(tag, msg);
        }
    }

    // Java: static void i(String tag, String msg, Throwable tr)
    pub fn i_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (ON_IO_LOG)
        if Self::on_io_log() {
            // Java: Log.i(tag, msg, tr);
            Log::i(tag, &format!("{} {:?}", msg, tr));
        }
    }

    // Java: static void w(String tag, String msg)
    pub fn w(tag: &str, msg: &str) {
        // Java: Log.w(tag, msg);
        Log::w(tag, msg);
    }

    // Java: static void w(String tag, String msg, Throwable tr)
    pub fn w_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: Log.w(tag, msg, tr);
        Log::w(tag, &format!("{} {:?}", msg, tr));
    }

    // Java: static void e(String tag, String msg)
    pub fn e(tag: &str, msg: &str) {
        // Java: Log.e(tag, msg);
        Log::e(tag, msg);
    }

    // rust: ★★模板引擎规范名别名（第 4 例，同 §60.3/§61.4）。
    //   Java 原文为 `e(String tag, String msg)`；模板生成 `e__string_string`。
    // Java: static void e(String tag, String msg)
    pub fn e__string_string(tag: &str, msg: &str) {
        // rust: 委派到同一 Java 重载的手写名。
        Self::e(tag, msg);
    }

    // Java: static void e(String tag, String msg, Throwable tr)
    pub fn e_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: Log.e(tag, msg, tr);
        Log::e(tag, &format!("{} {:?}", msg, tr));
    }
}
