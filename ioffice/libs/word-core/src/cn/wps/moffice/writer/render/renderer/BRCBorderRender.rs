#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
use crate::cn::wps::moffice::writer::layout::base::env::PageLayout::TableCellPaint;
use crate::cn::wps::moffice::writer::layout::base::tools::BRCBorderTool::BRCBorderTool;
use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::DrawLine;

/// Java BRCBorderRender.reset(TypoTableCell, Rect, boolean) and render, for
/// the solid border branch used by Sample 3. Geometry stays in layout twips.
pub struct BRCBorderRender;

impl BRCBorderRender {
    fn pad(brc: Option<&BRC>) -> i32 {
        brc.filter(|b| b.isValidTableBrc())
            .map(|b| (BRCBorderTool::getActualStrokeWidth(b.getBorderType(), b.getLineWidth()) * 20.0) as i32)
            .unwrap_or(0)
    }

    pub fn supports(cell: &TableCellPaint) -> bool {
        [&cell.brc_left, &cell.brc_top, &cell.brc_right, &cell.brc_bottom]
            .into_iter().all(|b| b.as_ref().is_none_or(|b| !b.isValidTableBrc() || b.getBorderType() == 1))
    }

    // Java: reset(cell, cellRect, rtl); render(maxVertBrcBottom, true).
    pub fn render(cell: &TableCellPaint, page: i32, page_x: i32, page_y: i32) -> Vec<DrawLine> {
        let left = cell.rect.0 + page_x;
        let top = cell.rect.1 + page_y;
        let right = cell.rect.2 + page_x;
        let bottom = cell.rect.3 + page_y;
        let pads = [Self::pad(cell.brc_left.as_ref()), Self::pad(cell.brc_top.as_ref()), Self::pad(cell.brc_right.as_ref()), Self::pad(cell.brc_bottom.as_ref())];
        // Java reset moves top/bottom by half their pads before calBorder.
        // calBorder's first rectangle then expands all four sides by half pad.
        let border_left = left - pads[0] / 2;
        let border_top = top + pads[1] / 2 - pads[1] / 2;
        let border_right = right + pads[2] / 2;
        let border_bottom = bottom - pads[3] / 2 + pads[3] / 2;
        let clip_bottom = cell.max_vert_brc_bottom.saturating_add(page_y);
        let mut out = Vec::new();
        for (side, brc) in [cell.brc_top.as_ref(), cell.brc_bottom.as_ref(), cell.brc_left.as_ref(), cell.brc_right.as_ref()].into_iter().enumerate() {
            let Some(brc) = brc.filter(|b| b.isValidTableBrc()) else { continue; };
            let width = (brc.getLineWidth() * 20.0) as i32;
            let color = brc.getColor() as u32;
            let (x1, y1, x2, y2) = match side {
                0 => (border_left, border_top + width / 2, border_right, border_top + width / 2),
                1 => (border_left, border_bottom - width / 2, border_right, border_bottom - width / 2),
                2 => (border_left + width / 2, border_top, border_left + width / 2, border_bottom.min(clip_bottom)),
                _ => (border_right - width / 2, border_top, border_right - width / 2, border_bottom.min(clip_bottom)),
            };
            if x1 == x2 && y1 == y2 { continue; }
            out.push(DrawLine { page, x1, y1, x2, y2, kind: "border".into(), color, width });
        }
        out
    }
}
