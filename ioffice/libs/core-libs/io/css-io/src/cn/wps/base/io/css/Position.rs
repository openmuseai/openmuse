use std::collections::HashMap;

// Java field: HashMap<String, Position> sMap
// Java field: String STATIC
// Java field: String ABSOLUTE
// Java field: String RELATIVE

// Java: public enum Position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    STATIC,
    ABSOLUTE,
    RELATIVE,
}

// Java: public enum Position
impl Position {
    // Java: Position(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", Position.NAME.sMap);
        // Java: Position.NAME.sMap.put(value, this);
        match self {
            Self::STATIC => "static",
            Self::ABSOLUTE => "absolute",
            Self::RELATIVE => "relative",
        }
    }

    // Java: static Position fromValue(String v)
    pub fn from_value(v: &str) -> Option<Position> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", Position.NAME.sMap);
        // Java: return (Position)Position.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String STATIC
    pub const STATIC: &str = "static";
    // Java field: String ABSOLUTE
    pub const ABSOLUTE: &str = "absolute";
    // Java field: String RELATIVE
    pub const RELATIVE: &str = "relative";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[Position] = &[Position::STATIC, Position::ABSOLUTE, Position::RELATIVE];
