// Java: import cn.wps.base.assertion.Assert
// use util::cn::wps::core::runtime::Assert; // cross-crate dep
// Java: import cn.wps.base.log.Log
// use kfs_kfs_app::cn::wps::moffice::kfs::log::Log; // cross-crate dep
// Java: import cn.wps.core.runtime.internal.InternalPlatform
use crate::cn::wps::core::runtime::internal::InternalPlatform::InternalPlatform;
// Java: import cn.wps.core.runtime.internal.LogFilter
use crate::cn::wps::core::runtime::internal::LogFilter::LogFilter;
// Java: import cn.wps.graphics.KDisplayMetrics
use core_libs_util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
// Java: import cn.wps.graphics.bitmap.IPicturePool
use core_libs_util::cn::wps::graphics::bitmap::IPicturePool::IPicturePool;
// Java: import cn.wps.math.IMathDrawer
use core_libs_util::cn::wps::math::IMathDrawer::IMathDrawer;
// Java: import cn.wps.moffice.clipboard.IClipboardManager
use crate::cn::wps::moffice::clipboard::IClipboardManager::IClipboardManager;
// Java: import cn.wps.moffice.define.UILanguage
use crate::cn::wps::moffice::define::UILanguage::UILanguage;
// Java: import cn.wps.moffice.kfs.File
use core_libs_kfs_app::cn::wps::moffice::kfs::File::File as KfsFile;
// Rust path operations implement the adjacent Java File API.
use std::path::Path;
// Java: import cn.wps.moffice.kfs.FileInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream; // cross-crate dep
// Java: import cn.wps.moffice.util.FileUtil
// Rust filesystem operations implement the adjacent Java FileUtil calls.
use std::fs;
// Java: import cn.wps.moffice.util.KSFileLog
use crate::cn::wps::moffice::util::KSFileLog::KSFileLog;
// Java: import cn.wps.moffice.util.PlatformAdapter
use core_libs_kfs_app::cn::wps::moffice::util::PlatformAdapter::{IPlatformFile, PlatformAdapter};
// Java: import cn.wps.moffice.util.XmlDocument
// use util::cn::wps::moffice::util::XmlDocument; // cross-crate dep
// Java: import cn.wps.util.CreatefileInfolder
// use util::cn::wps::util::CreatefileInfolder; // cross-crate dep
// Java: import cn.wps.util.Pair
// use util::cn::wps::util::Pair; // cross-crate dep
// Java: import java.io.IOException
// Mapped to std::io::Error at each Result-returning method boundary.
// Java: import java.io.InputStream
// Mapped to borrowed byte slices for the read-only filter parser input.
// Java: import java.util.ArrayList
// Mapped to Vec at each local collection declaration.
// Java: import java.util.List
// Mapped to Vec<String> at field and method boundaries.
// Java: import javax.xml.parsers.DocumentBuilder
// use crate::javax::xml::parsers::DocumentBuilder; // not available
// Java: import javax.xml.parsers.ParserConfigurationException
// use crate::javax::xml::parsers::ParserConfigurationException; // not available
// Java: import org.w3c.dom.Document
// use io_wps_io::cn::wps::io::dom::Document; // cross-crate dep
// Java: import org.w3c.dom.Element
// use io_wps_io::io::dom::Element; // cross-crate dep
// Java: import org.w3c.dom.NodeList
// use crate::org::w3c::dom::NodeList; // not available
// Java: import org.xml.sax.SAXException
// use crate::org::xml::sax::SAXException; // not available

// Rust synchronization support for the Java static fields below.
use std::sync::{Arc, Mutex, OnceLock};

