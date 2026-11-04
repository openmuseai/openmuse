use std::collections::HashMap;

// Java field: HashMap<String, PositionH> sMap
// Java field: String ABSOLUTE
// Java field: String LEFT
// Java field: String CENTER
// Java field: String RIGHT
// Java field: String INSIDE
// Java field: String OUTSIDE

// Java: public enum PositionH
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionH {
    ABSOLUTE,
    LEFT,
    CENTER,
    RIGHT,
    INSIDE,
    OUTSIDE,
}

// Java: public enum PositionH
impl PositionH {
    // Java: PositionH(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", PositionH.NAME.sMap);
        // Java: PositionH.NAME.sMap.put(value, this);
        match self {
            Self::ABSOLUTE => "absolute",
            Self::LEFT => "left",
            Self::CENTER => "center",
            Self::RIGHT => "right",
            Self::INSIDE => "inside",
            Self::OUTSIDE => "outside",
        }
    }

    // Java: static PositionH fromValue(String v)
    pub fn from_value(v: &str) -> Option<PositionH> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", PositionH.NAME.sMap);
        // Java: return (PositionH)PositionH.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String ABSOLUTE
    pub const ABSOLUTE: &str = "absolute";
    // Java field: String LEFT
    pub const LEFT: &str = "left";
    // Java field: String CENTER
    pub const CENTER: &str = "center";
    // Java field: String RIGHT
    pub const RIGHT: &str = "right";
    // Java field: String INSIDE
    pub const INSIDE: &str = "inside";
    // Java field: String OUTSIDE
    pub const OUTSIDE: &str = "outside";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[PositionH] = &[PositionH::ABSOLUTE, PositionH::LEFT, PositionH::CENTER, PositionH::RIGHT, PositionH::INSIDE, PositionH::OUTSIDE];
