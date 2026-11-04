#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::sync::Mutex;

// Java: public class KFSFileLogAdapter
#[derive(Default)]
pub struct KFSFileLogAdapter {}

// Java: interface IKFSFileLog
pub trait IKFSFileLog: Send + Sync {
    // Java: void eForSave(String var1, String var2, Throwable var3);
    fn eForSave(&self, tag: &str, msg: &str, tr: &str);
    // Java: void e(String var1, String var2, Throwable var3);
    fn e(&self, tag: &str, msg: &str, tr: &str);
}

// Java field:    static IKFSFileLog mKfsFileLog;
static M_KFS_FILE_LOG: Mutex<Option<Box<dyn IKFSFileLog>>> = Mutex::new(None);

// Java: public class KFSFileLogAdapter
impl KFSFileLogAdapter {
    // Java: public KFSFileLogAdapter()
    pub fn new() -> Self { Self {} }

    // Java: public static void eForSave(String tag, String msg, Throwable tr)
    pub fn eForSave(tag: &str, msg: &str, tr: &str) {
        // Java: if (null != mKfsFileLog)
        let lock = M_KFS_FILE_LOG.lock().unwrap();
        if let Some(ref log) = *lock {
            // Java: mKfsFileLog.eForSave(tag, msg, tr);
            log.eForSave(tag, msg, tr);
        } else {
            // Java: Log.e(tag, msg, tr);
            eprintln!("[{}] {} {}", tag, msg, tr);
        }
    }

    // Java: public static void e(String tag, String msg, Throwable tr)
    pub fn e(tag: &str, msg: &str, tr: &str) {
        // Java: if (null != mKfsFileLog)
        let lock = M_KFS_FILE_LOG.lock().unwrap();
        if let Some(ref log) = *lock {
            // Java: mKfsFileLog.e(tag, msg, tr);
            log.e(tag, msg, tr);
        } else {
            // Java: Log.e(tag, msg, tr);
            eprintln!("[{}] {} {}", tag, msg, tr);
        }
    }

    // Java: protected static void setIkfsFileLog(IKFSFileLog kfsFileLog)
    pub fn setIkfsFileLog(kfsFileLog: Box<dyn IKFSFileLog>) {
        // Java: mKfsFileLog = kfsFileLog;
        let mut lock = M_KFS_FILE_LOG.lock().unwrap();
        *lock = Some(kfsFileLog);
    }
}
