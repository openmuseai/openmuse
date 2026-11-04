use std::collections::HashMap;

// Java field: HashMap<String, VerticalAlign> sMap
// Java field: String BASELINE
// Java field: String SUB
// Java field: String SUPER
// Java field: String TOP
// Java field: String MIDDLE
// Java field: String BOTTOM
// Java field: String DISTRIBUTED
// Java field: String JUSTIFY

// Java field: String mValue

// Java: public enum VerticalAlign
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerticalAlign {
    BASELINE,
    SUB,
    SUPER,
    TOP,
    MIDDLE,
    BOTTOM,
    DISTRIBUTED,
    JUSTIFY,
}

// Java: public enum VerticalAlign
impl VerticalAlign {
    // Java: VerticalAlign(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", VerticalAlign.NAME.sMap);
        // Java: VerticalAlign.NAME.sMap.put(value, this);
        // Java: this.mValue = value;
        match self {
            Self::BASELINE => "baseline",
            Self::SUB => "sub",
            Self::SUPER => "super",
            Self::TOP => "top",
            Self::MIDDLE => "middle",
            Self::BOTTOM => "bottom",
            Self::DISTRIBUTED => "121",
            Self::JUSTIFY => "justify",
        }
    }

    // Java: String getValue()
    pub fn getValue(&self) -> &'static str {
        // Java: return this.mValue;
        self.name_str()
    }

    // Java: static VerticalAlign fromValue(String v)
    pub fn from_value(v: &str) -> Option<VerticalAlign> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", VerticalAlign.NAME.sMap);
        // Java: return (VerticalAlign)VerticalAlign.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String BASELINE
    pub const BASELINE: &str = "baseline";
    // Java field: String SUB
    pub const SUB: &str = "sub";
    // Java field: String SUPER
    pub const SUPER: &str = "super";
    // Java field: String TOP
    pub const TOP: &str = "top";
    // Java field: String MIDDLE
    pub const MIDDLE: &str = "middle";
    // Java field: String BOTTOM
    pub const BOTTOM: &str = "bottom";
    // Java field: String DISTRIBUTED
    pub const DISTRIBUTED: &str = "121";
    // Java field: String JUSTIFY
    pub const JUSTIFY: &str = "justify";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[VerticalAlign] = &[VerticalAlign::BASELINE, VerticalAlign::SUB, VerticalAlign::SUPER, VerticalAlign::TOP, VerticalAlign::MIDDLE, VerticalAlign::BOTTOM, VerticalAlign::DISTRIBUTED, VerticalAlign::JUSTIFY];
