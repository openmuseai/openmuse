use std::sync::{OnceLock, Mutex};

// Java field: String DebugInfomation
static DEBUG_INFOMATION: OnceLock<Mutex<String>> = OnceLock::new();

fn get_DebugInfomation() -> &'static Mutex<String> {
    DEBUG_INFOMATION.get_or_init(|| Mutex::new(String::new()))
}

// Java: public class GlobalDebugInformation
pub struct GlobalDebugInformation {
}

// Java: public class GlobalDebugInformation
impl GlobalDebugInformation {
    // Java: GlobalDebugInformation()
    pub fn new() -> Self {
      // Java: super();
      GlobalDebugInformation {}
    }

}
