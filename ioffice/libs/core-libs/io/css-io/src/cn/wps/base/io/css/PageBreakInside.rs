use std::collections::HashMap;

// Java field: HashMap<String, PageBreakInside> sMap
// Java field: String AVOID

// Java: public enum PageBreakInside
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageBreakInside {
    AVOID,
}

// Java: public enum PageBreakInside
impl PageBreakInside {
    // Java: PageBreakInside(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", PageBreakInside.NAME.sMap);
        // Java: PageBreakInside.NAME.sMap.put(value, this);
        match self {
            Self::AVOID => "avoid",
        }
    }

    // Java: static PageBreakInside fromValue(String v)
    pub fn from_value(v: &str) -> Option<PageBreakInside> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", PageBreakInside.NAME.sMap);
        // Java: return (PageBreakInside)PageBreakInside.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String AVOID
    pub const AVOID: &str = "avoid";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[PageBreakInside] = &[PageBreakInside::AVOID];
