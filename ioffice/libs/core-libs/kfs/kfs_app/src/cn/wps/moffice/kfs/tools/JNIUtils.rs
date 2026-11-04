// Java: import cn.wps.moffice.kfs.tools.KfsLong
use crate::kfs::tools::KfsLong::*;

// Java: public class JNIUtils 
#[derive(Default)]
pub struct JNIUtils {
}

// Java: public class JNIUtils 
impl JNIUtils {
    // Java: public JNIUtils() 
    pub fn new() -> JNIUtils {
      Self {}
    }

    // Java: public static KfsLong initialNativeHandle() 
    pub fn initialNativeHandle() -> KfsLong {
      // Java: return new KfsLong(0L);
      return KfsLong::new(0);
    }

}
