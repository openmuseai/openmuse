// Java: import java.util.Map
use std::collections::HashMap;

// Java: public interface IAnalysisExecutor
pub trait IAnalysisExecutor {
    // Java: abstract void eventNormal(String var1, Map<String, String> var2)
    fn eventNormal(&self, var1: &str, var2: &HashMap<String, String>);

    // Java: abstract void eventNormal(String var1, String var2, String var3)
    fn eventNormal_with_kv(&self, var1: &str, var2: &str, var3: &str);

}
