use std::collections::HashMap;

// Java field: HashMap<String, HeightRule> sMap
// Java field: String AT_LEAST
// Java field: String EXACTLY

// Java: public enum HeightRule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeightRule {
    AT_LEAST,
    EXACTLY,
}

// Java: public enum HeightRule
impl HeightRule {
    // Java: HeightRule(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", HeightRule.NAME.sMap);
        // Java: HeightRule.NAME.sMap.put(value, this);
        match self {
            Self::AT_LEAST => "at-least",
            Self::EXACTLY => "exactly",
        }
    }

    // Java: static HeightRule fromValue(String v)
    pub fn from_value(v: &str) -> Option<HeightRule> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", HeightRule.NAME.sMap);
        // Java: return (HeightRule)HeightRule.NAME.sMap.get(v);
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

const ALL_VARIANTS: &[HeightRule] = &[HeightRule::AT_LEAST, HeightRule::EXACTLY];
