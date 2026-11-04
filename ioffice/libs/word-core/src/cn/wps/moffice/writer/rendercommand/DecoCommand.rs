//! Underline / background → DrawLine / DrawRect.

use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    DrawLine, DrawRect, RenderCommand,
};

#[derive(Clone, Debug, Default)]
pub struct DecoStyle {
    pub underline: UnderlineKind,
    pub strike: bool,
    pub bg_color: Option<u32>,
    pub page: i32,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub baseline: i32,
    pub line_color: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UnderlineKind {
    #[default]
    None,
    Single,
    Double,
    Wave,
}

pub fn emit_deco(style: &DecoStyle) -> Vec<RenderCommand> {
    let mut out = Vec::new();
    if let Some(c) = style.bg_color {
        out.push(RenderCommand::Rect(DrawRect {
            page: style.page,
            x: style.x,
            y: style.y,
            w: style.w,
            h: style.h,
            color: c,
            kind: "bg".into(),
        }));
    }
    let y = style.baseline;
    match style.underline {
        UnderlineKind::None => {}
        UnderlineKind::Single => {
            out.push(line(style, style.x, y, style.x + style.w, y, "underline"));
        }
        UnderlineKind::Double => {
            out.push(line(style, style.x, y, style.x + style.w, y, "underline"));
            out.push(line(
                style,
                style.x,
                y + 20,
                style.x + style.w,
                y + 20,
                "underline",
            ));
        }
        UnderlineKind::Wave => {
            out.push(line(style, style.x, y, style.x + style.w, y, "wave"));
        }
    }
    if style.strike {
        let mid = style.y + style.h / 2;
        out.push(line(style, style.x, mid, style.x + style.w, mid, "strike"));
    }
    out
}

fn line(style: &DecoStyle, x1: i32, y1: i32, x2: i32, y2: i32, kind: &str) -> RenderCommand {
    RenderCommand::Line(DrawLine {
        page: style.page,
        x1,
        y1,
        x2,
        y2,
        kind: kind.into(),
        color: style.line_color,
        width: 15,
    })
}
