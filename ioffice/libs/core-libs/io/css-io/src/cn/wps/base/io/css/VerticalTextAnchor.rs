use std::collections::HashMap;

// Java field: HashMap<String, VerticalTextAnchor> sMap
// Java field: String TOP
// Java field: String MIDDLE
// Java field: String BOTTOM
// Java field: String TOP_CENTER
// Java field: String MIDDLE_CENTER
// Java field: String BOTTOM_CENTER
// Java field: String TOP_BASELINE
// Java field: String BOTTOM_BASELINE
// Java field: String TOP_CENTER_BASELINE
// Java field: String BOTTOM_CENTER_BASELINE

// Java: public enum VerticalTextAnchor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerticalTextAnchor {
    TOP,
    MIDDLE,
    BOTTOM,
    TOP_CENTER,
    MIDDLE_CENTER,
    BOTTOM_CENTER,
    TOP_BASELINE,
    BOTTOM_BASELINE,
    TOP_CENTER_BASELINE,
    BOTTOM_CENTER_BASELINE,
}

// Java: public enum VerticalTextAnchor
impl VerticalTextAnchor {
    // Java: VerticalTextAnchor(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", VerticalTextAnchor.NAME.sMap);
        // Java: VerticalTextAnchor.NAME.sMap.put(value, this);
        match self {
            Self::TOP => "top",
            Self::MIDDLE => "middle",
            Self::BOTTOM => "bottom",
            Self::TOP_CENTER => "top-center",
            Self::MIDDLE_CENTER => "middle-center",
            Self::BOTTOM_CENTER => "bottom-center",
            Self::TOP_BASELINE => "top-baseline",
            Self::BOTTOM_BASELINE => "bottom-baseline",
            Self::TOP_CENTER_BASELINE => "top-center-baseline",
            Self::BOTTOM_CENTER_BASELINE => "bottom-center-baseline",
        }
    }

    // Java: static VerticalTextAnchor fromValue(String v)
    pub fn from_value(v: &str) -> Option<VerticalTextAnchor> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", VerticalTextAnchor.NAME.sMap);
        // Java: return (VerticalTextAnchor)VerticalTextAnchor.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String TOP
    pub const TOP: &str = "top";
    // Java field: String MIDDLE
    pub const MIDDLE: &str = "middle";
    // Java field: String BOTTOM
    pub const BOTTOM: &str = "bottom";
    // Java field: String TOP_CENTER
    pub const TOP_CENTER: &str = "top-center";
    // Java field: String MIDDLE_CENTER
    pub const MIDDLE_CENTER: &str = "middle-center";
    // Java field: String BOTTOM_CENTER
    pub const BOTTOM_CENTER: &str = "bottom-center";
    // Java field: String TOP_BASELINE
    pub const TOP_BASELINE: &str = "top-baseline";
    // Java field: String BOTTOM_BASELINE
    pub const BOTTOM_BASELINE: &str = "bottom-baseline";
    // Java field: String TOP_CENTER_BASELINE
    pub const TOP_CENTER_BASELINE: &str = "top-center-baseline";
    // Java field: String BOTTOM_CENTER_BASELINE
    pub const BOTTOM_CENTER_BASELINE: &str = "bottom-center-baseline";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[VerticalTextAnchor] = &[VerticalTextAnchor::TOP, VerticalTextAnchor::MIDDLE, VerticalTextAnchor::BOTTOM, VerticalTextAnchor::TOP_CENTER, VerticalTextAnchor::MIDDLE_CENTER, VerticalTextAnchor::BOTTOM_CENTER, VerticalTextAnchor::TOP_BASELINE, VerticalTextAnchor::BOTTOM_BASELINE, VerticalTextAnchor::TOP_CENTER_BASELINE, VerticalTextAnchor::BOTTOM_CENTER_BASELINE];
