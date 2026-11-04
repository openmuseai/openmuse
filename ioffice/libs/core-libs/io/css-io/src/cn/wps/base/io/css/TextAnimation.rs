use std::collections::HashMap;

// Java field: HashMap<String, TextAnimation> sMap
// Java field: String MsoTextAnimationAntsBlack
// Java field: String MsoTextAnimationAntsRed
// Java field: String MsoTextAnimationBlinkBackground
// Java field: String MsoTextAnimationLights
// Java field: String MsoTextAnimationShimmer
// Java field: String MsoTextAnimationSparkle
// Java field: String None

// Java: public enum TextAnimation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAnimation {
    MsoTextAnimationAntsBlack,
    MsoTextAnimationAntsRed,
    MsoTextAnimationBlinkBackground,
    MsoTextAnimationLights,
    MsoTextAnimationShimmer,
    MsoTextAnimationSparkle,
    None,
}

// Java: public enum TextAnimation
impl TextAnimation {
    // Java: TextAnimation(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextAnimation.NAME.sMap);
        // Java: TextAnimation.NAME.sMap.put(value, this);
        match self {
            Self::MsoTextAnimationAntsBlack => "ants-black",
            Self::MsoTextAnimationAntsRed => "ants-red",
            Self::MsoTextAnimationBlinkBackground => "blink-background",
            Self::MsoTextAnimationLights => "lights",
            Self::MsoTextAnimationShimmer => "shimmer",
            Self::MsoTextAnimationSparkle => "sparkle",
            Self::None => "none",
        }
    }

    // Java: static TextAnimation fromValue(String v)
    pub fn from_value(v: &str) -> Option<TextAnimation> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", TextAnimation.NAME.sMap);
        // Java: return (TextAnimation)TextAnimation.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String MsoTextAnimationAntsBlack
    pub const MsoTextAnimationAntsBlack: &str = "ants-black";
    // Java field: String MsoTextAnimationAntsRed
    pub const MsoTextAnimationAntsRed: &str = "ants-red";
    // Java field: String MsoTextAnimationBlinkBackground
    pub const MsoTextAnimationBlinkBackground: &str = "blink-background";
    // Java field: String MsoTextAnimationLights
    pub const MsoTextAnimationLights: &str = "lights";
    // Java field: String MsoTextAnimationShimmer
    pub const MsoTextAnimationShimmer: &str = "shimmer";
    // Java field: String MsoTextAnimationSparkle
    pub const MsoTextAnimationSparkle: &str = "sparkle";
    // Java field: String None
    pub const None: &str = "none";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[TextAnimation] = &[TextAnimation::MsoTextAnimationAntsBlack, TextAnimation::MsoTextAnimationAntsRed, TextAnimation::MsoTextAnimationBlinkBackground, TextAnimation::MsoTextAnimationLights, TextAnimation::MsoTextAnimationShimmer, TextAnimation::MsoTextAnimationSparkle, TextAnimation::None];
