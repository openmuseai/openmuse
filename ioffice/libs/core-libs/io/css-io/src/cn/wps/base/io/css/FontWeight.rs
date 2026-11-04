use std::collections::HashMap;

// Java field: HashMap<String, FontWeight> sMap
// Java field: String NORMAL
// Java field: String LIGHER
// Java field: String HUNDRED
// Java field: String TWO_HUNDRED
// Java field: String THREE_HUNDRED
// Java field: String FOUR_HUNDRED
// Java field: String BOLD
// Java field: String BOLDER
// Java field: String FIVE_HUNDRED
// Java field: String SIX_HUNDRED
// Java field: String SEVEN_HUNDRED
// Java field: String EIGHT_HUNDRED
// Java field: String NINE_HUNDRED

// Java: public enum FontWeight
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
    NORMAL,
    LIGHER,
    HUNDRED,
    TWO_HUNDRED,
    THREE_HUNDRED,
    FOUR_HUNDRED,
    BOLD,
    BOLDER,
    FIVE_HUNDRED,
    SIX_HUNDRED,
    SEVEN_HUNDRED,
    EIGHT_HUNDRED,
    NINE_HUNDRED,
}

// Java: public enum FontWeight
impl FontWeight {
    // Java: FontWeight(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", FontWeight.NAME.sMap);
        // Java: FontWeight.NAME.sMap.put(value, this);
        match self {
            Self::NORMAL => "normal",
            Self::LIGHER => "lighter",
            Self::HUNDRED => "100",
            Self::TWO_HUNDRED => "200",
            Self::THREE_HUNDRED => "300",
            Self::FOUR_HUNDRED => "400",
            Self::BOLD => "bold",
            Self::BOLDER => "bolder",
            Self::FIVE_HUNDRED => "500",
            Self::SIX_HUNDRED => "600",
            Self::SEVEN_HUNDRED => "700",
            Self::EIGHT_HUNDRED => "800",
            Self::NINE_HUNDRED => "900",
        }
    }

    // Java: static FontWeight fromValue(String v)
    pub fn from_value(v: &str) -> Option<FontWeight> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", FontWeight.NAME.sMap);
        // Java: return (FontWeight)FontWeight.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String NORMAL
    pub const NORMAL: &str = "normal";
    // Java field: String LIGHER
    pub const LIGHER: &str = "lighter";
    // Java field: String HUNDRED
    pub const HUNDRED: &str = "100";
    // Java field: String TWO_HUNDRED
    pub const TWO_HUNDRED: &str = "200";
    // Java field: String THREE_HUNDRED
    pub const THREE_HUNDRED: &str = "300";
    // Java field: String FOUR_HUNDRED
    pub const FOUR_HUNDRED: &str = "400";
    // Java field: String BOLD
    pub const BOLD: &str = "bold";
    // Java field: String BOLDER
    pub const BOLDER: &str = "bolder";
    // Java field: String FIVE_HUNDRED
    pub const FIVE_HUNDRED: &str = "500";
    // Java field: String SIX_HUNDRED
    pub const SIX_HUNDRED: &str = "600";
    // Java field: String SEVEN_HUNDRED
    pub const SEVEN_HUNDRED: &str = "700";
    // Java field: String EIGHT_HUNDRED
    pub const EIGHT_HUNDRED: &str = "800";
    // Java field: String NINE_HUNDRED
    pub const NINE_HUNDRED: &str = "900";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[FontWeight] = &[FontWeight::NORMAL, FontWeight::LIGHER, FontWeight::HUNDRED, FontWeight::TWO_HUNDRED, FontWeight::THREE_HUNDRED, FontWeight::FOUR_HUNDRED, FontWeight::BOLD, FontWeight::BOLDER, FontWeight::FIVE_HUNDRED, FontWeight::SIX_HUNDRED, FontWeight::SEVEN_HUNDRED, FontWeight::EIGHT_HUNDRED, FontWeight::NINE_HUNDRED];
