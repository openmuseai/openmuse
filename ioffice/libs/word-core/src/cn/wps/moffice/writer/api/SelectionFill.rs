//! Emit selection `DrawRect` commands (`kind = "selection"`).

use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    DrawRect, RenderCommand,
};

use super::LayoutDocument::LayoutFrame;
use super::SelectionRange;
use super::SelectionRects;

pub const KIND_SELECTION: &str = "selection";
/// ARGB, ~30% alpha system-selection blue (product MVP).
pub const SELECTION_COLOR: u32 = 0x4D2B7DE9;

pub fn emit(rects: &[DrawRect]) -> Vec<RenderCommand> {
    rects
        .iter()
        .map(|r| {
            RenderCommand::Rect(DrawRect {
                page: r.page,
                x: r.x,
                y: r.y,
                w: r.w,
                h: r.h,
                color: SELECTION_COLOR,
                kind: KIND_SELECTION.into(),
            })
        })
        .collect()
}

pub fn overlay(frame: &LayoutFrame, start: i32, end: i32) -> Vec<RenderCommand> {
    let len: i32 = frame
        .commands
        .iter()
        .filter_map(|c| match c {
            RenderCommand::Text(t) => Some(t.text.chars().count() as i32),
            _ => None,
        })
        .sum();
    let Some((lo, hi)) = SelectionRange::clamp(start, end, len) else {
        return frame.commands.clone();
    };
    let rects = SelectionRects::rects_for_range(frame, lo, hi);
    let mut out = emit(&rects);
    out.extend(frame.commands.iter().cloned());
    out
}
