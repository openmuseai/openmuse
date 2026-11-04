/// Rust-only open error. Java keeps exception types; this enum is the M1 mapping target.
/// It is not a second workbook model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpreadsheetOpenError {
    NotWorkbook,
    BrokenRelationship,
    Encrypted,
    Unsupported { reason: &'static str },
}
