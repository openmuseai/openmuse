#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.base.log.Log
use crate::base::log::Log::Log;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{OnceLock, Mutex};

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: String RECYCLE
pub const RECYCLE: &str = "recycle";
// Java field: String WARN_MESSAGE
const WARN_MESSAGE: &str = "A resource was acquired at attached stack trace but never released. For example, recycle() method has NOT been called.";
// Java field: CloseGuard NOOP
static NOOP: OnceLock<CloseGuard> = OnceLock::new();
// Java field: boolean ENABLED
static ENABLED: AtomicBool = AtomicBool::new(true);
// Java field: Reporter REPORTER
static REPORTER: OnceLock<Mutex<Box<dyn Reporter + Send>>> = OnceLock::new();

// Java: public class CloseGuard
pub struct CloseGuard {
    // Java field: Throwable allocationSite
    pub(crate) allocationSite: Option<String>,
}

// Java: public interface Reporter
pub trait Reporter: Send {
    // Java: void report(String var1, Throwable var2)
    fn report(&self, var1: &str, var2: &str);
}

// Java: public class DefaultReporter
pub struct DefaultReporter;

// Java: public class DefaultReporter
impl DefaultReporter {
    // Java: DefaultReporter()
    pub fn new() -> Self {
        // Java: super();
        Self
    }
}

impl Reporter for DefaultReporter {
    // Java: void report(String message, Throwable allocationSite)
    fn report(&self, message: &str, allocationSite: &str) {
        // Java: Log.w(CloseGuard.TAG, message, allocationSite);
        Log::w(TAG.unwrap_or(""), message);
    }
}

// Java: public class CloseGuard
impl CloseGuard {
    // Java: static CloseGuard get()
    pub fn get() -> CloseGuard {
        // Java: return !ENABLED ? NOOP : new CloseGuard();
        if !ENABLED.load(Ordering::Relaxed) {
            CloseGuard { allocationSite: None }
        } else {
            CloseGuard::new()
        }
    }

    // Java: static void setEnabled(boolean enabled)
    pub fn setEnabled(enabled: bool) {
        // Java: ENABLED = enabled;
        ENABLED.store(enabled, Ordering::Relaxed);
    }

    // Java: static void setReporter(Reporter reporter)
    pub fn setReporter(reporter: Box<dyn Reporter + Send>) {
        // Java: if (reporter == null)
        // Java: throw new NullPointerException("reporter == null");
        // Java: else
        // Java: REPORTER = reporter;
        let lock = REPORTER.get_or_init(|| Mutex::new(Box::new(DefaultReporter::new())));
        let mut guard = lock.lock().unwrap();
        *guard = reporter;
    }

    // Java: static Reporter getReporter()
    pub fn getReporter() -> &'static Mutex<Box<dyn Reporter + Send>> {
        // Java: return REPORTER;
        REPORTER.get_or_init(|| Mutex::new(Box::new(DefaultReporter::new())))
    }

    // Java: CloseGuard()
    pub fn new() -> Self {
        // Java: super();
        CloseGuard { allocationSite: None }
    }

    // Java: void open(String closer)
    pub fn open(&mut self, closer: &str) {
        // Java: if (closer == null)
        // Java: throw new NullPointerException("closer == null");
        // Java: else if (this != NOOP && ENABLED)
        if ENABLED.load(Ordering::Relaxed) {
            // Java: String message = "Explicit termination method '" + closer + "' not called";
            let message = format!("Explicit termination method '{}' not called", closer);
            // Java: this.allocationSite = new Throwable(message);
            self.allocationSite = Some(message);
        }
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.allocationSite = null;
        self.allocationSite = None;
    }

    // Java: void warnIfOpen()
    pub fn warnIfOpen(&self) {
        // Java: if (this.allocationSite != null && ENABLED)
        if self.allocationSite.is_some() && ENABLED.load(Ordering::Relaxed) {
            // Java: String message = "A resource was acquired at attached stack trace but never released. For example, recycle() method has NOT been called.";
            let _message = WARN_MESSAGE;
        }
    }
}
