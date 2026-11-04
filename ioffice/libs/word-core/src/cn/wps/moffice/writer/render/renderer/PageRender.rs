#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::env::PageLayout::PaginatedPage;
use crate::cn::wps::moffice::writer::render::renderer::TableRender::TableRender;
use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::RenderCommand;

pub struct PageRender;

impl PageRender {
    // Java PageRender.drawLines dispatches type-5 rows to TableRender.
    pub fn drawLines(page: &PaginatedPage, page_x: i32, page_y: i32, commands: &mut Vec<RenderCommand>) {
        let mut index = 0;
        while index < page.table_cells.len() {
            let cp = page.table_cells[index].row_start_cp;
            let end = page.table_cells[index..].iter().position(|c| c.row_start_cp != cp).map_or(page.table_cells.len(), |n| index + n);
            TableRender::drawTableRow(&page.table_cells[index..end], page.page_index, page_x, page_y, commands);
            index = end;
        }
    }
}
