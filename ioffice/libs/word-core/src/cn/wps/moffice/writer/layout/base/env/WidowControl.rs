#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;

#[derive(Clone)]
pub struct PageLine {
    pub line: TypoTextLine,
    /// Product bridge for Java TypoPara's graph-width plane after adjustRun.
    /// Each entry is (source CP, adjusted graph advance in twips).
    pub layout_advances: Vec<(i32, i32)>,
    pub para_id: i32,
    pub widow_control: bool,
    /// A source `\f` ends the current page after this line.
    pub force_page_after: bool,
    /// Java table cell: text shift and border box for this line. None outside a table.
    pub cell: Option<CellFrame>,
    /// Java embed box from LayoutState.setDrawing: cp, content w/h, pads, ascent, border.
    pub embeds: Vec<(i32, i32, i32, i32, i32, i32, i32, i32, i32)>,
}

/// Java `TypoTableCell` left/right plus `BRCBorderTool` stroke widths, in twips.
#[derive(Clone, Copy)]
pub struct CellFrame {
    pub text_dx: i32,
    pub border_left: i32,
    pub border_right: i32,
    pub content_width: i32,
    pub row_start: i32,
    pub row_height: i32,
    pub first_row: bool,
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub top_w: i32,
    pub left_w: i32,
    pub bottom_w: i32,
    pub right_w: i32,
    pub color: u32,
}

impl PageLine {
    pub fn height(&self) -> i32 {
        // Java TextLayouter.onStage2Line and TypoObjBase page flow use the
        // committed line box (bottom - top), including line spacing.
        self.line.height.max(0)
    }
}

pub struct WidowControl;

impl WidowControl {
    /// Rebalance every page boundary. One last paragraph line on the next page
    /// pulls one predecessor forward (widow); one first paragraph line left on
    /// the previous page moves forward (orphan).
    pub fn rebalance(pages: &mut Vec<Vec<PageLine>>) {
        let mut boundary = 0usize;
        while boundary + 1 < pages.len() {
            if pages[boundary].is_empty() || pages[boundary + 1].is_empty() {
                boundary += 1;
                continue;
            }
            let para = pages[boundary].last().unwrap().para_id;
            if pages[boundary + 1][0].para_id != para
                || !pages[boundary].last().unwrap().widow_control
            {
                boundary += 1;
                continue;
            }

            let left_count = pages[boundary]
                .iter()
                .rev()
                .take_while(|line| line.para_id == para)
                .count();
            let right_count = pages[boundary + 1]
                .iter()
                .take_while(|line| line.para_id == para)
                .count();

            let move_count = if left_count == 1 {
                1
            } else if right_count == 1 && left_count > 1 {
                1
            } else {
                0
            };
            if move_count != 0 {
                let split = pages[boundary].len() - move_count;
                let mut moved = pages[boundary].split_off(split);
                moved.append(&mut pages[boundary + 1]);
                pages[boundary + 1] = moved;
            }
            boundary += 1;
        }
        pages.retain(|page| !page.is_empty());
    }
}