// Java field: String TAG
pub(crate) const TAG: &str = "Platform";
// Java field: Integer sPid
static S_PID: OnceLock<Mutex<Option<i32>>> = OnceLock::new();
// Java field: boolean mKillProcess
static M_KILL_PROCESS: OnceLock<Mutex<bool>> = OnceLock::new();
// Java field: KDisplayMetrics mDisplayMetrics
static M_DISPLAY_METRICS: OnceLock<Mutex<Option<Arc<KDisplayMetrics>>>> = OnceLock::new();
// Java field: IAssetManager mAssetManager
static M_ASSET_MANAGER: OnceLock<Mutex<Option<Arc<dyn crate::cn::wps::core::runtime::IAssetManager::IAssetManager + Send + Sync>>>> = OnceLock::new();
// Java field: IResourceManager mResourceManager
static M_RESOURCE_MANAGER: OnceLock<Mutex<Option<Arc<dyn crate::cn::wps::core::runtime::IResourceManager::IResourceManager + Send + Sync>>>> = OnceLock::new();
// Java field: IHistoryManager mHistoryManager
static M_HISTORY_MANAGER: OnceLock<Mutex<Option<Arc<dyn crate::cn::wps::core::runtime::IHistoryManager::IHistoryManager + Send + Sync>>>> = OnceLock::new();
// Java field: IClipboardManager mClipboardManager
static M_CLIPBOARD_MANAGER: OnceLock<Mutex<Option<Arc<dyn IClipboardManager + Send + Sync>>>> = OnceLock::new();
// Java field: IConfigManager mConfigManager
static M_CONFIG_MANAGER: OnceLock<Mutex<Option<Arc<dyn crate::cn::wps::core::runtime::IConfigManager::IConfigManager + Send + Sync>>>> = OnceLock::new();
// Java field: IAnalysisExecutor sAnalysisExecutor
static S_ANALYSIS_EXECUTOR: OnceLock<Mutex<Option<Arc<dyn crate::cn::wps::core::runtime::IAnalysisExecutor::IAnalysisExecutor + Send + Sync>>>> = OnceLock::new();
// Java field: Object mApplicationContext
static M_APPLICATION_CONTEXT: OnceLock<Mutex<Option<Arc<dyn std::any::Any + Send + Sync>>>> = OnceLock::new();
// Java field: Object mComponentContext
static M_COMPONENT_CONTEXT: OnceLock<Mutex<Option<Arc<dyn std::any::Any + Send + Sync>>>> = OnceLock::new();
// Java field: ILogFilter mLogFilter
static M_LOG_FILTER: OnceLock<Arc<Mutex<LogFilter>>> = OnceLock::new();
// Java field: String mUserName
static M_USER_NAME: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: String mLanguageCode
static M_LANGUAGE_CODE: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: String mLanguageDefault
static M_LANGUAGE_DEFAULT: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: UILanguage mLanguage
static M_LANGUAGE: OnceLock<Mutex<Option<UILanguage>>> = OnceLock::new();
// Java field: boolean mMultiDexConfig
static M_MULTI_DEX_CONFIG: OnceLock<Mutex<bool>> = OnceLock::new();
// Java field: int SDK_VERSION
static SDK_VERSION_VAL: OnceLock<Mutex<i32>> = OnceLock::new();
// Java field: boolean sIsMZ606
static S_IS_MZ606: OnceLock<Mutex<bool>> = OnceLock::new();
// Java field: boolean sIsHTC_Flyer_P510e
static S_IS_HTC_FLYER_P510E: OnceLock<Mutex<bool>> = OnceLock::new();
// Java field: IMathDrawer mMathDrawer
static M_MATH_DRAWER: OnceLock<Mutex<Option<Arc<dyn IMathDrawer + Send + Sync>>>> = OnceLock::new();
// Java field: ISharedInfo mSharedInfo
static M_SHARED_INFO: OnceLock<Mutex<Option<Arc<dyn crate::cn::wps::core::runtime::ISharedInfo::ISharedInfo + Send + Sync>>>> = OnceLock::new();
// Java field: boolean mIsTvMeetingVersion
static M_IS_TV_MEETING_VERSION: OnceLock<Mutex<bool>> = OnceLock::new();
// Java field: boolean WATER_MARK_ON
static WATER_MARK_ON_VAL: OnceLock<Mutex<bool>> = OnceLock::new();
// Java field: GetPlatformDataCallback sGetPlatformDataCallback
static S_GET_PLATFORM_DATA_CALLBACK: OnceLock<Mutex<Option<Arc<dyn GetPlatformDataCallback + Send + Sync>>>> = OnceLock::new();
// Java field: String mDictPath
static M_DICT_PATH: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: IGetUserInfo mGetUserInfo
static M_GET_USER_INFO: OnceLock<Mutex<Option<Arc<dyn IGetUserInfo + Send + Sync>>>> = OnceLock::new();
// Java field: String mUserId
static M_USER_ID: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: IGetUserName mGetUserNameCB
static M_GET_USER_NAME_CB: OnceLock<Mutex<Option<Arc<dyn IGetUserName + Send + Sync>>>> = OnceLock::new();
// Java field: IPicturePool mPicturePool
static M_PICTURE_POOL: OnceLock<Mutex<Option<Arc<dyn IPicturePool + Send + Sync>>>> = OnceLock::new();
// Java field: File TEMP_PRESENTATION
static TEMP_PRESENTATION_DIR: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: File BACKUP_PRESENTATION
static BACKUP_PRESENTATION_DIR: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: File TEMP_SPREADSHEET
static TEMP_SPREADSHEET_DIR: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: File TEMP_PDF
static TEMP_PDF_DIR: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: File BACKUP_SPREADSHEET
static BACKUP_SPREADSHEET_DIR: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: String mShapePath
static M_SHAPE_PATH: OnceLock<Mutex<Option<String>>> = OnceLock::new();
// Java field: String mClipAuthority
static M_CLIP_AUTHORITY: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn get_str_lock(lock: &'static OnceLock<Mutex<String>>) -> &'static Mutex<String> {
    lock.get_or_init(|| Mutex::new(String::new()))
}

fn get_bool_lock(lock: &'static OnceLock<Mutex<bool>>) -> &'static Mutex<bool> {
    lock.get_or_init(|| Mutex::new(false))
}

fn get_i32_lock(lock: &'static OnceLock<Mutex<i32>>) -> &'static Mutex<i32> {
    lock.get_or_init(|| Mutex::new(0))
}

fn get_opt_str_lock(lock: &'static OnceLock<Mutex<Option<String>>>) -> &'static Mutex<Option<String>> {
    lock.get_or_init(|| Mutex::new(None))
}

fn get_opt_any_lock(lock: &'static OnceLock<Mutex<Option<Arc<dyn std::any::Any + Send + Sync>>>>) -> &'static Mutex<Option<Arc<dyn std::any::Any + Send + Sync>>> {
    lock.get_or_init(|| Mutex::new(None))
}

fn get_callback_lock() -> &'static Mutex<Option<Arc<dyn GetPlatformDataCallback + Send + Sync>>> {
    S_GET_PLATFORM_DATA_CALLBACK.get_or_init(|| Mutex::new(None))
}

// Java: public interface GetPlatformDataCallback
pub trait GetPlatformDataCallback {
    // Java: abstract void initSharePlayDirIfNeed()
    fn initSharePlayDirIfNeed(&self);
    // Java: abstract String getNormalTmpDirectory()
    fn getNormalTmpDirectory(&self) -> String;
    // Java: abstract String getInternalTmpDirectory()
    fn getInternalTmpDirectory(&self) -> String;
    // Java: abstract void updateTmpPath()
    fn updateTmpPath(&self);
    // Java: abstract String getSharePlayDir()
    fn getSharePlayDir(&self) -> String;
    // Java: abstract String getTempDir()
    fn getTempDir(&self) -> String;
    // Java: abstract String getTempDir(String var1)
    fn getTempDir_with_path(&self, var1: &str) -> String;
    // Java: abstract String getBackupDir()
    fn getBackupDir(&self) -> String;
    // Java: abstract String getBackupDir(String var1)
    fn getBackupDir_with_path(&self, var1: &str) -> String;
    // Java: abstract String getSaveDir()
    fn getSaveDir(&self) -> String;
    // Java: abstract String getClipboardDir()
    fn getClipboardDir(&self) -> String;
    // Java: abstract String getDragDir()
    fn getDragDir(&self) -> String;
    // Java: abstract String getLogFilePath()
    fn getLogFilePath(&self) -> String;
    // Java: abstract String getCrashLogFilePath()
    fn getCrashLogFilePath(&self) -> String;
    // Java: abstract String getExitReasonLogFilePath()
    fn getExitReasonLogFilePath(&self) -> String;
    // Java: abstract String getSaveLogFilePath()
    fn getSaveLogFilePath(&self) -> String;
    // Java: abstract String getCloudStorageLogFilePath()
    fn getCloudStorageLogFilePath(&self) -> String;
    // Java: abstract String getFileRoamingLogFilePath()
    fn getFileRoamingLogFilePath(&self) -> String;
    // Java: abstract String getSharePlayLogFilePath()
    fn getSharePlayLogFilePath(&self) -> String;
    // Java: abstract int getPid()
    fn getPid(&self) -> i32;
    // Java: abstract String getExternalBackupFile(String var1)
    fn getExternalBackupFile(&self, var1: &str) -> String;
}

// Java: public interface IGetUserInfo
pub trait IGetUserInfo {
    // Java: abstract String getUserId()
    fn getUserId(&self) -> String;
}

// Java: public interface IGetUserName
pub trait IGetUserName {
    // Java: abstract String getUserName()
    fn getUserName(&self) -> String;
}

// Java: public class Platform
pub struct Platform {
}

struct PlatformFileBridge;

impl IPlatformFile for PlatformFileBridge {
    fn createTempFile(&self, prefix: &str, suffix: &str) -> Option<KfsFile> {
        Platform::createTempFile(prefix, suffix).map(KfsFile::new__string)
    }

    fn getCurrentPidTmpDir(&self) -> String {
        Platform::getCurrentPidTmpDir()
    }
}

// Java: public class Platform
impl Platform {
    // Java: Platform()
    pub fn new() -> Self {
        // Java: super();
        Platform {}
    }

    // Java: static String getCurrentPidTmpDir()
    pub fn getCurrentPidTmpDir() -> String {
        // Java: return getCurrentPidTmpDir(getTempDirectory());
        Self::getCurrentPidTmpDir_with_root(&Self::getTempDirectory())
    }

    // Java: static String getCurrentPidTmpDir(String rootPath)
    pub fn getCurrentPidTmpDir_with_root(rootPath: &str) -> String {
        // Java: Assert.assertNotNull("rootPath should not be null", rootPath);
        assert!(!rootPath.is_empty(), "rootPath should not be null");
        // Java: return rootPath.endsWith(File.separator) ? rootPath + getPid() : rootPath + File.separator + getPid();
        let sep = std::path::MAIN_SEPARATOR;
        if rootPath.ends_with(sep) {
            format!("{}{}", rootPath, Self::getPid())
        } else {
            format!("{}{}{}", rootPath, sep, Self::getPid())
        }
    }

    // Java: static String getCurrentPptTmpDir(String docUUID)
    pub fn getCurrentPptTmpDir(docUUID: &str) -> String {
        // Java: String rootPidDir = getCurrentPidTmpDir();
        let rootPidDir = Self::getCurrentPidTmpDir();
        // Java: String docTypeDir = rootPidDir + File.separator + "presentation" + File.separator + docUUID + File.separator;
        let sep = std::path::MAIN_SEPARATOR;
        let docTypeDir = format!("{}{}presentation{}{}{}", rootPidDir, sep, sep, docUUID, sep);
        // Java: return docTypeDir;
        docTypeDir
    }

    // Java: static String getCurrentSsTmpDir(String docUUID)
    pub fn getCurrentSsTmpDir(docUUID: &str) -> String {
        // Java: String rootPidDir = getCurrentPidTmpDir();
        let rootPidDir = Self::getCurrentPidTmpDir();
        // Java: String docTypeDir = rootPidDir + File.separator + "spreadsheet" + File.separator + docUUID + File.separator;
        let sep = std::path::MAIN_SEPARATOR;
        let docTypeDir = format!("{}{}spreadsheet{}{}{}", rootPidDir, sep, sep, docUUID, sep);
        // Java: return docTypeDir;
        docTypeDir
    }

    // Java: static String getCurrentWriterTmpDir(String docUUID)
    pub fn getCurrentWriterTmpDir(docUUID: &str) -> String {
        // Java: String rootPidDir = getCurrentPidTmpDir();
        let rootPidDir = Self::getCurrentPidTmpDir();
        // Java: String docTypeDir = rootPidDir + File.separator + "writer" + File.separator + docUUID + File.separator;
        let sep = std::path::MAIN_SEPARATOR;
        let docTypeDir = format!("{}{}writer{}{}{}", rootPidDir, sep, sep, docUUID, sep);
        // Java: return docTypeDir;
        docTypeDir
    }

    // Java: static boolean removeCurrentPidTmpDir(String rootPath)
    pub fn removeCurrentPidTmpDir_with_root(rootPath: &str) -> bool {
        // Java: return doRemoveCurrentPidTmpDir(getCurrentPidTmpDir(rootPath));
        Self::doRemoveCurrentPidTmpDir(&Self::getCurrentPidTmpDir_with_root(rootPath))
    }

    // Java: static boolean removeCurrentPidTmpDir()
    pub fn removeCurrentPidTmpDir() -> bool {
        // Java: return doRemoveCurrentPidTmpDir(getCurrentPidTmpDir());
        Self::doRemoveCurrentPidTmpDir(&Self::getCurrentPidTmpDir())
    }

    // Java: static boolean removeCurrentPptTmpDir(String UUID)
    pub fn removeCurrentPptTmpDir(uuid: &str) -> bool {
        // Java: return doRemoveCurrentPidTmpDir(getCurrentPptTmpDir(UUID));
        Self::doRemoveCurrentPidTmpDir(&Self::getCurrentPptTmpDir(uuid))
    }

    // Java: static boolean removeCurrentSsTmpDir(String UUID)
    pub fn removeCurrentSsTmpDir(uuid: &str) -> bool {
        // Java: return doRemoveCurrentPidTmpDir(getCurrentSsTmpDir(UUID));
        Self::doRemoveCurrentPidTmpDir(&Self::getCurrentSsTmpDir(uuid))
    }

    // Java: static boolean removeCurrentWriterTmpDir(String UUID)
    pub fn removeCurrentWriterTmpDir(uuid: &str) -> bool {
        // Java: return doRemoveCurrentPidTmpDir(getCurrentWriterTmpDir(UUID));
        Self::doRemoveCurrentPidTmpDir(&Self::getCurrentWriterTmpDir(uuid))
    }

    // Java: static boolean removeCurrentPidPptTmpDir()
    pub fn removeCurrentPidPptTmpDir() -> bool {
        // Java: String rootPidDir = getCurrentPidTmpDir();
        let rootPidDir = Self::getCurrentPidTmpDir();
        // Java: String docTypeDir = rootPidDir + File.separator + "presentation";
        let docTypeDir = format!("{}{}presentation", rootPidDir, std::path::MAIN_SEPARATOR);
        // Java: return doRemoveCurrentPidTmpDir(docTypeDir);
        Self::doRemoveCurrentPidTmpDir(&docTypeDir)
    }

    // Java: static boolean removeCurrentPidSsTmpDir()
    pub fn removeCurrentPidSsTmpDir() -> bool {
        // Java: String rootPidDir = getCurrentPidTmpDir();
        let rootPidDir = Self::getCurrentPidTmpDir();
        // Java: String docTypeDir = rootPidDir + File.separator + "spreadsheet";
        let docTypeDir = format!("{}{}spreadsheet", rootPidDir, std::path::MAIN_SEPARATOR);
        // Java: return doRemoveCurrentPidTmpDir(docTypeDir);
        Self::doRemoveCurrentPidTmpDir(&docTypeDir)
    }

    // Java: static boolean removeCurrentPidWriterTmpDir()
    pub fn removeCurrentPidWriterTmpDir() -> bool {
        // Java: String rootPidDir = getCurrentPidTmpDir();
        let rootPidDir = Self::getCurrentPidTmpDir();
        // Java: String docTypeDir = rootPidDir + File.separator + "writer";
        let docTypeDir = format!("{}{}writer", rootPidDir, std::path::MAIN_SEPARATOR);
        // Java: return doRemoveCurrentPidTmpDir(docTypeDir);
        Self::doRemoveCurrentPidTmpDir(&docTypeDir)
    }

    // Java: static boolean doRemoveCurrentPidTmpDir(String rootPath)
    fn doRemoveCurrentPidTmpDir(rootPath: &str) -> bool {
        // Java: Assert.assertNotNull("rootPath should not be null.", rootPath);
        assert!(!rootPath.is_empty(), "rootPath should not be null.");
        // Java: boolean delSucces = false;
        let mut delSucces = false;
        // Java: if ((new File(rootPath)).exists())
        if Path::new(rootPath).exists()
        {
            // Java: delSucces = FileUtil.delFolder(rootPath);
            delSucces = fs::remove_dir_all(rootPath).is_ok();
        }
        // Java: return delSucces;
        delSucces
    }

    // Java: static String getFileExistPath(String fileName)
    pub fn getFileExistPath(fileName: &str) -> Option<String> {
        // Java: return doGetFileExistPath(getCurrentPidTmpDir(), fileName);
        Self::doGetFileExistPath(&Self::getCurrentPidTmpDir(), fileName)
    }

    // Java: static String getFileExistPath(String rootPath, String fileName)
    pub fn getFileExistPath_with_root(rootPath: &str, fileName: &str) -> Option<String> {
        // Java: return doGetFileExistPath(getCurrentPidTmpDir(rootPath), fileName);
        Self::doGetFileExistPath(&Self::getCurrentPidTmpDir_with_root(rootPath), fileName)
    }

    // Java: static String doGetFileExistPath(String rootPath, String fileName)
    fn doGetFileExistPath(rootPath: &str, fileName: &str) -> Option<String> {
        // Java: Assert.assertNotNull("rootPath should not be null.", rootPath);
        assert!(!rootPath.is_empty(), "rootPath should not be null.");
        // Java: Assert.assertNotNull("fileName should not be null.", fileName);
        assert!(!fileName.is_empty(), "fileName should not be null.");
        // Java: boolean isPathValid = makesureDirExist(new File(rootPath));
        let isPathValid = Self::makesureDirExist(rootPath);
        // Java: Assert.assertTrue("isPathValid should be true.", isPathValid);
        // Java: return isPathValid ? rootPath + File.separator + fileName : null;
        if isPathValid {
            Some(format!("{}{}{}", rootPath, std::path::MAIN_SEPARATOR, fileName))
        } else {
            None
        }
    }

    // Java: static boolean makesureDirExist(File rootPath)
    pub fn makesureDirExist(rootPath: &str) -> bool {
        // Java: Assert.assertNotNull("rootPath should not be null.", rootPath);
        assert!(!rootPath.is_empty(), "rootPath should not be null.");
        // Java: boolean ret = rootPath.exists() ? true : rootPath.mkdirs();
        let p = Path::new(rootPath);
        let ret = if p.exists() { true } else { fs::create_dir_all(p).is_ok() };
        // Java: return ret;
        ret
    }

    // Java: static public static Pair<File, Boolean> createFile(String fileName)
    pub fn createFile(fileName: &str) -> Option<(String, bool)> {
        // Java: return doCreateFile(getCurrentPidTmpDir(), fileName);
        Self::doCreateFile(&Self::getCurrentPidTmpDir(), fileName)
    }

    // Java: static public static Pair<File, Boolean> createFile(String rootPath, String fileName)
    pub fn createFile_with_root(rootPath: &str, fileName: &str) -> Option<(String, bool)> {
        // Java: return doCreateFile(getCurrentPidTmpDir(rootPath), fileName);
        Self::doCreateFile(&Self::getCurrentPidTmpDir_with_root(rootPath), fileName)
    }

    // Java: static private static Pair<File, Boolean> doCreateFile(String rootPath, String fileName)
    fn doCreateFile(rootPath: &str, fileName: &str) -> Option<(String, bool)> {
        // Java: Assert.assertNotNull("rootPath should not be null.", rootPath);
        assert!(!rootPath.is_empty(), "rootPath should not be null.");
        // Java: Assert.assertNotNull("fileName should not be null.", fileName);
        assert!(!fileName.is_empty(), "fileName should not be null.");
        // Java: boolean isPathValid = makesureDirExist(new File(rootPath));
        let isPathValid = Self::makesureDirExist(rootPath);
        // Java: Assert.assertTrue("isPathValid should be true.", isPathValid);
        // Java: return isPathValid ? CreatefileInfolder.createFile(rootPath, fileName) : null;
        if isPathValid {
            let file_path = format!("{}{}{}", rootPath, std::path::MAIN_SEPARATOR, fileName);
            let p = Path::new(&file_path);
            let existed = p.exists();
            if !existed {
                let _ = std::fs::File::create(&file_path);
            }
            Some((file_path, !existed))
        } else {
            None
        }
    }

    // Java: static File createDir(String dirName)
    pub fn createDir(dirName: &str) -> Option<String> {
        // Java: return doCreateDir(getCurrentPidTmpDir(), dirName);
        Self::doCreateDir(&Self::getCurrentPidTmpDir(), dirName)
    }

    // Java: static File doCreateDir(String rootPath, String dirName)
    fn doCreateDir(rootPath: &str, dirName: &str) -> Option<String> {
        // Java: Assert.assertNotNull("rootPath should not be null.", rootPath);
        assert!(!rootPath.is_empty(), "rootPath should not be null.");
        // Java: Assert.assertNotNull("dirName should not be null.", dirName);
        assert!(!dirName.is_empty(), "dirName should not be null.");
        // Java: File dir = new File(rootPath, dirName);
        let dir = format!("{}{}{}", rootPath, std::path::MAIN_SEPARATOR, dirName);
        // Java: dir.mkdirs();
        let _ = fs::create_dir_all(&dir);
        // Java: return dir;
        Some(dir)
    }

    // Java: static File createTempFile(String prefix, String suffix)
    pub fn createTempFile(prefix: &str, suffix: &str) -> Option<String> {
        // Java: return doCreateTempFile(getCurrentPidTmpDir(), prefix, suffix);
        Self::doCreateTempFile(&Self::getCurrentPidTmpDir(), prefix, suffix)
    }

    // Java: static File createTempFileEx(String prefix, String suffix)
    pub fn createTempFileEx(prefix: &str, suffix: &str) -> Result<String, std::io::Error> {
        // Java: return doCreateTempFileEx(getCurrentPidTmpDir(), prefix, suffix);
        Self::doCreateTempFileEx(&Self::getCurrentPidTmpDir(), prefix, suffix)
    }

    // Java: static File createTempFile(String rootPath, String prefix, String suffix)
    pub fn createTempFile_with_root(rootPath: &str, prefix: &str, suffix: &str) -> Option<String> {
        // Java: return doCreateTempFile(getCurrentPidTmpDir(rootPath), prefix, suffix);
        Self::doCreateTempFile(&Self::getCurrentPidTmpDir_with_root(rootPath), prefix, suffix)
    }

    // Java: static File doCreateTempFile(String rootPath, String prefix, String suffix)
    fn doCreateTempFile(rootPath: &str, prefix: &str, suffix: &str) -> Option<String> {
        // Java: try
        match Self::doCreateTempFileEx(rootPath, prefix, suffix) {
            Ok(path) => {
                // Java: return doCreateTempFileEx(rootPath, prefix, suffix);
                Some(path)
            }
            // Java: catch (IOException e)
            Err(_e) => {
                // Java: Log.e(TAG, "IOException", e);
                // Java: return null;
                None
            }
        }
    }

    // Java: static File doCreateTempFileEx(String rootPath, String prefix, String suffix)
    fn doCreateTempFileEx(rootPath: &str, prefix: &str, suffix: &str) -> Result<String, std::io::Error> {
        // Java: Assert.assertNotNull("rootPath should not be null.", rootPath);
        assert!(!rootPath.is_empty(), "rootPath should not be null.");
        // Java: File rootPathFile = new File(rootPath);
        // Java: boolean isPathValid = makesureDirExist(rootPathFile);
        let isPathValid = Self::makesureDirExist(rootPath);
        // Java: Assert.assertTrue("isPathValid should be true.", isPathValid);
        // Java: return isPathValid ? File.createTempFile(prefix, suffix, rootPathFile) : File.createTempFile(prefix, suffix);
        let tmp_dir = std::env::temp_dir();
        let tmp_str = tmp_dir.to_str().unwrap_or("/tmp");
        let dir = if isPathValid { rootPath } else { tmp_str };
        // Rust: `File.createTempFile(prefix, suffix, dir)` 的 Java 语义是**保证唯一**。
        // 原实现只用**纳秒时间戳**做文件名 ⇒ 并行线程/同纳秒内会拿到**同一路径**并互相覆盖。
        // 实测：`43_function_docprops_custom_xml::dh19_deterministic` 在全量并行跑时偶发失败
        // （单跑 8/8 通过）——根因即此处。
        // ⇒ 追加**进程内原子序号**并用 `create_new` 保证真正唯一（碰撞则重试）。
        use std::sync::atomic::{AtomicU64, Ordering};
        static TMP_SEQ: AtomicU64 = AtomicU64::new(0);
        for _ in 0..1024 {
            let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
            let file_name = format!(
                "{}{}-{}{}",
                prefix,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos(),
                seq,
                suffix
            );
            let file_path = format!("{}{}{}", dir, std::path::MAIN_SEPARATOR, file_name);
            match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&file_path)
            {
                Ok(_) => return Ok(file_path),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e),
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "createTempFile: 1024 次尝试内未能找到唯一文件名",
        ))
    }

    // Java: static void hookTestPlatform()
    pub fn hookTestPlatform() {
    }

    // Java: static ILog getLog()
    pub fn getLog() -> Arc<dyn crate::cn::wps::core::runtime::ILog::ILog + Send + Sync> {
        // Java: return InternalPlatform.getInstance().getLog();
        InternalPlatform::getInstance().lock().unwrap().getLog()
    }

    // Java: static void setLog(ILog log)
    pub fn setLog(log: Option<Arc<dyn crate::cn::wps::core::runtime::ILog::ILog + Send + Sync>>) {
        // Java: InternalPlatform.getInstance().setLog(log);
        InternalPlatform::getInstance().lock().unwrap().setLog(log);
    }

    // Java: static ILogFilter getLogFilter()
    pub fn getLogFilter() -> Arc<Mutex<LogFilter>> {
        // Java: return mLogFilter;
        Arc::clone(M_LOG_FILTER.get_or_init(|| Arc::new(Mutex::new(LogFilter::new()))))
    }

    // Java: static void setLogFilter(ILogFilter pLogFilter)
    pub fn setLogFilter(pLogFilter: LogFilter) {
        // Java: mLogFilter = pLogFilter;
        *M_LOG_FILTER.get_or_init(|| Arc::new(Mutex::new(LogFilter::new()))).lock().unwrap() = pLogFilter;
    }

    // Java: static void readLogFilter(String path)
    pub fn readLogFilter(path: &str) {
        // Java: try
        // Java: readLogFilter((InputStream)(new FileInputStream(path)));
        let content = fs::read_to_string(path);
        if let Ok(content) = content {
            Self::readLogFilter_from_stream(content.as_bytes());
        }
        // Java: catch (IOException var2)
    }

    // Java: static void readLogFilter(InputStream input)
    pub fn readLogFilter_from_stream(input: &[u8]) {
        // Java: ILogFilter filter = getLogFilter();
        let filter = Self::getLogFilter();
        // Java: if (filter != null)
        if let Ok(text) = std::str::from_utf8(input) {
        // Java: try
        // Java: DocumentBuilder docBuilder = XmlDocument.getDocumentBuilderFactory();
        // Java: Document doc = docBuilder.parse(input);
        // Java: parseFilterFile(doc, filter);
            Self::parseFilterFile(text, &mut filter.lock().unwrap());
        // Java: catch (IOException var4)
        // Java: catch (SAXException var5)
        // Java: catch (ParserConfigurationException var6)
        }
    }

    // Java: static void parseFilterFile(Document doc, ILogFilter filter)
    fn parseFilterFile(doc: &str, filter: &mut LogFilter) {
        // Java: if (doc != null && filter != null)
        // Java: Element root = doc.getDocumentElement();
        // Java: Element print_caller = XmlDocument.xmlGetFirstChild(root, "print-caller");
        // Java: if (print_caller != null)
        // Java: String txt = XmlDocument.xmlGetTextContent(print_caller);
        // Java: if (txt != null)
        // Java: if (txt.equals("true"))
        // Java: filter.setPrintCaller(true);
        // Java: else if (txt.equals("false"))
        // Java: filter.setPrintCaller(false);
        // Java: Element enable = XmlDocument.xmlGetFirstChild(root, "enable");
        // Java: if (enable != null)
        // Java: Element class_filter = XmlDocument.xmlGetFirstChild(enable, "class-filter");
        // Java: if (class_filter != null)
        // Java: NodeList class_list = class_filter.getElementsByTagName("class");
        // Java: for(int i = 0; class_list != null && i < class_list.getLength(); ++i)
        // Java: filter.enableClass(XmlDocument.xmlGetTextContent(class_list.item(i)));
        // Java: Element tag_filter = XmlDocument.xmlGetFirstChild(enable, "tag-filter");
        // Java: if (tag_filter != null)
        // Java: NodeList tag_list = tag_filter.getElementsByTagName("tag");
        // Java: for(int i = 0; tag_list != null && i < tag_list.getLength(); ++i)
        // Java: filter.enableTag(XmlDocument.xmlGetTextContent(tag_list.item(i)));
        // Java: Element disable = XmlDocument.xmlGetFirstChild(root, "disable");
        // Java: if (disable != null)
        // Java: Element class_filter = XmlDocument.xmlGetFirstChild(disable, "class-filter");
        // Java: if (class_filter != null)
        // Java: NodeList class_list = class_filter.getElementsByTagName("class");
        // Java: for(int i = 0; class_list != null && i < class_list.getLength(); ++i)
        // Java: filter.disableClass(XmlDocument.xmlGetTextContent(class_list.item(i)));
        // Java: Element tag_filter = XmlDocument.xmlGetFirstChild(disable, "tag-filter");
        // Java: if (tag_filter != null)
        // Java: NodeList tag_list = tag_filter.getElementsByTagName("tag");
        // Java: for(int i = 0; tag_list != null && i < tag_list.getLength(); ++i)
        // Java: filter.disableTag(XmlDocument.xmlGetTextContent(tag_list.item(i)));
        fn section<'a>(text: &'a str, tag: &str) -> Option<&'a str> {
            let start_tag = format!("<{tag}>");
            let end_tag = format!("</{tag}>");
            let start = text.find(&start_tag)? + start_tag.len();
            let end = text[start..].find(&end_tag)? + start;
            Some(&text[start..end])
        }
        fn values(text: &str, tag: &str) -> Vec<String> {
            let mut result = Vec::new();
            let start_tag = format!("<{tag}>");
            let end_tag = format!("</{tag}>");
            let mut rest = text;
            while let Some(start) = rest.find(&start_tag) {
                let value_start = start + start_tag.len();
                let Some(relative_end) = rest[value_start..].find(&end_tag) else { break };
                result.push(rest[value_start..value_start + relative_end].trim().to_string());
                rest = &rest[value_start + relative_end + end_tag.len()..];
            }
            result
        }
        if let Some(value) = section(doc, "print-caller") {
            match value.trim() {
                "true" => filter.setPrintCaller(true),
                "false" => filter.setPrintCaller(false),
                _ => {}
            }
        }
        if let Some(enable) = section(doc, "enable") {
            if let Some(classes) = section(enable, "class-filter") {
                for value in values(classes, "class") { filter.enableClass(Some(&value)); }
            }
            if let Some(tags) = section(enable, "tag-filter") {
                for value in values(tags, "tag") { filter.enableTag(Some(&value)); }
            }
        }
        if let Some(disable) = section(doc, "disable") {
            if let Some(classes) = section(disable, "class-filter") {
                for value in values(classes, "class") { filter.disableClass(Some(&value)); }
            }
            if let Some(tags) = section(disable, "tag-filter") {
                for value in values(tags, "tag") { filter.disableTag(Some(&value)); }
            }
        }
    }

    // Java: static String getTempDirectory()
    pub fn getTempDirectory() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getInternalTmpDirectory() : InternalPlatform.getInstance().getTempDirectory();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getInternalTmpDirectory()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getTempDirectory().to_string()
        }
    }

    // Java: static String getSharePlayDir()
    pub fn getSharePlayDir() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getSharePlayDir() : InternalPlatform.getInstance().getTempDirectory() + "shareplay/";
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getSharePlayDir()
        } else {
            format!("{}shareplay/", InternalPlatform::getInstance().lock().unwrap().getTempDirectory())
        }
    }

    // Java: static void initSharePlayDirIfNeed()
    pub fn initSharePlayDirIfNeed() {
        // Java: if (sGetPlatformDataCallback != null)
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb
        {
            // Java: sGetPlatformDataCallback.initSharePlayDirIfNeed();
            callback.initSharePlayDirIfNeed();
        }
    }

    // Java: static void updateTmpPath()
    pub fn updateTmpPath() {
        // Java: if (sGetPlatformDataCallback != null)
        let mut cb = get_callback_lock().lock().unwrap();
        if let Some(ref mut callback) = *cb
        {
            // Java: sGetPlatformDataCallback.updateTmpPath();
            callback.updateTmpPath();
        }
    }

    // Java: static String getNormalTmpDirectory()
    pub fn getNormalTmpDirectory() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getNormalTmpDirectory() : InternalPlatform.getInstance().getTempDirectory();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getNormalTmpDirectory()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getTempDirectory().to_string()
        }
    }

    // Java: static String getTempDirectory(String pTempPath)
    pub fn getTempDirectory_with_path(pTempPath: &str) -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getTempDir(pTempPath) : InternalPlatform.getInstance().getTempDirectory();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getTempDir_with_path(pTempPath)
        } else {
            InternalPlatform::getInstance().lock().unwrap().getTempDirectory().to_string()
        }
    }

    // Java: static void setTempDirectory(String tempDirectory)
    pub fn setTempDirectory(tempDirectory: &str) {
        // Java: InternalPlatform.getInstance().setTempDirectory(tempDirectory);
        InternalPlatform::getInstance().lock().unwrap().setTempDirectory(tempDirectory);
    }

    // Java: static String getClipboardDirectory()
    pub fn getClipboardDirectory() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getClipboardDir() : InternalPlatform.getInstance().getClipboardDirectory();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getClipboardDir()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getClipboardDirectory().to_string()
        }
    }

    // Java: static void setClipboardDirectory(String clipboardDirectory)
    pub fn setClipboardDirectory(clipboardDirectory: &str) {
        // Java: InternalPlatform.getInstance().setClipboardDirectory(clipboardDirectory);
        InternalPlatform::getInstance().lock().unwrap().setClipboardDirectory(clipboardDirectory);
    }

    // Java: static String getDragDirectory()
    pub fn getDragDirectory() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getDragDir() : InternalPlatform.getInstance().getDragDirectory();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getDragDir()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getDragDirectory().to_string()
        }
    }

    // Java: static void setDragDirectory(String dragDirectory)
    pub fn setDragDirectory(dragDirectory: &str) {
        // Java: InternalPlatform.getInstance().setDragDirectory(dragDirectory);
        InternalPlatform::getInstance().lock().unwrap().setDragDirectory(dragDirectory);
    }

    // Java: static String getBackupDirectory()
    pub fn getBackupDirectory() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getBackupDir() : InternalPlatform.getInstance().getBackupDirectory();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getBackupDir()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getBackupDirectory().to_string()
        }
    }

    // Java: static String getBackupDirectory(String pSavePath)
    pub fn getBackupDirectory_with_path(pSavePath: &str) -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getBackupDir(pSavePath) : InternalPlatform.getInstance().getBackupDirectory();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getBackupDir_with_path(pSavePath)
        } else {
            InternalPlatform::getInstance().lock().unwrap().getBackupDirectory().to_string()
        }
    }

    // Java: static String getSaveDirectory()
    pub fn getSaveDirectory() -> String {
        // Java: return InternalPlatform.getInstance().getSaveDirectory();
        InternalPlatform::getInstance().lock().unwrap().getSaveDirectory().to_string()
    }

    // Java: static void setBackupDirectory(String backupDirectory)
    pub fn setBackupDirectory(backupDirectory: &str) {
        // Java: InternalPlatform.getInstance().setBackupDirectory(backupDirectory);
        InternalPlatform::getInstance().lock().unwrap().setBackupDirectory(backupDirectory);
    }

    // Java: static void setSaveDirectory(String saveDirectory)
    pub fn setSaveDirectory(saveDirectory: &str) {
        // Java: InternalPlatform.getInstance().setSaveDirectory(saveDirectory);
        InternalPlatform::getInstance().lock().unwrap().setSaveDirectory(saveDirectory);
    }

    // Java: static String getLogFilePath()
    pub fn getLogFilePath() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getLogFilePath() : InternalPlatform.getInstance().getLogFilePath();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getLogFilePath()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getLogFilePath().to_string()
        }
    }

    // Java: static String getCrashLogFilePath()
    pub fn getCrashLogFilePath() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getCrashLogFilePath() : InternalPlatform.getInstance().getCrashLogFilePath();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getCrashLogFilePath()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getCrashLogFilePath().to_string()
        }
    }

    // Java: static String getExitReasonLogFilePath()
    pub fn getExitReasonLogFilePath() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getExitReasonLogFilePath() : InternalPlatform.getInstance().getExitReasonLogFilePath();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getExitReasonLogFilePath()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getExitReasonLogFilePath().to_string()
        }
    }

    // Java: static String getSaveLogFilePath()
    pub fn getSaveLogFilePath() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getSaveLogFilePath() : InternalPlatform.getInstance().getSaveLogFilePath();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getSaveLogFilePath()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getSaveLogFilePath().to_string()
        }
    }

    // Java: static String getCloudStorageLogFilePath()
    pub fn getCloudStorageLogFilePath() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getCloudStorageLogFilePath() : InternalPlatform.getInstance().getCloudStorageLogFilePath();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getCloudStorageLogFilePath()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getCloudStorageLogFilePath().to_string()
        }
    }

    // Java: static String getFileRoamingLogFilePath()
    pub fn getFileRoamingLogFilePath() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getFileRoamingLogFilePath() : InternalPlatform.getInstance().getRoamingFileLogFilePath();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getFileRoamingLogFilePath()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getRoamingFileLogFilePath().to_string()
        }
    }

    // Java: static String getSharePlayLogFilePath()
    pub fn getSharePlayLogFilePath() -> String {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getSharePlayLogFilePath() : InternalPlatform.getInstance().getSharePlayLogFilePath();
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            callback.getSharePlayLogFilePath()
        } else {
            InternalPlatform::getInstance().lock().unwrap().getSharePlayLogFilePath().to_string()
        }
    }

    // Java: static String getExternalBackupFile(String filePath)
    pub fn getExternalBackupFile(filePath: &str) -> Option<String> {
        // Java: return sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getExternalBackupFile(filePath) : null;
        let cb = get_callback_lock().lock().unwrap();
        if let Some(ref callback) = *cb {
            Some(callback.getExternalBackupFile(filePath))
        } else {
            None
        }
    }

    // Java: static int getPid()
    pub fn getPid() -> i32 {
        let pid_lock = S_PID.get_or_init(|| Mutex::new(None));
        let mut pid = pid_lock.lock().unwrap();
        // Java: if (null == sPid)
        if pid.is_none()
        {
            // Java: sPid = sGetPlatformDataCallback != null ? sGetPlatformDataCallback.getPid() : InternalPlatform.getInstance().getPid();
            let cb = get_callback_lock().lock().unwrap();
            let val = if let Some(ref callback) = *cb {
                callback.getPid()
            } else {
                InternalPlatform::getInstance().lock().unwrap().getPid()
            };
            *pid = Some(val);
        }
        // Java: return sPid;
        pid.unwrap_or(0)
    }

    // Java: static String getDictPath()
    pub fn getDictPath() -> Option<String> {
        // Java: return mDictPath;
        get_opt_str_lock(&M_DICT_PATH).lock().unwrap().clone()
    }

    // Java: static void setDictPath(String path)
    pub fn setDictPath(path: Option<&str>) {
        // Java: mDictPath = path;
        *get_opt_str_lock(&M_DICT_PATH).lock().unwrap() = path.map(str::to_string);
    }

    // Java: static void setKillProcess(boolean bKill)
    pub fn setKillProcess(bKill: bool) {
        // Java: mKillProcess = bKill;
        *get_bool_lock(&M_KILL_PROCESS).lock().unwrap() = bKill;
    }

    // Java: static boolean getKillProcess()
    pub fn getKillProcess() -> bool {
        // Java: return mKillProcess;
        *get_bool_lock(&M_KILL_PROCESS).lock().unwrap()
    }

    // Java: static void onOutOfMemory()
    pub fn onOutOfMemory() {
        // Java: if (null != mPicturePool)
        if let Some(pool) = M_PICTURE_POOL.get_or_init(|| Mutex::new(None)).lock().unwrap().as_ref() {
            // Java: mPicturePool.onOutOfMemory();
            pool.onOutOfMemory();
        }
        // Java: setKillProcess(true);
        Self::setKillProcess(true);
    }

    // Java: static KDisplayMetrics getDisplayMetrics()
    pub fn getDisplayMetrics() -> Option<Arc<KDisplayMetrics>> {
        // Java: return mDisplayMetrics;
        M_DISPLAY_METRICS.get_or_init(|| Mutex::new(None)).lock().unwrap().clone()
    }

    // Java: static void setDisplayMetrics(KDisplayMetrics pDisplayMetrics)
    pub fn setDisplayMetrics(pDisplayMetrics: Arc<KDisplayMetrics>) {
        // Java: mDisplayMetrics = pDisplayMetrics;
        *M_DISPLAY_METRICS.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(pDisplayMetrics);
    }

    // Java: static IClipboardManager getClipboardManager()
    pub fn getClipboardManager() -> Option<Arc<dyn IClipboardManager + Send + Sync>> {
        // Java: return mClipboardManager;
        M_CLIPBOARD_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap().clone()
    }

    // Java: static void setClipboardManager(IClipboardManager pClipboard)
    pub fn setClipboardManager(pClipboard: Arc<dyn IClipboardManager + Send + Sync>) {
        // Java: mClipboardManager = pClipboard;
        *M_CLIPBOARD_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(pClipboard);
    }

    // Java: static IAssetManager getAssetManager()
    pub fn getAssetManager() -> Option<Arc<dyn crate::cn::wps::core::runtime::IAssetManager::IAssetManager + Send + Sync>> {
        // Java: return mAssetManager;
        M_ASSET_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap().clone()
    }

    // Java: static void setAssetManager(IAssetManager pAssetManager)
    pub fn setAssetManager(pAssetManager: Arc<dyn crate::cn::wps::core::runtime::IAssetManager::IAssetManager + Send + Sync>) {
        // Java: mAssetManager = pAssetManager;
        *M_ASSET_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(pAssetManager);
    }

    // Java: static IResourceManager getResourceManager()
    pub fn getResourceManager() -> Option<Arc<dyn crate::cn::wps::core::runtime::IResourceManager::IResourceManager + Send + Sync>> {
        // Java: return mResourceManager;
        M_RESOURCE_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap().clone()
    }

    // Java: static void setResourceManager(IResourceManager pResourceManager)
    pub fn setResourceManager(pResourceManager: Arc<dyn crate::cn::wps::core::runtime::IResourceManager::IResourceManager + Send + Sync>) {
        // Java: mResourceManager = pResourceManager;
        *M_RESOURCE_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(pResourceManager);
    }

    // Java: static Object getApplicationContext()
    pub fn getApplicationContext() -> Option<Arc<dyn std::any::Any + Send + Sync>> {
        // Java: return mApplicationContext;
        get_opt_any_lock(&M_APPLICATION_CONTEXT).lock().unwrap().clone()
    }

    // Java: static void setApplicationContext(Object pApp)
    pub fn setApplicationContext(pApp: Arc<dyn std::any::Any + Send + Sync>) {
        // Java: mApplicationContext = pApp;
        *get_opt_any_lock(&M_APPLICATION_CONTEXT).lock().unwrap() = Some(pApp);
    }

    // Java: static Object getComponentContext()
    pub fn getComponentContext() -> Option<Arc<dyn std::any::Any + Send + Sync>> {
        // Java: return mComponentContext;
        get_opt_any_lock(&M_COMPONENT_CONTEXT).lock().unwrap().clone()
    }

    // Java: static void setComponentContext(Object pComponent)
    pub fn setComponentContext(pComponent: Arc<dyn std::any::Any + Send + Sync>) {
        // Java: mComponentContext = pComponent;
        *get_opt_any_lock(&M_COMPONENT_CONTEXT).lock().unwrap() = Some(pComponent);
    }

    // Java: static IHistoryManager getHistoryManager()
    pub fn getHistoryManager() -> Option<Arc<dyn crate::cn::wps::core::runtime::IHistoryManager::IHistoryManager + Send + Sync>> {
        // Java: return mHistoryManager;
        M_HISTORY_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap().clone()
    }

    // Java: static void setHistoryManager(IHistoryManager pHistoryManager)
    pub fn setHistoryManager(pHistoryManager: Arc<dyn crate::cn::wps::core::runtime::IHistoryManager::IHistoryManager + Send + Sync>) {
        // Java: mHistoryManager = pHistoryManager;
        *M_HISTORY_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(pHistoryManager);
    }

    // Java: static String getUserId()
    pub fn getUserId() -> Option<String> {
        // Java: if ((mUserId == null || "".equals(mUserId)) && mGetUserInfo != null)
        let mut user_id = get_opt_str_lock(&M_USER_ID).lock().unwrap();
        if user_id.as_deref().is_none_or(str::is_empty) {
            let info = M_GET_USER_INFO.get_or_init(|| Mutex::new(None)).lock().unwrap();
            if let Some(ref callback) = *info
            {
                // Java: mUserId = mGetUserInfo.getUserId();
                *user_id = Some(callback.getUserId());
            }
        }
        // Java: return mUserId;
        user_id.clone()
    }

    // Java: static void setUserId(String mUserId)
    pub fn setUserId(userId: Option<&str>) {
        // Java: Platform.mUserId = mUserId;
        *get_opt_str_lock(&M_USER_ID).lock().unwrap() = userId.map(str::to_string);
    }

    // Java: static void setIGetUserInfo(IGetUserInfo userInfo)
    pub fn setIGetUserInfo(userInfo: Arc<dyn IGetUserInfo + Send + Sync>) {
        // Java: mGetUserInfo = userInfo;
        *M_GET_USER_INFO.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(userInfo);
    }

    // Java: static void setWaterFontsDirectory(String sdCustomFontsDirectory)
    pub fn setWaterFontsDirectory(sdCustomFontsDirectory: &str) {
        // Java: InternalPlatform.getInstance().setWaterMarkFontsDirectory(sdCustomFontsDirectory);
        InternalPlatform::getInstance().lock().unwrap().setWaterMarkFontsDirectory(sdCustomFontsDirectory);
    }

    // Java: static String getWaterFontDirectory()
    pub fn getWaterFontDirectory() -> String {
        // Java: return InternalPlatform.getInstance().getWaterMarkFontsDirectory();
        InternalPlatform::getInstance().lock().unwrap().getWaterMarkFontsDirectory().to_string()
    }

    // Java: static String getUserName()
    pub fn getUserName() -> Option<String> {
        // Java: if ((mUserName == null || "".equals(mUserName)) && mGetUserNameCB != null)
        let mut user_name = M_USER_NAME.get_or_init(|| Mutex::new(Some(String::new()))).lock().unwrap();
        if user_name.as_deref().is_none_or(str::is_empty) {
            let cb = M_GET_USER_NAME_CB.get_or_init(|| Mutex::new(None)).lock().unwrap();
            if let Some(ref callback) = *cb
            {
                // Java: mUserName = mGetUserNameCB.getUserName();
                *user_name = Some(callback.getUserName());
            }
        }
        // Java: return mUserName;
        user_name.clone()
    }

    // Java: static void setGetUserNameCB(IGetUserName gun)
    pub fn setGetUserNameCB(gun: Arc<dyn IGetUserName + Send + Sync>) {
        // Java: mGetUserNameCB = gun;
        *M_GET_USER_NAME_CB.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(gun);
    }

    // Java: static void setUserName(String userName)
    pub fn setUserName(userName: Option<&str>) {
        // Java: mUserName = userName;
        *M_USER_NAME.get_or_init(|| Mutex::new(Some(String::new()))).lock().unwrap() = userName.map(str::to_string);
    }

    // Java: static void setLanguage(UILanguage language)
    pub fn setLanguage(language: UILanguage) {
        // Java: mLanguage = language;
        *M_LANGUAGE.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(language);
    }

    // Java: static UILanguage getLanguage()
    pub fn getLanguage() -> Option<UILanguage> {
        // Java: return mLanguage;
        *M_LANGUAGE.get_or_init(|| Mutex::new(None)).lock().unwrap()
    }

    // Java: static void setLanguageCode(String languageCode)
    pub fn setLanguageCode(languageCode: Option<&str>) {
        // Java: mLanguageCode = languageCode;
        *get_opt_str_lock(&M_LANGUAGE_CODE).lock().unwrap() = languageCode.map(str::to_string);
    }

    // Java: static String getLanguageCode()
    pub fn getLanguageCode() -> Option<String> {
        // Java: return mLanguageCode;
        get_opt_str_lock(&M_LANGUAGE_CODE).lock().unwrap().clone()
    }

    // Java: static void setLanguageDefault(String languageDefault)
    pub fn setLanguageDefault(languageDefault: Option<&str>) {
        // Java: mLanguageDefault = languageDefault;
        *get_opt_str_lock(&M_LANGUAGE_DEFAULT).lock().unwrap() = languageDefault.map(str::to_string);
    }

    // Java: static String getLanguageDefault()
    pub fn getLanguageDefault() -> Option<String> {
        // Java: return mLanguageDefault;
        get_opt_str_lock(&M_LANGUAGE_DEFAULT).lock().unwrap().clone()
    }

    // Java: static GetPlatformDataCallback getGetPlatformData()
    pub fn getGetPlatformData() -> Option<Arc<dyn GetPlatformDataCallback + Send + Sync>> {
        // Java: return sGetPlatformDataCallback;
        get_callback_lock().lock().unwrap().clone()
    }

    // Java: static void setGetPlatformData(GetPlatformDataCallback callback)
    pub fn setGetPlatformData(callback: Arc<dyn GetPlatformDataCallback + Send + Sync>) {
        // Java: sGetPlatformDataCallback = callback;
        *get_callback_lock().lock().unwrap() = Some(callback);
        // Java: initAdapter();
        Self::initAdapter();
    }

    // Java: static boolean getMultiDexConfig()
    pub fn getMultiDexConfig() -> bool {
        // Java: return mMultiDexConfig;
        *get_bool_lock(&M_MULTI_DEX_CONFIG).lock().unwrap()
    }

    // Java: static void setMultiDexConfig(boolean multiDexConfig)
    pub fn setMultiDexConfig(multiDexConfig: bool) {
        // Java: mMultiDexConfig = multiDexConfig;
        *get_bool_lock(&M_MULTI_DEX_CONFIG).lock().unwrap() = multiDexConfig;
        // Java: PlatformAdapter.setMultiDexConfig(multiDexConfig);
        PlatformAdapter::setMultiDexConfig(multiDexConfig);
    }

    // Java: static String getCustomFontsDirectory()
    pub fn getCustomFontsDirectory() -> String {
        // Java: return InternalPlatform.getInstance().getCustomFontsDirectory();
        InternalPlatform::getInstance().lock().unwrap().getCustomFontsDirectory().to_string()
    }

    // Java: static void setCustomFontsDirectory(String customFontsDirectory)
    pub fn setCustomFontsDirectory(customFontsDirectory: &str) {
        // Java: InternalPlatform.getInstance().setCustomFontsDirectory(customFontsDirectory);
        InternalPlatform::getInstance().lock().unwrap().setCustomFontsDirectory(customFontsDirectory);
    }

    // Java: static String getHandWrittenDirectory()
    pub fn getHandWrittenDirectory() -> String {
        // Java: return InternalPlatform.getInstance().getHandWrittenDirectory();
        InternalPlatform::getInstance().lock().unwrap().getHandWrittenDirectory().to_string()
    }

    // Java: static void setHandWrittenDirectory(String handWrittenDirectory)
    pub fn setHandWrittenDirectory(handWrittenDirectory: &str) {
        // Java: InternalPlatform.getInstance().setHandWrittenDirectory(handWrittenDirectory);
        InternalPlatform::getInstance().lock().unwrap().setHandWrittenDirectory(handWrittenDirectory);
    }

    // Java: static String getOldCustomFontsDirectory()
    pub fn getOldCustomFontsDirectory() -> String {
        // Java: return InternalPlatform.getInstance().getOldCustomFontsDirectory();
        InternalPlatform::getInstance().lock().unwrap().getOldCustomFontsDirectory().to_string()
    }

    // Java: static void setOldCustomFontsDirectory(String oldCustomFontsDirectory)
    pub fn setOldCustomFontsDirectory(oldCustomFontsDirectory: &str) {
        // Java: InternalPlatform.getInstance().setOldCustomFontsDirectory(oldCustomFontsDirectory);
        InternalPlatform::getInstance().lock().unwrap().setOldCustomFontsDirectory(oldCustomFontsDirectory);
    }

    // Java: static String getSystemFontsDirectory()
    pub fn getSystemFontsDirectory() -> String {
        // Java: return InternalPlatform.getInstance().getSystemFontsDirectory();
        InternalPlatform::getInstance().lock().unwrap().getSystemFontsDirectory().to_string()
    }

    // Java: static void setSystemFontsDirectory(String systemFontsDirectory)
    pub fn setSystemFontsDirectory(systemFontsDirectory: &str) {
        // Java: InternalPlatform.getInstance().setSystemFontsDirectory(systemFontsDirectory);
        InternalPlatform::getInstance().lock().unwrap().setSystemFontsDirectory(systemFontsDirectory);
    }

    // Java: static void setSDCustomFontsDirectory(String sdCustomFontsDirectory)
    pub fn setSDCustomFontsDirectory(sdCustomFontsDirectory: &str) {
        // Java: InternalPlatform.getInstance().setSDCustomFontsDirectory(sdCustomFontsDirectory);
        InternalPlatform::getInstance().lock().unwrap().setSDCustomFontsDirectory(sdCustomFontsDirectory);
    }

    // Java: static String getSDCustomFontDirectory()
    pub fn getSDCustomFontDirectory() -> String {
        // Java: return InternalPlatform.getInstance().getSDCustomFontsDirectory();
        InternalPlatform::getInstance().lock().unwrap().getSDCustomFontsDirectory().to_string()
    }

    // Java: static List<String> getSDCustomFontRootAndSubDirectory()
    pub fn getSDCustomFontRootAndSubDirectory() -> Vec<String> {
        // Java: List<String> fontDirs = new ArrayList();
        let mut fontDirs = Vec::new();
        // Java: String sdCustomFontDirectory = getSDCustomFontDirectory();
        let sdCustomFontDirectory = Self::getSDCustomFontDirectory();
        // Java: if (sdCustomFontDirectory != "" && sdCustomFontDirectory != null)
        if !sdCustomFontDirectory.is_empty()
        {
            // Java: File sdCustomFontDirFile = new File(sdCustomFontDirectory);
            let sdCustomFontDirFile = Path::new(&sdCustomFontDirectory);
            // Java: if (sdCustomFontDirFile.exists())
            if sdCustomFontDirFile.exists()
            {
                // Java: recursiveScanFolders(sdCustomFontDirectory, 3, fontDirs);
                Self::recursiveScanFolders(&sdCustomFontDirectory, 3, &mut fontDirs);
            }
        }
        // Java: return fontDirs;
        fontDirs
    }

    // Java: static String[] getAllFontsDirectory()
    pub fn getAllFontsDirectory() -> Vec<String> {
        // Java: List<String> fontDirs = new ArrayList();
        let mut fontDirs = Vec::new();
        // Java: fontDirs.add(getSystemFontsDirectory());
        fontDirs.push(Self::getSystemFontsDirectory());
        // Java: fontDirs.add(getHandWrittenDirectory());
        fontDirs.push(Self::getHandWrittenDirectory());
        // Java: fontDirs.add(getCustomFontsDirectory());
        fontDirs.push(Self::getCustomFontsDirectory());
        // Java: String sdCustomFontDirectory = getSDCustomFontDirectory();
        let sdCustomFontDirectory = Self::getSDCustomFontDirectory();
        // Java: if (sdCustomFontDirectory != "" && sdCustomFontDirectory != null)
        if !sdCustomFontDirectory.is_empty()
        {
            // Java: File sdCustomFontDirFile = new File(sdCustomFontDirectory);
            let sdCustomFontDirFile = Path::new(&sdCustomFontDirectory);
            // Java: if (sdCustomFontDirFile.exists())
            if sdCustomFontDirFile.exists()
            {
                // Java: recursiveScanFolders(sdCustomFontDirectory, 3, fontDirs);
                Self::recursiveScanFolders(&sdCustomFontDirectory, 3, &mut fontDirs);
            }
        }
        // Java: File customFontsDir = new File(getCustomFontsDirectory());
        let customFontsPath = Self::getCustomFontsDirectory();
        let customFontsDir = Path::new(&customFontsPath);
        // Java: if (customFontsDir.exists() && customFontsDir.isDirectory())
        if customFontsDir.exists() && customFontsDir.is_dir()
        {
            // Java: File[] firstLevelFiles = customFontsDir.listFiles();
            // Java: if (firstLevelFiles != null)
            if let Ok(firstLevelFiles) = fs::read_dir(customFontsDir)
            {
                // Java: for(File f1 : firstLevelFiles)
                for f1_entry in firstLevelFiles.flatten()
                {
                    // Java: if (f1.isDirectory())
                    if f1_entry.path().is_dir()
                    {
                        // Java: File[] secondLevelFiles = f1.listFiles();
                        // Java: if (secondLevelFiles != null)
                        if let Ok(secondLevelFiles) = fs::read_dir(f1_entry.path())
                        {
                            // Java: for(File f2 : secondLevelFiles)
                            for f2_entry in secondLevelFiles.flatten()
                            {
                                // Java: if (f2.isDirectory())
                                if f2_entry.path().is_dir()
                                {
                                    // Java: fontDirs.add(f2.getPath());
                                    fontDirs.push(f2_entry.path().to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        // Java: return (String[])fontDirs.toArray(new String[fontDirs.size()]);
        fontDirs
    }

    // Java: static void recursiveScanFolders(String path, int maxLevel, List<String> dirs)
    pub fn recursiveScanFolders(path: &str, maxLevel: i32, dirs: &mut Vec<String>) {
        // Java: if (maxLevel >= 1)
        if maxLevel >= 1
        {
            // Java: File file = new File(path);
            let file = Path::new(path);
            // Java: if (file.isDirectory())
            if file.is_dir()
            {
                // Java: dirs.add(path);
                dirs.push(path.to_string());
                // Java: --maxLevel;
                let maxLevel = maxLevel - 1;
                // Java: if (maxLevel >= 1)
                if maxLevel >= 1
                {
                    // Java: File[] subFiles = file.listFiles();
                    // Java: if (subFiles != null && subFiles.length != 0)
                    if let Ok(subFiles) = fs::read_dir(file)
                    {
                        // Java: for(File subFile : subFiles)
                        for subFile in subFiles.flatten()
                        {
                            // Java: if (subFile.isDirectory())
                            if subFile.path().is_dir()
                            {
                                // Java: recursiveScanFolders(subFile.getPath(), maxLevel, dirs);
                                Self::recursiveScanFolders(&subFile.path().to_string_lossy(), maxLevel, dirs);
                            }
                        }
                    }
                }
            }
        }
    }

    // Java: static String getFontsCacheFilePath()
    pub fn getFontsCacheFilePath() -> String {
        // Java: return InternalPlatform.getInstance().getFontsCacheFilePath();
        InternalPlatform::getInstance().lock().unwrap().getFontsCacheFilePath().to_string()
    }

    // Java: static void setFontsCacheFilePath(String path)
    pub fn setFontsCacheFilePath(path: &str) {
        // Java: InternalPlatform.getInstance().setFontsCacheFilePath(path);
        InternalPlatform::getInstance().lock().unwrap().setFontsCacheFilePath(path);
    }

    // Java: static void setSDKVersion(int sdkVersion)
    pub fn setSDKVersion(sdkVersion: i32) {
        // Java: SDK_VERSION = sdkVersion;
        *get_i32_lock(&SDK_VERSION_VAL).lock().unwrap() = sdkVersion;
    }

    // Java: static int getSDKVersion()
    pub fn getSDKVersion() -> i32 {
        // Java: return SDK_VERSION;
        *get_i32_lock(&SDK_VERSION_VAL).lock().unwrap()
    }

    // Java: static void setIsMZ606(boolean isMZ606)
    pub fn setIsMZ606(isMZ606: bool) {
        // Java: sIsMZ606 = isMZ606;
        *get_bool_lock(&S_IS_MZ606).lock().unwrap() = isMZ606;
    }

    // Java: static boolean isMZ606()
    pub fn isMZ606() -> bool {
        // Java: return sIsMZ606;
        *get_bool_lock(&S_IS_MZ606).lock().unwrap()
    }

    // Java: static void setIsHTC_Flyer_P510e(boolean value)
    pub fn setIsHTC_Flyer_P510e(value: bool) {
        // Java: sIsHTC_Flyer_P510e = value;
        *get_bool_lock(&S_IS_HTC_FLYER_P510E).lock().unwrap() = value;
    }

    // Java: static boolean isHTC_Flyer_P510e()
    pub fn isHTC_Flyer_P510e() -> bool {
        // Java: return sIsHTC_Flyer_P510e;
        *get_bool_lock(&S_IS_HTC_FLYER_P510E).lock().unwrap()
    }

    // Java: static void setPicturePool(IPicturePool picturePool)
    pub fn setPicturePool(picturePool: Arc<dyn IPicturePool + Send + Sync>) {
        // Java: mPicturePool = picturePool;
        *M_PICTURE_POOL.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(picturePool);
    }

    // Java: static IPicturePool getPicturePool()
    pub fn getPicturePool() -> Option<Arc<dyn IPicturePool + Send + Sync>> {
        // Java: return mPicturePool;
        M_PICTURE_POOL.get_or_init(|| Mutex::new(None)).lock().unwrap().clone()
    }

    // Java: static void setMathDrawer(IMathDrawer mathDrawer)
    pub fn setMathDrawer(mathDrawer: Arc<dyn IMathDrawer + Send + Sync>) {
        // Java: mMathDrawer = mathDrawer;
        *M_MATH_DRAWER.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(mathDrawer);
    }

    // Java: static IMathDrawer getMathDrawer()
    pub fn getMathDrawer() -> Option<Arc<dyn IMathDrawer + Send + Sync>> {
        // Java: return mMathDrawer;
        M_MATH_DRAWER.get_or_init(|| Mutex::new(None)).lock().unwrap().clone()
    }

    // Java: static void setIsTVMeetingVersion(boolean tvMeetingVersion)
    pub fn setIsTVMeetingVersion(tvMeetingVersion: bool) {
        // Java: mIsTvMeetingVersion = tvMeetingVersion;
        *get_bool_lock(&M_IS_TV_MEETING_VERSION).lock().unwrap() = tvMeetingVersion;
    }

    // Java: static boolean isTVMeetingVersion()
    pub fn isTVMeetingVersion() -> bool {
        // Java: return mIsTvMeetingVersion;
        *get_bool_lock(&M_IS_TV_MEETING_VERSION).lock().unwrap()
    }

    // Java: static void resetTmpFileDir()
    pub fn resetTmpFileDir() {
        // Java: TEMP_PRESENTATION = null;
        *get_opt_str_lock(&TEMP_PRESENTATION_DIR).lock().unwrap() = None;
        // Java: BACKUP_PRESENTATION = null;
        *get_opt_str_lock(&BACKUP_PRESENTATION_DIR).lock().unwrap() = None;
        // Java: TEMP_SPREADSHEET = null;
        *get_opt_str_lock(&TEMP_SPREADSHEET_DIR).lock().unwrap() = None;
        // Java: BACKUP_SPREADSHEET = null;
        *get_opt_str_lock(&BACKUP_SPREADSHEET_DIR).lock().unwrap() = None;
    }

    // Java: static void setSharedInfo(ISharedInfo sharedInfo)
    pub fn setSharedInfo(sharedInfo: Arc<dyn crate::cn::wps::core::runtime::ISharedInfo::ISharedInfo + Send + Sync>) {
        // Java: mSharedInfo = sharedInfo;
        *M_SHARED_INFO.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(sharedInfo);
    }

    // Java: static ISharedInfo getSharedInfo()
    pub fn getSharedInfo() -> Option<Arc<dyn crate::cn::wps::core::runtime::ISharedInfo::ISharedInfo + Send + Sync>> {
        // Java: return mSharedInfo;
        M_SHARED_INFO.get_or_init(|| Mutex::new(None)).lock().unwrap().clone()
    }

    // Java: static String getPresTempDirectory()
    pub fn getPresTempDirectory() -> String {
        // Java: if (TEMP_PRESENTATION == null || !TEMP_PRESENTATION.exists())
        let mut dir = get_opt_str_lock(&TEMP_PRESENTATION_DIR).lock().unwrap();
        if dir.is_none() || !Path::new(dir.as_ref().unwrap()).exists()
        {
            // Java: int n = 0;
            let mut n: i32 = 0;
            // Java: TEMP_PRESENTATION = initDirectory(getTempDirectory(), SaveHelper.AppType.Presentation, n++);
            *dir = Some(Self::initDirectory(&Self::getTempDirectory(), "presentation", n));
            n += 1;
            // Java: File temp = null;
            let mut temp_created = false;
            // Java: try
            {
                // Java: temp = File.createTempFile("ppt_", ".tmp", TEMP_PRESENTATION);
                let temp_path = format!("{}{}ppt_.tmp", dir.as_ref().unwrap(), std::path::MAIN_SEPARATOR);
                temp_created = std::fs::File::create(&temp_path).is_ok();
                if temp_created { let _ = fs::remove_file(&temp_path); }
            }
            // Java: catch (IOException var5)
            if !temp_created
            {
                // Java: while(true)
                loop {
                    // Java: TEMP_PRESENTATION = initDirectory(getTempDirectory(), SaveHelper.AppType.Presentation, n++);
                    *dir = Some(Self::initDirectory(&Self::getTempDirectory(), "presentation", n));
                    n += 1;
                    // Java: try
                    {
                        // Java: temp = File.createTempFile("ppt_", ".tmp", TEMP_PRESENTATION);
                        let tp = format!("{}{}ppt_.tmp", dir.as_ref().unwrap(), std::path::MAIN_SEPARATOR);
                        if std::fs::File::create(&tp).is_ok() {
                            let _ = fs::remove_file(&tp);
                            // Java: break;
                            break;
                        }
                    }
                    // Java: catch (IOException var4)
                    {
                        // Java: if (n >= 32)
                        if n >= 32
                        {
                            // Java: break;
                            break;
                        }
                    }
                }
            }
            // Java: if (temp != null && temp.exists())
            {
                // Java: temp.delete();
            }
        }
        // Java: return TEMP_PRESENTATION == null ? getTempDirectory() : TEMP_PRESENTATION.getAbsolutePath();
        dir.clone().unwrap_or_else(|| Self::getTempDirectory())
    }

    // Java: static String getPresBackupDirectory()
    pub fn getPresBackupDirectory() -> String {
        // Java: if (BACKUP_PRESENTATION == null)
        let mut dir = get_opt_str_lock(&BACKUP_PRESENTATION_DIR).lock().unwrap();
        if dir.is_none()
        {
            // Java: int n = 0;
            let mut n: i32 = 0;
            // Java: BACKUP_PRESENTATION = initDirectory(getBackupDirectory(), SaveHelper.AppType.Presentation, n++);
            *dir = Some(Self::initDirectory(&Self::getBackupDirectory(), "presentation", n));
            n += 1;
            // Java: File temp = null;
            let mut temp_created = false;
            // Java: try
            {
                // Java: temp = File.createTempFile("ppt_", ".tmp", BACKUP_PRESENTATION);
                let tp = format!("{}{}ppt_.tmp", dir.as_ref().unwrap(), std::path::MAIN_SEPARATOR);
                temp_created = std::fs::File::create(&tp).is_ok();
                if temp_created { let _ = fs::remove_file(&tp); }
            }
            // Java: catch (IOException var5)
            if !temp_created
            {
                // Java: while(true)
                loop {
                    // Java: BACKUP_PRESENTATION = initDirectory(getBackupDirectory(), SaveHelper.AppType.Presentation, n++);
                    *dir = Some(Self::initDirectory(&Self::getBackupDirectory(), "presentation", n));
                    n += 1;
                    // Java: try
                    {
                        // Java: temp = File.createTempFile("ppt_", ".tmp", BACKUP_PRESENTATION);
                        let tp = format!("{}{}ppt_.tmp", dir.as_ref().unwrap(), std::path::MAIN_SEPARATOR);
                        if std::fs::File::create(&tp).is_ok() {
                            let _ = fs::remove_file(&tp);
                            // Java: break;
                            break;
                        }
                    }
                    // Java: catch (IOException var4)
                    {
                        // Java: if (n >= 32)
                        if n >= 32
                        {
                            // Java: break;
                            break;
                        }
                    }
                }
            }
            // Java: if (temp != null && temp.exists())
            {
                // Java: temp.delete();
            }
        }
        // Java: return BACKUP_PRESENTATION == null ? getBackupDirectory() : BACKUP_PRESENTATION.getAbsolutePath();
        dir.clone().unwrap_or_else(|| Self::getBackupDirectory())
    }

    // Java: static String getETTempDirectory()
    pub fn getETTempDirectory() -> String {
        // Java: if (TEMP_SPREADSHEET == null || !TEMP_SPREADSHEET.exists())
        let mut dir = get_opt_str_lock(&TEMP_SPREADSHEET_DIR).lock().unwrap();
        if dir.is_none() || !Path::new(dir.as_ref().unwrap()).exists()
        {
            // Java: int n = 0;
            let mut n: i32 = 0;
            // Java: TEMP_SPREADSHEET = initDirectory(getTempDirectory(), SaveHelper.AppType.SpreadSheet, n++);
            *dir = Some(Self::initDirectory(&Self::getTempDirectory(), "spreadsheet", n));
            n += 1;
            // Java: File temp = null;
            let mut temp_created = false;
            // Java: try
            {
                // Java: temp = File.createTempFile("xls_", ".tmp", TEMP_SPREADSHEET);
                let tp = format!("{}{}xls_.tmp", dir.as_ref().unwrap(), std::path::MAIN_SEPARATOR);
                temp_created = std::fs::File::create(&tp).is_ok();
                if temp_created { let _ = fs::remove_file(&tp); }
            }
            // Java: catch (IOException var5)
            if !temp_created
            {
                // Java: while(true)
                loop {
                    // Java: TEMP_SPREADSHEET = initDirectory(getTempDirectory(), SaveHelper.AppType.SpreadSheet, n++);
                    *dir = Some(Self::initDirectory(&Self::getTempDirectory(), "spreadsheet", n));
                    n += 1;
                    // Java: try
                    {
                        // Java: temp = File.createTempFile("xls_", ".tmp", TEMP_SPREADSHEET);
                        let tp = format!("{}{}xls_.tmp", dir.as_ref().unwrap(), std::path::MAIN_SEPARATOR);
                        if std::fs::File::create(&tp).is_ok() {
                            let _ = fs::remove_file(&tp);
                            // Java: break;
                            break;
                        }
                    }
                    // Java: catch (IOException var4)
                    {
                        // Java: if (n >= 32)
                        if n >= 32
                        {
                            // Java: break;
                            break;
                        }
                    }
                }
            }
            // Java: if (temp != null && temp.exists())
            {
                // Java: temp.delete();
            }
        }
        // Java: return TEMP_SPREADSHEET == null ? getTempDirectory() : TEMP_SPREADSHEET.getAbsolutePath();
        dir.clone().unwrap_or_else(|| Self::getTempDirectory())
    }

    // Java: static String getPDFTempDirectory()
    pub fn getPDFTempDirectory() -> String {
        // Java: if (TEMP_PDF == null)
        let mut dir = get_opt_str_lock(&TEMP_PDF_DIR).lock().unwrap();
        if dir.is_none()
        {
            // Java: int n = 0;
            let mut n: i32 = 0;
            // Java: TEMP_PDF = initDirectory(getTempDirectory(), SaveHelper.AppType.PDF, n++);
            *dir = Some(Self::initDirectory(&Self::getTempDirectory(), "pdf", n));
            n += 1;
            // Java: File temp = null;
            let mut temp_created = false;
            // Java: try
            {
                // Java: temp = File.createTempFile("pdf_", ".tmp", TEMP_PDF);
                let tp = format!("{}{}pdf_.tmp", dir.as_ref().unwrap(), std::path::MAIN_SEPARATOR);
                temp_created = std::fs::File::create(&tp).is_ok();
                if temp_created { let _ = fs::remove_file(&tp); }
            }
            // Java: catch (IOException var5)
            if !temp_created
            {
                // Java: while(true)
                loop {
                    // Java: TEMP_PDF = initDirectory(getTempDirectory(), SaveHelper.AppType.PDF, n++);
                    *dir = Some(Self::initDirectory(&Self::getTempDirectory(), "pdf", n));
                    n += 1;
                    // Java: try
                    {
                        // Java: temp = File.createTempFile("pdf_", ".tmp", TEMP_PDF);
                        let tp = format!("{}{}pdf_.tmp", dir.as_ref().unwrap(), std::path::MAIN_SEPARATOR);
                        if std::fs::File::create(&tp).is_ok() {
                            let _ = fs::remove_file(&tp);
                            // Java: break;
                            break;
                        }
                    }
                    // Java: catch (IOException var4)
                    {
                        // Java: if (n >= 32)
                        if n >= 32
                        {
                            // Java: break;
                            break;
                        }
                    }
                }
            }
            // Java: if (temp != null && temp.exists())
            {
                // Java: temp.delete();
            }
        }
        // Java: return TEMP_PDF == null ? getTempDirectory() : TEMP_PDF.getAbsolutePath();
        dir.clone().unwrap_or_else(|| Self::getTempDirectory())
    }

    // Java: static String getETBackupDirectory()
    pub fn getETBackupDirectory() -> String {
        // Java: if (BACKUP_SPREADSHEET == null)
        let mut dir = get_opt_str_lock(&BACKUP_SPREADSHEET_DIR).lock().unwrap();
        if dir.is_none()
        {
            // Java: int n = 0;
            let mut n: i32 = 0;
            // Java: BACKUP_SPREADSHEET = initDirectory(getBackupDirectory(), SaveHelper.AppType.SpreadSheet, n++);
            *dir = Some(Self::initDirectory(&Self::getBackupDirectory(), "spreadsheet", n));
            n += 1;
            // Java: File temp = null;
            let mut temp_created = false;
            // Java: try
            {
                // Java: temp = File.createTempFile("xls_", ".tmp", BACKUP_SPREADSHEET);
                let tp = format!("{}{}xls_.tmp", dir.as_ref().unwrap(), std::path::MAIN_SEPARATOR);
                temp_created = std::fs::File::create(&tp).is_ok();
                if temp_created { let _ = fs::remove_file(&tp); }
            }
            // Java: catch (IOException var5)
            if !temp_created
            {
                // Java: while(true)
                loop {
                    // Java: BACKUP_SPREADSHEET = initDirectory(getBackupDirectory(), SaveHelper.AppType.SpreadSheet, n++);
                    *dir = Some(Self::initDirectory(&Self::getBackupDirectory(), "spreadsheet", n));
                    n += 1;
                    // Java: try
                    {
                        // Java: temp = File.createTempFile("xls_", ".tmp", BACKUP_SPREADSHEET);
                        let tp = format!("{}{}xls_.tmp", dir.as_ref().unwrap(), std::path::MAIN_SEPARATOR);
                        if std::fs::File::create(&tp).is_ok() {
                            let _ = fs::remove_file(&tp);
                            // Java: break;
                            break;
                        }
                    }
                    // Java: catch (IOException var4)
                    {
                        // Java: if (n >= 32)
                        if n >= 32
                        {
                            // Java: break;
                            break;
                        }
                    }
                }
            }
            // Java: if (temp != null && temp.exists())
            {
                // Java: temp.delete();
            }
        }
        // Java: return BACKUP_SPREADSHEET == null ? getBackupDirectory() : BACKUP_SPREADSHEET.getAbsolutePath();
        dir.clone().unwrap_or_else(|| Self::getBackupDirectory())
    }

    // Java: static File initDirectory(String parentDir, SaveHelper.AppType appType, int n)
    fn initDirectory(parentDir: &str, dirName: &str, n: i32) -> String {
        // Java: String dirName = appType.getDirName();
        // Java: if (!parentDir.endsWith(File.separator))
        let sep = std::path::MAIN_SEPARATOR;
        let mut parent = parentDir.to_string();
        if !parent.ends_with(sep)
        {
            // Java: parentDir = parentDir + File.separator;
            parent.push(sep);
        }
        // Java: if (n == 0)
        if n == 0
        {
            // Java: parentDir = parentDir + dirName;
            parent.push_str(dirName);
        }
        // Java: else
        else
        {
            // Java: parentDir = parentDir + dirName + "(" + Integer.toString(n) + ")";
            parent.push_str(&format!("{}({})", dirName, n));
        }
        // Java: File destDir = new File(parentDir);
        // Java: if (!destDir.exists())
        if !Path::new(&parent).exists()
        {
            // Java: destDir.mkdirs();
            let _ = fs::create_dir_all(&parent);
        }
        // Java: return destDir;
        parent
    }

    // Java: static String getShapePath()
    pub fn getShapePath() -> Option<String> {
        // Java: return mShapePath;
        get_opt_str_lock(&M_SHAPE_PATH).lock().unwrap().clone()
    }

    // Java: static void setShapePath(String path)
    pub fn setShapePath(path: &str) {
        // Java: mShapePath = path;
        *get_opt_str_lock(&M_SHAPE_PATH).lock().unwrap() = Some(path.to_string());
    }

    // Java: static void setConfigManager(IConfigManager configManager)
    pub fn setConfigManager(configManager: Arc<dyn crate::cn::wps::core::runtime::IConfigManager::IConfigManager + Send + Sync>) {
        let is_oversea = configManager.isOverseaVersion();
        // Java: mConfigManager = configManager;
        *M_CONFIG_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(configManager);
        // Java: PlatformAdapter.setOverSeaVersion(configManager.isOverseaVersion());
        PlatformAdapter::setOverSeaVersion(is_oversea);
    }

    // Java: static IConfigManager getConfigManager()
    pub fn getConfigManager() -> Option<Arc<dyn crate::cn::wps::core::runtime::IConfigManager::IConfigManager + Send + Sync>> {
        // Java: return mConfigManager;
        M_CONFIG_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap().clone()
    }

    // Java: static IAnalysisExecutor getAnalysisExecutor()
    pub fn getAnalysisExecutor() -> Option<Arc<dyn crate::cn::wps::core::runtime::IAnalysisExecutor::IAnalysisExecutor + Send + Sync>> {
        // Java: return sAnalysisExecutor;
        S_ANALYSIS_EXECUTOR.get_or_init(|| Mutex::new(None)).lock().unwrap().clone()
    }

    // Java: static void setAnalysisExecutor(IAnalysisExecutor sAnalysisExecutor)
    pub fn setAnalysisExecutor(executor: Arc<dyn crate::cn::wps::core::runtime::IAnalysisExecutor::IAnalysisExecutor + Send + Sync>) {
        // Java: Platform.sAnalysisExecutor = sAnalysisExecutor;
        *S_ANALYSIS_EXECUTOR.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(executor);
    }

    // Java: static boolean isRtl()
    pub fn isRtl() -> bool {
        // Java: return mConfigManager != null && mConfigManager.isRtl();
        M_CONFIG_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap().as_ref().is_some_and(|manager| manager.isRtl())
    }

    // Java: static boolean isOverseaVersion()
    pub fn isOverseaVersion() -> bool {
        // Java: return mConfigManager != null && mConfigManager.isOverseaVersion();
        M_CONFIG_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap().as_ref().is_some_and(|manager| manager.isOverseaVersion())
    }

    // Java: static boolean isDebugLogVersion()
    pub fn isDebugLogVersion() -> bool {
        // Java: return mConfigManager != null && mConfigManager.isDebugLogVersion();
        M_CONFIG_MANAGER.get_or_init(|| Mutex::new(None)).lock().unwrap().as_ref().is_some_and(|manager| manager.isDebugLogVersion())
    }

    // Java: static String getClipAuthority()
    pub fn getClipAuthority() -> Option<String> {
        // Java: return mClipAuthority;
        get_opt_str_lock(&M_CLIP_AUTHORITY).lock().unwrap().clone()
    }

    // Java: static void setClipAuthority(String clipAuthority)
    pub fn setClipAuthority(clipAuthority: Option<&str>) {
        // Java: mClipAuthority = clipAuthority;
        *get_opt_str_lock(&M_CLIP_AUTHORITY).lock().unwrap() = clipAuthority.map(str::to_string);
    }

    // Java: static void initAdapter()
    pub fn initAdapter() {
        // Java: KSFileLog.initKFSFileLogAapater();
        KSFileLog::initKFSFileLogAapater();
        // Java: setPlatformAdapter();
        Self::setPlatformAdapter();
    }

    // Java: static void setPlatformAdapter()
    pub fn setPlatformAdapter() {
        // Java: PlatformAdapter.setPlatformFile(new PlatformAdapter.IPlatformFile()
        PlatformAdapter::setPlatformFile(Box::new(PlatformFileBridge));
        // Java: public File createTempFile(String prefix, String suffix)
        // Java: return Platform.createTempFile(prefix, suffix);
        // Java: public String getCurrentPidTmpDir()
        // Java: return Platform.getCurrentPidTmpDir();
    }

}
