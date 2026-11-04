use std::path::Path;
use std::fs;
use std::sync::Arc;
use crate::cn::wps::core::runtime::ILog::ILog;
use crate::cn::wps::core::runtime::internal::NullLog::NullLog;

// Java field: InternalPlatform instance

// Java: public class InternalPlatform
pub struct InternalPlatform {
    // Java field: String mTempDirectory
    pub(crate) mTempDirectory: String,
    // Java field: String mBackupDirectory
    pub(crate) mBackupDirectory: String,
    // Java field: String mSaveDirectory
    pub(crate) mSaveDirectory: String,
    // Java field: String mCustomFontsDirectory
    pub(crate) mCustomFontsDirectory: String,
    // Java field: String mHandWrittenDirectory
    pub(crate) mHandWrittenDirectory: String,
    // Java field: String mOldCustomFontsDirectory
    pub(crate) mOldCustomFontsDirectory: String,
    // Java field: String mClipboardDirectory
    pub(crate) mClipboardDirectory: String,
    // Java field: String mDragDirectory
    pub(crate) mDragDirectory: String,
    // Java field: String mFontsCacheFilePath
    pub(crate) mFontsCacheFilePath: String,
    // Java field: ILog mLog
    pub(crate) mLog: Arc<dyn ILog + Send + Sync>,
    // Java field: String mSystemFontsDirectory
    pub(crate) mSystemFontsDirectory: String,
    // Java field: String mSDCustomFontsDirectory
    pub(crate) mSDCustomFontsDirectory: String,
    // Java field: String mWaterMarkFontsDirectory
    pub(crate) mWaterMarkFontsDirectory: String,
}

static INSTANCE: std::sync::OnceLock<std::sync::Mutex<InternalPlatform>> = std::sync::OnceLock::new();

