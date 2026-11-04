#![allow(dead_code, non_snake_case, unused_variables)]

use std::sync::OnceLock;

// Java field: ISoUtil sInstance
static S_INSTANCE: OnceLock<Box<dyn ISoUtilTrait + Send + Sync>> = OnceLock::new();

// Java: public class ISoUtil
pub struct ISoUtil;

pub trait ISoUtilTrait {
    fn init(&self, var1: &str) -> String;
    fn initPlugin(&self, var1: &str, var2: i64) -> bool;
    fn initPlugin_with_path(&self, var1: &str, var2: i64, var4: &str) -> bool;
    fn getSoRecoverDirPath(&self) -> String;
}

// Java: public class ISoUtil
impl ISoUtil {
    // Java: ISoUtil()
    pub fn new() -> Self {
        // Java: super();
        ISoUtil
    }

    // Java: static ISoUtil getInstance()
    pub fn getInstance() -> Option<&'static (dyn ISoUtilTrait + Send + Sync)> {
        // Java: return sInstance;
        S_INSTANCE.get().map(|b| b.as_ref())
    }

    // Java: abstract String init(String var1)
    // LLM Abstract method: declared in ISoUtilTrait above
    // Java: abstract boolean initPlugin(String var1, long var2)
    // LLM Abstract method: declared in ISoUtilTrait above
    // Java: abstract boolean initPlugin(String var1, long var2, String var4)
    // LLM Abstract method: declared in ISoUtilTrait above
    // Java: abstract String getSoRecoverDirPath()
    // LLM Abstract method: declared in ISoUtilTrait above
}
