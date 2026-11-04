use super::types::DOPAbstractType::DOPAbstractType;
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

pub struct DocumentProperties {
    pub base: DOPAbstractType,
}

impl DocumentProperties {
    pub fn new(table_stream: &mut DocumentInputStream, offset: i32, size: i32) -> Self {
        let mut base = DOPAbstractType::new();
        base.fillFields(table_stream, offset, size);
        DocumentProperties { base }
    }
}

impl std::ops::Deref for DocumentProperties {
    type Target = DOPAbstractType;
    fn deref(&self) -> &DOPAbstractType { &self.base }
}

impl std::ops::DerefMut for DocumentProperties {
    fn deref_mut(&mut self) -> &mut DOPAbstractType { &mut self.base }
}
