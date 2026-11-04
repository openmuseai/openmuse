// Java field: ISoUtil sInstance
static sInstance: std::sync::OnceLock<std::sync::Mutex<ISoUtil>> = std::sync::OnceLock::new();

// Java: public class ISoUtil
pub struct ISoUtil {
}

// Java: public class ISoUtil
impl ISoUtil {
    // Java: ISoUtil()
    pub fn ISoUtil(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: static ISoUtil getInstance()
    pub fn getInstance(&self) -> ISoUtil {
      // Java: return sInstance;
      // TODO_RUST_CODE
    }

    // Java: abstract String init(String var1)
    pub fn init(&self, _var1: &str) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract boolean initPlugin(String var1, long var2)
    pub fn initPlugin(&self, _var1: &str, _var2: i64) -> bool {
    // TODO_RUST_CODE
    }

    // Java: abstract boolean initPlugin(String var1, long var2, String var4)
    pub fn initPlugin(&self, _var1: &str, _var2: i64, _var4: &str) -> bool {
    // TODO_RUST_CODE
    }

    // Java: abstract String getSoRecoverDirPath()
    pub fn getSoRecoverDirPath(&self) -> String {
    // TODO_RUST_CODE
    }

}
