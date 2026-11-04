use std::collections::HashMap;

// Java field: HashMap<String, Visibility> sMap
// Java field: String VISIBLE
// Java field: String HIDDEN
// Java field: String INHERIT

// Java: public enum Visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Visibility {
    VISIBLE,
    HIDDEN,
    INHERIT,
}

// Java: public enum Visibility
impl Visibility {
    // Java: Visibility(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", Visibility.NAME.sMap);
        // Java: Visibility.NAME.sMap.put(value, this);
        match self {
            Self::VISIBLE => "visible",
            Self::HIDDEN => "hidden",
            Self::INHERIT => "inherit",
        }
    }

    // Java: static Visibility fromValue(String v)
    pub fn from_value(v: &str) -> Option<Visibility> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", Visibility.NAME.sMap);
        // Java: return (Visibility)Visibility.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String VISIBLE
    pub const VISIBLE: &str = "visible";
    // Java field: String HIDDEN
    pub const HIDDEN: &str = "hidden";
    // Java field: String INHERIT
    pub const INHERIT: &str = "inherit";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[Visibility] = &[Visibility::VISIBLE, Visibility::HIDDEN, Visibility::INHERIT];
