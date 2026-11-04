use std::collections::HashMap;

// Java field: HashMap<String, TextJustify> sMap
// Java field: String NONE
// Java field: String DISTRIBUTEALLLINES
// Java field: String KASHIDA
// Java field: String INTER_IDEOGRAPH

// Java: public enum TextJustify
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextJustify {
    NONE,
    DISTRIBUTEALLLINES,
    KASHIDA,
    INTER_IDEOGRAPH,
}

// Java: public enum TextJustify
impl TextJustify {
    // Java: TextJustify(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextJustify.NAME.sMap);
        // Java: TextJustify.NAME.sMap.put(value, this);
        match self {
            Self::NONE => "none",
            Self::DISTRIBUTEALLLINES => "distribute-all-lines",
            Self::KASHIDA => "kashida",
            Self::INTER_IDEOGRAPH => "inter-ideograph",
        }
    }

    // Java: static TextJustify fromValue(String v)
    pub fn from_value(v: &str) -> Option<TextJustify> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextJustify.NAME.sMap);
        // Java: return (TextJustify)TextJustify.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String NONE
    pub const NONE: &str = "none";
    // Java field: String DISTRIBUTEALLLINES
    pub const DISTRIBUTEALLLINES: &str = "distribute-all-lines";
    // Java field: String KASHIDA
    pub const KASHIDA: &str = "kashida";
    // Java field: String INTER_IDEOGRAPH
    pub const INTER_IDEOGRAPH: &str = "inter-ideograph";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TextJustify] = &[TextJustify::NONE, TextJustify::DISTRIBUTEALLLINES, TextJustify::KASHIDA, TextJustify::INTER_IDEOGRAPH];
