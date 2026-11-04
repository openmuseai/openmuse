use std::collections::HashMap;

// Java field: HashMap<String, TableOverlap> sMap
// Java field: String AUTO
// Java field: String NEVER

// Java: public enum TableOverlap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableOverlap {
    AUTO,
    NEVER,
}

// Java: public enum TableOverlap
impl TableOverlap {
    // Java: TableOverlap(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableOverlap.NAME.sMap);
        // Java: TableOverlap.NAME.sMap.put(value, this);
        match self {
            Self::AUTO => "auto",
            Self::NEVER => "never",
        }
    }

    // Java: static TableOverlap fromValue(String v)
    pub fn from_value(v: &str) -> Option<TableOverlap> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableOverlap.NAME.sMap);
        // Java: return (TableOverlap)TableOverlap.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String AUTO
    pub const AUTO: &str = "auto";
    // Java field: String NEVER
    pub const NEVER: &str = "never";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TableOverlap] = &[TableOverlap::AUTO, TableOverlap::NEVER];
