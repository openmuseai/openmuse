use std::collections::HashMap;

// Java field: HashMap<String, Vjc> sMap
// Java field: String Justified
// Java field: String Bottom
// Java field: String Center
// Java field: String Top

// Java: public enum Vjc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vjc {
    Justified,
    Bottom,
    Center,
    Top,
}

// Java: public enum Vjc
impl Vjc {
    // Java: Vjc(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", Vjc.NAME.sMap);
        // Java: Vjc.NAME.sMap.put(value, this);
        match self {
            Self::Justified => "justified",
            Self::Bottom => "bottom",
            Self::Center => "center",
            Self::Top => "top",
        }
    }

    // Java: static Vjc fromValue(String v)
    pub fn from_value(v: &str) -> Option<Vjc> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", Vjc.NAME.sMap);
        // Java: return (Vjc)Vjc.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String Justified
    pub const Justified: &str = "justified";
    // Java field: String Bottom
    pub const Bottom: &str = "bottom";
    // Java field: String Center
    pub const Center: &str = "center";
    // Java field: String Top
    pub const Top: &str = "top";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[Vjc] = &[Vjc::Justified, Vjc::Bottom, Vjc::Center, Vjc::Top];
