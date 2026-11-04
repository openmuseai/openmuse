// Java: public class FlagEntry
pub struct FlagEntry {
    // Java field: String def
    def: Option<String>,
}

// Java: public class FlagEntry
impl FlagEntry {
    // Java: FlagEntry()
    pub fn new() -> Self {
        // Java: super();
        Self { def: None }
    }
}
