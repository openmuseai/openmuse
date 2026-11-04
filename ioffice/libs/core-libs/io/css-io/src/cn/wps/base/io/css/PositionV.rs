use std::collections::HashMap;

// Java field: HashMap<String, PositionV> sMap
// Java field: String ABSOLUTE
// Java field: String TOP
// Java field: String CENTER
// Java field: String BOTTOM
// Java field: String INSIDE
// Java field: String OUTSIDE

// Java: public enum PositionV
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionV {
    ABSOLUTE,
    TOP,
    CENTER,
    BOTTOM,
    INSIDE,
    OUTSIDE,
}

// Java: public enum PositionV
impl PositionV {
    // Java: PositionV(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", PositionV.NAME.sMap);
        // Java: PositionV.NAME.sMap.put(value, this);
        match self {
            Self::ABSOLUTE => "absolute",
            Self::TOP => "top",
            Self::CENTER => "center",
            Self::BOTTOM => "bottom",
            Self::INSIDE => "inside",
            Self::OUTSIDE => "outside",
        }
    }

    // Java: static PositionV fromValue(String v)
    pub fn from_value(v: &str) -> Option<PositionV> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", PositionV.NAME.sMap);
        // Java: return (PositionV)PositionV.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String ABSOLUTE
    pub const ABSOLUTE: &str = "absolute";
    // Java field: String TOP
    pub const TOP: &str = "top";
    // Java field: String CENTER
    pub const CENTER: &str = "center";
    // Java field: String BOTTOM
    pub const BOTTOM: &str = "bottom";
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

const ALL_VARIANTS: &[PositionV] = &[PositionV::ABSOLUTE, PositionV::TOP, PositionV::CENTER, PositionV::BOTTOM, PositionV::INSIDE, PositionV::OUTSIDE];
