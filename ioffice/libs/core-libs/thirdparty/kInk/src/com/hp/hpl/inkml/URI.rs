// Java: public class URI
pub struct URI {
    // Java field: String uriString
    pub(crate) uriString: String,
}

// Java: public class URI
impl URI {
    // Java: URI(String uriString)
    pub fn URI(uriString: &str) -> Self {
        // Java: super();
        // Java: this.uriString = uriString;
        Self { uriString: uriString.to_string() }
    }

    // Java: String getURIString()
    pub fn getURIString(&self) -> &str {
        // Java: return this.uriString;
        &self.uriString
    }
}
