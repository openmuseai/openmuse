// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.base.log.Log
// Java: import cn.wps.core.runtime.IAssetManager
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.graphics.KDisplayMetrics
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.lang.reflect.InvocationTargetException
// Java: import java.lang.reflect.Method

use super::TestPlatform::TestPlatform;

// Java field: String TAG
static TAG: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String SAMPLE_DIR_NAME
static SAMPLE_DIR_NAME: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String sSDCardPath
static sSDCardPath: std::sync::OnceLock<std::sync::Mutex<Option<String>>> = std::sync::OnceLock::new();
// Java field: String sTempDir
static sTempDir: std::sync::OnceLock<std::sync::Mutex<Option<String>>> = std::sync::OnceLock::new();
// Java field: String sSamplesDir
static sSamplesDir: std::sync::OnceLock<std::sync::Mutex<Option<String>>> = std::sync::OnceLock::new();
// Java field: String sAssetsDir
static sAssetsDir: std::sync::OnceLock<std::sync::Mutex<Option<String>>> = std::sync::OnceLock::new();
// Java field: String sWriterSamplesDir
static sWriterSamplesDir: std::sync::OnceLock<std::sync::Mutex<Option<String>>> = std::sync::OnceLock::new();

fn get_TAG() -> &'static std::sync::Mutex<String> {
    TAG.get_or_init(|| std::sync::Mutex::new("TestPlatformAndroid".to_string()))
}
fn get_SAMPLE_DIR_NAME() -> &'static std::sync::Mutex<String> {
    SAMPLE_DIR_NAME.get_or_init(|| std::sync::Mutex::new("samples".to_string()))
}
fn get_sSDCardPath() -> &'static std::sync::Mutex<Option<String>> {
    sSDCardPath.get_or_init(|| std::sync::Mutex::new(None))
}
fn get_sTempDir() -> &'static std::sync::Mutex<Option<String>> {
    sTempDir.get_or_init(|| std::sync::Mutex::new(None))
}
fn get_sSamplesDir() -> &'static std::sync::Mutex<Option<String>> {
    sSamplesDir.get_or_init(|| std::sync::Mutex::new(None))
}
fn get_sAssetsDir() -> &'static std::sync::Mutex<Option<String>> {
    sAssetsDir.get_or_init(|| std::sync::Mutex::new(None))
}
fn get_sWriterSamplesDir() -> &'static std::sync::Mutex<Option<String>> {
    sWriterSamplesDir.get_or_init(|| std::sync::Mutex::new(None))
}

// Java: public class TestPlatformAndroid extends TestPlatform
pub struct TestPlatformAndroid {
    pub(crate) base: TestPlatform,
}

// Java: public class AssetsManager
pub struct AssetsManager {
}

// Java: public class TestPlatformAndroid extends TestPlatform
impl TestPlatformAndroid {
    // Java: TestPlatformAndroid()
    pub fn new() -> Self {
      // Java: super();
      TestPlatformAndroid {
          base: TestPlatform::new(),
      }
    }

    // Java: static String getSDCardPath()
    pub fn getSDCardPath() -> String {
      // Java: if (null == sSDCardPath)
      {
         // Java: String className = "android.os.Environment";
         // Java: try
         {
            // Java: Class<?> clazz = Class.forName("android.os.Environment");
            // Java: if (null != clazz)
            {
               // Java: String methodName = "getExternalStorageDirectory";
               // Java: Method method = clazz.getDeclaredMethod("getExternalStorageDirectory");
               // Java: File file = (File)method.invoke(clazz);
               // Java: sSDCardPath = file.getAbsolutePath();
            }
         }
         // Java: catch (ClassNotFoundException e)
         {
            // Java: Log.e(TAG, "", e);
            // Java: Assert.fail("It should not reach to here.");
         }
         // Java: catch (NoSuchMethodException e)
         {
            // Java: Log.e(TAG, "", e);
            // Java: Assert.fail("It should not reach to here.");
         }
         // Java: catch (IllegalArgumentException e)
         {
            // Java: Log.e(TAG, "", e);
            // Java: Assert.fail("It should not reach to here.");
         }
         // Java: catch (IllegalAccessException e)
         {
            // Java: Log.e(TAG, "", e);
            // Java: Assert.fail("It should not reach to here.");
         }
         // Java: catch (InvocationTargetException e)
         {
            // Java: Log.e(TAG, "", e);
            // Java: Assert.fail("It should not reach to here.");
         }
      }
      // Java: return sSDCardPath;
      todo!()
    }

