#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::env::PageLayout::TableCellPaint;
use crate::cn::wps::moffice::writer::render::renderer::BRCBorderRender::BRCBorderRender;
use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::RenderCommand;

/// Java TableRender.drawTableRow consumes the committed TypoTableCell plane.
pub struct TableRender;

impl TableRender {
    pub fn drawTableRow(cells: &[TableCellPaint], page: i32, page_x: i32, page_y: i32, commands: &mut Vec<RenderCommand>) {
        for cell in cells {
            commands.extend(BRCBorderRender::render(cell, page, page_x, page_y).into_iter().map(RenderCommand::Line));
        }
    }

    pub fn supports(cells: &[TableCellPaint]) -> bool {
        cells.iter().all(BRCBorderRender::supports)
    }
}
