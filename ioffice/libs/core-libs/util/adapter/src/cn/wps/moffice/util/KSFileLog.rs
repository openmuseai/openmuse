// Java: import cn.wps.base.log.Log
// use kfs_kfs_app::cn::wps::moffice::kfs::log::Log; // dep not yet available
// Java: import cn.wps.core.runtime.Platform
// use crate::core::runtime::Platform; // dep not yet available
// Java: import java.io.File
use std::path::Path;
// Java: import java.io.FileOutputStream
use std::fs::{self, OpenOptions};
// Java: import java.io.IOException
use std::io::Write;
// Java: import java.util.Date
use chrono::{Local, NaiveDate, Duration};
// Java: import java.util.Map
use std::collections::HashMap;

use std::sync::{Mutex, OnceLock};

// Java field: StringBuilder builder_
static BUILDER: OnceLock<Mutex<String>> = OnceLock::new();
// Java field: String SPACE
pub(crate) const SPACE: &str = " ";
// Java field: String LINE_BREAK
pub(crate) const LINE_BREAK: &str = "\n";
// Java field: String LOG_SUFFIX
pub(crate) const LOG_SUFFIX: &str = "log";
// Java field: String VERBOSE
pub const VERBOSE: &str = "VERBOSE";
// Java field: String DEBUG
pub const DEBUG: &str = "DEBUG";
// Java field: String INFO
pub const INFO: &str = "INFO";
// Java field: String WARN
pub const WARN: &str = "WARN";
// Java field: String ERROR
pub const ERROR: &str = "ERROR";
// Java field: String ASSERT
pub const ASSERT: &str = "ASSERT";
// Java field: int LOG_DAY_LENGTH
pub(crate) const LOG_DAY_LENGTH: i32 = 7;
// Java field: String VERSION_INFO
static VERSION_INFO: OnceLock<Mutex<String>> = OnceLock::new();

fn get_builder() -> &'static Mutex<String> {
    BUILDER.get_or_init(|| Mutex::new(String::new()))
}

fn get_version_info() -> &'static Mutex<String> {
    VERSION_INFO.get_or_init(|| Mutex::new(String::new()))
}

// Java: public class KSFileLog
pub struct KSFileLog {
}

// Java: public class KSFileLog
impl KSFileLog {
    // Java: KSFileLog()
    pub fn new() -> Self {
        // Java: super();
        KSFileLog {}
    }

    // Java: static void deleteExpiredLog()
    pub fn deleteExpiredLog() {
        // Java: deleteLog(Platform.getSaveLogFilePath());
        // Self::deleteLog(&Platform::getSaveLogFilePath()); // TODO: uncomment when Platform available
        // Java: deleteLog(Platform.getCrashLogFilePath());
        // Self::deleteLog(&Platform::getCrashLogFilePath());
        // Java: deleteLog(Platform.getCloudStorageLogFilePath());
        // Self::deleteLog(&Platform::getCloudStorageLogFilePath());
        // Java: deleteLog(Platform.getFileRoamingLogFilePath());
        // Self::deleteLog(&Platform::getFileRoamingLogFilePath());
        // Java: deleteLog(Platform.getSharePlayLogFilePath());
        // Self::deleteLog(&Platform::getSharePlayLogFilePath());
        // Java: deleteLog(Platform.getExitReasonLogFilePath());
        // Self::deleteLog(&Platform::getExitReasonLogFilePath());
    }

    // Java: static void deleteExpiredExitReasonLog()
    pub fn deleteExpiredExitReasonLog() {
        // Java: deleteLog(Platform.getExitReasonLogFilePath());
        // Self::deleteLog(&Platform::getExitReasonLogFilePath()); // TODO: uncomment when Platform available
    }

