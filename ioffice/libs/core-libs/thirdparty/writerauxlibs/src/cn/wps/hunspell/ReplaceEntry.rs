// Java: public class ReplaceEntry
pub struct ReplaceEntry {
    // Java field: String pattern
    pattern: Option<String>,
    // Java field: String pattern2
    pattern2: Option<String>,
    // Java field: boolean start
    start: bool,
    // Java field: boolean end
    end: bool,
}

// Java: public class ReplaceEntry
impl ReplaceEntry {
    // Java: ReplaceEntry()
    pub fn new() -> Self {
        // Java: super();
        Self {
            pattern: None,
            pattern2: None,
            start: false,
            end: false,
        }
    }
}
