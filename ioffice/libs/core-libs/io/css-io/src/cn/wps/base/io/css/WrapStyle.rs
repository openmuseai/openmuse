use std::collections::HashMap;

// Java field: HashMap<String, WrapStyle> sMap
// Java field: String SQUARE
// Java field: String NONE

// Java: public enum WrapStyle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapStyle {
    SQUARE,
    NONE,
}

// Java: public enum WrapStyle
impl WrapStyle {
    // Java: WrapStyle(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", WrapStyle.NAME.sMap);
        // Java: WrapStyle.NAME.sMap.put(value, this);
        match self {
            Self::SQUARE => "square",
            Self::NONE => "none",
        }
    }

    // Java: static WrapStyle fromValue(String v)
    pub fn from_value(v: &str) -> Option<WrapStyle> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", WrapStyle.NAME.sMap);
        // Java: return (WrapStyle)WrapStyle.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String SQUARE
    pub const SQUARE: &str = "square";
    // Java field: String NONE
    pub const NONE: &str = "none";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[WrapStyle] = &[WrapStyle::SQUARE, WrapStyle::NONE];
