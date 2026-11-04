//! Run width = Σ glyph advances in twip via FontStack / ttf-parser.
//! Java `FreeTypeJNI.getGlyphAdvances` is not linked (no .so); Backend A is ttf-parser.

use crate::cn::wps::moffice::writer::layout::base::env::FaceSelect::FaceSelect;
use crate::cn::wps::moffice::writer::layout::base::env::TypoFontStyle::TypoFontStyle;
use crate::cn::wps::moffice::writer::layout::text::FontStack::{FontError, FontStack};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunMeasure {
    pub width_twip: i32,
    pub ascent_twip: i32,
    pub descent_twip: i32,
}

/// Skip Unicode control characters; empty string → width 0.
/// Missing glyphs return `FontError::MissingGlyph` (never `width = size`).
pub fn measure_run(stack: &FontStack, text: &str, size_pt: f64) -> Result<RunMeasure, FontError> {
    let (ascent_twip, descent_twip) = stack.line_metrics_twip(size_pt)?;
    if text.is_empty() {
        return Ok(RunMeasure {
            width_twip: 0,
            ascent_twip,
            descent_twip,
        });
    }
    let mut width_twip = 0i32;
    for ch in text.chars() {
        if ch.is_control() {
            continue;
        }
        width_twip = width_twip.saturating_add(stack.glyph_advance_twip(None, ch, size_pt)?);
    }
    Ok(RunMeasure {
        width_twip,
        ascent_twip,
        descent_twip,
    })
}

/// wr-11 product: per-cp advances + line box (includes line_gap).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunMetrics {
    pub advances: Vec<i32>,
    pub width: i32,
    pub ascent: i32,
    pub descent: i32,
    pub line_gap: i32,
}

/// Measure with wr-10 TypoFontStyle. Missing cmap on the **selected** face fails.
/// Controls contribute advance 0. Bold/italic use the same injected TTF hmtx.
// java TypefaceTool.getCharWidths + getFontMetrics → RunMetrics
pub fn measure_run_metrics(
    stack: &FontStack,
    style: &TypoFontStyle,
    text: &str,
) -> Result<RunMetrics, FontError> {
    let size_pt = (style.fontSize as f64) / 20.0;
    let first_face = if text.chars().any(|c| !c.is_control()) {
        let ch = text.chars().find(|c| !c.is_control()).unwrap();
        FaceSelect::selectFace(style, ch)
    } else if !style.ascii.is_empty() {
        style.ascii.as_str()
    } else {
        style.fontName.as_str()
    };
    let (ascent, descent, line_gap) = stack.line_box_twip(Some(first_face), size_pt)?;
    if text.is_empty() {
        return Ok(RunMetrics {
            advances: Vec::new(),
            width: 0,
            ascent,
            descent,
            line_gap,
        });
    }
    let mut advances = Vec::new();
    let mut width = 0i32;
    for ch in text.chars() {
        if ch.is_control() {
            advances.push(0);
            continue;
        }
        let face = FaceSelect::selectFace(style, ch);
        let adv = stack.glyph_advance_twip_on_face(Some(face), ch, size_pt)?;
        advances.push(adv);
        width = width.saturating_add(adv);
    }
    Ok(RunMetrics {
        advances,
        width,
        ascent,
        descent,
        line_gap,
    })
}
