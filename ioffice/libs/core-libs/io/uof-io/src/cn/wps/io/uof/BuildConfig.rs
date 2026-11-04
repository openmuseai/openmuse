// Java: import cn.wps.base.assertion.Assert
// use util::cn::wps::core::runtime::Assert; // TODO: crate not available yet
// Java: import cn.wps.base.log.Log
// use kfs_kfs_app::cn::wps::moffice::kfs::log::Log; // TODO: crate not available yet
// Java: import java.lang.reflect.Field
// No Rust equivalent for java.lang.reflect.Field

// Java field: String TAG
pub(crate) const TAG: &str = "BuildConfig";
// Java field: boolean DEBUG
pub(crate) const DEBUG: bool = true;
// Java field: boolean RELEASE
pub(crate) const RELEASE: bool = false;
// Java field: Integer sSdkVersionCode
static sSdkVersionCode: std::sync::OnceLock<std::sync::Mutex<Option<i32>>> = std::sync::OnceLock::new();
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
    pub fn BuildConfig(&self) {
      // Java: super();
    }

    // Java: static int getRuntimeSdkVersion()
    pub fn getRuntimeSdkVersion() -> i32 {
      // Java: if (null != sSdkVersionCode)
      let pool = sSdkVersionCode.get_or_init(|| std::sync::Mutex::new(None));
      let mut guard = pool.lock().unwrap();
      if let Some(code) = *guard {
         // Java: return sSdkVersionCode;
         return code;
      }
      // Java: else
      {
         // Java: try
         {
            // Java: Class<?> buildClazz = Class.forName("android.os.Build");
            // Android reflection not available in Rust - using fallback
            // Java: Assert.assertNotNull("clazz should not be null.");
            // Java: try
            {
               // Java: Class<?> versionClazz = getDeclaredClass(buildClazz, "android.os.Build$VERSION");
               // Java: Assert.assertNotNull("versionClazz should not be null.");
               // Java: Field sdkIntField = versionClazz.getDeclaredField("SDK_INT");
               // Java: Assert.assertNotNull("sdkIntField should not be null.");
               // Java: sSdkVersionCode = sdkIntField.getInt(versionClazz);
               // Java: Assert.assertNotNull("sSdkVersionCode should not be null.");
               // Java: return sSdkVersionCode;
            }
            // Java: catch (SecurityException e)
            {
               // Java: Log.e(TAG, "SecurityException", e);
               // Java: Assert.fail("It should not reach to here");
            }
            // Java: catch (NoSuchFieldException e)
            {
               // Java: Log.e(TAG, "NoSuchFieldException", e);
               // Java: Assert.fail("It should not reach to here");
            }
            // Java: catch (IllegalArgumentException e)
            {
               // Java: Log.e(TAG, "IllegalArgumentException", e);
               // Java: Assert.fail("It should not reach to here");
            }
            // Java: catch (IllegalAccessException e)
            {
               // Java: Log.e(TAG, "IllegalAccessException", e);
               // Java: Assert.fail("It should not reach to here");
            }
         }
         // Java: catch (ClassNotFoundException var7)
         {
         }
         // Java: sSdkVersionCode = -2;
         *guard = Some(-2);
         // Java: return sSdkVersionCode;
         return -2;
      }
    }

    // Java: static private static Class<?> getDeclaredClass(Class<?> enclosingClazz, String declaredClass)
    fn getDeclaredClass(_enclosing_clazz: &str, _declared_class: &str) -> Option<String> {
      // Java: Class<?>[] declaredClazzes = enclosingClazz.getDeclaredClasses();
      // Java: int i = 0;
      // Java: for(int len = declaredClazzes.length; i < len; ++i)
      {
         // Java: Class<?> clazz = declaredClazzes[i];
         // Java: if (null != clazz && clazz.getName().equals(declaredClass))
         {
            // Java: return clazz;
         }
      }
      // Java: return null;
      None
    }

}

// Java: public class VERSION_CODES
impl VERSION_CODES {
    // Java: VERSION_CODES()
    pub fn VERSION_CODES(&self) {
         // Java: super();
    }

}
