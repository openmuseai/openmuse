#![allow(non_snake_case)]

use super::TypoFontStyle::TypoFontStyle;

/// wr-11 I/O: pick ascii vs eastAsia family name from TypoFontStyle + char.
pub struct FaceSelect {}

impl FaceSelect {
    // java FontStyleFactory.createTypeface langType 2/3 → eastAsia; mixed run is per-cp
    pub fn selectFace(style: &TypoFontStyle, ch: char) -> &str {
        if is_east_asia(ch) {
            if !style.eastAsia.is_empty() {
                return style.eastAsia.as_str();
            }
        } else if !style.ascii.is_empty() {
            return style.ascii.as_str();
        }
        if !style.fontName.is_empty() {
            style.fontName.as_str()
        } else if !style.ascii.is_empty() {
            style.ascii.as_str()
        } else {
            style.eastAsia.as_str()
        }
    }
}

pub fn is_east_asia(ch: char) -> bool {
    matches!(ch as u32,
        0x1100..=0x11FF
            | 0x2E80..=0x2EFF
            | 0x2F00..=0x2FDF
            | 0x3000..=0x303F
            | 0x3040..=0x309F
            | 0x30A0..=0x30FF
            | 0x3100..=0x312F
            | 0x3130..=0x318F
            | 0x3190..=0x319F
            | 0x31A0..=0x31BF
            | 0x31C0..=0x31EF
            | 0x31F0..=0x31FF
            | 0x3200..=0x32FF
            | 0x3300..=0x33FF
            | 0x3400..=0x4DBF
            | 0x4E00..=0x9FFF
            | 0xA960..=0xA97F
            | 0xAC00..=0xD7AF
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE1F
            | 0xFE30..=0xFE4F
            | 0xFF00..=0xFFEF
            | 0x20000..=0x2A6DF
            | 0x2A700..=0x2B73F
            | 0x2B740..=0x2B81F
            | 0x2B820..=0x2CEAF
            | 0x2F800..=0x2FA1F
    )
}
