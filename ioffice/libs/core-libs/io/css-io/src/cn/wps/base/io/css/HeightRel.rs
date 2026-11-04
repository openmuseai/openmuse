use std::collections::HashMap;

// Java field: HashMap<String, HeightRel> sMap
// Java field: String MARGIN
// Java field: String PAGE
// Java field: String TOP_MARGIN_AREA
// Java field: String BOTTOM_MARGIN_AREA
// Java field: String INNER_MARGIN_AREA
// Java field: String OUTER_MARGIN_AREA

// Java: public enum HeightRel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeightRel {
    MARGIN,
    PAGE,
    TOP_MARGIN_AREA,
    BOTTOM_MARGIN_AREA,
    INNER_MARGIN_AREA,
    OUTER_MARGIN_AREA,
}

// Java: public enum HeightRel
impl HeightRel {
    // Java: HeightRel(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", HeightRel.NAME.sMap);
        // Java: HeightRel.NAME.sMap.put(value, this);
        match self {
            Self::MARGIN => "margin",
            Self::PAGE => "page",
            Self::TOP_MARGIN_AREA => "top-margin-area",
            Self::BOTTOM_MARGIN_AREA => "bottom-margin-area",
            Self::INNER_MARGIN_AREA => "inner-margin-area",
            Self::OUTER_MARGIN_AREA => "outer-margin-area",
        }
    }

    // Java: static HeightRel fromValue(String v)
    pub fn from_value(v: &str) -> Option<HeightRel> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", HeightRel.NAME.sMap);
        // Java: return (HeightRel)HeightRel.NAME.sMap.get(v);
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
    // Java field: String TOP_MARGIN_AREA
    pub const TOP_MARGIN_AREA: &str = "top-margin-area";
    // Java field: String BOTTOM_MARGIN_AREA
    pub const BOTTOM_MARGIN_AREA: &str = "bottom-margin-area";
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

const ALL_VARIANTS: &[HeightRel] = &[HeightRel::MARGIN, HeightRel::PAGE, HeightRel::TOP_MARGIN_AREA, HeightRel::BOTTOM_MARGIN_AREA, HeightRel::INNER_MARGIN_AREA, HeightRel::OUTER_MARGIN_AREA];
