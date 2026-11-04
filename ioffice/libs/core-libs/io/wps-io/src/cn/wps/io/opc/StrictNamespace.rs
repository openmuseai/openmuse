// Java: public class StrictNamespace
pub struct StrictNamespace {
}

// Java: public class StrictNamespace
impl StrictNamespace {
    // Java field: String MAIN
    pub const MAIN: &'static str = "http://purl.oclc.org/ooxml/wordprocessingml/main";
    // Java field: String R
    pub const R: &'static str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
    // Java field: String RELATIONSHIPS
    pub const RELATIONSHIPS: &'static str = "http://purl.oclc.org/ooxml/officeDocument/relationships";

    // Java: StrictNamespace()
    pub fn new() -> Self {
        // Java: super();
        StrictNamespace {}
    }
}
