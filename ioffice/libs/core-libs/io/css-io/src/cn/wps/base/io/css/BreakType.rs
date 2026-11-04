use std::collections::HashMap;

// Java field: HashMap<String, BreakType> sMap
// Java field: String Auto
// Java field: String SECTION_BREAK

// Java: public enum BreakType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BreakType {
    Auto,
    SectionBreak,
}

// Java: public enum BreakType
impl BreakType {
    // Java: BreakType(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", BreakType.NAME.sMap);
        // Java: BreakType.NAME.sMap.put(value, this);
        match self {
            Self::Auto => "auto",
            Self::SectionBreak => "section-break",
        }
    }

    // Java: static BreakType fromValue(String v)
    pub fn from_value(v: &str) -> Option<BreakType> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", BreakType.NAME.sMap);
        // Java: return (BreakType)BreakType.NAME.sMap.get(v);
        ALL_BREAK_TYPES.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    pub const Auto: &str = "auto";
    pub const SECTION_BREAK: &str = "section-break";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_BREAK_TYPES: &[BreakType] = &[BreakType::Auto, BreakType::SectionBreak];
