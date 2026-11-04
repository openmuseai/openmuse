#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::env::PageFit::PageFit;
use crate::cn::wps::moffice::writer::layout::base::env::WidowControl::{PageLine, WidowControl};
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;

/// The cell rectangle and resolved BRC values written by TableCellPageLayouter
/// into TypoSnapshot. PageRender/TableRender consume this after the layouter
/// releases its snapshot; line text frames are not a border source in Java.
#[derive(Clone)]
pub struct TableCellPaint {
    pub row_start_cp: i32,
    pub rect: (i32, i32, i32, i32),
    pub brc_left: Option<BRC>,
    pub brc_top: Option<BRC>,
    pub brc_right: Option<BRC>,
    pub brc_bottom: Option<BRC>,
    pub is_rtl_row: bool,
    pub max_vert_brc_bottom: i32,
}

#[derive(Clone)]
pub struct PaginatedPage {
    pub page_index: i32,
    pub lines: Vec<PageLine>,
    pub table_cells: Vec<TableCellPaint>,
}

pub fn paginate(lines: Vec<PageLine>, content_height: i32) -> Vec<PaginatedPage> {
    let mut raw: Vec<Vec<PageLine>> = Vec::new();
    let mut current: Vec<PageLine> = Vec::new();
    let mut used_height = 0i32;

    for line in lines {
        if PageFit::shouldBreakBefore(
            used_height,
            line.height(),
            content_height,
            !current.is_empty(),
        ) {
            raw.push(std::mem::take(&mut current));
            used_height = 0;
        }
        used_height = used_height.saturating_add(line.height());
        let force = line.force_page_after;
        current.push(line);
        if force {
            raw.push(std::mem::take(&mut current));
            used_height = 0;
        }
    }
    if !current.is_empty() {
        raw.push(current);
    }

    WidowControl::rebalance(&mut raw);
    raw.into_iter()
        .enumerate()
        .map(|(page_index, mut page_lines)| {
            let mut y = 0i32;
            let last = page_lines.len().saturating_sub(1);
            for (index, line) in page_lines.iter_mut().enumerate() {
                line.line.setYPos(y);
                line.line.setLastLineOfPage(index == last);
                y = y.saturating_add(line.height());
            }
            PaginatedPage {
                page_index: page_index as i32,
                lines: page_lines,
                table_cells: Vec::new(),
            }
        })
        .collect()
}
