// Java: import cn.wps.base.log.Log
use kfs_kfs_app::cn::wps::moffice::kfs::log::Log;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::core::runtime::Platform;
// Java: import java.io.File
// TODO_RUST_CODE use ???; // java.io.File
// Java: import java.io.FileOutputStream
// TODO_RUST_CODE use ???; // java.io.FileOutputStream
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import java.util.Date
// TODO_RUST_CODE use ???; // java.util.Date
// Java: import java.util.Map
// TODO_RUST_CODE use ???; // java.util.Map

// Java field: StringBuilder builder_
static builder_: std::sync::OnceLock<StringBuilder> = std::sync::OnceLock::new();
// Java field: String SPACE
pub(crate) const SPACE: &str = " ";
// Java field: String LINE_BREAK
pub(crate) const LINE_BREAK: &str = "\n";
// Java field: String LOG_SUFFIX
pub(crate) const LOG_SUFFIX: &str = "log";
// Java field: String VERBOSE
pub(crate) const VERBOSE: &str = "VERBOSE";
// Java field: String DEBUG
pub(crate) const DEBUG: &str = "DEBUG";
// Java field: String INFO
pub(crate) const INFO: &str = "INFO";
// Java field: String WARN
pub(crate) const WARN: &str = "WARN";
// Java field: String ERROR
pub(crate) const ERROR: &str = "ERROR";
// Java field: String ASSERT
pub(crate) const ASSERT: &str = "ASSERT";
// Java field: int LOG_DAY_LENGTH
pub(crate) const LOG_DAY_LENGTH: i32 = 7;
// Java field: String VERSION_INFO
static VERSION_INFO: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();

// Java: public class KSFileLog
pub struct KSFileLog {
}

