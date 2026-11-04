// Java: public interface ILog
pub trait ILog {
    // Java: abstract void d(String var1, String var2)
    fn d(&self, var1: &str, var2: &str);

    // Java: abstract void d(String var1, String var2, Throwable var3)
    fn d_tr(&self, var1: &str, var2: &str, var3: &dyn std::error::Error);

    // Java: abstract void e(String var1, String var2)
    fn e(&self, var1: &str, var2: &str);

    // Java: abstract void e(String var1, String var2, Throwable var3)
    fn e_tr(&self, var1: &str, var2: &str, var3: &dyn std::error::Error);

    // Java: abstract void i(String var1, String var2)
    fn i(&self, var1: &str, var2: &str);

    // Java: abstract void i(String var1, String var2, Throwable var3)
    fn i_tr(&self, var1: &str, var2: &str, var3: &dyn std::error::Error);

    // Java: abstract void v(String var1, String var2)
    fn v(&self, var1: &str, var2: &str);

    // Java: abstract void v(String var1, String var2, Throwable var3)
    fn v_tr(&self, var1: &str, var2: &str, var3: &dyn std::error::Error);

    // Java: abstract void w(String var1, String var2)
    fn w(&self, var1: &str, var2: &str);

    // Java: abstract void w(String var1, String var2, Throwable var3)
    fn w_tr(&self, var1: &str, var2: &str, var3: &dyn std::error::Error);

    // Java: abstract void w(String var1, Throwable var2)
    fn w_tr_only(&self, var1: &str, var2: &dyn std::error::Error);

    // Java: abstract void waitForDebugger()
    fn waitForDebugger(&self);

}
