#![allow(non_snake_case, non_upper_case_globals, dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};

// Java field: boolean isPrintModeEnabled
static IS_PRINT_MODE_ENABLED: AtomicBool = AtomicBool::new(false);
// Java field: boolean isSwitchEnabled
static IS_SWITCH_ENABLED: AtomicBool = AtomicBool::new(false);
// Java field: boolean isLimitSample
static IS_LIMIT_SAMPLE: AtomicBool = AtomicBool::new(false);

// Java: public class ExportContext
pub struct ExportContext {
}

// Java: public class ExportContext
impl ExportContext {
    // Java: ExportContext()
    pub fn new() -> Self {
        // Java: super();
        ExportContext {}
    }

    // Java: static void enablePrintMode()
    pub fn enablePrintMode() {
        // Java: isPrintModeEnabled = true;
        IS_PRINT_MODE_ENABLED.store(true, Ordering::SeqCst);
    }

    // Java: static void disablePrintMode()
    pub fn disablePrintMode() {
        // Java: isPrintModeEnabled = false;
        IS_PRINT_MODE_ENABLED.store(false, Ordering::SeqCst);
    }

    // Java: static boolean isPrintModeEnabled()
    pub fn isPrintModeEnabled() -> bool {
        // Java: return isPrintModeEnabled;
        IS_PRINT_MODE_ENABLED.load(Ordering::SeqCst)
    }

    // Java: static void enableSwitch()
    pub fn enableSwitch() {
        // Java: isSwitchEnabled = true;
        IS_SWITCH_ENABLED.store(true, Ordering::SeqCst);
    }

    // Java: static void disableSwitch()
    pub fn disableSwitch() {
        // Java: isSwitchEnabled = false;
        IS_SWITCH_ENABLED.store(false, Ordering::SeqCst);
    }

    // Java: static boolean isSwitchEnabled()
    pub fn isSwitchEnabled() -> bool {
        // Java: return isSwitchEnabled;
        IS_SWITCH_ENABLED.load(Ordering::SeqCst)
    }

    // Java: static void enableLimitSample()
    pub fn enableLimitSample() {
        // Java: isLimitSample = true;
        IS_LIMIT_SAMPLE.store(true, Ordering::SeqCst);
    }

    // Java: static void disableLimitSample()
    pub fn disableLimitSample() {
        // Java: isLimitSample = false;
        IS_LIMIT_SAMPLE.store(false, Ordering::SeqCst);
    }

    // Java: static boolean isLimitSample()
    pub fn isLimitSample() -> bool {
        // Java: return isLimitSample;
        IS_LIMIT_SAMPLE.load(Ordering::SeqCst)
    }
}
