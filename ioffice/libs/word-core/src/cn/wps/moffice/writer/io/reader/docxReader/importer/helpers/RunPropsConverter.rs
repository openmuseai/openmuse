// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion::*;

// java public final class RunPropsConverter
#[derive(Default)]
pub struct RunPropsConverter {
}

// java public final class RunPropsConverter
impl RunPropsConverter {
    // java public static int conver2coreColor(String srcVal)
    pub fn conver2coreColor(srcVal: &str) -> i32 {
      // java int dstVal = -2;
      let mut dstVal: i32 = -2;
      // java if (srcVal.equalsIgnoreCase("auto"))
      if srcVal.eq_ignore_ascii_case("auto") {
        // java dstVal = -16777216;
        dstVal = -16777216;
      }
      // java else
      else {
        // java Integer val = TypeConversion.hexString2Integer(srcVal);
        let val = TypeConversion::hexString2Integer(srcVal);
        // java if (null != val)
        if val.is_some() {
          // java dstVal = val;
          dstVal = val.unwrap();
        }
      }
      // java return dstVal;
      return dstVal;
    }

}
