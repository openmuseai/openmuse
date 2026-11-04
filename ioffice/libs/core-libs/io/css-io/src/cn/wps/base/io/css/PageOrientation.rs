use std::collections::HashMap;

// Java field: HashMap<String, PageOrientation> sMap
// Java field: String Portrait
// Java field: String Landscape

// Java: public enum PageOrientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

// Java: public enum PageOrientation
impl PageOrientation {
    // Java: PageOrientation(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", PageOrientation.NAME.sMap);
        // Java: PageOrientation.NAME.sMap.put(value, this);
        match self {
            Self::Portrait => "portrait",
            Self::Landscape => "landscape",
        }
    }

    // Java: static PageOrientation fromValue(String v)
    pub fn from_value(v: &str) -> Option<PageOrientation> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", PageOrientation.NAME.sMap);
        // Java: return (PageOrientation)PageOrientation.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String Portrait
    pub const Portrait: &str = "portrait";
    // Java field: String Landscape
    pub const Landscape: &str = "landscape";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[PageOrientation] = &[PageOrientation::Portrait, PageOrientation::Landscape];
