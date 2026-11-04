#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::text::Alignment;

/// wr-12 I/O: PAPX 193 jc + line width + content width → x from content left.
///
/// Java: `AlignLeftHandler` / `AlignCenterHandler` / `AlignRightHandler.adjustLine`.
/// `AlignBothHandler` is off the rewrite list → same as left.
/// Formula matches wr-04 `align_x`; do not stretch justify.

pub const JC_LEFT: i32 = Alignment::JC_LEFT;
pub const JC_CENTER: i32 = Alignment::JC_CENTER;
pub const JC_RIGHT: i32 = Alignment::JC_RIGHT;
pub const JC_JUSTIFY: i32 = Alignment::JC_JUSTIFY;

pub struct LineAlign {}

impl LineAlign {
    // java if (lineWidth >= layoutRight - layoutLeft) run.setLeft(layoutLeft);
    // java AlignCenterHandler: setLeft(layoutLeft + (contentWidth - lineWidth) / 2);
    // java AlignRightHandler: setLeft(layoutRight - lineWidth);
    // java AlignLeftHandler / AlignBothHandler (stub): setLeft(layoutLeft);
    pub fn alignX(jc: i32, line_width: i32, content_width: i32) -> i32 {
        Alignment::align_x(jc, line_width, content_width)
    }
}