    // Java: static void deleteLog(String logFilePath)
    pub fn deleteLog(logFilePath: &str) {
        // Java: Date lastDate = DateUtil.getDateByDays(-7);
        let last_date = Local::now().date_naive() - Duration::days(LOG_DAY_LENGTH as i64);
        // Java: if (null != logFilePath && logFilePath.length() > 0)
        if !logFilePath.is_empty()
        {
            // Java: File dir = new File(logFilePath);
            let dir = Path::new(logFilePath);
            // Java: if (dir.exists() && dir.isDirectory())
            if dir.exists() && dir.is_dir()
            {
                // Java: File[] files = dir.listFiles();
                let files = fs::read_dir(dir);
                // Java: if (files != null)
                if let Ok(entries) = files
                {
                    // Java: for(File file : files)
                    for entry in entries.flatten()
                    {
                        let file_name = entry.file_name();
                        let file_name_str = file_name.to_string_lossy();
                        // Java: if ("log".equalsIgnoreCase(StringUtil.pathExtension(file.getName())))
                        let ext = Path::new(&*file_name_str).extension().and_then(|e| e.to_str()).unwrap_or("");
                        if ext.eq_ignore_ascii_case("log")
                        {
                            // Java: Date date = DateUtil.parseDate(StringUtil.stringByDeletingPathExtension(file.getName()), "yyyyMMdd");
                            let stem = Path::new(&*file_name_str).file_stem().and_then(|s| s.to_str()).unwrap_or("");
                            let date = NaiveDate::parse_from_str(stem, "%Y%m%d").ok();
                            // Java: if (null != date && date.before(lastDate))
                            if let Some(d) = date {
                                if d < last_date
                                {
                                    // Java: file.delete();
                                    let _ = fs::remove_file(entry.path());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Java: static String getNowData()
    pub fn getNowData() -> String {
        // Java: return DateUtil.formatDate(new Date(), "yyyy-MM-dd  HH:mm:ss.SSS");
        Local::now().format("%Y-%m-%d  %H:%M:%S%.3f").to_string()
    }

    // Java: static String createLogContent(String priority, String tag, String msg)
    pub fn createLogContent(priority: &str, tag: &str, msg: &str) -> String {
        // Java: return createLogContent(priority, tag, msg, (String)null);
        Self::createLogContent_with_stack(priority, tag, msg, None)
    }

    // Java: static String createLogContent(String priority, String tag, String msg, String stackTraceString)
    pub fn createLogContent_with_stack(priority: &str, tag: &str, msg: &str, stackTraceString: Option<&str>) -> String {
        let mut builder = get_builder().lock().unwrap();
        // Java: builder_.setLength(0);
        builder.clear();
        // Java: builder_.append(priority);
        builder.push_str(priority);
        // Java: builder_.append(" ");
        builder.push_str(SPACE);
        // Java: builder_.append(getNowData());
        builder.push_str(&Self::getNowData());
        // Java: builder_.append(" ");
        builder.push_str(SPACE);
        // Java: builder_.append(tag);
        builder.push_str(tag);
        // Java: builder_.append(" ");
        builder.push_str(SPACE);
        // Java: builder_.append(msg);
        builder.push_str(msg);
        // Java: builder_.append("\n");
        builder.push_str(LINE_BREAK);
        // Java: if (null != stackTraceString && stackTraceString.length() > 0)
        if let Some(stack) = stackTraceString {
            if !stack.is_empty()
            {
                // Java: builder_.append(stackTraceString);
                builder.push_str(stack);
                // Java: builder_.append("\n");
                builder.push_str(LINE_BREAK);
            }
        }
        // Java: return builder_.toString();
        builder.clone()
    }

    // Java: static void writeToFile(String content)
    fn writeToFile_content(content: &str) {
        // Java: doWriteToFile(Platform.getCrashLogFilePath(), content);
        // Self::doWriteToFile(&Platform::getCrashLogFilePath(), content); // TODO: uncomment when Platform available
    }

    // Java: static void writeToFile(String logFile, String content)
    pub fn writeToFile_with_logfile(logFile: &str, content: &str) {
        // Java: doWriteToFile(logFile, content);
        Self::doWriteToFile(logFile, content);
    }

    // Java: static void writeToFile(String logFile, String content, Map<String, String> values)
    pub fn writeToFile_with_values(logFile: &str, content: &str, values: &mut HashMap<String, String>) {
        // Java: doWriteToFile(logFile, content, values);
        Self::doWriteToFile_with_values(logFile, content, values);
    }

    // Java: static void writeSaveErrorToFile(String content)
    fn writeSaveErrorToFile(content: &str) {
        // Java: doWriteToFile(Platform.getSaveLogFilePath(), content);
        // Self::doWriteToFile(&Platform::getSaveLogFilePath(), content); // TODO: uncomment when Platform available
    }

    // Java: static void writeCloudStorageErrorToFile(String content)
    fn writeCloudStorageErrorToFile(content: &str) {
        // Java: doWriteToFile(Platform.getCloudStorageLogFilePath(), content);
        // Self::doWriteToFile(&Platform::getCloudStorageLogFilePath(), content); // TODO: uncomment when Platform available
    }

    // Java: static void writeFileRoamingLogToFile(String content)
    fn writeFileRoamingLogToFile(content: &str) {
        // Java: doWriteToFile(Platform.getFileRoamingLogFilePath(), content);
        // Self::doWriteToFile(&Platform::getFileRoamingLogFilePath(), content); // TODO: uncomment when Platform available
    }

    // Java: static void writeSharePlayLogToFile(String content)
    fn writeSharePlayLogToFile(content: &str) {
        // Java: doWriteToFile(Platform.getSharePlayLogFilePath(), content);
        // Self::doWriteToFile(&Platform::getSharePlayLogFilePath(), content); // TODO: uncomment when Platform available
    }

    // Java: static void doWriteToFile(String file, String content, Map<String, String> values)
    fn doWriteToFile_with_values(file: &str, content: &str, values: &mut HashMap<String, String>) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: if (null != file && file.length() > 0)
            if !file.is_empty()
            {
                // Java: String filePath = createLogFilePath(file);
                let filePath = Self::createLogFilePath(file);
                // Java: File f = new File(filePath);
                let f = Path::new(&filePath);
                // Java: if (!f.exists())
                if !f.exists()
                {
                    // Java: try
                    if let Some(parent) = f.parent()
                    {
                        // Java: File parent = f.getParentFile();
                        // Java: parent.mkdirs();
                        let _ = fs::create_dir_all(parent);
                        // Java: f.createNewFile();
                        let _ = fs::File::create(&filePath);
                    }
                    // Java: catch (IOException e)
                    // errors silently ignored (matching Java behavior)
                    // Java: e.printStackTrace();
                }
                // Java: StringBuilder sb = new StringBuilder();
                let mut sb = String::new();
                // Java: boolean hasUuid = false;
                let mut hasUuid = false;
                // Java: if (values != null && !values.isEmpty())
                if !values.is_empty()
                {
                    // Java: if (values.containsKey("uuid"))
                    if values.contains_key("uuid")
                    {
                        // Java: VERSION_INFO = VERSION_INFO + " [uuid=" + (String)values.get("uuid") + "]";
                        let mut vi = get_version_info().lock().unwrap();
                        *vi = format!("{} [uuid={}]", *vi, values.get("uuid").unwrap());
                        // Java: hasUuid = true;
                        hasUuid = true;
                        // Java: values.remove("uuid");
                        values.remove("uuid");
                    }
                    // Java: for(Map.Entry<String, String> entry : values.entrySet())
                    for (key, value) in values.iter()
                    {
                        // Java: String key = (String)entry.getKey();
                        // Java: sb.append(key).append("=").append((String)entry.getValue()).append(",");
                        sb.push_str(key);
                        sb.push('=');
                        sb.push_str(value);
                        sb.push(',');
                    }
                }
                // Java: if (sb.length() > 0)
                if !sb.is_empty()
                {
                    // Java: String strValue = sb.deleteCharAt(sb.length() - 1).toString();
                    sb.pop();
                    let strValue = sb;
                    let mut vi = get_version_info().lock().unwrap();
                    // Java: if (hasUuid)
                    if hasUuid
                    {
                        // Java: VERSION_INFO = VERSION_INFO + "\n [" + strValue + "]";
                        *vi = format!("{}\n [{}]", *vi, strValue);
                    }
                    // Java: else
                    else
                    {
                        // Java: VERSION_INFO = VERSION_INFO + " [" + strValue + "]";
                        *vi = format!("{} [{}]", *vi, strValue);
                    }
                }
                // Java: writeToFile(filePath, VERSION_INFO + "\n" + content, true);
                let vi = get_version_info().lock().unwrap();
                let full_content = format!("{}\n{}", *vi, content);
                drop(vi);
                Self::writeToFile_append(&filePath, &full_content, true);
            }
        }
    }

    // Java: static void doWriteToFile(String file, String content)
    fn doWriteToFile(file: &str, content: &str) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: if (null != file && file.length() > 0)
            if !file.is_empty()
            {
                // Java: String filePath = createLogFilePath(file);
                let filePath = Self::createLogFilePath(file);
                // Java: File f = new File(filePath);
                let f = Path::new(&filePath);
                // Java: if (!f.exists())
                if !f.exists()
                {
                    // Java: try
                    if let Some(parent) = f.parent()
                    {
                        // Java: File parent = f.getParentFile();
                        // Java: parent.mkdirs();
                        let _ = fs::create_dir_all(parent);
                        // Java: f.createNewFile();
                        let _ = fs::File::create(&filePath);
                    }
                    // Java: catch (IOException e)
                    // errors silently ignored (matching Java behavior)
                    // Java: e.printStackTrace();
                }
                // Java: writeToFile(filePath, VERSION_INFO + "\n" + content, true);
                let vi = get_version_info().lock().unwrap();
                let full_content = format!("{}\n{}", *vi, content);
                drop(vi);
                Self::writeToFile_append(&filePath, &full_content, true);
            }
        }
    }

    // Java: static boolean writeToFile(String filename, String data, boolean append)
    fn writeToFile_append(filename: &str, data: &str, append: bool) -> bool {
        // Java: FileOutputStream out = null;
        // Java: try
        let result = if append {
            // Java: out = new FileOutputStream(filename, append);
            OpenOptions::new().create(true).append(true).open(filename)
        } else {
            OpenOptions::new().create(true).write(true).truncate(true).open(filename)
        };
        match result {
            Ok(mut out) => {
                // Java: out.write(data.getBytes());
                if out.write_all(data.as_bytes()).is_ok() {
                    // Java: boolean ex = true;
                    // Java: return ex;
                    return true;
                }
                // Java: catch (IOException ex)
                // Java: ex.printStackTrace();
                false
            }
            // Java: catch (IOException ex)
            Err(_e) => {
                // Java: ex.printStackTrace();
                eprintln!("Failed to open file: {}", filename);
                false
            }
        }
        // Java: finally
        // Java: FileUtil.closeQuietly(out);
        // Rust: file handle automatically closed when dropped
        // Java: return false;
    }

    // Java: static String createLogFilePath(String folder)
    pub fn createLogFilePath(folder: &str) -> String {
        // Java: if (folder == null)
        if folder.is_empty()
        {
            // Java: return "";
            return String::new();
        }
        // Java: else
        {
            // Java: String date = DateUtil.formatDate(new Date(), "yyyyMMdd");
            let date = Local::now().format("%Y%m%d").to_string();
            // Java: return folder + date + "." + "log";
            format!("{}{}.{}", folder, date, LOG_SUFFIX)
        }
    }

    // Java: static void v(String tag, String msg)
    pub fn v(tag: &str, msg: &str) {
        // Java: writeToFile(createLogContent("VERBOSE", tag, msg));
        Self::writeToFile_content(&Self::createLogContent(VERBOSE, tag, msg));
    }

    // Java: static void v(String tag, String msg, Throwable tr)
    pub fn v_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(createLogContent("VERBOSE", tag, msg, Log.getStackTraceString(tr)));
            Self::writeToFile_content(&Self::createLogContent_with_stack(VERBOSE, tag, msg, Some(&format!("{}", tr))));
        }
    }

    // Java: static void d(String tag, String msg)
    pub fn d(tag: &str, msg: &str) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(createLogContent("DEBUG", tag, msg));
            Self::writeToFile_content(&Self::createLogContent(DEBUG, tag, msg));
        }
    }

