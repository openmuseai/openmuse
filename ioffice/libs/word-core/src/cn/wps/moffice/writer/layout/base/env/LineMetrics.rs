#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::layout::text::FontStack::{FontError, FontStack};

/// wr-11 I/O: hhea ascent / descent / lineGap in twip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineBox {
    pub ascent_twip: i32,
    pub descent_twip: i32,
    pub line_gap_twip: i32,
}

pub struct LineMetrics {}

impl LineMetrics {
    // java TypefaceTool.getFontMetrics — hhea scaled to twip
    pub fn lineBoxTwip(
        stack: &FontStack,
        fontName: Option<&str>,
        sizePt: f64,
    ) -> Result<LineBox, FontError> {
        let (ascent_twip, descent_twip, line_gap_twip) = stack.line_box_twip(fontName, sizePt)?;
        Ok(LineBox {
            ascent_twip,
            descent_twip,
            line_gap_twip,
        })
    }
}
