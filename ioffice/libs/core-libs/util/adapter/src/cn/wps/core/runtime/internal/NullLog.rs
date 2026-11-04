// Java: import cn.wps.core.runtime.ILog

use crate::cn::wps::core::runtime::ILog::ILog;
use std::sync::OnceLock;

// Java field: NullLog instance
static INSTANCE: OnceLock<NullLog> = OnceLock::new();

// Java: public class NullLog implements ILog
pub struct NullLog {
}

// Java: public class NullLog implements ILog
impl NullLog {
    // Java: NullLog()
    pub fn new() -> Self {
      // Java: super();
      NullLog {}
    }

    // Java: static NullLog getDefault()
    pub fn getDefault() -> &'static NullLog {
      // Java: return instance;
      INSTANCE.get_or_init(|| NullLog::new())
    }

    // Java: void d(String tag, String msg)
    // Java: void d(String tag, String msg, Throwable tr)
    // Java: void e(String tag, String msg)
    // Java: void e(String tag, String msg, Throwable tr)
    // Java: void i(String tag, String msg)
    // Java: void i(String tag, String msg, Throwable tr)
    // Java: void v(String tag, String msg)
    // Java: void v(String tag, String msg, Throwable tr)
    // Java: void w(String tag, String msg)
    // Java: void w(String tag, String msg, Throwable tr)
    // Java: void w(String tag, Throwable tr)
    // Java: void waitForDebugger()
}

impl ILog for NullLog {
    fn d(&self, _var1: &str, _var2: &str) {}
    fn d_tr(&self, _var1: &str, _var2: &str, _var3: &dyn std::error::Error) {}
    fn e(&self, _var1: &str, _var2: &str) {}
    fn e_tr(&self, _var1: &str, _var2: &str, _var3: &dyn std::error::Error) {}
    fn i(&self, _var1: &str, _var2: &str) {}
    fn i_tr(&self, _var1: &str, _var2: &str, _var3: &dyn std::error::Error) {}
    fn v(&self, _var1: &str, _var2: &str) {}
    fn v_tr(&self, _var1: &str, _var2: &str, _var3: &dyn std::error::Error) {}
    fn w(&self, _var1: &str, _var2: &str) {}
    fn w_tr(&self, _var1: &str, _var2: &str, _var3: &dyn std::error::Error) {}
    fn w_tr_only(&self, _var1: &str, _var2: &dyn std::error::Error) {}
    fn waitForDebugger(&self) {}
}
