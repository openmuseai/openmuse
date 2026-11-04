use std::collections::HashMap;

// Java field: HashMap<String, TextDecoration> sMap
// Java field: String NONE
// Java field: String UNDERLINE
// Java field: String OVERLINE
// Java field: String LINE_THROUGH
// Java field: String BLINK
// Java field: String LINE_THROUGH_UNDERLINE

// Java: public enum TextDecoration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDecoration {
    NONE,
    UNDERLINE,
    OVERLINE,
    LINE_THROUGH,
    BLINK,
    LINE_THROUGH_UNDERLINE,
}

// Java: public enum TextDecoration
impl TextDecoration {
    // Java: TextDecoration(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextDecoration.NAME.sMap);
        // Java: TextDecoration.NAME.sMap.put(value, this);
        match self {
            Self::NONE => "none",
            Self::UNDERLINE => "underline",
            Self::OVERLINE => "overline",
            Self::LINE_THROUGH => "line-through",
            Self::BLINK => "blink",
            Self::LINE_THROUGH_UNDERLINE => "line-through underline",
        }
    }

    // Java: static TextDecoration fromValue(String v)
    pub fn from_value(v: &str) -> Option<TextDecoration> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextDecoration.NAME.sMap);
        // Java: return (TextDecoration)TextDecoration.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String NONE
    pub const NONE: &str = "none";
    // Java field: String UNDERLINE
    pub const UNDERLINE: &str = "underline";
    // Java field: String OVERLINE
    pub const OVERLINE: &str = "overline";
    // Java field: String LINE_THROUGH
    pub const LINE_THROUGH: &str = "line-through";
    // Java field: String BLINK
    pub const BLINK: &str = "blink";
    // Java field: String LINE_THROUGH_UNDERLINE
    pub const LINE_THROUGH_UNDERLINE: &str = "line-through underline";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TextDecoration] = &[TextDecoration::NONE, TextDecoration::UNDERLINE, TextDecoration::OVERLINE, TextDecoration::LINE_THROUGH, TextDecoration::BLINK, TextDecoration::LINE_THROUGH_UNDERLINE];
