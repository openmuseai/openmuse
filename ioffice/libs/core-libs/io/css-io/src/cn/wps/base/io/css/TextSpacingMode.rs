use std::collections::HashMap;

// Java field: HashMap<String, TextSpacingMode> sMap
// Java field: String TIGHTENING
// Java field: String TRACKING

// Java: public enum TextSpacingMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextSpacingMode {
    TIGHTENING,
    TRACKING,
}

// Java: public enum TextSpacingMode
impl TextSpacingMode {
    // Java: TextSpacingMode(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextSpacingMode.NAME.sMap);
        // Java: TextSpacingMode.NAME.sMap.put(value, this);
        match self {
            Self::TIGHTENING => "tightening",
            Self::TRACKING => "tracking",
        }
    }

    // Java: static TextSpacingMode fromValue(String v)
    pub fn from_value(v: &str) -> Option<TextSpacingMode> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextSpacingMode.NAME.sMap);
        // Java: return (TextSpacingMode)TextSpacingMode.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String TIGHTENING
    pub const TIGHTENING: &str = "tightening";
    // Java field: String TRACKING
    pub const TRACKING: &str = "tracking";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TextSpacingMode] = &[TextSpacingMode::TIGHTENING, TextSpacingMode::TRACKING];
