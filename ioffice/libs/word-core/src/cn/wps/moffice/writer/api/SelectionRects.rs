//! Map a half-open visible-cp range onto `DrawText.xpos` boxes.
//!
//! Java `LineSelecitonRect.setSelection(left, top, right, bottom)` — xpos
//! stand in for `LocateTool.getCharPos`; not a port of that class.

use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    DrawRect, RenderCommand,
};

use super::LayoutDocument::LayoutFrame;

pub fn rects_for_range(frame: &LayoutFrame, lo: i32, hi: i32) -> Vec<DrawRect> {
    let mut out = Vec::new();
    let mut base: i32 = 0;
    for cmd in &frame.commands {
        let RenderCommand::Text(run) = cmd else {
            continue;
        };
        let n = run.text.chars().count() as i32;
        let run_lo = lo.max(base);
        let run_hi = hi.min(base + n);
        if run_lo < run_hi {
            let i0 = (run_lo - base) as usize;
            let i1 = (run_hi - base) as usize;
            let origin = run.xpos.first().copied().unwrap_or(run.x);
            let x = run.xpos.get(i0).copied().unwrap_or(origin);
            let right = glyph_right(&run.xpos, i1, origin);
            let w = (right - x).max(0);
            let h = run.size.max(0);
            let y = run.y - h;
            out.push(DrawRect {
                page: run.page,
                x,
                y,
                w,
                h,
                color: 0,
                kind: String::new(),
            });
        }
        base += n;
    }
    out
}

fn glyph_right(xpos: &[i32], i1: usize, origin: i32) -> i32 {
    if i1 < xpos.len() {
        xpos[i1]
    } else if xpos.len() >= 2 {
        let last = xpos[xpos.len() - 1];
        last + (xpos[xpos.len() - 1] - xpos[xpos.len() - 2])
    } else if let Some(&x0) = xpos.first() {
        x0
    } else {
        origin
    }
}
