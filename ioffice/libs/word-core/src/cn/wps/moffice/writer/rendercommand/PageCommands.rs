use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    BeginPage, CommandDocument, DrawRect, EndPage, RenderCommand,
};
use crate::cn::wps::moffice::writer::rendercommand::StyleCommands::{emit_styled_line, StyledLine};

#[derive(Clone, Debug)]
pub struct StyledPage {
    pub page: i32,
    pub width: i32,
    pub height: i32,
    pub background: u32,
    pub lines: Vec<StyledLine>,
}

pub fn emit_pages(pages: &[StyledPage]) -> CommandDocument {
    let mut commands = Vec::new();
    for page in pages {
        commands.push(RenderCommand::BeginPage(BeginPage {
            page: page.page,
            width: page.width,
            height: page.height,
        }));
        commands.push(RenderCommand::Rect(DrawRect {
            page: page.page,
            x: 0,
            y: 0,
            w: page.width,
            h: page.height,
            color: page.background,
            kind: "page-background".to_string(),
        }));
        for line in &page.lines {
            commands.extend(emit_styled_line(line));
        }
        commands.push(RenderCommand::EndPage(EndPage { page: page.page }));
    }
    CommandDocument { version: 2, commands }
}
