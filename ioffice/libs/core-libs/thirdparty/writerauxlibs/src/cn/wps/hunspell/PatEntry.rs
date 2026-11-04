// Java: public class PatEntry
pub struct PatEntry {
    // Java field: String pattern
    pattern: Option<String>,
    // Java field: String pattern2
    pattern2: Option<String>,
    // Java field: String pattern3
    pattern3: Option<String>,
    // Java field: char cond
    cond: char,
    // Java field: char cond2
    cond2: char,
}

// Java: public class PatEntry
impl PatEntry {
    // Java: PatEntry()
    pub fn new() -> Self {
        // Java: super();
        Self {
            pattern: None,
            pattern2: None,
            pattern3: None,
            cond: '\0',
            cond2: '\0',
        }
    }
}