    // Java: String getTempDir()
    pub fn getTempDir(&self) -> String {
      // Java: if (null == sTempDir)
      {
         // Java: File tempDir = new File(getSDCardPath() + File.separatorChar + "temp");
         // Java: tempDir.mkdirs();
         // Java: Assert.assertTrue("tempDir should be created.", tempDir.exists());
         // Java: sTempDir = tempDir.getAbsolutePath();
      }
      // Java: return sTempDir;
      todo!()
    }

    // Java: void setTempDir()
    pub fn setTempDir(&self) {
      // Java: Platform.setTempDirectory(this.getTempDir() + File.separatorChar);
    }

    // Java: void setAssetManager()
    pub fn setAssetManager(&self) {
      // Java: Platform.setAssetManager(new AssetsManager());
    }

    // Java: void setClipboardManager()
    pub fn setClipboardManager(&self) {
      // Java: Platform.setClipboardManager(new TestPlatformPc.MockClipboardManager());
    }

    // Java: void setDisplayMetrics()
    pub fn setDisplayMetrics(&self) {
      // Java: Platform.setDisplayMetrics(new KDisplayMetrics(800, 600, 1.0F, 96, 1.0F, 1.0F, 1.0F));
    }

    // Java: void setLogFilter()
    pub fn setLogFilter(&self) {
    }

    // Java: static String _getSamplesDir()
    pub fn _getSamplesDir() -> String {
      // Java: if (null == sSamplesDir)
      {
         // Java: String samplesDir = getSDCardPath() + File.separatorChar + SAMPLE_DIR_NAME;
         // Java: Assert.assertTrue("samples dir should exist: " + samplesDir, (new File(samplesDir)).exists());
         // Java: sSamplesDir = samplesDir;
      }
      // Java: return sSamplesDir;
      todo!()
    }

    // Java: String getSamplesDir()
    pub fn getSamplesDir(&self) -> String {
      // Java: return _getSamplesDir();
      todo!()
    }

    // Java: String getAssetsDir()
    pub fn getAssetsDir(&self) -> String {
      // Java: return TestPlatformAndroid.AssetsManager.getAssetsDir();
      todo!()
    }

    // Java: String getWriterSamplesDir()
    pub fn getWriterSamplesDir(&self) -> String {
      // Java: if (null == sWriterSamplesDir)
      {
         // Java: sWriterSamplesDir = this.getSamplesDir() + File.separatorChar + "writer";
      }
      // Java: return sWriterSamplesDir;
      todo!()
    }

}

// Java: public class AssetsManager
impl AssetsManager {
    // Java: AssetsManager()
    pub fn new() -> Self {
         // Java: super();
         AssetsManager {}
    }

    // Java: static String getAssetsDir()
    pub fn getAssetsDir() -> String {
         // Java: if (null == TestPlatformAndroid.sAssetsDir)
         {
            // Java: String dir = System.getenv("MOFFICE_ASSETS_DIR");
            // Java: if (dir == null || dir.length() == 0)
            {
               // Java: dir = System.getProperty("MOFFICE_ASSETS_DIR");
            }
            // Java: if (dir == null || dir.length() == 0)
            {
               // Java: dir = TestPlatformAndroid._getSamplesDir() + File.separatorChar + "assets";
            }
            // Java: if (dir == null || dir.length() == 0)
            {
               // Java: throw new RuntimeException("Please set a valid MOFFICE_ASSETS_DIR");
            }
            // Java: TestPlatformAndroid.sAssetsDir = dir;
         }
         // Java: return TestPlatformAndroid.sAssetsDir;
         todo!()
    }

    // Java: InputStream open(String arg0)
    pub fn open(&self, _arg0: &str) -> Box<dyn std::any::Any> {
         // Java: String path = getAssetsDir() + "/" + arg0;
         // Java: return new FileInputStream(path);
         todo!()
    }

    // Java: String[] list(String directoryName)
    pub fn list(&self, _directoryName: &str) -> Vec<String> {
         // Java: File dir = new File(directoryName);
         // Java: return dir.list();
         todo!()
    }

}
