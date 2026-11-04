use std::collections::HashMap;

// Java field: HashMap<String, FontVariant> sMap
// Java field: String NORMAL
// Java field: String SMALLCAPS
// Java field: String NONE

// Java: public enum FontVariant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontVariant {
    NORMAL,
    SMALLCAPS,
    NONE,
}

// Java: public enum FontVariant
impl FontVariant {
    // Java: FontVariant(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", FontVariant.NAME.sMap);
        // Java: FontVariant.NAME.sMap.put(value, this);
        match self {
            Self::NORMAL => "normal",
            Self::SMALLCAPS => "small-caps",
            Self::NONE => "none",
        }
    }

    // Java: static FontVariant fromValue(String v)
    pub fn from_value(v: &str) -> Option<FontVariant> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", FontVariant.NAME.sMap);
        // Java: return (FontVariant)FontVariant.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String NORMAL
    pub const NORMAL: &str = "normal";
    // Java field: String SMALLCAPS
    pub const SMALLCAPS: &str = "small-caps";
    // Java field: String NONE
    pub const NONE: &str = "none";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[FontVariant] = &[FontVariant::NORMAL, FontVariant::SMALLCAPS, FontVariant::NONE];