// Java: public class InternalPlatform
impl InternalPlatform {
    // Java: static InternalPlatform getInstance()
    pub fn getInstance() -> &'static std::sync::Mutex<InternalPlatform> {
        // Java: return instance;
        INSTANCE.get_or_init(|| std::sync::Mutex::new(InternalPlatform::new()))
    }

    // Java: InternalPlatform()
    pub fn new() -> Self {
        // Java: super();
        let temp = std::env::temp_dir().to_string_lossy().to_string();
        // Java: this.mBackupDirectory = this.mTempDirectory;
        // Java: this.mSaveDirectory = this.mTempDirectory;
        // Java: this.mCustomFontsDirectory = this.mTempDirectory;
        // Java: this.mHandWrittenDirectory = this.mTempDirectory;
        // Java: this.mOldCustomFontsDirectory = this.mTempDirectory;
        // Java: this.mClipboardDirectory = this.mTempDirectory + File.separator + ".clipboard" + File.separator;
        let clipboard_dir = format!("{}/.clipboard/", &temp);
        // Java: this.mDragDirectory = this.mTempDirectory + File.separator + ".drag" + File.separator;
        let drag_dir = format!("{}/.drag/", &temp);
        // Java: this.mFontsCacheFilePath = this.mTempDirectory + File.pathSeparator + ".fontsCache";
        let fonts_cache = format!("{}/.fontsCache", &temp);
        // Java: this.mLog = NullLog.getDefault();
        // Java: this.mSystemFontsDirectory = this.mTempDirectory;
        // Java: this.mSDCustomFontsDirectory = "";
        // Java: this.mWaterMarkFontsDirectory = "";
        InternalPlatform {
            mBackupDirectory: temp.clone(),
            mSaveDirectory: temp.clone(),
            mCustomFontsDirectory: temp.clone(),
            mHandWrittenDirectory: temp.clone(),
            mOldCustomFontsDirectory: temp.clone(),
            mClipboardDirectory: clipboard_dir,
            mDragDirectory: drag_dir,
            mFontsCacheFilePath: fonts_cache,
            mLog: Arc::new(NullLog::new()),
            mSystemFontsDirectory: temp.clone(),
            mSDCustomFontsDirectory: String::new(),
            mWaterMarkFontsDirectory: String::new(),
            mTempDirectory: temp,
        }
    }

    // Java: String getBackupDirectory()
    pub fn getBackupDirectory(&self) -> &str {
        // Java: Assert.assertTrue(this.mBackupDirectory != null && this.mBackupDirectory.length() > 0);
        // Java: return this.mBackupDirectory;
        &self.mBackupDirectory
    }

    // Java: String getSaveDirectory()
    pub fn getSaveDirectory(&self) -> &str {
        // Java: Assert.assertTrue(this.mSaveDirectory != null && this.mSaveDirectory.length() > 0);
        // Java: return this.mSaveDirectory;
        &self.mSaveDirectory
    }

    // Java: String getCustomFontsDirectory()
    pub fn getCustomFontsDirectory(&self) -> &str {
        // Java: Assert.assertTrue(this.mCustomFontsDirectory != null && this.mCustomFontsDirectory.length() > 0);
        // Java: return this.mCustomFontsDirectory;
        &self.mCustomFontsDirectory
    }

    // Java: String getWaterMarkFontsDirectory()
    pub fn getWaterMarkFontsDirectory(&self) -> &str {
        // Java: Assert.assertTrue(this.mWaterMarkFontsDirectory != null && this.mWaterMarkFontsDirectory.length() > 0);
        // Java: return this.mWaterMarkFontsDirectory;
        &self.mWaterMarkFontsDirectory
    }

    // Java: String getHandWrittenDirectory()
    pub fn getHandWrittenDirectory(&self) -> &str {
        // Java: Assert.assertTrue(this.mHandWrittenDirectory != null && this.mHandWrittenDirectory.length() > 0);
        // Java: return this.mHandWrittenDirectory;
        &self.mHandWrittenDirectory
    }

    // Java: String getOldCustomFontsDirectory()
    pub fn getOldCustomFontsDirectory(&self) -> &str {
        // Java: Assert.assertTrue(this.mOldCustomFontsDirectory != null && this.mOldCustomFontsDirectory.length() > 0);
        // Java: return this.mOldCustomFontsDirectory;
        &self.mOldCustomFontsDirectory
    }

    // Java: String getFontsCacheFilePath()
    pub fn getFontsCacheFilePath(&self) -> &str {
        // Java: return this.mFontsCacheFilePath;
        &self.mFontsCacheFilePath
    }

    // Java: ILog getLog()
    pub fn getLog(&self) -> Arc<dyn ILog + Send + Sync> {
        // Java: return this.mLog;
        Arc::clone(&self.mLog)
    }

    // Java: String getSystemFontsDirectory()
    pub fn getSystemFontsDirectory(&self) -> &str {
        // Java: return this.mSystemFontsDirectory;
        &self.mSystemFontsDirectory
    }

    // Java: String getSDCustomFontsDirectory()
    pub fn getSDCustomFontsDirectory(&self) -> &str {
        // Java: return this.mSDCustomFontsDirectory;
        &self.mSDCustomFontsDirectory
    }

    // Java: String getTempDirectory()
    pub fn getTempDirectory(&self) -> &str {
        // Java: Assert.assertTrue(this.mTempDirectory != null && this.mTempDirectory.length() > 0);
        // Java: return this.mTempDirectory;
        &self.mTempDirectory
    }

    // Java: String getClipboardDirectory()
    pub fn getClipboardDirectory(&self) -> &str {
        // Java: return this.mClipboardDirectory;
        &self.mClipboardDirectory
    }

    // Java: String getDragDirectory()
    pub fn getDragDirectory(&self) -> &str {
        // Java: return this.mDragDirectory;
        &self.mDragDirectory
    }

    // Java: void setBackupDirectory(String backupDirectory)
    pub fn setBackupDirectory(&mut self, backup_directory: &str) {
        // Java: Assert.assertTrue(backupDirectory != null && backupDirectory.length() > 0);
        // Java: this.mBackupDirectory = backupDirectory;
        self.mBackupDirectory = backup_directory.to_string();
        // Java: File f = new File(this.mBackupDirectory);
        let f = Path::new(&self.mBackupDirectory);
        // Java: if (!f.exists())
        if !f.exists() {
            // Java: f.mkdirs();
            let _ = fs::create_dir_all(f);
        }
    }

    // Java: void setSaveDirectory(String saveDirectory)
    pub fn setSaveDirectory(&mut self, save_directory: &str) {
        // Java: Assert.assertTrue(saveDirectory != null && saveDirectory.length() > 0);
        // Java: this.mSaveDirectory = saveDirectory;
        self.mSaveDirectory = save_directory.to_string();
        // Java: File f = new File(this.mSaveDirectory);
        let f = Path::new(&self.mSaveDirectory);
        // Java: if (!f.exists())
        if !f.exists() {
            // Java: f.mkdirs();
            let _ = fs::create_dir_all(f);
        }
    }

    // Java: void setCustomFontsDirectory(String customFontsDirectory)
    pub fn setCustomFontsDirectory(&mut self, custom_fonts_directory: &str) {
        // Java: Assert.assertTrue(customFontsDirectory != null && customFontsDirectory.length() > 0);
        // Java: this.mCustomFontsDirectory = customFontsDirectory;
        self.mCustomFontsDirectory = custom_fonts_directory.to_string();
        // Java: File f = new File(this.mCustomFontsDirectory);
        let f = Path::new(&self.mCustomFontsDirectory);
        // Java: if (!f.exists())
        if !f.exists() {
            // Java: f.mkdirs();
            let _ = fs::create_dir_all(f);
        }
    }

    // Java: void setHandWrittenDirectory(String handWrittenDirectory)
    pub fn setHandWrittenDirectory(&mut self, hand_written_directory: &str) {
        // Java: Assert.assertTrue(handWrittenDirectory != null && handWrittenDirectory.length() > 0);
        // Java: this.mHandWrittenDirectory = handWrittenDirectory;
        self.mHandWrittenDirectory = hand_written_directory.to_string();
        // Java: File f = new File(this.mHandWrittenDirectory);
        let f = Path::new(&self.mHandWrittenDirectory);
        // Java: if (!f.exists())
        if !f.exists() {
            // Java: f.mkdirs();
            let _ = fs::create_dir_all(f);
        }
    }

    // Java: void setOldCustomFontsDirectory(String oldCustomFontsDirectory)
    pub fn setOldCustomFontsDirectory(&mut self, old_custom_fonts_directory: &str) {
        // Java: Assert.assertTrue(oldCustomFontsDirectory != null && oldCustomFontsDirectory.length() > 0);
        // Java: this.mOldCustomFontsDirectory = oldCustomFontsDirectory;
        self.mOldCustomFontsDirectory = old_custom_fonts_directory.to_string();
    }

    // Java: void setFontsCacheFilePath(String fontsCachePath)
    pub fn setFontsCacheFilePath(&mut self, fonts_cache_path: &str) {
        // Java: this.mFontsCacheFilePath = fontsCachePath;
        self.mFontsCacheFilePath = fonts_cache_path.to_string();
    }

    // Java: void setLog(ILog log)
    pub fn setLog(&mut self, log: Option<Arc<dyn ILog + Send + Sync>>) {
        // Java: if (log != null)
        if let Some(log) = log {
            // Java: this.mLog = log;
            self.mLog = log;
        }
    }

    // Java: void setSystemFontsDirectory(String systemFontsDirectory)
    pub fn setSystemFontsDirectory(&mut self, system_fonts_directory: &str) {
        // Java: this.mSystemFontsDirectory = systemFontsDirectory;
        self.mSystemFontsDirectory = system_fonts_directory.to_string();
    }

    // Java: void setSDCustomFontsDirectory(String sdCustomFontsDirectory)
    pub fn setSDCustomFontsDirectory(&mut self, sd_custom_fonts_directory: &str) {
        // Java: this.mSDCustomFontsDirectory = sdCustomFontsDirectory;
        self.mSDCustomFontsDirectory = sd_custom_fonts_directory.to_string();
    }

    // Java: void setWaterMarkFontsDirectory(String watermarkFontsDirectory)
    pub fn setWaterMarkFontsDirectory(&mut self, watermark_fonts_directory: &str) {
        // Java: this.mWaterMarkFontsDirectory = watermarkFontsDirectory;
        self.mWaterMarkFontsDirectory = watermark_fonts_directory.to_string();
    }

    // Java: void setTempDirectory(String tempDirectory)
    pub fn setTempDirectory(&mut self, temp_directory: &str) {
        // Java: Assert.assertTrue(tempDirectory != null && tempDirectory.length() > 0);
        // Java: this.mTempDirectory = tempDirectory;
        self.mTempDirectory = temp_directory.to_string();
        // Java: File f = new File(this.mTempDirectory);
        let f = Path::new(&self.mTempDirectory);
        // Java: if (!f.exists())
        if !f.exists() {
            // Java: f.mkdirs();
            let _ = fs::create_dir_all(f);
        }
    }

    // Java: void setClipboardDirectory(String clipboardDirectory)
    pub fn setClipboardDirectory(&mut self, clipboard_directory: &str) {
        // Java: this.mClipboardDirectory = clipboardDirectory;
        self.mClipboardDirectory = clipboard_directory.to_string();
        // Java: File f = new File(this.mClipboardDirectory);
        let f = Path::new(&self.mClipboardDirectory);
        // Java: if (!f.exists())
        if !f.exists() {
            // Java: f.mkdirs();
            let _ = fs::create_dir_all(f);
        }
    }

    // Java: void setDragDirectory(String dragDirectory)
    pub fn setDragDirectory(&mut self, drag_directory: &str) {
        // Java: this.mDragDirectory = dragDirectory;
        self.mDragDirectory = drag_directory.to_string();
        // Java: File f = new File(this.mDragDirectory);
        let f = Path::new(&self.mDragDirectory);
        // Java: if (!f.exists())
        if !f.exists() {
            // Java: f.mkdirs();
            let _ = fs::create_dir_all(f);
        }
    }

    // Java: String getLogFilePath()
    pub fn getLogFilePath(&self) -> &str {
        // Java: return this.mTempDirectory;
        &self.mTempDirectory
    }

    // Java: String getCrashLogFilePath()
    pub fn getCrashLogFilePath(&self) -> &str {
        // Java: return this.mTempDirectory;
        &self.mTempDirectory
    }

    // Java: String getExitReasonLogFilePath()
    pub fn getExitReasonLogFilePath(&self) -> &str {
        // Java: return this.mTempDirectory;
        &self.mTempDirectory
    }

    // Java: String getSaveLogFilePath()
    pub fn getSaveLogFilePath(&self) -> &str {
        // Java: return this.mTempDirectory;
        &self.mTempDirectory
    }

    // Java: String getCloudStorageLogFilePath()
    pub fn getCloudStorageLogFilePath(&self) -> &str {
        // Java: return this.mTempDirectory;
        &self.mTempDirectory
    }

    // Java: String getRoamingFileLogFilePath()
    pub fn getRoamingFileLogFilePath(&self) -> &str {
        // Java: return this.mTempDirectory;
        &self.mTempDirectory
    }

    // Java: String getSharePlayLogFilePath()
    pub fn getSharePlayLogFilePath(&self) -> &str {
        // Java: return this.mTempDirectory;
        &self.mTempDirectory
    }

    // Java: int getPid()
    pub fn getPid(&self) -> i32 {
        // Java: return 0;
        0
    }
}
