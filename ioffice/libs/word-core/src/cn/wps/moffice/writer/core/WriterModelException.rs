#![allow(non_snake_case, dead_code)]
// soft shell for PLC.getRange
#[derive(Debug, Default, Clone)]
pub struct WriterModelException {
    pub(crate) msg: String,
}

impl WriterModelException {
    pub fn new(_status: &WriterModelStatus) -> Self { Self::default() }
    pub fn WriterModelException(status: &WriterModelStatus) -> Self { Self::new(status) }
}

#[derive(Debug, Default, Clone)]
pub struct WriterModelStatus {
    pub(crate) code: i32,
    pub(crate) msg: String,
}

impl WriterModelStatus {
    pub fn new(code: i32, msg: &str) -> Self {
        Self { code, msg: msg.to_string() }
    }
    pub fn WriterModelStatus(code: i32, msg: &str) -> Self { Self::new(code, msg) }
}
