use std::collections::HashMap;

// Java field: HashMap<String, MsoLayoutFlowAlt> sMap
// Java field: String TOP_TO_BOTTOM
// Java field: String BOTTOM_TO_TOP

// Java: public enum MsoLayoutFlowAlt
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MsoLayoutFlowAlt {
    TOP_TO_BOTTOM,
    BOTTOM_TO_TOP,
}

// Java: public enum MsoLayoutFlowAlt
impl MsoLayoutFlowAlt {
    // Java: MsoLayoutFlowAlt(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", MsoLayoutFlowAlt.NAME.sMap);
        // Java: MsoLayoutFlowAlt.NAME.sMap.put(value, this);
        match self {
            Self::TOP_TO_BOTTOM => "top-to-bottom",
            Self::BOTTOM_TO_TOP => "bottom-to-top",
        }
    }

    // Java: static MsoLayoutFlowAlt fromValue(String v)
    pub fn from_value(v: &str) -> Option<MsoLayoutFlowAlt> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", MsoLayoutFlowAlt.NAME.sMap);
        // Java: return (MsoLayoutFlowAlt)MsoLayoutFlowAlt.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String TOP_TO_BOTTOM
    pub const TOP_TO_BOTTOM: &str = "top-to-bottom";
    // Java field: String BOTTOM_TO_TOP
    pub const BOTTOM_TO_TOP: &str = "bottom-to-top";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[MsoLayoutFlowAlt] = &[MsoLayoutFlowAlt::TOP_TO_BOTTOM, MsoLayoutFlowAlt::BOTTOM_TO_TOP];
