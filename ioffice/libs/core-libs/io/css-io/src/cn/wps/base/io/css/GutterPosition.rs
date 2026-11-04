use std::collections::HashMap;

// Java field: HashMap<String, GutterPosition> sMap
// Java field: String SIDE
// Java field: String TOP

// Java: public enum GutterPosition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GutterPosition {
    SIDE,
    TOP,
}

// Java: public enum GutterPosition
impl GutterPosition {
    // Java: GutterPosition(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", GutterPosition.NAME.sMap);
        // Java: GutterPosition.NAME.sMap.put(value, this);
        match self {
            Self::SIDE => "side",
            Self::TOP => "top",
        }
    }

    // Java: static GutterPosition fromValue(String v)
    pub fn from_value(v: &str) -> Option<GutterPosition> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", GutterPosition.NAME.sMap);
        // Java: return (GutterPosition)GutterPosition.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String SIDE
    pub const SIDE: &str = "side";
    // Java field: String TOP
    pub const TOP: &str = "top";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[GutterPosition] = &[GutterPosition::SIDE, GutterPosition::TOP];
