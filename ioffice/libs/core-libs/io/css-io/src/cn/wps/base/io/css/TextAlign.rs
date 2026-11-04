use std::collections::HashMap;

// Java field: HashMap<String, TextAlign> sMap
// Java field: String GENERAL
// Java field: String LEFT
// Java field: String RIGHT
// Java field: String CENTER
// Java field: String JUSTIFY
// Java field: String LETTER_JUSTIFY
// Java field: String STRETCH_JUSTTIFY
// Java field: String CENTER_ACROSS
// Java field: String DISTRIBUTED
// Java field: String FILL

// Java field: String mValue

// Java: public enum TextAlign
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAlign {
    GENERAL,
    LEFT,
    RIGHT,
    CENTER,
    JUSTIFY,
    LETTER_JUSTIFY,
    STRETCH_JUSTTIFY,
    CENTER_ACROSS,
    DISTRIBUTED,
    FILL,
}

// Java: public enum TextAlign
impl TextAlign {
    // Java: TextAlign(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextAlign.NAME.sMap);
        // Java: TextAlign.NAME.sMap.put(value, this);
        // Java: this.mValue = value;
        match self {
            Self::GENERAL => "general",
            Self::LEFT => "left",
            Self::RIGHT => "right",
            Self::CENTER => "center",
            Self::JUSTIFY => "justify",
            Self::LETTER_JUSTIFY => "letter-justify",
            Self::STRETCH_JUSTTIFY => "stretch-justify",
            Self::CENTER_ACROSS => "center-across",
            Self::DISTRIBUTED => "121",
            Self::FILL => "fill",
        }
    }

    // Java: String getValue()
    pub fn getValue(&self) -> &'static str {
        // Java: return this.mValue;
        self.name_str()
    }

    // Java: static TextAlign fromValue(String v)
    pub fn from_value(v: &str) -> Option<TextAlign> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextAlign.NAME.sMap);
        // Java: return (TextAlign)TextAlign.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String GENERAL
    pub const GENERAL: &str = "general";
    // Java field: String LEFT
    pub const LEFT: &str = "left";
    // Java field: String RIGHT
    pub const RIGHT: &str = "right";
    // Java field: String CENTER
    pub const CENTER: &str = "center";
    // Java field: String JUSTIFY
    pub const JUSTIFY: &str = "justify";
    // Java field: String LETTER_JUSTIFY
    pub const LETTER_JUSTIFY: &str = "letter-justify";
    // Java field: String STRETCH_JUSTTIFY
    pub const STRETCH_JUSTTIFY: &str = "stretch-justify";
    // Java field: String CENTER_ACROSS
    pub const CENTER_ACROSS: &str = "center-across";
    // Java field: String DISTRIBUTED
    pub const DISTRIBUTED: &str = "121";
    // Java field: String FILL
    pub const FILL: &str = "fill";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TextAlign] = &[TextAlign::GENERAL, TextAlign::LEFT, TextAlign::RIGHT, TextAlign::CENTER, TextAlign::JUSTIFY, TextAlign::LETTER_JUSTIFY, TextAlign::STRETCH_JUSTTIFY, TextAlign::CENTER_ACROSS, TextAlign::DISTRIBUTED, TextAlign::FILL];
