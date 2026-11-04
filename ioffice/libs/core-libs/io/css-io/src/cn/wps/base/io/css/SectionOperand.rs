use std::collections::HashMap;

// Java field: HashMap<String, SectionOperand> sMap
// Java field: String Continuous
// Java field: String NewPage
// Java field: String NewSection

// Java: public enum SectionOperand
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionOperand {
    Continuous,
    NewPage,
    NewSection,
}

// Java: public enum SectionOperand
impl SectionOperand {
    // Java: SectionOperand(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", SectionOperand.NAME.sMap);
        // Java: SectionOperand.NAME.sMap.put(value, this);
        match self {
            Self::Continuous => "continuous",
            Self::NewPage => "newPage",
            Self::NewSection => "newSection",
        }
    }

    // Java: static SectionOperand fromValue(String v)
    pub fn from_value(v: &str) -> Option<SectionOperand> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", SectionOperand.NAME.sMap);
        // Java: return (SectionOperand)SectionOperand.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String Continuous
    pub const Continuous: &str = "continuous";
    // Java field: String NewPage
    pub const NewPage: &str = "newPage";
    // Java field: String NewSection
    pub const NewSection: &str = "newSection";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[SectionOperand] = &[SectionOperand::Continuous, SectionOperand::NewPage, SectionOperand::NewSection];
