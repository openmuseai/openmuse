// Java: import cn.wps.base.assertion.Assert
// Java: import java.io.IOException
// Java: import java.io.PrintWriter
// Java: import java.io.StringWriter
// Java: import java.lang.reflect.Field
// Java: import java.lang.reflect.InvocationTargetException
// Java: import java.lang.reflect.Method

// Java: public class Log
pub struct Log;

// Java: public class Log
impl Log {
    // Java field: Method sPrintlnAndroid
    // Java field: Method sPrintlnSystem
    // Java field: Object sSystemOut
    // Java field: Method sWaitForDebugger
    // Java field: int VERBOSE
    pub const VERBOSE: i32 = 2;
    // Java field: int DEBUG
    pub const DEBUG: i32 = 3;
    // Java field: int INFO
    pub const INFO: i32 = 4;
    // Java field: int WARN
    pub const WARN: i32 = 5;
    // Java field: int ERROR
    pub const ERROR: i32 = 6;
    // Java field: int ASSERT
    pub const ASSERT: i32 = 7;
    // Java field: int MAX_ENABLED_LOG_LEVEL

    // Java: Log()
    pub fn new() -> Self {
        // Java: super();
        Log
    }

    // Java: static boolean initialize()
    fn initialize() -> bool {
        // Java: if (initAndroidLog())
        // Java: return true;
        // Java: else if (initSystemOut())
        // Java: return true;
        // Java: else
        // Java: Assert.fail("It should not reach to here.");
        // Java: return false;
        true
    }

    // Java: static boolean isAndroid()
    fn isAndroid() -> bool {
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

    // Java: static boolean initAndroidLog()
    fn initAndroidLog() -> bool {
        // Java: try
        {
            // Java: Class<?> log = Class.forName("android.util.Log");
            // Java: if (log != null)
            {
                // Java: sPrintlnAndroid = log.getDeclaredMethod("println", Integer.TYPE, String.class, String.class);
            }
            // Java: return true;
        }
        // Java: catch (ClassNotFoundException var1)
        {
        }
        // Java: catch (SecurityException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (NoSuchMethodException var3)
        {
        }
        // Java: catch (IllegalArgumentException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: return false;
        false
    }

    // Java: static boolean initSystemOut()
    fn initSystemOut() -> bool {
        // Java: try
        {
            // Java: Class<?> system = Class.forName("java.lang.System");
            // Java: if (system == null)
            {
                // Java: return false;
            }
            // Java: Field fs = system.getField("out");
            // Java: if (fs == null)
            {
                // Java: return false;
            }
            // Java: sSystemOut = fs.get(null);
            // Java: Class<?> printStream = Class.forName("java.io.PrintStream");
            // Java: if (printStream == null)
            {
                // Java: return false;
            }
            // Java: sPrintlnSystem = printStream.getDeclaredMethod("println", String.class);
            // Java: return true;
        }
        // Java: catch (ClassNotFoundException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (SecurityException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (NoSuchFieldException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (NoSuchMethodException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (IllegalArgumentException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (IllegalAccessException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: return false;
        true
    }

    // Java: static boolean initAndroidDebug()
    fn initAndroidDebug() -> bool {
        // Java: try
        {
            // Java: Class<?> debug = Class.forName("android.os.Debug");
            // Java: if (debug != null)
            {
                // Java: sWaitForDebugger = debug.getDeclaredMethod("waitForDebugger");
            }
            // Java: return true;
        }
        // Java: catch (ClassNotFoundException var1)
        {
        }
        // Java: catch (SecurityException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (NoSuchMethodException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (IllegalArgumentException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: return false;
        false
    }

    // Java: static void setMaxEnabledLogLevel(int level)
    pub fn setMaxEnabledLogLevel(_level: i32) {
        // Java: MAX_ENABLED_LOG_LEVEL = level;
    }

    // Java: static boolean isLoggable(String tag, int priority)
    fn isLoggable(_tag: &str, _priority: i32) -> bool {
        // Java: return true;
        true
    }

    // Java: static void v(String tag, String msg)
    pub fn v(_tag: &str, _msg: &str) {
    }

    // Java: static void v(String tag, String msg, Throwable tr)
    pub fn v_tr(_tag: &str, _msg: &str, _tr: &dyn std::error::Error) {
    }

    // Java: static void d(String tag, String msg)
    pub fn d(_tag: &str, _msg: &str) {
    }

    // Java: static void d(String tag, String msg, Throwable tr)
    pub fn d_tr(_tag: &str, _msg: &str, _tr: &dyn std::error::Error) {
    }

    // Java: static void i(String tag, String msg)
    pub fn i(_tag: &str, _msg: &str) {
    }

    // Java: static void i(String tag, String msg, Throwable tr)
    pub fn i_tr(_tag: &str, _msg: &str, _tr: &dyn std::error::Error) {
    }

    // Java: static void w(String tag, String msg)
    pub fn w(tag: &str, msg: &str) {
        // Java: printLog(5, tag, msg);
        Self::printLog(5, tag, msg);
    }

    // Java: static void w(String tag, String msg, Throwable tr)
    pub fn w_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: printLog(5, tag, msg + '\n' + getStackTraceString(tr));
        let full = format!("{}\n{}", msg, Self::getStackTraceString(Some(tr)));
        Self::printLog(5, tag, &full);
    }

