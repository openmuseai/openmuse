use std::collections::HashMap;

// Java field: HashMap<String, SectionBKC> sMap
// Java field: String Always
// Java field: String Auto
// Java field: String Left
// Java field: String Right

// Java: public enum SectionBKC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionBKC {
    Always,
    Auto,
    Left,
    Right,
}

// Java: public enum SectionBKC
impl SectionBKC {
    // Java: SectionBKC(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", SectionBKC.NAME.sMap);
        // Java: SectionBKC.NAME.sMap.put(value, this);
        match self {
            Self::Always => "always",
            Self::Auto => "auto",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    // Java: static SectionBKC fromValue(String v)
    pub fn from_value(v: &str) -> Option<SectionBKC> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", SectionBKC.NAME.sMap);
        // Java: return (SectionBKC)SectionBKC.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String Always
    pub const Always: &str = "always";
    // Java field: String Auto
    pub const Auto: &str = "auto";
    // Java field: String Left
    pub const Left: &str = "left";
    // Java field: String Right
    pub const Right: &str = "right";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[SectionBKC] = &[SectionBKC::Always, SectionBKC::Auto, SectionBKC::Left, SectionBKC::Right];
