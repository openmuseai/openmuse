use std::collections::HashSet;

// Java: public class LogFilter implements ILogFilter
pub struct LogFilter {
    // Java field: FilterPatterns mDisabledCaller
    pub(crate) mDisabledCaller: FilterPatterns,
    // Java field: FilterPatterns mDisabledClass
    pub(crate) mDisabledClass: FilterPatterns,
    // Java field: FilterPatterns mDisabledTag
    pub(crate) mDisabledTag: FilterPatterns,
    // Java field: FilterPatterns mEnabledCaller
    pub(crate) mEnabledCaller: FilterPatterns,
    // Java field: FilterPatterns mEnabledClass
    pub(crate) mEnabledClass: FilterPatterns,
    // Java field: FilterPatterns mEnabledTag
    pub(crate) mEnabledTag: FilterPatterns,
    // Java field: boolean mPrintCaller
    pub(crate) mPrintCaller: bool,
}

// Java: public class LogFilter implements ILogFilter
impl LogFilter {
    // Java: LogFilter()
    pub fn new() -> Self {
        // Java: super();
        LogFilter {
            mDisabledCaller: FilterPatterns::new(),
            mDisabledClass: FilterPatterns::new(),
            mDisabledTag: FilterPatterns::new(),
            mEnabledCaller: FilterPatterns::new(),
            mEnabledClass: FilterPatterns::new(),
            mEnabledTag: FilterPatterns::new(),
            mPrintCaller: false,
        }
    }

    // Java: static boolean empty(String str)
    pub fn empty(str_val: Option<&str>) -> bool {
        // Java: return str == null || str.length() == 0;
        str_val.map_or(true, |s| s.is_empty())
    }

    // Java: void disableCaller(String callerClass, String callerMethod)
    pub fn disableCaller(&mut self, caller_class: Option<&str>, caller_method: Option<&str>) {
        // Java: if (callerClass != null)
        let caller_class = if let Some(c) = caller_class {
            // Java: callerClass = callerClass.trim();
            Some(c.trim())
        } else { None };
        // Java: if (callerMethod != null)
        let caller_method = if let Some(m) = caller_method {
            // Java: callerMethod = callerMethod.trim();
            Some(m.trim())
        } else { None };
        // Java: if (!empty(callerClass) && !empty(callerMethod))
        if !Self::empty(caller_class) && !Self::empty(caller_method) {
            // Java: this.mDisabledCaller.addPattern(callerClass + "." + callerMethod);
            self.mDisabledCaller.addPattern(Some(&format!("{}.{}", caller_class.unwrap(), caller_method.unwrap())));
        }
    }

    // Java: void disableClass(String className)
    pub fn disableClass(&mut self, class_name: Option<&str>) {
        // Java: this.mDisabledClass.addPattern(className);
        self.mDisabledClass.addPattern(class_name);
    }

    // Java: void disableTag(String tag)
    pub fn disableTag(&mut self, tag: Option<&str>) {
        // Java: this.mDisabledTag.addPattern(tag);
        self.mDisabledTag.addPattern(tag);
    }

    // Java: void enableCaller(String callerClass, String callerMethod)
    pub fn enableCaller(&mut self, caller_class: Option<&str>, caller_method: Option<&str>) {
        // Java: if (callerClass != null)
        let caller_class = if let Some(c) = caller_class {
            // Java: callerClass = callerClass.trim();
            Some(c.trim())
        } else { None };
        // Java: if (callerMethod != null)
        let caller_method = if let Some(m) = caller_method {
            // Java: callerMethod = callerMethod.trim();
            Some(m.trim())
        } else { None };
        // Java: if (!empty(callerClass) && !empty(callerMethod))
        if !Self::empty(caller_class) && !Self::empty(caller_method) {
            // Java: this.mEnabledCaller.addPattern(callerClass + "." + callerMethod);
            self.mEnabledCaller.addPattern(Some(&format!("{}.{}", caller_class.unwrap(), caller_method.unwrap())));
        }
    }

