// Shared SAX-like fixture for wc-03 ConvertHelper / converter tests.
// ConvertHelper reads getValue(WMap.MAIN_NAMESPACING, localName).
// Loop must accept this map (adapter or the same type ConvertHelper takes).

#![allow(dead_code)]

pub const MAIN_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
pub const MATH_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";

/// Java String.hashCode literals used by converters / ConvertHelper.
pub const HASH_LEFT: i32 = 3317767;
pub const HASH_CENTER: i32 = -1364013995;
pub const HASH_RIGHT: i32 = 108511772;
pub const HASH_BOTH: i32 = 3029889;
pub const HASH_NONE: i32 = 3387192;
pub const HASH_SINGLE: i32 = -902265784;
pub const HASH_SOLID: i32 = 109618859;
pub const HASH_SUPERSCRIPT: i32 = -1165985786;

pub const EID_B: i32 = 98;
pub const EID_I: i32 = 105;
pub const EID_U: i32 = 117;
pub const EID_SZ: i32 = 3687;
pub const EID_SZCS: i32 = 3545399;
pub const EID_COLOR: i32 = 94842723;
pub const EID_RSTYLE: i32 = -951016353;
pub const EID_RFONTS: i32 = -963181390;
pub const EID_STRIKE: i32 = -891985998;
pub const EID_DSTRIKE: i32 = 1959036182;
pub const EID_VERTALIGN: i32 = -1415507884;
pub const EID_JC: i32 = 3385;
pub const EID_PSTYLE: i32 = -1008274655;
pub const EID_IND: i32 = 104415;
pub const EID_ILVL: i32 = 3235609;
pub const EID_NUMID: i32 = 105179969;
pub const EID_SPACING: i32 = -2012158909;
pub const EID_SNAPTOGRID: i32 = -1996049205;

pub const COLOR_AUTO: i32 = -16777216;
pub const COLOR_FF0000: i32 = 16711680;

#[derive(Clone, Default)]
pub struct WordAttrs {
    pub pairs: Vec<((String, String), String)>,
}

impl WordAttrs {
    pub fn empty() -> Self {
        Self { pairs: Vec::new() }
    }

    pub fn val(v: &str) -> Self {
        Self::empty().with(MAIN_NS, "val", v)
    }

    pub fn named(local: &str, v: &str) -> Self {
        Self::empty().with(MAIN_NS, local, v)
    }

    pub fn with(mut self, ns: &str, local: &str, v: &str) -> Self {
        self.pairs
            .push(((ns.to_string(), local.to_string()), v.to_string()));
        self
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn get_value(&self, ns: &str, local: &str) -> Option<&str> {
        self.pairs
            .iter()
            .find(|((n, l), _)| n == ns && l == local)
            .map(|(_, v)| v.as_str())
    }
}
