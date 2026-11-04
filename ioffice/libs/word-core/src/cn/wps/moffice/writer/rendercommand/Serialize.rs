//! Stage3 JSON: `{version, commands}` tagged union.

use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::CommandDocument;

pub const STAGE3_VERSION: i32 = 1;

pub fn to_json(doc: &CommandDocument) -> Result<String, serde_json::Error> {
    serde_json::to_string(doc)
}

pub fn from_json(s: &str) -> Result<CommandDocument, serde_json::Error> {
    serde_json::from_str(s)
}
