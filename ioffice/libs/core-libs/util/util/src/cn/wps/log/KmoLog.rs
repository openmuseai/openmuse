// Java: import cn.wps.moffice.util.PlatformAdapter

use std::sync::Mutex;
use super::IKmoLog::IKmoLog;

// Java field: IKmoLog kmoLogImpl
static KMOLOG_IMPL: Mutex<Option<Box<dyn IKmoLog>>> = Mutex::new(None);

// Java: public class KmoLog
pub struct KmoLog;

// Java: public class KmoLog
impl KmoLog {
    // Java: KmoLog()
    fn new() -> Self {
        // Java: super();
        KmoLog
    }

    // Java: static void startSaveLog(String path)
    pub fn startSaveLog(path: &str) {
        // Java: if (!PlatformAdapter.isOverSeaVersion())
        {
            // Java: StringBuilder stringBuffer = new StringBuilder();
            let mut stringBuffer = String::new();
            // Java: stringBuffer.append("save log start in thread ").append(Thread.currentThread().getName()).append("path:" + path);
            stringBuffer.push_str("save log start in thread main path:");
            stringBuffer.push_str(path);
            // Java: writeLog(stringBuffer.toString());
            Self::writeLog(&stringBuffer);
        }
    }

    // Java: static void logPoint(StackTraceElement[] stackTraceElements)
    pub fn logPoint_stack(stackTraceElements: &[String]) {
        // Java: if (stackTraceElements != null)
        {
            // Java: for(StackTraceElement e : stackTraceElements)
            for e in stackTraceElements {
                // Java: logPoint(e.toString());
                Self::logPoint_msg(e);
            }
        }
    }

    // Java: static void logPoint()
    pub fn logPoint() {
        // Java: if (!PlatformAdapter.isOverSeaVersion())
        {
            // Java: StackTraceElement element = Thread.currentThread().getStackTrace()[3];
            // Java: String log = element.getClassName() + ":" + element.getMethodName() + "-" + element.getLineNumber();
            let log_str = "logPoint";
            // Java: writeLog(log);
            Self::writeLog(log_str);
        }
    }

    // Java: static void logPoint(String msg)
    pub fn logPoint_msg(msg: &str) {
        // Java: if (!PlatformAdapter.isOverSeaVersion())
        {
            // Java: StackTraceElement element = Thread.currentThread().getStackTrace()[3];
            // Java: String log = element.getClassName() + ":" + element.getMethodName() + "-" + element.getLineNumber();
            let log_str = format!("logPoint-{}", msg);
            // Java: writeLog(log + "-" + msg);
            Self::writeLog(&log_str);
        }
    }

    // Java: static void e(String tag, String msg, Throwable thr)
    pub fn e(tag: &str, msg: &str, thr: Option<&dyn std::error::Error>) {
        // Java: if (!PlatformAdapter.isOverSeaVersion())
        {
            // Java: if (kmoLogImpl != null)
            if let Ok(guard) = KMOLOG_IMPL.lock() {
                if let Some(ref impl_) = *guard {
                    // Java: kmoLogImpl.e(tag, msg, thr);
                    impl_.e(tag, msg, thr);
                    return;
                }
            }
            // Java: else
            {
                // Java: String log = tag + ":" + msg + "- throwable:" + (thr == null ? "null." : thr.getMessage());
                let thr_msg = thr.map(|t| t.to_string()).unwrap_or_else(|| "null.".to_string());
                let log_str = format!("{}:{}- throwable:{}", tag, msg, thr_msg);
                // Java: System.out.println(tag);
                println!("{}", log_str);
            }
        }
    }

    // Java: static void writeLog(String s)
    pub fn writeLog(s: &str) {
        // Java: if (!PlatformAdapter.isOverSeaVersion())
        {
            // Java: if (kmoLogImpl != null)
            if let Ok(guard) = KMOLOG_IMPL.lock() {
                if let Some(ref impl_) = *guard {
                    // Java: kmoLogImpl.log(s);
                    impl_.log(s);
                    return;
                }
            }
            // Java: else
            {
                // Java: System.out.print(s);
                print!("{}", s);
            }
        }
    }

    // Java: static void writeLog(String tag, String s)
    pub fn writeLog_tag(tag: &str, s: &str) {
        // Java: if (kmoLogImpl != null)
        if let Ok(guard) = KMOLOG_IMPL.lock() {
            if let Some(ref impl_) = *guard {
                // Java: kmoLogImpl.log(tag, s);
                impl_.log_tag(tag, s);
                return;
            }
        }
        // Java: else
        {
            // Java: System.out.print(tag + " " + s);
            print!("{} {}", tag, s);
        }
    }

    // Java: static void endSaveLog()
    pub fn endSaveLog() {
        // Java: writeLog("Save end");
        Self::writeLog("Save end");
    }

    // Java: static void install(IKmoLog iKmoLog)
    pub fn install(iKmoLog: Box<dyn IKmoLog>) {
        // Java: kmoLogImpl = iKmoLog;
        if let Ok(mut guard) = KMOLOG_IMPL.lock() {
            *guard = Some(iKmoLog);
        }
    }
}
