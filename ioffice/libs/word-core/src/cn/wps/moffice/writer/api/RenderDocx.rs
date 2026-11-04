//! Compatibility: `render_docx` is `layout_document` + Stage3 `CommandDocument`.

use crate::cn::wps::moffice::writer::api::LayoutDocument::{
    command_document_from_frame, layout_document, LayoutError, ViewParams,
};
use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::CommandDocument;

#[derive(Debug)]
pub struct RenderError {
    pub message: String,
}

impl From<LayoutError> for RenderError {
    fn from(e: LayoutError) -> Self {
        RenderError {
            message: e.message,
        }
    }
}

pub fn render_docx(bytes: &[u8]) -> Result<CommandDocument, RenderError> {
    let frame = layout_document(bytes, &[], &ViewParams::default_a4())?;
    Ok(command_document_from_frame(&frame))
}