    // Java: static void e(String tag, String msg)
    pub fn e(tag: &str, msg: &str) {
        // Java: printLog(6, tag, msg);
        Self::printLog(6, tag, msg);
    }

    // Java: static void e(String tag, String msg, Throwable tr)
    pub fn e_tr(tag: &str, msg: &str, tr: &dyn std::error::Error) {
        // Java: printLog(6, tag, msg + '\n' + getStackTraceString(tr));
        let full = format!("{}\n{}", msg, Self::getStackTraceString(Some(tr)));
        Self::printLog(6, tag, &full);
    }

    // Java: static String getStackTraceString(Throwable tr)
    pub fn getStackTraceString(tr: Option<&dyn std::error::Error>) -> String {
        // Java: if (tr == null)
        if tr.is_none() {
            // Java: return "";
            return String::new();
        // Java: else
        }
        // Java: StringWriter sw = new StringWriter();
        // Java: for(PrintWriter pw = new PrintWriter(sw); tr != null; tr = tr.getCause())
        // Java: try
        {
            // Java: tr.printStackTrace(pw);
        }
        // Java: catch (Exception var4)
        {
        }
        // Java: String result = sw.toString();
        // Java: return result == null ? "" : result;
        format!("{}", tr.unwrap())
    }

    // Java: static String getNoSystemStackTraceString(Throwable tr)
    pub fn getNoSystemStackTraceString(tr: Option<&dyn std::error::Error>) -> String {
        // Java: if (tr == null)
        if tr.is_none() {
            // Java: return "";
            return String::new();
        // Java: else
        }
        // Java: StringWriter sw = new StringWriter();
        // Java: PrintWriter pw = new PrintWriter(sw);
        // Java: String result = "";
        // Java: try
        {
            // Java: StackTraceElement[] stackTrace = tr.getStackTrace();
            // Java: pw.println(tr);
            // Java: for(StackTraceElement stackTraceElement : stackTrace)
            {
                // Java: String className = stackTraceElement.getClassName().toLowerCase();
                // Java: if (className.startsWith("android.") || className.startsWith("java."))
                {
                    // Java: break;
                }
                // Java: pw.println("\tat " + stackTraceElement);
            }
            // Java: result = sw.toString();
        }
        // Java: catch (Exception e)
        {
            // Java: e.printStackTrace();
        }
        // Java: finally
        {
            // Java: try
            {
                // Java: sw.close();
                // Java: pw.close();
            }
            // Java: catch (IOException e)
            {
                // Java: e.printStackTrace();
            }
        }
        // Java: return result;
        format!("{}", tr.unwrap())
    }

    // Java: static boolean printLog(int priority, String tag, String msg)
    fn printLog(priority: i32, tag: &str, msg: &str) -> bool {
        // Java: if (null == tag)
        {
            // Java: tag = "";
        }
        // Java: if (!isLoggable(tag, priority))
        {
            // Java: return false;
        }
        // Java: else if (sPrintlnAndroid != null)
        {
            // Java: return printLog_android_log(priority, tag, msg);
        }
        // Java: else if (sPrintlnSystem != null && sSystemOut != null)
        {
            // Java: return printLog_system_out(priority, tag, msg);
        }
        // Java: else
        {
            // Java: Assert.fail("It should not reach to here.");
            // Java: return false;
        }
        eprintln!("{}: [{}]: {}", priority, tag, msg);
        true
    }

    // Java: static boolean printLog_android_log(int priority, String tag, String msg)
    fn printLog_android_log(_priority: i32, _tag: &str, _msg: &str) -> bool {
        // Java: try
        {
            // Java: sPrintlnAndroid.invoke(null, priority, tag, msg);
            // Java: return true;
        }
        // Java: catch (SecurityException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (IllegalArgumentException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (IllegalAccessException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (InvocationTargetException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: return false;
        false
    }

    // Java: static boolean printLog_system_out(int priority, String tag, String msg)
    fn printLog_system_out(priority: i32, tag: &str, msg: &str) -> bool {
        // Java: try
        {
            // Java: sPrintlnSystem.invoke(sSystemOut, priority + ": [" + tag + "]: " + msg);
            // Java: return true;
        }
        // Java: catch (SecurityException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (IllegalArgumentException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (IllegalAccessException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: catch (InvocationTargetException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: return false;
        eprintln!("{}: [{}]: {}", priority, tag, msg);
        true
    }

    // Java: static void waitForDebugger()
    pub fn waitForDebugger() {
    }
}
