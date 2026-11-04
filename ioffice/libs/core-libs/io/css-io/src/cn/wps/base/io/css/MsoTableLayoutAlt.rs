use std::collections::HashMap;

// Java field: HashMap<String, MsoTableLayoutAlt> sMap
// Java field: String Auto
// Java field: String Fixed

// Java: public enum MsoTableLayoutAlt
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MsoTableLayoutAlt {
    Auto,
    Fixed,
}

// Java: public enum MsoTableLayoutAlt
impl MsoTableLayoutAlt {
    // Java: MsoTableLayoutAlt(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", MsoTableLayoutAlt.NAME.sMap);
        // Java: MsoTableLayoutAlt.NAME.sMap.put(value, this);
        match self {
            Self::Auto => "auto",
            Self::Fixed => "fixed",
        }
    }

    // Java: static MsoTableLayoutAlt fromValue(String v)
    pub fn from_value(v: &str) -> Option<MsoTableLayoutAlt> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", MsoTableLayoutAlt.NAME.sMap);
        // Java: return (MsoTableLayoutAlt)MsoTableLayoutAlt.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String Auto
    pub const Auto: &str = "auto";
    // Java field: String Fixed
    pub const Fixed: &str = "fixed";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[MsoTableLayoutAlt] = &[MsoTableLayoutAlt::Auto, MsoTableLayoutAlt::Fixed];
