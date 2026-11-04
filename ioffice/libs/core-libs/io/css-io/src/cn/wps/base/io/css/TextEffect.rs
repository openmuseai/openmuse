use std::collections::HashMap;

// Java field: HashMap<String, TextEffect> sMap
// Java field: String TextEffectEmboss
// Java field: String TextEffectEngrave
// Java field: String TextEffectOutline
// Java field: String None

// Java: public enum TextEffect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextEffect {
    TextEffectEmboss,
    TextEffectEngrave,
    TextEffectOutline,
    None,
}

// Java: public enum TextEffect
impl TextEffect {
    // Java: TextEffect(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextEffect.NAME.sMap);
        // Java: TextEffect.NAME.sMap.put(value, this);
        match self {
            Self::TextEffectEmboss => "emboss",
            Self::TextEffectEngrave => "engrave",
            Self::TextEffectOutline => "outline",
            Self::None => "none",
        }
    }

    // Java: static TextEffect fromValue(String v)
    pub fn from_value(v: &str) -> Option<TextEffect> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextEffect.NAME.sMap);
        // Java: return (TextEffect)TextEffect.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String TextEffectEmboss
    pub const TextEffectEmboss: &str = "emboss";
    // Java field: String TextEffectEngrave
    pub const TextEffectEngrave: &str = "engrave";
    // Java field: String TextEffectOutline
    pub const TextEffectOutline: &str = "outline";
    // Java field: String None
    pub const None: &str = "none";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TextEffect] = &[TextEffect::TextEffectEmboss, TextEffect::TextEffectEngrave, TextEffect::TextEffectOutline, TextEffect::None];
