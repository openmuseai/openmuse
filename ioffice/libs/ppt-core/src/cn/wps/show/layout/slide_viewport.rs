//! Slide content rect (white-background viewport) in render (POINT) units.
//! Java sources: `PptxrPresentation.DEFAULT_SLDSIZE_*`, `UnitsConverter.render().fromEmuX/Y`.

use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;

// Java field: int DEFAULT_SLDSIZE_CX (PptxrPresentation)
pub const DEFAULT_SLDSIZE_CX: i32 = 12192000;
// Java field: int DEFAULT_SLDSIZE_CY (PptxrPresentation)
pub const DEFAULT_SLDSIZE_CY: i32 = 6858000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SlideContentRect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewportError {
    NonPositiveSize,
}

/// `None` uses `PptxrPresentation` default sldSz. `Some((cx,cy))` with `cx<=0` or `cy<=0` is rejected.
pub fn slide_viewport(size: Option<(i32, i32)>) -> Result<SlideContentRect, ViewportError> {
    let (cx, cy) = match size {
        None => (DEFAULT_SLDSIZE_CX, DEFAULT_SLDSIZE_CY),
        Some((cx, cy)) => {
            if cx <= 0 || cy <= 0 {
                return Err(ViewportError::NonPositiveSize);
            }
            (cx, cy)
        }
    };
    let render = UnitsConverter::render();
    Ok(SlideContentRect {
        left: 0.0,
        top: 0.0,
        width: render.fromEmuX(cx as f32),
        height: render.fromEmuY(cy as f32),
    })
}
