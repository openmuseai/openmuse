#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::layout::text::FontStack::{FontError, FontStack};

/// wr-11 I/O: one cmap/hmtx advance on the **named** face (no covering chain).
pub struct GlyphAdvance {}

impl GlyphAdvance {
    // java TypefaceTool.getCharWidths — one cp, strict face
    pub fn glyphAdvanceTwip(
        stack: &FontStack,
        fontName: Option<&str>,
        ch: char,
        sizePt: f64,
    ) -> Result<i32, FontError> {
        stack.glyph_advance_twip_on_face(fontName, ch, sizePt)
    }
}
