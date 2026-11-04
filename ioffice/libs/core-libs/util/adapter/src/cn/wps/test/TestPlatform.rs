// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.core.runtime.Platform

// Java field: TestPlatform sInstance
static sInstance: std::sync::OnceLock<std::sync::Mutex<Option<Box<dyn std::any::Any + Send>>>> = std::sync::OnceLock::new();

fn get_sInstance() -> &'static std::sync::Mutex<Option<Box<dyn std::any::Any + Send>>> {
    sInstance.get_or_init(|| std::sync::Mutex::new(None))
}

// Java: public class TestPlatform
pub struct TestPlatform {
}

// Java: public enum PlatformType
pub enum PlatformType {
    Android,
    PC,
}

// Java: public class TestPlatform
impl TestPlatform {
    // Java: TestPlatform()
    pub fn new() -> Self {
      // Java: super();
      TestPlatform {}
    }

    // Java: static PlatformType getCurrentPlatform()
    pub fn getCurrentPlatform() -> PlatformType {
      // Java: return isAndroid() ? TestPlatform.PlatformType.Android : TestPlatform.PlatformType.PC;
      if TestPlatform::isAndroid() {
          PlatformType::Android
      } else {
          PlatformType::PC
      }
    }

    // Java: static boolean isAndroid()
    pub fn isAndroid() -> bool {
      // Java: String SWING = "javax.swing.JFrame";
      // Java: try
      {
         // Java: Class<?> clazz = Class.forName("javax.swing.JFrame");
         // Java: return false;
      }
      // Java: catch (ClassNotFoundException var2)
      {
         // Java: return true;
      }
      false
    }

    // Java: static TestPlatform getInstance()
    pub fn getInstance() -> Box<dyn std::any::Any> {
      // Java: if (null == sInstance)
      {
         // Java: PlatformType type = getCurrentPlatform();
         // Java: switch (type)
         {
            // Java: case Android:
               // Java: sInstance = new TestPlatformAndroid();
               // Java: break;
            // Java: case PC:
               // Java: sInstance = new TestPlatformPc();
               // Java: break;
            // Java: default:
               // Java: Assert.fail("Not supported: " + type);
         }
      }
      // Java: return sInstance;
      todo!()
    }

    // Java: static void init()
    pub fn init() {
      // Java: if (Platform.getAssetManager() == null)
      {
         // Java: TestPlatform platform = getInstance();
         // Java: platform.setTempDir();
         // Java: platform.setAssetManager();
         // Java: platform.setClipboardManager();
         // Java: platform.setDisplayMetrics();
         // Java: platform.setLogFilter();
         // Java: Platform.setLanguageCode("zh-CN");
      }
    }

    // Java: abstract String getTempDir()
    pub fn getTempDir(&self) -> String {
      todo!()
    }

    // Java: abstract void setTempDir()
    pub fn setTempDir(&self) {
      todo!()
    }

    // Java: abstract void setAssetManager()
    pub fn setAssetManager(&self) {
      todo!()
    }

    // Java: abstract void setClipboardManager()
    pub fn setClipboardManager(&self) {
      todo!()
    }

    // Java: abstract void setDisplayMetrics()
    pub fn setDisplayMetrics(&self) {
      todo!()
    }

    // Java: abstract void setLogFilter()
    pub fn setLogFilter(&self) {
      todo!()
    }

    // Java: abstract String getSamplesDir()
    pub fn getSamplesDir(&self) -> String {
      todo!()
    }

    // Java: abstract String getAssetsDir()
    pub fn getAssetsDir(&self) -> String {
      todo!()
    }

    // Java: abstract String getWriterSamplesDir()
    pub fn getWriterSamplesDir(&self) -> String {
      todo!()
    }

}

// Java: public enum PlatformType
impl PlatformType {
    // Java: PlatformType()
    pub fn new() -> Self {
      PlatformType::PC
    }

}