    // Java: void enableClass(String className)
    pub fn enableClass(&mut self, class_name: Option<&str>) {
        // Java: this.mEnabledClass.addPattern(className);
        self.mEnabledClass.addPattern(class_name);
    }

    // Java: void enableTag(String tag)
    pub fn enableTag(&mut self, tag: Option<&str>) {
        // Java: this.mEnabledTag.addPattern(tag);
        self.mEnabledTag.addPattern(tag);
    }

    // Java: boolean getPrintCaller()
    pub fn getPrintCaller(&self) -> bool {
        // Java: return this.mPrintCaller;
        self.mPrintCaller
    }

    // Java: boolean isEnabled(String callerClass, String callerMethod, String tag)
    pub fn isEnabled(&self, caller_class: &str, caller_method: &str, tag: &str) -> bool {
        // Java: if (this.mEnabledClass.pass(callerClass))
        if self.mEnabledClass.pass(caller_class) {
            // Java: return true;
            return true;
        // Java: else
        } else {
            // Java: String caller = callerClass + "." + callerMethod;
            let caller = format!("{}.{}", caller_class, caller_method);
            // Java: if (this.mEnabledCaller.pass(caller))
            if self.mEnabledCaller.pass(&caller) {
                // Java: return true;
                return true;
            // Java: else if (this.mEnabledTag.pass(tag))
            } else if self.mEnabledTag.pass(tag) {
                // Java: return true;
                return true;
            // Java: else if (this.mDisabledClass.pass(callerClass))
            } else if self.mDisabledClass.pass(caller_class) {
                // Java: return false;
                return false;
            // Java: else if (this.mDisabledCaller.pass(caller))
            } else if self.mDisabledCaller.pass(&caller) {
                // Java: return false;
                return false;
            // Java: else
            } else {
                // Java: return !this.mDisabledTag.pass(tag);
                return !self.mDisabledTag.pass(tag);
            }
        }
    }

    // Java: void setPrintCaller(boolean bPrintCaller)
    pub fn setPrintCaller(&mut self, b_print_caller: bool) {
        // Java: this.mPrintCaller = bPrintCaller;
        self.mPrintCaller = b_print_caller;
    }
}

// Java: public class FilterPatterns
pub struct FilterPatterns {
    // Java field: HashSet<String> mCache
    pub(crate) mCache: HashSet<String>,
    // Java field: ArrayList<Pattern> mPatterns
    pub(crate) mPatterns: Vec<String>,
}

// Java: public class FilterPatterns
impl FilterPatterns {
    // Java: FilterPatterns()
    pub fn new() -> Self {
        // Java: super();
        FilterPatterns { mCache: HashSet::new(), mPatterns: Vec::new() }
    }

    // Java: void addPattern(String pattern)
    pub fn addPattern(&mut self, pattern: Option<&str>) {
        // Java: if (pattern != null)
        let pattern = if let Some(p) = pattern {
            // Java: pattern = pattern.trim();
            Some(p.trim().to_string())
        } else { None };
        // Java: if (pattern != null && pattern.length() > 0)
        if let Some(ref p) = pattern {
            if !p.is_empty() {
                // Java: this.mPatterns.add(Pattern.compile(pattern));
                self.mPatterns.push(p.clone());
            }
        }
    }

    // Java: boolean pass(String str)
    pub fn pass(&self, str_val: &str) -> bool {
        // Java: if (this.mCache.contains(str))
        if self.mCache.contains(str_val) {
            // Java: return true;
            return true;
        // Java: else
        } else {
            // Java: for(int i = 0; i < this.mPatterns.size(); ++i)
            for i in 0..self.mPatterns.len() {
                // Java: if (((Pattern)this.mPatterns.get(i)).matcher(str).matches())
                if str_val.contains(&self.mPatterns[i]) || self.mPatterns[i] == str_val {
                    // Java: this.mCache.add(str);
                    // Java: return true;
                    return true;
                }
            }
            // Java: return false;
            return false;
        }
    }
}
