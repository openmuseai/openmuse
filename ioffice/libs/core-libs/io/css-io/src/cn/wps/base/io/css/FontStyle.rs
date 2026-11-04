use std::collections::HashMap;

// Java field: HashMap<String, FontStyle> sMap
// Java field: String NORMAL
// Java field: String ITALIC
// Java field: String OBLIQUE

// Java: public enum FontStyle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
    NORMAL,
    ITALIC,
    OBLIQUE,
}

// Java: public enum FontStyle
impl FontStyle {
    // Java: FontStyle(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", FontStyle.NAME.sMap);
        // Java: FontStyle.NAME.sMap.put(value, this);
        match self {
            Self::NORMAL => "normal",
            Self::ITALIC => "italic",
            Self::OBLIQUE => "oblique",
        }
    }

    // Java: static FontStyle fromValue(String v)
    pub fn from_value(v: &str) -> Option<FontStyle> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", FontStyle.NAME.sMap);
        // Java: return (FontStyle)FontStyle.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String NORMAL
    pub const NORMAL: &str = "normal";
    // Java field: String ITALIC
    pub const ITALIC: &str = "italic";
    // Java field: String OBLIQUE
    pub const OBLIQUE: &str = "oblique";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[FontStyle] = &[FontStyle::NORMAL, FontStyle::ITALIC, FontStyle::OBLIQUE];
