#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.kfs.File
use crate::kfs::File::File;

use std::sync::Mutex;

// Java: public interface IPlatformFile
pub trait IPlatformFile: Send + Sync {
    // Java: File createTempFile(String var1, String var2);
    fn createTempFile(&self, prefix: &str, suffix: &str) -> Option<File>;
    // Java: String getCurrentPidTmpDir();
    fn getCurrentPidTmpDir(&self) -> String;
}

// Java field:    private static final String TAG = PlatformAdapter.class.getSimpleName();
const TAG: &str = "PlatformAdapter";
// Java field:    private static IPlatformFile mPlatformFile;
static M_PLATFORM_FILE: Mutex<Option<Box<dyn IPlatformFile>>> = Mutex::new(None);
// Java field:    private static boolean mOverseaVersion = false;
static M_OVERSEA_VERSION: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
// Java field:    private static boolean mMultiDexConfig = false;
static M_MULTI_DEX_CONFIG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// Java: public class PlatformAdapter
#[derive(Default)]
pub struct PlatformAdapter {}

// Java: public class PlatformAdapter
impl PlatformAdapter {
    // Java: public PlatformAdapter()
    pub fn new() -> Self { Self {} }

    // Java: public static File createTempFile(String prefix, String suffix)
    pub fn createTempFile(prefix: &str, suffix: &str) -> Option<File> {
        // Java: if (null != mPlatformFile)
        let lock = M_PLATFORM_FILE.lock().unwrap();
        if let Some(ref pf) = *lock {
            // Java: return mPlatformFile.createTempFile(prefix, suffix);
            pf.createTempFile(prefix, suffix)
        } else {
            // Java: try { return File.createTempFile(prefix, suffix); }
            // Java: catch (IOException e) { Log.e(TAG, "IOException", e); return null; }
            Some(File::createTempFile(prefix, suffix))
        }
    }

    // Java: public static void setPlatformFile(IPlatformFile platformFile)
    pub fn setPlatformFile(platformFile: Box<dyn IPlatformFile>) {
        // Java: mPlatformFile = platformFile;
        let mut lock = M_PLATFORM_FILE.lock().unwrap();
        *lock = Some(platformFile);
    }

    // Java: public static void setOverSeaVersion(boolean isOverseaVersion)
    pub fn setOverSeaVersion(isOverseaVersion: bool) {
        // Java: mOverseaVersion = isOverseaVersion;
        M_OVERSEA_VERSION.store(isOverseaVersion, std::sync::atomic::Ordering::Relaxed);
    }

    // Java: public static boolean isOverSeaVersion()
    pub fn isOverSeaVersion() -> bool {
        // Java: return mOverseaVersion;
        M_OVERSEA_VERSION.load(std::sync::atomic::Ordering::Relaxed)
    }

    // Java: public static void setMultiDexConfig(boolean multiDexConfig)
    pub fn setMultiDexConfig(multiDexConfig: bool) {
        // Java: mMultiDexConfig = multiDexConfig;
        M_MULTI_DEX_CONFIG.store(multiDexConfig, std::sync::atomic::Ordering::Relaxed);
    }

    // Java: public static boolean isMultiDexConfig()
    pub fn isMultiDexConfig() -> bool {
        // Java: return mMultiDexConfig;
        M_MULTI_DEX_CONFIG.load(std::sync::atomic::Ordering::Relaxed)
    }

    // Java: public static String getCurrentPidTmpDir()
    pub fn getCurrentPidTmpDir() -> Option<String> {
        // Java: if (null != mPlatformFile)
        let lock = M_PLATFORM_FILE.lock().unwrap();
        if let Some(ref pf) = *lock {
            // Java: return mPlatformFile.getCurrentPidTmpDir();
            Some(pf.getCurrentPidTmpDir())
        } else {
            // Java: return File.createTempFile("pid_test", "test").getParent();
            let tmp = std::env::temp_dir().to_string_lossy().to_string();
            Some(tmp)
        }
    }
}
