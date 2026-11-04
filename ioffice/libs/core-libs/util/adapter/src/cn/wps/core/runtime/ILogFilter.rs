// Java: public interface ILogFilter
pub trait ILogFilter {
    // Java: abstract void disableCaller(String var1, String var2)
    fn disableCaller(&self, var1: &str, var2: &str);

    // Java: abstract void disableClass(String var1)
    fn disableClass(&self, var1: &str);

    // Java: abstract void disableTag(String var1)
    fn disableTag(&self, var1: &str);

    // Java: abstract void enableCaller(String var1, String var2)
    fn enableCaller(&self, var1: &str, var2: &str);

    // Java: abstract void enableClass(String var1)
    fn enableClass(&self, var1: &str);

    // Java: abstract void enableTag(String var1)
    fn enableTag(&self, var1: &str);

    // Java: abstract boolean getPrintCaller()
    fn getPrintCaller(&self) -> bool;

    // Java: abstract boolean isEnabled(String var1, String var2, String var3)
    fn isEnabled(&self, var1: &str, var2: &str, var3: &str) -> bool;

    // Java: abstract void setPrintCaller(boolean var1)
    fn setPrintCaller(&self, var1: bool);

}
