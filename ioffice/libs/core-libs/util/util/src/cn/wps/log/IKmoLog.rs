// Java: public interface IKmoLog
pub trait IKmoLog: Send + Sync {
    // Java: abstract void log(String var1)
    fn log(&self, var1: &str);

    // Java: abstract void log(String var1, String var2)
    fn log_tag(&self, var1: &str, var2: &str);

    // Java: abstract void e(String var1, String var2, Throwable var3)
    fn e(&self, var1: &str, var2: &str, var3: Option<&dyn std::error::Error>);
}
