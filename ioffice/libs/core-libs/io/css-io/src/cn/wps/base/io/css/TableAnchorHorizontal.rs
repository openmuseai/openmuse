use std::collections::HashMap;

// Java field: HashMap<String, TableAnchorHorizontal> sMap
// Java field: String COLUMN
// Java field: String PAGE
// Java field: String MARGIN

// Java: public enum TableAnchorHorizontal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableAnchorHorizontal {
    COLUMN,
    PAGE,
    MARGIN,
}

// Java: public enum TableAnchorHorizontal
impl TableAnchorHorizontal {
    // Java: TableAnchorHorizontal(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableAnchorHorizontal.NAME.sMap);
        // Java: TableAnchorHorizontal.NAME.sMap.put(value, this);
        match self {
            Self::COLUMN => "column",
            Self::PAGE => "page",
            Self::MARGIN => "margin",
        }
    }

    // Java: static TableAnchorHorizontal fromValue(String v)
    pub fn from_value(v: &str) -> Option<TableAnchorHorizontal> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableAnchorHorizontal.NAME.sMap);
        // Java: return (TableAnchorHorizontal)TableAnchorHorizontal.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String COLUMN
    pub const COLUMN: &str = "column";
    // Java field: String PAGE
    pub const PAGE: &str = "page";
    // Java field: String MARGIN
    pub const MARGIN: &str = "margin";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TableAnchorHorizontal] = &[TableAnchorHorizontal::COLUMN, TableAnchorHorizontal::PAGE, TableAnchorHorizontal::MARGIN];
