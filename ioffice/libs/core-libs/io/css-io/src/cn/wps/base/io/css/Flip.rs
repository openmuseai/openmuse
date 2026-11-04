use std::collections::HashMap;

// Java field: HashMap<String, Flip> sMap
// Java field: String X
// Java field: String Y
// Java field: String XY
// Java field: String YX

// Java: public enum Flip
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flip {
    X,
    Y,
    XY,
    YX,
}

// Java: public enum Flip
impl Flip {
    // Java: Flip(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", Flip.NAME.sMap);
        // Java: Flip.NAME.sMap.put(value, this);
        match self {
            Self::X => "x",
            Self::Y => "y",
            Self::XY => "xy",
            Self::YX => "yx",
        }
    }

    // Java: static Flip fromValue(String v)
    pub fn from_value(v: &str) -> Option<Flip> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", Flip.NAME.sMap);
        // Java: return (Flip)Flip.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String X
    pub const X: &str = "x";
    // Java field: String Y
    pub const Y: &str = "y";
    // Java field: String XY
    pub const XY: &str = "xy";
    // Java field: String YX
    pub const YX: &str = "yx";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[Flip] = &[Flip::X, Flip::Y, Flip::XY, Flip::YX];
