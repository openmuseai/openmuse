use std::collections::HashMap;

// Java field: HashMap<String, TableCondition> sMap
// Java field: String FIRST_ROW
// Java field: String LAST_ROW
// Java field: String FIRST_COL
// Java field: String LAST_COL
// Java field: String ODD_COLUMN
// Java field: String EVEN_COLUMN
// Java field: String ODD_ROW
// Java field: String EVEN_ROW
// Java field: String NE_CELL
// Java field: String NW_CELL
// Java field: String SE_CELL
// Java field: String SW_CELL

// Java: public enum TableCondition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableCondition {
    FIRST_ROW,
    LAST_ROW,
    FIRST_COL,
    LAST_COL,
    ODD_COLUMN,
    EVEN_COLUMN,
    ODD_ROW,
    EVEN_ROW,
    NE_CELL,
    NW_CELL,
    SE_CELL,
    SW_CELL,
}

// Java: public enum TableCondition
impl TableCondition {
    // Java: TableCondition(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableCondition.NAME.sMap);
        // Java: TableCondition.NAME.sMap.put(value, this);
        match self {
            Self::FIRST_ROW => "first-row",
            Self::LAST_ROW => "last-row",
            Self::FIRST_COL => "first-column",
            Self::LAST_COL => "last-column",
            Self::ODD_COLUMN => "odd-column",
            Self::EVEN_COLUMN => "even-column",
            Self::ODD_ROW => "odd-row",
            Self::EVEN_ROW => "even-row",
            Self::NE_CELL => "ne-cell",
            Self::NW_CELL => "nw-cell",
            Self::SE_CELL => "se-cell",
            Self::SW_CELL => "sw-cell",
        }
    }

    // Java: static TableCondition fromValue(String v)
    pub fn from_value(v: &str) -> Option<TableCondition> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableCondition.NAME.sMap);
        // Java: return (TableCondition)TableCondition.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String FIRST_ROW
    pub const FIRST_ROW: &str = "first-row";
    // Java field: String LAST_ROW
    pub const LAST_ROW: &str = "last-row";
    // Java field: String FIRST_COL
    pub const FIRST_COL: &str = "first-column";
    // Java field: String LAST_COL
    pub const LAST_COL: &str = "last-column";
    // Java field: String ODD_COLUMN
    pub const ODD_COLUMN: &str = "odd-column";
    // Java field: String EVEN_COLUMN
    pub const EVEN_COLUMN: &str = "even-column";
    // Java field: String ODD_ROW
    pub const ODD_ROW: &str = "odd-row";
    // Java field: String EVEN_ROW
    pub const EVEN_ROW: &str = "even-row";
    // Java field: String NE_CELL
    pub const NE_CELL: &str = "ne-cell";
    // Java field: String NW_CELL
    pub const NW_CELL: &str = "nw-cell";
    // Java field: String SE_CELL
    pub const SE_CELL: &str = "se-cell";
    // Java field: String SW_CELL
    pub const SW_CELL: &str = "sw-cell";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TableCondition] = &[TableCondition::FIRST_ROW, TableCondition::LAST_ROW, TableCondition::FIRST_COL, TableCondition::LAST_COL, TableCondition::ODD_COLUMN, TableCondition::EVEN_COLUMN, TableCondition::ODD_ROW, TableCondition::EVEN_ROW, TableCondition::NE_CELL, TableCondition::NW_CELL, TableCondition::SE_CELL, TableCondition::SW_CELL];
