use std::collections::HashMap;

// Java field: HashMap<String, TextTransfrom> sMap
// Java field: String UPPERCASE
// Java field: String NONE

// Java: public enum TextTransfrom
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextTransfrom {
    UPPERCASE,
    NONE,
}

// Java: public enum TextTransfrom
impl TextTransfrom {
    // Java: TextTransfrom(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextTransfrom.NAME.sMap);
        // Java: TextTransfrom.NAME.sMap.put(value, this);
        match self {
            Self::UPPERCASE => "uppercase",
            Self::NONE => "none",
        }
    }

    // Java: static TextTransfrom fromValue(String v)
    pub fn from_value(v: &str) -> Option<TextTransfrom> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextTransfrom.NAME.sMap);
        // Java: return (TextTransfrom)TextTransfrom.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String UPPERCASE
    pub const UPPERCASE: &str = "uppercase";
    // Java field: String NONE
    pub const NONE: &str = "none";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TextTransfrom] = &[TextTransfrom::UPPERCASE, TextTransfrom::NONE];
