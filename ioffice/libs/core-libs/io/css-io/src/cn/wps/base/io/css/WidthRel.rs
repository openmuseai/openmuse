use std::collections::HashMap;

// Java field: HashMap<String, WidthRel> sMap
// Java field: String MARGIN
// Java field: String PAGE
// Java field: String LEFT_MARGIN_AREA
// Java field: String RIGHT_MARGIN_AREA
// Java field: String INNER_MARGIN_AREA
// Java field: String OUTER_MARGIN_AREA

// Java: public enum WidthRel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WidthRel {
    MARGIN,
    PAGE,
    LEFT_MARGIN_AREA,
    RIGHT_MARGIN_AREA,
    INNER_MARGIN_AREA,
    OUTER_MARGIN_AREA,
}

// Java: public enum WidthRel
impl WidthRel {
    // Java: WidthRel(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", WidthRel.NAME.sMap);
        // Java: WidthRel.NAME.sMap.put(value, this);
        match self {
            Self::MARGIN => "margin",
            Self::PAGE => "page",
            Self::LEFT_MARGIN_AREA => "left-margin-area",
            Self::RIGHT_MARGIN_AREA => "right-margin-area",
            Self::INNER_MARGIN_AREA => "inner-margin-area",
            Self::OUTER_MARGIN_AREA => "outer-margin-area",
        }
    }

    // Java: static WidthRel fromValue(String v)
    pub fn from_value(v: &str) -> Option<WidthRel> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", WidthRel.NAME.sMap);
        // Java: return (WidthRel)WidthRel.NAME.sMap.get(v);
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

const ALL_VARIANTS: &[WidthRel] = &[WidthRel::MARGIN, WidthRel::PAGE, WidthRel::LEFT_MARGIN_AREA, WidthRel::RIGHT_MARGIN_AREA, WidthRel::INNER_MARGIN_AREA, WidthRel::OUTER_MARGIN_AREA];
