use std::collections::HashMap;

// Java field: HashMap<String, FontEmphasize> sMap
// Java field: String FontEmphasizeAccent
// Java field: String FontEmphasizeDot
// Java field: String FontEmphasizeCircle
// Java field: String FontEmphasizeDisc
// Java field: String None

// Java: public enum FontEmphasize
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontEmphasize {
    FontEmphasizeAccent,
    FontEmphasizeDot,
    FontEmphasizeCircle,
    FontEmphasizeDisc,
    None,
}

// Java: public enum FontEmphasize
impl FontEmphasize {
    // Java: FontEmphasize(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", FontEmphasize.NAME.sMap);
        // Java: FontEmphasize.NAME.sMap.put(value, this);
        match self {
            Self::FontEmphasizeAccent => "accent",
            Self::FontEmphasizeDot => "dot",
            Self::FontEmphasizeCircle => "circle",
            Self::FontEmphasizeDisc => "disc",
            Self::None => "none",
        }
    }

    // Java: static FontEmphasize fromValue(String v)
    pub fn from_value(v: &str) -> Option<FontEmphasize> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", FontEmphasize.NAME.sMap);
        // Java: return (FontEmphasize)FontEmphasize.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String FontEmphasizeAccent
    pub const FontEmphasizeAccent: &str = "accent";
    // Java field: String FontEmphasizeDot
    pub const FontEmphasizeDot: &str = "dot";
    // Java field: String FontEmphasizeCircle
    pub const FontEmphasizeCircle: &str = "circle";
    // Java field: String FontEmphasizeDisc
    pub const FontEmphasizeDisc: &str = "disc";
    // Java field: String None
    pub const None: &str = "none";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[FontEmphasize] = &[FontEmphasize::FontEmphasizeAccent, FontEmphasize::FontEmphasizeDot, FontEmphasize::FontEmphasizeCircle, FontEmphasize::FontEmphasizeDisc, FontEmphasize::None];
