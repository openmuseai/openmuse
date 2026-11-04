use std::collections::HashMap;

// Java field: HashMap<String, TableTop> sMap
// Java field: String BOTTOM
// Java field: String INSIDE
// Java field: String MIDDLE
// Java field: String OUTSIDE
// Java field: String TOP

// Java: public enum TableTop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableTop {
    BOTTOM,
    INSIDE,
    MIDDLE,
    OUTSIDE,
    TOP,
}

// Java: public enum TableTop
impl TableTop {
    // Java: TableTop(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableTop.NAME.sMap);
        // Java: TableTop.NAME.sMap.put(value, this);
        match self {
            Self::BOTTOM => "bottom",
            Self::INSIDE => "inside",
            Self::MIDDLE => "middle",
            Self::OUTSIDE => "outside",
            Self::TOP => "top",
        }
    }

    // Java: static TableTop fromValue(String v)
    pub fn from_value(v: &str) -> Option<TableTop> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableTop.NAME.sMap);
        // Java: return (TableTop)TableTop.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String BOTTOM
    pub const BOTTOM: &str = "bottom";
    // Java field: String INSIDE
    pub const INSIDE: &str = "inside";
    // Java field: String MIDDLE
    pub const MIDDLE: &str = "middle";
    // Java field: String OUTSIDE
    pub const OUTSIDE: &str = "outside";
    // Java field: String TOP
    pub const TOP: &str = "top";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TableTop] = &[TableTop::BOTTOM, TableTop::INSIDE, TableTop::MIDDLE, TableTop::OUTSIDE, TableTop::TOP];
