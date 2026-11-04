use std::collections::HashMap;

// Java field: HashMap<String, LineHeightRule> sMap
// Java field: String AT_LEAST
// Java field: String EXACTLY

// Java: public enum LineHeightRule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineHeightRule {
    AT_LEAST,
    EXACTLY,
}

// Java: public enum LineHeightRule
impl LineHeightRule {
    // Java: LineHeightRule(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", LineHeightRule.NAME.sMap);
        // Java: LineHeightRule.NAME.sMap.put(value, this);
        match self {
            Self::AT_LEAST => "at-least",
            Self::EXACTLY => "exactly",
        }
    }

    // Java: static LineHeightRule fromValue(String v)
    pub fn from_value(v: &str) -> Option<LineHeightRule> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", LineHeightRule.NAME.sMap);
        // Java: return (LineHeightRule)LineHeightRule.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String AT_LEAST
    pub const AT_LEAST: &str = "at-least";
    // Java field: String EXACTLY
    pub const EXACTLY: &str = "exactly";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[LineHeightRule] = &[LineHeightRule::AT_LEAST, LineHeightRule::EXACTLY];
