// Java: import cn.wps.core.runtime.IAssetManager
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.graphics.KDisplayMetrics
// Java: import cn.wps.moffice.clipboard.IAssetFileDescriptor
// Java: import cn.wps.moffice.clipboard.IClipDescription
// Java: import cn.wps.moffice.clipboard.IClipboardManager
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.HashMap

use std::collections::HashMap;
use super::TestPlatform::TestPlatform;

// Java field: String sTempDir
static sTempDir: std::sync::OnceLock<std::sync::Mutex<Option<String>>> = std::sync::OnceLock::new();
// Java field: String sSamplesDir
static sSamplesDir: std::sync::OnceLock<std::sync::Mutex<Option<String>>> = std::sync::OnceLock::new();
// Java field: String sAssetsDir
static sAssetsDir: std::sync::OnceLock<std::sync::Mutex<Option<String>>> = std::sync::OnceLock::new();
// Java field: String sWriterSamplesDir
static sWriterSamplesDir: std::sync::OnceLock<std::sync::Mutex<Option<String>>> = std::sync::OnceLock::new();

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

// Java: public class TestPlatformPc extends TestPlatform
pub struct TestPlatformPc {
    pub(crate) base: TestPlatform,
}

// Java: public class AssetsManager
pub struct PcAssetsManager {
}

// Java: public class MockClipboardManager
pub struct MockClipboardManager {
    // Java field: CharSequence mContent
    pub(crate) mContent: Option<String>,
}

// Java: public class TestPlatformPc extends TestPlatform
impl TestPlatformPc {
    // Java: TestPlatformPc()
    pub fn new() -> Self {
      // Java: super();
      TestPlatformPc {
          base: TestPlatform::new(),
      }
    }

    // Java: String getTempDir()
    pub fn getTempDir(&self) -> String {
      // Java: if (null == sTempDir)
      {
         // Java: sTempDir = Util.getMOfficeAppDir() + File.separatorChar + "temp";
         // Java: File tempDir = new File(sTempDir);
         // Java: tempDir.mkdirs();
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
      // Java: Platform.setClipboardManager(new MockClipboardManager());
    }

    // Java: void setDisplayMetrics()
    pub fn setDisplayMetrics(&self) {
      // Java: Platform.setDisplayMetrics(new KDisplayMetrics(800, 600, 1.0F, 96, 1.0F, 96.0F, 96.0F));
    }

    // Java: void setLogFilter()
    pub fn setLogFilter(&self) {
      // Java: Platform.readLogFilter(Util.getMOfficeAppDir() + "/log.xml");
      // Java: String user_home = System.getenv("user.home");
      // Java: Platform.readLogFilter(user_home + "/.log.xml");
    }

    // Java: String getSamplesDir()
    pub fn getSamplesDir(&self) -> String {
      // Java: if (null == sSamplesDir)
      {
         // Java: String dir = null;
         // Java: if (dir == null || dir.length() == 0)
         {
            // Java: dir = System.getenv("MOFFICE_SAMPLES_DIR");
         }
         // Java: if (dir == null || dir.length() == 0)
         {
            // Java: dir = System.getProperty("MOFFICE_SAMPLES_DIR");
         }
         // Java: if (dir == null || dir.length() == 0)
         {
            // Java: dir = Util.getConfiguration().getProperty("samples.dir");
         }
         // Java: if (dir == null || dir.length() == 0)
         {
            // Java: throw new RuntimeException("Please set a valid MOFFICE_SAMPLES_DIR");
         }
         // Java: sSamplesDir = dir;
      }
      // Java: return sSamplesDir;
      todo!()
    }

    // Java: String getAssetsDir()
    pub fn getAssetsDir(&self) -> String {
      // Java: return TestPlatformPc.AssetsManager.getAssetsDir();
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
impl PcAssetsManager {
    // Java: AssetsManager()
    pub fn new() -> Self {
         // Java: super();
         PcAssetsManager {}
    }

    // Java: static String getAssetsDir()
    pub fn getAssetsDir() -> String {
         // Java: if (null == TestPlatformPc.sAssetsDir)
         {
            // Java: String dir = System.getenv("MOFFICE_ASSETS_DIR");
            // Java: if (dir == null || dir.length() == 0)
            {
               // Java: dir = System.getProperty("MOFFICE_ASSETS_DIR");
            }
            // Java: if (dir == null || dir.length() == 0)
            {
               // Java: dir = Util.getMOfficeAppDir() + "/moffice/assets";
            }
            // Java: if (dir == null || dir.length() == 0)
            {
               // Java: throw new RuntimeException("Please set a valid MOFFICE_ASSETS_DIR");
            }
            // Java: TestPlatformPc.sAssetsDir = dir;
         }
         // Java: return TestPlatformPc.sAssetsDir;
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

// Java: public class MockClipboardManager
impl MockClipboardManager {
    // Java: CharSequence getText()
    pub fn getText(&self) -> Option<&String> {
         // Java: return this.mContent;
         self.mContent.as_ref()
    }

    // Java: void setText(CharSequence text)
    pub fn setText(&mut self, _text: &str) {
         // Java: this.mContent = text;
         self.mContent = Some(_text.to_string());
    }

    // Java: void setPrimaryClip(String text, String uri, String[] mimeTypes)
    pub fn setPrimaryClip(&mut self, _text: &str, _uri: &str, _mimeTypes: &[String]) {
    }

    // Java: boolean hasPrimaryClip()
    pub fn hasPrimaryClip(&self) -> bool {
         // Java: return false;
         false
    }

    // Java: IAssetFileDescriptor openTypedAssetFileDescriptor(String mimeType, HashMap<String, String> bundle)
    pub fn openTypedAssetFileDescriptor(&self, _mimeType: &str, _bundle: &HashMap<String, String>) -> Option<Box<dyn std::any::Any>> {
         // Java: return null;
         None
    }

    // Java: IClipDescription getClipDescription()
    pub fn getClipDescription(&self) -> Option<Box<dyn std::any::Any>> {
         // Java: return null;
         None
    }

    // Java: boolean hasText()
    pub fn hasText(&self) -> bool {
         // Java: return this.mContent != null && this.mContent.length() > 0;
         self.mContent.as_ref().map_or(false, |c| !c.is_empty())
    }

    // Java: void setPlainTextPrimaryClip(String label, String text)
    pub fn setPlainTextPrimaryClip(&mut self, _label: &str, _text: &str) {
    }

}
