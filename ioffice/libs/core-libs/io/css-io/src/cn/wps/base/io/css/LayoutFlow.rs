use std::collections::HashMap;

// Java field: HashMap<String, LayoutFlow> sMap
// Java field: String HORIZONTAL
// Java field: String VERTICAL
// Java field: String VERTICAL_IDEOGRAPHIC
// Java field: String VERTICALLY
// Java field: String HORIZONTAL_IDEOGRAPHIC
// Java field: String HORIZONTALLY

// Java: public enum LayoutFlow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutFlow {
    HORIZONTAL,
    VERTICAL,
    VERTICAL_IDEOGRAPHIC,
    VERTICALLY,
    HORIZONTAL_IDEOGRAPHIC,
    HORIZONTALLY,
}

// Java: public enum LayoutFlow
impl LayoutFlow {
    // Java: LayoutFlow(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", LayoutFlow.NAME.sMap);
        // Java: LayoutFlow.NAME.sMap.put(value, this);
        match self {
            Self::HORIZONTAL => "horizontal",
            Self::VERTICAL => "vertical",
            Self::VERTICAL_IDEOGRAPHIC => "vertical-ideographic",
            Self::VERTICALLY => "vertically",
            Self::HORIZONTAL_IDEOGRAPHIC => "horizontal-ideographic",
            Self::HORIZONTALLY => "horizontally",
        }
    }

    // Java: static LayoutFlow fromValue(String v)
    pub fn from_value(v: &str) -> Option<LayoutFlow> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", LayoutFlow.NAME.sMap);
        // Java: return (LayoutFlow)LayoutFlow.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String HORIZONTAL
    pub const HORIZONTAL: &str = "horizontal";
    // Java field: String VERTICAL
    pub const VERTICAL: &str = "vertical";
    // Java field: String VERTICAL_IDEOGRAPHIC
    pub const VERTICAL_IDEOGRAPHIC: &str = "vertical-ideographic";
    // Java field: String VERTICALLY
    pub const VERTICALLY: &str = "vertically";
    // Java field: String HORIZONTAL_IDEOGRAPHIC
    pub const HORIZONTAL_IDEOGRAPHIC: &str = "horizontal-ideographic";
    // Java field: String HORIZONTALLY
    pub const HORIZONTALLY: &str = "horizontally";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[LayoutFlow] = &[LayoutFlow::HORIZONTAL, LayoutFlow::VERTICAL, LayoutFlow::VERTICAL_IDEOGRAPHIC, LayoutFlow::VERTICALLY, LayoutFlow::HORIZONTAL_IDEOGRAPHIC, LayoutFlow::HORIZONTALLY];
