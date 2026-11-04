#![allow(dead_code, non_snake_case)]

// Java: public class KSLog
pub struct KSLog {
    // Java field: String OUT_PUT_PATH
}


// Java: public class KSLog
impl KSLog {
    // Java: KSLog()
    pub fn new() -> Self {
        // Java: super();
        KSLog {}
    }

    // Java: static void v(String tag, String msg)
    pub fn v(_tag: &str, _msg: &str) {
    }

    // Java: static void v(String tag, String msg, Throwable tr)
    pub fn v_with_throwable(_tag: &str, _msg: &str, _tr: &str) {
    }

    // Java: static void d(String tag, String msg)
    pub fn d(_tag: &str, _msg: &str) {
    }

    // Java: static void d(String tag, String msg, Throwable tr)
    pub fn d_with_throwable(_tag: &str, _msg: &str, _tr: &str) {
    }

    // Java: static void i(String tag, String msg)
    pub fn i(_tag: &str, _msg: &str) {
    }

    // Java: static void i(String tag, String msg, Throwable tr)
    pub fn i_with_throwable(_tag: &str, _msg: &str, _tr: &str) {
    }

    // Java: static void w(String tag, String msg)
    pub fn w(_tag: &str, _msg: &str) {
    }

    // Java: static void w(String tag, String msg, Throwable tr)
    pub fn w_with_throwable(_tag: &str, _msg: &str, _tr: &str) {
    }

    // Java: static void w(String tag, Throwable tr)
    pub fn w_throwable_only(_tag: &str, _tr: &str) {
    }

    // Java: static void e(String tag, String msg)
    pub fn e(_tag: &str, _msg: &str) {
    }

    // Java: static void e(String tag, String msg, Throwable tr)
    pub fn e_with_throwable(_tag: &str, _msg: &str, _tr: &str) {
    }

    // Java: static void initOutPutPath(String outputPath)
    pub fn initOutPutPath(_outputPath: &str) {
        // Java: OUT_PUT_PATH = outputPath;
    }

    // Java: static void output(String tag, String msg)
    pub fn output(_tag: &str, _msg: &str) {
    }
}