    // Java: static void d(String tag, String msg, Throwable tr)
    pub fn d_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(createLogContent("DEBUG", tag, msg, Log.getStackTraceString(tr)));
            Self::writeToFile_content(&Self::createLogContent_with_stack(DEBUG, tag, msg, Some(&format!("{}", tr))));
        }
    }

    // Java: static void i(String tag, String msg)
    pub fn i(tag: &str, msg: &str) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(createLogContent("INFO", tag, msg));
            Self::writeToFile_content(&Self::createLogContent(INFO, tag, msg));
        }
    }

    // Java: static void i(String tag, String msg, Throwable tr)
    pub fn i_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(createLogContent("DEBUG", tag, msg, Log.getStackTraceString(tr)));
            Self::writeToFile_content(&Self::createLogContent_with_stack(DEBUG, tag, msg, Some(&format!("{}", tr))));
        }
    }

    // Java: static void w(String tag, String msg)
    pub fn w(tag: &str, msg: &str) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(createLogContent("WARN", tag, msg));
            Self::writeToFile_content(&Self::createLogContent(WARN, tag, msg));
        }
    }

    // Java: static void w(String tag, String msg, Throwable tr)
    pub fn w_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(createLogContent("WARN", tag, msg, Log.getStackTraceString(tr)));
            Self::writeToFile_content(&Self::createLogContent_with_stack(WARN, tag, msg, Some(&format!("{}", tr))));
        }
    }

    // Java: static void w(String tag, Throwable tr)
    pub fn w_tr_only(tag: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(createLogContent("WARN", tag, "", Log.getStackTraceString(tr)));
            Self::writeToFile_content(&Self::createLogContent_with_stack(WARN, tag, "", Some(&format!("{}", tr))));
        }
    }

    // Java: static void e(String tag, String msg)
    pub fn e(tag: &str, msg: &str) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(createLogContent("ERROR", tag, msg));
            Self::writeToFile_content(&Self::createLogContent(ERROR, tag, msg));
        }
    }

    // Java: static void e(String tag, String msg, Throwable tr)
    pub fn e_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(createLogContent("ERROR", tag, msg, Log.getStackTraceString(tr)));
            Self::writeToFile_content(&Self::createLogContent_with_stack(ERROR, tag, msg, Some(&format!("{}", tr))));
        }
    }

    // Java: static void e(String logFile, String tag, String msg, Throwable tr)
    pub fn e_to_file_tr(logFile: &str, tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeToFile(logFile, createLogContent("ERROR", tag, msg, Log.getStackTraceString(tr)));
            Self::writeToFile_with_logfile(logFile, &Self::createLogContent_with_stack(ERROR, tag, msg, Some(&format!("{}", tr))));
        }
    }

    // Java: static void e(String logFile, String tag, String msg, String content)
    pub fn e_to_file(logFile: &str, tag: &str, msg: &str, content: &str) {
        // Java: writeToFile(logFile, createLogContent("ERROR", tag, msg, content));
        Self::writeToFile_with_logfile(logFile, &Self::createLogContent_with_stack(ERROR, tag, msg, Some(content)));
    }

    // Java: static void e(String logFile, String tag, String msg, String content, Map<String, String> values)
    pub fn e_to_file_with_values(logFile: &str, tag: &str, msg: &str, content: &str, values: &mut HashMap<String, String>) {
        // Java: writeToFile(logFile, createLogContent("ERROR", tag, msg, content), values);
        Self::writeToFile_with_values(logFile, &Self::createLogContent_with_stack(ERROR, tag, msg, Some(content)), values);
    }

    // Java: static void eForSave(String tag, String msg, Throwable tr)
    pub fn eForSave(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeSaveErrorToFile(createLogContent("ERROR", tag, msg, Log.getStackTraceString(tr)));
            Self::writeSaveErrorToFile(&Self::createLogContent_with_stack(ERROR, tag, msg, Some(&format!("{}", tr))));
        }
    }

    // Java: static void eForCloudStorage(String tag, String msg, Throwable tr)
    pub fn eForCloudStorage(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeCloudStorageErrorToFile(createLogContent("ERROR", tag, msg, Log.getStackTraceString(tr)));
            Self::writeCloudStorageErrorToFile(&Self::createLogContent_with_stack(ERROR, tag, msg, Some(&format!("{}", tr))));
        }
    }

    // Java: static void logForCloudStorage(String tag, String msg)
    pub fn logForCloudStorage(tag: &str, msg: &str) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeCloudStorageErrorToFile(createLogContent("INFO", tag, msg));
            Self::writeCloudStorageErrorToFile(&Self::createLogContent(INFO, tag, msg));
        }
    }

    // Java: static void logForFileRoaming(String level, String tag, String msg)
    pub fn logForFileRoaming(level: &str, tag: &str, msg: &str) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeFileRoamingLogToFile(createLogContent(level, tag, msg));
            Self::writeFileRoamingLogToFile(&Self::createLogContent(level, tag, msg));
        }
    }

    // Java: static void logForFileRoaming(String level, String tag, String msg, Throwable tr)
    pub fn logForFileRoaming_tr(level: &str, tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeFileRoamingLogToFile(createLogContent(level, tag, msg, Log.getStackTraceString(tr)));
            Self::writeFileRoamingLogToFile(&Self::createLogContent_with_stack(level, tag, msg, Some(&format!("{}", tr))));
        }
    }

    // Java: static void logForSharePlay(String level, String tag, String msg)
    pub fn logForSharePlay(level: &str, tag: &str, msg: &str) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeSharePlayLogToFile(createLogContent(level, tag, msg));
            Self::writeSharePlayLogToFile(&Self::createLogContent(level, tag, msg));
        }
    }

    // Java: static void logForSharePlay(String level, String tag, String msg, Throwable tr)
    pub fn logForSharePlay_tr(level: &str, tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO: add Platform check when available
        {
            // Java: writeSharePlayLogToFile(createLogContent(level, tag, msg, Log.getStackTraceString(tr)));
            Self::writeSharePlayLogToFile(&Self::createLogContent_with_stack(level, tag, msg, Some(&format!("{}", tr))));
        }
    }

    // Java: static void initKFSFileLogAapater()
    pub fn initKFSFileLogAapater() {
        // Java: KFSFileLogAdapter.setIkfsFileLog(new KFSFileLogAdapter.IKFSFileLog()
        // TODO: implement when KFSFileLogAdapter is available
        // Java: public void eForSave(String tag, String msg, Throwable tr)
        // Java: KSFileLog.eForSave(tag, msg, tr);
        // Java: public void e(String tag, String msg, Throwable tr)
        // Java: KSFileLog.e(tag, msg, tr);
    }

}
