//! Typo run → DrawText.

use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    DrawText, RenderCommand, STYLE_BOLD,
};

#[derive(Clone, Debug)]
pub struct TextRunEmit {
    pub page: i32,
    /// Written to `DrawText.y`. P3: baseline (page-absolute twip), not line-top.
    pub line_top: i32,
    pub align_x: i32,
    pub run_x: i32,
    pub text: String,
    pub font_id: String,
    pub size: i32,
    pub color: u32,
    pub bold: bool,
    /// Page-absolute twip per visible glyph; empty in unit tests that only check x/y.
    pub xpos: Vec<i32>,
}

pub fn emit_text(run: &TextRunEmit) -> RenderCommand {
    let mut style = 0;
    if run.bold {
        style |= STYLE_BOLD;
    }
    RenderCommand::Text(DrawText {
        page: run.page,
        x: run.align_x + run.run_x,
        y: run.line_top,
        text: run.text.clone(),
        font_id: run.font_id.clone(),
        size: run.size,
        color: run.color,
        style,
        xpos: run.xpos.clone(),
    })
}
