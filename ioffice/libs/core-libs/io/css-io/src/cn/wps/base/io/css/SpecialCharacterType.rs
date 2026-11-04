use std::collections::HashMap;

// Java field: HashMap<String, SpecialCharacterType> sMap
// Java field: String COMMENT
// Java field: String FOOTNOTE
// Java field: String FOOTNOTE_CONTINUATION_SEPARATOR
// Java field: String FOOTNOTE_SEPARATOR
// Java field: String LINE_BREAK
// Java field: String NONE

// Java: public enum SpecialCharacterType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialCharacterType {
    COMMENT,
    FOOTNOTE,
    FOOTNOTE_CONTINUATION_SEPARATOR,
    FOOTNOTE_SEPARATOR,
    LINE_BREAK,
    NONE,
}

// Java: public enum SpecialCharacterType
impl SpecialCharacterType {
    // Java: SpecialCharacterType(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", SpecialCharacterType.NAME.sMap);
        // Java: SpecialCharacterType.NAME.sMap.put(value, this);
        match self {
            Self::COMMENT => "comment",
            Self::FOOTNOTE => "footnote",
            Self::FOOTNOTE_CONTINUATION_SEPARATOR => "footnote-continuation-separator",
            Self::FOOTNOTE_SEPARATOR => "footnote-separator",
            Self::LINE_BREAK => "line-break",
            Self::NONE => "none",
        }
    }

    // Java: static SpecialCharacterType fromValue(String v)
    pub fn from_value(v: &str) -> Option<SpecialCharacterType> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", SpecialCharacterType.NAME.sMap);
        // Java: return (SpecialCharacterType)SpecialCharacterType.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String COMMENT
    pub const COMMENT: &str = "comment";
    // Java field: String FOOTNOTE
    pub const FOOTNOTE: &str = "footnote";
    // Java field: String FOOTNOTE_CONTINUATION_SEPARATOR
    pub const FOOTNOTE_CONTINUATION_SEPARATOR: &str = "footnote-continuation-separator";
    // Java field: String FOOTNOTE_SEPARATOR
    pub const FOOTNOTE_SEPARATOR: &str = "footnote-separator";
    // Java field: String LINE_BREAK
    pub const LINE_BREAK: &str = "line-break";
    // Java field: String NONE
    pub const NONE: &str = "none";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[SpecialCharacterType] = &[SpecialCharacterType::COMMENT, SpecialCharacterType::FOOTNOTE, SpecialCharacterType::FOOTNOTE_CONTINUATION_SEPARATOR, SpecialCharacterType::FOOTNOTE_SEPARATOR, SpecialCharacterType::LINE_BREAK, SpecialCharacterType::NONE];
