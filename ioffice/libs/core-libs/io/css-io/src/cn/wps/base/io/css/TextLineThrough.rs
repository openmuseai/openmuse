use std::collections::HashMap;

// Java field: HashMap<String, TextLineThrough> sMap
// Java field: String Double
// Java field: String Single
// Java field: String None

// Java: public enum TextLineThrough
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextLineThrough {
    Double,
    Single,
    None,
}

// Java: public enum TextLineThrough
impl TextLineThrough {
    // Java: TextLineThrough(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextLineThrough.NAME.sMap);
        // Java: TextLineThrough.NAME.sMap.put(value, this);
        match self {
            Self::Double => "double",
            Self::Single => "single",
            Self::None => "none",
        }
    }

    // Java: static TextLineThrough fromValue(String v)
    pub fn from_value(v: &str) -> Option<TextLineThrough> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextLineThrough.NAME.sMap);
        // Java: return (TextLineThrough)TextLineThrough.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String Double
    pub const Double: &str = "double";
    // Java field: String Single
    pub const Single: &str = "single";
    // Java field: String None
    pub const None: &str = "none";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TextLineThrough] = &[TextLineThrough::Double, TextLineThrough::Single, TextLineThrough::None];
