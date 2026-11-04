use std::collections::HashMap;

// Java field: HashMap<String, PositionHRel> sMap
// Java field: String MARGIN
// Java field: String PAGE
// Java field: String TEXT
// Java field: String CHAR
// Java field: String LEFT_MARGIN_AREA
// Java field: String RIGHT_MARGIN_AREA
// Java field: String INNER_MARGIN_AREA
// Java field: String OUTER_MARGIN_AREA

// Java: public enum PositionHRel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionHRel {
    MARGIN,
    PAGE,
    TEXT,
    CHAR,
    LEFT_MARGIN_AREA,
    RIGHT_MARGIN_AREA,
    INNER_MARGIN_AREA,
    OUTER_MARGIN_AREA,
}

// Java: public enum PositionHRel
impl PositionHRel {
    // Java: PositionHRel(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", PositionHRel.NAME.sMap);
        // Java: PositionHRel.NAME.sMap.put(value, this);
        match self {
            Self::MARGIN => "margin",
            Self::PAGE => "page",
            Self::TEXT => "text",
            Self::CHAR => "char",
            Self::LEFT_MARGIN_AREA => "left-margin-area",
            Self::RIGHT_MARGIN_AREA => "right-margin-area",
            Self::INNER_MARGIN_AREA => "inner-margin-area",
            Self::OUTER_MARGIN_AREA => "outer-margin-area",
        }
    }

    // Java: static PositionHRel fromValue(String v)
    pub fn from_value(v: &str) -> Option<PositionHRel> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", PositionHRel.NAME.sMap);
        // Java: return (PositionHRel)PositionHRel.NAME.sMap.get(v);
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
    // Java field: String TEXT
    pub const TEXT: &str = "text";
    // Java field: String CHAR
    pub const CHAR: &str = "char";
    // Java field: String LEFT_MARGIN_AREA
    pub const LEFT_MARGIN_AREA: &str = "left-margin-area";
    // Java field: String RIGHT_MARGIN_AREA
    pub const RIGHT_MARGIN_AREA: &str = "right-margin-area";
    // Java field: String INNER_MARGIN_AREA
    pub const INNER_MARGIN_AREA: &str = "inner-margin-area";
    // Java field: String OUTER_MARGIN_AREA
    pub const OUTER_MARGIN_AREA: &str = "outer-margin-area";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[PositionHRel] = &[PositionHRel::MARGIN, PositionHRel::PAGE, PositionHRel::TEXT, PositionHRel::CHAR, PositionHRel::LEFT_MARGIN_AREA, PositionHRel::RIGHT_MARGIN_AREA, PositionHRel::INNER_MARGIN_AREA, PositionHRel::OUTER_MARGIN_AREA];
