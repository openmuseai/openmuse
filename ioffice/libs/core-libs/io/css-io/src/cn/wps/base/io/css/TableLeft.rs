use std::collections::HashMap;

// Java field: HashMap<String, TableLeft> sMap
// Java field: String CENTER
// Java field: String INSIDE
// Java field: String LEFT
// Java field: String OUTSIDE
// Java field: String RIGHT

// Java: public enum TableLeft
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableLeft {
    CENTER,
    INSIDE,
    LEFT,
    OUTSIDE,
    RIGHT,
}

// Java: public enum TableLeft
impl TableLeft {
    // Java: TableLeft(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableLeft.NAME.sMap);
        // Java: TableLeft.NAME.sMap.put(value, this);
        match self {
            Self::CENTER => "center",
            Self::INSIDE => "inside",
            Self::LEFT => "left",
            Self::OUTSIDE => "outside",
            Self::RIGHT => "right",
        }
    }

    // Java: static TableLeft fromValue(String v)
    pub fn from_value(v: &str) -> Option<TableLeft> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableLeft.NAME.sMap);
        // Java: return (TableLeft)TableLeft.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String CENTER
    pub const CENTER: &str = "center";
    // Java field: String INSIDE
    pub const INSIDE: &str = "inside";
    // Java field: String LEFT
    pub const LEFT: &str = "left";
    // Java field: String OUTSIDE
    pub const OUTSIDE: &str = "outside";
    // Java field: String RIGHT
    pub const RIGHT: &str = "right";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TableLeft] = &[TableLeft::CENTER, TableLeft::INSIDE, TableLeft::LEFT, TableLeft::OUTSIDE, TableLeft::RIGHT];
