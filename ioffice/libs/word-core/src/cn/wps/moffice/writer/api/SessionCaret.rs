//! Product caret seam: `LayoutFrame` + page-space point → insertion cp.
//!
//! wr-17 `hit_test` eats Stage2 rows. Session layout emits `DrawText.xpos`.
//! This adapter builds a one-run [`Stage2Line`] and reuses [`cp_for_x`]
//! (half-advance). Java: `LocateTool` / `LayoutLocater` — not ported here.

use crate::cn::wps::moffice::writer::cache::CpHit::cp_for_x;
use crate::cn::wps::moffice::writer::cache::Stage2Rows::Stage2Line;
use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    DrawText, RenderCommand,
};

use super::LayoutDocument::LayoutFrame;

/// Insertion cp for a page-absolute twip point, or `None` if no text on `page`.
/// Cp is the visible `DrawText` scalar order (same as [`SelectionFill`](super::SelectionFill)).
pub fn cp_for_point(frame: &LayoutFrame, page: i32, x: i32, y: i32) -> Option<i32> {
    let mut best: Option<(i32, i32)> = None;
    let mut origin_cp: i32 = 0;
    for cmd in &frame.commands {
        let RenderCommand::Text(run) = cmd else {
            continue;
        };
        let n = run.text.chars().count() as i32;
        if run.page == page {
            let band = run.size.max(1);
            let dy = (y - run.y).abs();
            if dy <= band {
                let cp = origin_cp + cp_in_run(run, x);
                match best {
                    None => best = Some((dy, cp)),
                    Some((old_dy, _)) if dy < old_dy => best = Some((dy, cp)),
                    _ => {}
                }
            }
        }
        origin_cp += n;
    }
    best.map(|(_, cp)| cp)
}

/// Caret bar in page twip for a visible cp (`None` if no text).
pub fn geom_for_cp(frame: &LayoutFrame, cp: i32) -> Option<(i32, i32, i32, i32)> {
    let mut origin_cp: i32 = 0;
    for cmd in &frame.commands {
        let RenderCommand::Text(run) = cmd else {
            continue;
        };
        let n = run.text.chars().count() as i32;
        if cp >= origin_cp && cp <= origin_cp + n {
            let i = (cp - origin_cp) as usize;
            let x = if i < run.xpos.len() {
                run.xpos[i]
            } else if run.xpos.len() >= 2 {
                let last = *run.xpos.last().unwrap();
                let prev = run.xpos[run.xpos.len() - 2];
                last + (last - prev)
            } else {
                run.x
            };
            return Some((run.page, x, run.y, run.size.max(1)));
        }
        origin_cp += n;
    }
    None
}

fn cp_in_run(run: &DrawText, x: i32) -> i32 {
    let n = run.text.chars().count() as i32;
    let origin = run.xpos.first().copied().unwrap_or(run.x);
    let mut advances: Vec<i32> = Vec::new();
    if run.xpos.len() >= 2 {
        for pair in run.xpos.windows(2) {
            advances.push(pair[1] - pair[0]);
        }
    }
    let line = Stage2Line {
        page: run.page,
        line: 0,
        start_cp: 0,
        end_cp: n,
        x: origin,
        y: run.y,
        width: 0,
        height: run.size,
        xpos: Vec::new(),
    };
    cp_for_x(&line, &advances, x)
}
