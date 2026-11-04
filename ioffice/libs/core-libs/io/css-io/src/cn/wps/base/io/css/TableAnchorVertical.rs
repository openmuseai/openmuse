use std::collections::HashMap;

// Java field: HashMap<String, TableAnchorVertical> sMap
// Java field: String MARGIN
// Java field: String PAGE
// Java field: String PARAGRAPH

// Java: public enum TableAnchorVertical
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableAnchorVertical {
    MARGIN,
    PAGE,
    PARAGRAPH,
}

// Java: public enum TableAnchorVertical
impl TableAnchorVertical {
    // Java: TableAnchorVertical(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableAnchorVertical.NAME.sMap);
        // Java: TableAnchorVertical.NAME.sMap.put(value, this);
        match self {
            Self::MARGIN => "margin",
            Self::PAGE => "page",
            Self::PARAGRAPH => "paragraph",
        }
    }

    // Java: static TableAnchorVertical fromValue(String v)
    pub fn from_value(v: &str) -> Option<TableAnchorVertical> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TableAnchorVertical.NAME.sMap);
        // Java: return (TableAnchorVertical)TableAnchorVertical.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String MARGIN
    pub const MARGIN: &str = "margin";
    // Java field: String PAGE
    pub const PAGE: &str = "page";
    // Java field: String PARAGRAPH
    pub const PARAGRAPH: &str = "paragraph";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TableAnchorVertical] = &[TableAnchorVertical::MARGIN, TableAnchorVertical::PAGE, TableAnchorVertical::PARAGRAPH];
