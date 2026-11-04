// Java: public enum ENMLMediaType
pub struct ENMLMediaType {
    // Java field: String mText
    mText: Option<String>,
}

// Java: public enum ENMLMediaType
impl ENMLMediaType {
    pub const JPEG: &'static str = "image/jpeg";
    pub const GIF: &'static str = "image/gif";
    pub const PNG: &'static str = "image/png";

    // Java: ENMLMediaType(String str)
    pub fn new(str: Option<&str>) -> Self {
        // Java: this.mText = str;
        ENMLMediaType { mText: str.map(|s| s.to_string()) }
    }

    // Java: static ENMLMediaType fromString(String str)
    pub fn fromString(str: Option<&str>) -> ENMLMediaType {
        // Java: if (null == str)
        if str.is_none() {
            // Java: return UNKNOWN;
            return ENMLMediaType::new(None);
        // Java: else
        } else {
            // Java: ENMLMediaType[] types = values();
            let types = [Some("image/jpeg"), Some("image/gif"), Some("image/png"), None];
            // Java: int i = 0;
            let mut i = 0;
            // Java: for(int n = types.length; i < n; ++i)
            let n = types.len();
            while i < n {
                // Java: if (str.equals(types[i].mText))
                if str == types[i] {
                    // Java: return types[i];
                    return ENMLMediaType::new(types[i]);
                }
                i += 1;
            }
            // Java: return UNKNOWN;
            return ENMLMediaType::new(None);
        }
    }

}