// Java: public class KSFileLog
impl KSFileLog {
    // Java: KSFileLog()
    pub fn KSFileLog(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: static void deleteExpiredLog()
    pub fn deleteExpiredLog(&self) {
      // Java: deleteLog(Platform.getSaveLogFilePath());
      // TODO_RUST_CODE
      // Java: deleteLog(Platform.getCrashLogFilePath());
      // TODO_RUST_CODE
      // Java: deleteLog(Platform.getCloudStorageLogFilePath());
      // TODO_RUST_CODE
      // Java: deleteLog(Platform.getFileRoamingLogFilePath());
      // TODO_RUST_CODE
      // Java: deleteLog(Platform.getSharePlayLogFilePath());
      // TODO_RUST_CODE
      // Java: deleteLog(Platform.getExitReasonLogFilePath());
      // TODO_RUST_CODE
    }

    // Java: static void deleteExpiredExitReasonLog()
    pub fn deleteExpiredExitReasonLog(&self) {
      // Java: deleteLog(Platform.getExitReasonLogFilePath());
      // TODO_RUST_CODE
    }

    // Java: static void deleteLog(String logFilePath)
    pub fn deleteLog(&self, String logFilePath) {
      // Java: Date lastDate = DateUtil.getDateByDays(-7);
      // TODO_RUST_CODE
      // Java: if (null != logFilePath && logFilePath.length() > 0)
        // TODO_RUST_CODE
      {
         // Java: File dir = new File(logFilePath);
         // TODO_RUST_CODE
         // Java: if (dir.exists() && dir.isDirectory())
            // TODO_RUST_CODE
         {
            // Java: File[] files = dir.listFiles();
            // TODO_RUST_CODE
            // Java: if (files != null)
                // TODO_RUST_CODE
            {
               // Java: for(File file : files)
                    // TODO_RUST_CODE
               {
                  // Java: if ("log".equalsIgnoreCase(StringUtil.pathExtension(file.getName())))
                        // TODO_RUST_CODE
                  {
                     // Java: Date date = DateUtil.parseDate(StringUtil.stringByDeletingPathExtension(file.getName()), "yyyyMMdd");
                     // TODO_RUST_CODE
                     // Java: if (null != date && date.before(lastDate))
                            // TODO_RUST_CODE
                     {
                        // Java: file.delete();
                        // TODO_RUST_CODE
                     }
                  }
               }
            }
         }
      }
    }

    // Java: static String getNowData()
    pub fn getNowData(&self) -> String {
      // Java: return DateUtil.formatDate(new Date(), "yyyy-MM-dd  HH:mm:ss.SSS");
      // TODO_RUST_CODE
    }

    // Java: static String createLogContent(String priority, String tag, String msg)
    pub fn createLogContent(&self, String priority, String tag, String msg) -> String {
      // Java: return createLogContent(priority, tag, msg, (String)null);
      // TODO_RUST_CODE
    }

    // Java: static String createLogContent(String priority, String tag, String msg, String stackTraceString)
    pub fn createLogContent(&self, String priority, String tag, String msg, String stackTraceString) -> String {
      // Java: builder_.setLength(0);
      // TODO_RUST_CODE
      // Java: builder_.append(priority);
      // TODO_RUST_CODE
      // Java: builder_.append(" ");
      // TODO_RUST_CODE
      // Java: builder_.append(getNowData());
      // TODO_RUST_CODE
      // Java: builder_.append(" ");
      // TODO_RUST_CODE
      // Java: builder_.append(tag);
      // TODO_RUST_CODE
      // Java: builder_.append(" ");
      // TODO_RUST_CODE
      // Java: builder_.append(msg);
      // TODO_RUST_CODE
      // Java: builder_.append("\n");
      // TODO_RUST_CODE
      // Java: if (null != stackTraceString && stackTraceString.length() > 0)
        // TODO_RUST_CODE
      {
         // Java: builder_.append(stackTraceString);
         // TODO_RUST_CODE
         // Java: builder_.append("\n");
         // TODO_RUST_CODE
      }
      // Java: return builder_.toString();
      // TODO_RUST_CODE
    }

    // Java: static void writeToFile(String content)
    pub fn writeToFile(&self, String content) {
      // Java: doWriteToFile(Platform.getCrashLogFilePath(), content);
      // TODO_RUST_CODE
    }

    // Java: static void writeToFile(String logFile, String content)
    pub fn writeToFile(&self, String logFile, String content) {
      // Java: doWriteToFile(logFile, content);
      // TODO_RUST_CODE
    }

    // Java: static void writeToFile(String logFile, String content, Map<String, String> values)
    pub fn writeToFile(&self, String logFile, String content, Map<String, String> values) {
      // Java: doWriteToFile(logFile, content, values);
      // TODO_RUST_CODE
    }

    // Java: static void writeSaveErrorToFile(String content)
    pub fn writeSaveErrorToFile(&self, String content) {
      // Java: doWriteToFile(Platform.getSaveLogFilePath(), content);
      // TODO_RUST_CODE
    }

    // Java: static void writeCloudStorageErrorToFile(String content)
    pub fn writeCloudStorageErrorToFile(&self, String content) {
      // Java: doWriteToFile(Platform.getCloudStorageLogFilePath(), content);
      // TODO_RUST_CODE
    }

    // Java: static void writeFileRoamingLogToFile(String content)
    pub fn writeFileRoamingLogToFile(&self, String content) {
      // Java: doWriteToFile(Platform.getFileRoamingLogFilePath(), content);
      // TODO_RUST_CODE
    }

    // Java: static void writeSharePlayLogToFile(String content)
    pub fn writeSharePlayLogToFile(&self, String content) {
      // Java: doWriteToFile(Platform.getSharePlayLogFilePath(), content);
      // TODO_RUST_CODE
    }

    // Java: static void doWriteToFile(String file, String content, Map<String, String> values)
    pub fn doWriteToFile(&self, String file, String content, Map<String, String> values) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: if (null != file && file.length() > 0)
            // TODO_RUST_CODE
         {
            // Java: String filePath = createLogFilePath(file);
            // TODO_RUST_CODE
            // Java: File f = new File(filePath);
            // TODO_RUST_CODE
            // Java: if (!f.exists())
                // TODO_RUST_CODE
            {
               // Java: try
                    // TODO_RUST_CODE
               {
                  // Java: File parent = f.getParentFile();
                  // TODO_RUST_CODE
                  // Java: parent.mkdirs();
                  // TODO_RUST_CODE
                  // Java: f.createNewFile();
                  // TODO_RUST_CODE
               }
               // Java: catch (IOException e)
                    // TODO_RUST_CODE
               {
                  // Java: e.printStackTrace();
                  // TODO_RUST_CODE
               }
            }
            // Java: StringBuilder sb = new StringBuilder();
            // TODO_RUST_CODE
            // Java: boolean hasUuid = false;
            // TODO_RUST_CODE
            // Java: if (values != null && !values.isEmpty())
                // TODO_RUST_CODE
            {
               // Java: if (values.containsKey("uuid"))
                    // TODO_RUST_CODE
               {
                  // Java: VERSION_INFO = VERSION_INFO + " [uuid=" + (String)values.get("uuid") + "]";
                  // TODO_RUST_CODE
                  // Java: hasUuid = true;
                  // TODO_RUST_CODE
                  // Java: values.remove("uuid");
                  // TODO_RUST_CODE
               }
               // Java: for(Map.Entry<String, String> entry : values.entrySet())
                    // TODO_RUST_CODE
               {
                  // Java: String key = (String)entry.getKey();
                  // TODO_RUST_CODE
                  // Java: sb.append(key).append("=").append((String)entry.getValue()).append(",");
                  // TODO_RUST_CODE
               }
            }
            // Java: if (sb.length() > 0)
                // TODO_RUST_CODE
            {
               // Java: String strValue = sb.deleteCharAt(sb.length() - 1).toString();
               // TODO_RUST_CODE
               // Java: if (hasUuid)
                    // TODO_RUST_CODE
               {
                  // Java: VERSION_INFO = VERSION_INFO + "\n [" + strValue + "]";
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: VERSION_INFO = VERSION_INFO + " [" + strValue + "]";
                  // TODO_RUST_CODE
               }
            }
            // Java: writeToFile(filePath, VERSION_INFO + "\n" + content, true);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: static void doWriteToFile(String file, String content)
    pub fn doWriteToFile(&self, String file, String content) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: if (null != file && file.length() > 0)
            // TODO_RUST_CODE
         {
            // Java: String filePath = createLogFilePath(file);
            // TODO_RUST_CODE
            // Java: File f = new File(filePath);
            // TODO_RUST_CODE
            // Java: if (!f.exists())
                // TODO_RUST_CODE
            {
               // Java: try
                    // TODO_RUST_CODE
               {
                  // Java: File parent = f.getParentFile();
                  // TODO_RUST_CODE
                  // Java: parent.mkdirs();
                  // TODO_RUST_CODE
                  // Java: f.createNewFile();
                  // TODO_RUST_CODE
               }
               // Java: catch (IOException e)
                    // TODO_RUST_CODE
               {
                  // Java: e.printStackTrace();
                  // TODO_RUST_CODE
               }
            }
            // Java: writeToFile(filePath, VERSION_INFO + "\n" + content, true);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: static boolean writeToFile(String filename, String data, boolean append)
    pub fn writeToFile(&self, String filename, String data, boolean append) -> bool {
      // Java: FileOutputStream out = null;
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: out = new FileOutputStream(filename, append);
         // TODO_RUST_CODE
         // Java: out.write(data.getBytes());
         // TODO_RUST_CODE
         // Java: boolean ex = true;
         // TODO_RUST_CODE
         // Java: return ex;
         // TODO_RUST_CODE
      }
      // Java: catch (IOException ex)
        // TODO_RUST_CODE
      {
         // Java: ex.printStackTrace();
         // TODO_RUST_CODE
      }
      // Java: finally
        // TODO_RUST_CODE
      {
         // Java: FileUtil.closeQuietly(out);
         // TODO_RUST_CODE
      }
      // Java: return false;
      // TODO_RUST_CODE
    }

    // Java: static String createLogFilePath(String folder)
    pub fn createLogFilePath(&self, String folder) -> String {
      // Java: if (folder == null)
        // TODO_RUST_CODE
      {
         // Java: return "";
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: String date = DateUtil.formatDate(new Date(), "yyyyMMdd");
         // TODO_RUST_CODE
         // Java: return folder + date + "." + "log";
         // TODO_RUST_CODE
      }
    }

    // Java: static void v(String tag, String msg)
    pub fn v(&self, String tag, String msg) {
      // Java: writeToFile(createLogContent("VERBOSE", tag, msg));
      // TODO_RUST_CODE
    }

    // Java: static void v(String tag, String msg, Throwable tr)
    pub fn v(&self, String tag, String msg, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(createLogContent("VERBOSE", tag, msg, Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void d(String tag, String msg)
    pub fn d(&self, String tag, String msg) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(createLogContent("DEBUG", tag, msg));
         // TODO_RUST_CODE
      }
    }

    // Java: static void d(String tag, String msg, Throwable tr)
    pub fn d(&self, String tag, String msg, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(createLogContent("DEBUG", tag, msg, Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void i(String tag, String msg)
    pub fn i(&self, String tag, String msg) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(createLogContent("INFO", tag, msg));
         // TODO_RUST_CODE
      }
    }

    // Java: static void i(String tag, String msg, Throwable tr)
    pub fn i(&self, String tag, String msg, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(createLogContent("DEBUG", tag, msg, Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void w(String tag, String msg)
    pub fn w(&self, String tag, String msg) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(createLogContent("WARN", tag, msg));
         // TODO_RUST_CODE
      }
    }

    // Java: static void w(String tag, String msg, Throwable tr)
    pub fn w(&self, String tag, String msg, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(createLogContent("WARN", tag, msg, Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void w(String tag, Throwable tr)
    pub fn w(&self, String tag, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(createLogContent("WARN", tag, "", Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void e(String tag, String msg)
    pub fn e(&self, String tag, String msg) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(createLogContent("ERROR", tag, msg));
         // TODO_RUST_CODE
      }
    }

    // Java: static void e(String tag, String msg, Throwable tr)
    pub fn e(&self, String tag, String msg, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(createLogContent("ERROR", tag, msg, Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void e(String logFile, String tag, String msg, Throwable tr)
    pub fn e(&self, String logFile, String tag, String msg, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeToFile(logFile, createLogContent("ERROR", tag, msg, Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void e(String logFile, String tag, String msg, String content)
    pub fn e(&self, String logFile, String tag, String msg, String content) {
      // Java: writeToFile(logFile, createLogContent("ERROR", tag, msg, content));
      // TODO_RUST_CODE
    }

    // Java: static void e(String logFile, String tag, String msg, String content, Map<String, String> values)
    pub fn e(&self, String logFile, String tag, String msg, String content, Map<String, String> values) {
      // Java: writeToFile(logFile, createLogContent("ERROR", tag, msg, content), values);
      // TODO_RUST_CODE
    }

    // Java: static void eForSave(String tag, String msg, Throwable tr)
    pub fn eForSave(&self, String tag, String msg, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeSaveErrorToFile(createLogContent("ERROR", tag, msg, Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void eForCloudStorage(String tag, String msg, Throwable tr)
    pub fn eForCloudStorage(&self, String tag, String msg, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeCloudStorageErrorToFile(createLogContent("ERROR", tag, msg, Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void logForCloudStorage(String tag, String msg)
    pub fn logForCloudStorage(&self, String tag, String msg) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeCloudStorageErrorToFile(createLogContent("INFO", tag, msg));
         // TODO_RUST_CODE
      }
    }

    // Java: static void logForFileRoaming(String level, String tag, String msg)
    pub fn logForFileRoaming(&self, String level, String tag, String msg) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeFileRoamingLogToFile(createLogContent(level, tag, msg));
         // TODO_RUST_CODE
      }
    }

    // Java: static void logForFileRoaming(String level, String tag, String msg, Throwable tr)
    pub fn logForFileRoaming(&self, String level, String tag, String msg, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeFileRoamingLogToFile(createLogContent(level, tag, msg, Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void logForSharePlay(String level, String tag, String msg)
    pub fn logForSharePlay(&self, String level, String tag, String msg) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeSharePlayLogToFile(createLogContent(level, tag, msg));
         // TODO_RUST_CODE
      }
    }

    // Java: static void logForSharePlay(String level, String tag, String msg, Throwable tr)
    pub fn logForSharePlay(&self, String level, String tag, String msg, Throwable tr) {
      // Java: if (!Platform.isOverseaVersion() || Platform.isDebugLogVersion())
        // TODO_RUST_CODE
      {
         // Java: writeSharePlayLogToFile(createLogContent(level, tag, msg, Log.getStackTraceString(tr)));
         // TODO_RUST_CODE
      }
    }

    // Java: static void initKFSFileLogAapater()
    pub fn initKFSFileLogAapater(&self) {
      // Java: KFSFileLogAdapter.setIkfsFileLog(new KFSFileLogAdapter.IKFSFileLog()
        // TODO_RUST_CODE
      {
         // Java: public void eForSave(String tag, String msg, Throwable tr)
            // TODO_RUST_CODE
         {
            // Java: KSFileLog.eForSave(tag, msg, tr);
            // TODO_RUST_CODE
         }
         // Java: public void e(String tag, String msg, Throwable tr)
            // TODO_RUST_CODE
         {
            // Java: KSFileLog.e(tag, msg, tr);
            // TODO_RUST_CODE
         }
      }
    }

}
