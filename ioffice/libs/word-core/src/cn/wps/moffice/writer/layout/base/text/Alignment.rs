//! Paragraph jc → line x offset. Justify MVP = left.

pub const JC_LEFT: i32 = 0;
pub const JC_CENTER: i32 = 1;
pub const JC_RIGHT: i32 = 2;
pub const JC_JUSTIFY: i32 = 3;

pub fn align_x(jc: i32, line_width: i32, content_width: i32) -> i32 {
    if line_width >= content_width {
        return 0;
    }
    match jc {
        JC_CENTER => (content_width - line_width) / 2,
        JC_RIGHT => content_width - line_width,
        JC_JUSTIFY | JC_LEFT | _ => 0,
    }
}
