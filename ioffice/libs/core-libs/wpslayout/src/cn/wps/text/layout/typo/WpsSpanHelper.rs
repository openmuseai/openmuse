#![allow(dead_code, unused_variables, non_snake_case)]

// Java: import android.os.Build.VERSION

// Java field: int SDK_INT
pub(crate) const SDK_INT: i32 = 0;

// Java: public class WpsSpanHelper
pub struct WpsSpanHelper {
}

// Java: public class WpsSpanHelper
impl WpsSpanHelper {
    // Java: WpsSpanHelper()
    pub fn new(&self) {
      // Java: super();

    }

    // Java: static boolean isAndroid4_4()
    pub fn isAndroid4_4(&self) -> bool {
      // Java: return SDK_INT == 19;

      false
    }

    // Java: static boolean isAndroid4_0()
    pub fn isAndroid4_0(&self) -> bool {
      // Java: return SDK_INT == 15;

      false
    }

    // Java: static boolean isAndroid6_0()
    pub fn isAndroid6_0(&self) -> bool {
      // Java: return SDK_INT == 23;

      false
    }

    // Java: static boolean isGEAndroid6_0()
    pub fn isGEAndroid6_0(&self) -> bool {
      // Java: return SDK_INT >= 23;

      false
    }

}
