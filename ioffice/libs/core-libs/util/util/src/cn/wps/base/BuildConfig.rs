// Java: import cn.wps.base.assertion.Assert
// use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.base.log.Log
// use kfs_kfs_app::cn::wps::moffice::kfs::log::Log;
// Java: import java.lang.reflect.Field
// No Rust equivalent for java.lang.reflect.Field

use std::sync::{Mutex, OnceLock};

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: boolean DEBUG
pub(crate) const DEBUG: bool = false;
// Java field: boolean RELEASE
pub(crate) const RELEASE: bool = true;
// Java field: Integer sSdkVersionCode
static S_SDK_VERSION_CODE: OnceLock<Mutex<Option<i32>>> = OnceLock::new();
fn get_sSdkVersionCode() -> &'static Mutex<Option<i32>> {
    S_SDK_VERSION_CODE.get_or_init(|| Mutex::new(None))
}
// Java field: int JDK
pub(crate) const JDK: i32 = -2;
// Java field: int CUR_DEVELOPMENT
pub(crate) const CUR_DEVELOPMENT: i32 = 10000;
// Java field: int BASE
pub(crate) const BASE: i32 = 1;
// Java field: int BASE_1_1
pub(crate) const BASE_1_1: i32 = 2;
// Java field: int CUPCAKE
pub(crate) const CUPCAKE: i32 = 3;
// Java field: int DONUT
pub(crate) const DONUT: i32 = 4;
// Java field: int ECLAIR
pub(crate) const ECLAIR: i32 = 5;
// Java field: int ECLAIR_0_1
pub(crate) const ECLAIR_0_1: i32 = 6;
// Java field: int ECLAIR_MR1
pub(crate) const ECLAIR_MR1: i32 = 7;
// Java field: int FROYO
pub(crate) const FROYO: i32 = 8;
// Java field: int GINGERBREAD
pub(crate) const GINGERBREAD: i32 = 9;
// Java field: int GINGERBREAD_MR1
pub(crate) const GINGERBREAD_MR1: i32 = 10;
// Java field: int HONEYCOMB
pub(crate) const HONEYCOMB: i32 = 11;
// Java field: int HONEYCOMB_MR1
pub(crate) const HONEYCOMB_MR1: i32 = 12;
// Java field: int HONEYCOMB_MR2
pub(crate) const HONEYCOMB_MR2: i32 = 13;
// Java field: int ICE_CREAM_SANDWICH
pub(crate) const ICE_CREAM_SANDWICH: i32 = 14;
// Java field: int ICE_CREAM_SANDWICH_MR1
pub(crate) const ICE_CREAM_SANDWICH_MR1: i32 = 15;
// Java field: int JELLY_BEAN
pub(crate) const JELLY_BEAN: i32 = 16;

// Java: public class BuildConfig
pub struct BuildConfig {
}

// Java: public class VERSION_CODES
pub struct VERSION_CODES {
}

// Java: public class BuildConfig
impl BuildConfig {
    // Java: BuildConfig()
    pub fn new() -> Self {
      // Java: super();
      BuildConfig {}
    }

    // Java: static int getRuntimeSdkVersion()
    pub fn getRuntimeSdkVersion() -> i32 {
      // Java: if (null != sSdkVersionCode)
      let guard = get_sSdkVersionCode().lock().unwrap();
      if guard.is_some()
      {
         // Java: return sSdkVersionCode;
         return guard.unwrap();
      }
      drop(guard);
      // Java: else
      {
         // Java: try
         {
            // Java: Class<?> buildClazz = Class.forName("android.os.Build");
            // No Rust equivalent for Class.forName - stubbed
            // Java: Assert.assertNotNull("clazz should not be null.");
            // Stubbed - no reflection in Rust
            // Java: try
            {
               // Java: Class<?> versionClazz = getDeclaredClass(buildClazz, "android.os.Build$VERSION");
               // Stubbed - no reflection in Rust
               // Java: Assert.assertNotNull("versionClazz should not be null.");
               // Stubbed
               // Java: Field sdkIntField = versionClazz.getDeclaredField("SDK_INT");
               // Stubbed - no reflection in Rust
               // Java: Assert.assertNotNull("sdkIntField should not be null.");
               // Stubbed
               // Java: sSdkVersionCode = sdkIntField.getInt(versionClazz);
               // Stubbed - no reflection in Rust
               // Java: Assert.assertNotNull("sSdkVersionCode should not be null.");
               // Stubbed
               // Java: return sSdkVersionCode;
               // Stubbed - will fall through to default
            }
            // Java: catch (SecurityException e)
            {
               // Java: Log.e(TAG, "SecurityException", e);
               // Stubbed
               // Java: Assert.fail("It should not reach to here");
               // Stubbed
            }
            // Java: catch (NoSuchFieldException e)
            {
               // Java: Log.e(TAG, "NoSuchFieldException", e);
               // Stubbed
               // Java: Assert.fail("It should not reach to here");
               // Stubbed
            }
            // Java: catch (IllegalArgumentException e)
            {
               // Java: Log.e(TAG, "IllegalArgumentException", e);
               // Stubbed
               // Java: Assert.fail("It should not reach to here");
               // Stubbed
            }
            // Java: catch (IllegalAccessException e)
            {
               // Java: Log.e(TAG, "IllegalAccessException", e);
               // Stubbed
               // Java: Assert.fail("It should not reach to here");
               // Stubbed
            }
         }
         // Java: catch (ClassNotFoundException var7)
         {
         }
         // Java: sSdkVersionCode = -2;
         let mut guard = get_sSdkVersionCode().lock().unwrap();
         *guard = Some(-2);
         // Java: return sSdkVersionCode;
         return guard.unwrap();
      }
    }

    // Java: static private static Class<?> getDeclaredClass(Class<?> enclosingClazz, String declaredClass)
    fn getDeclaredClass(_enclosing_clazz: &str, _declared_class: &str) -> Option<&'static str> {
      // Java: Class<?>[] declaredClazzes = enclosingClazz.getDeclaredClasses();
      // No Rust equivalent for reflection - stubbed
      // Java: int i = 0;
      // Stubbed
      // Java: for(int len = declaredClazzes.length; i < len; ++i)
      {
         // Java: Class<?> clazz = declaredClazzes[i];
         // Stubbed
         // Java: if (null != clazz && clazz.getName().equals(declaredClass))
         {
            // Java: return clazz;
            // Stubbed
         }
      }
      // Java: return null;
      None
    }

}

// Java: public class VERSION_CODES
impl VERSION_CODES {
    // Java: VERSION_CODES()
    pub fn new() -> Self {
         // Java: super();
         VERSION_CODES {}
    }

}
