use std::collections::HashMap;

// Java field: HashMap<String, LayoutGridMode> sMap
// Java field: String LayoutGridModeBoth
// Java field: String LayoutGridModeLine
// Java field: String LayoutGridModeChar
// Java field: String LayoutGridModeNone

// Java: public enum LayoutGridMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutGridMode {
    LayoutGridModeBoth,
    LayoutGridModeLine,
    LayoutGridModeChar,
    LayoutGridModeNone,
}

// Java: public enum LayoutGridMode
impl LayoutGridMode {
    // Java: LayoutGridMode(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", LayoutGridMode.NAME.sMap);
        // Java: LayoutGridMode.NAME.sMap.put(value, this);
        match self {
            Self::LayoutGridModeBoth => "both",
            Self::LayoutGridModeLine => "line",
            Self::LayoutGridModeChar => "char",
            Self::LayoutGridModeNone => "none",
        }
    }

    // Java: static LayoutGridMode fromValue(String v)
    pub fn from_value(v: &str) -> Option<LayoutGridMode> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", LayoutGridMode.NAME.sMap);
        // Java: return (LayoutGridMode)LayoutGridMode.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String LayoutGridModeBoth
    pub const LayoutGridModeBoth: &str = "both";
    // Java field: String LayoutGridModeLine
    pub const LayoutGridModeLine: &str = "line";
    // Java field: String LayoutGridModeChar
    pub const LayoutGridModeChar: &str = "char";
    // Java field: String LayoutGridModeNone
    pub const LayoutGridModeNone: &str = "none";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[LayoutGridMode] = &[LayoutGridMode::LayoutGridModeBoth, LayoutGridMode::LayoutGridModeLine, LayoutGridMode::LayoutGridModeChar, LayoutGridMode::LayoutGridModeNone];
