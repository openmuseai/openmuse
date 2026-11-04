// Java: import cn.wps.font.IFontTable
use crate::cn::wps::font::IFontTable::IFontTable;
use std::sync::Arc;
use super::FontTableImpl::FontTableImpl;

// Java: public class FontTableFactory
pub struct FontTableFactory {
}

// Java: public class FontTableFactory
impl FontTableFactory {
    // Java: FontTableFactory()
    pub fn new() -> Self {
        // Java: super();
        FontTableFactory {}
    }

    // Java: static IFontTable createFontTable()
    pub fn createFontTable() -> Arc<dyn IFontTable> {
        // Java: return new FontTableImpl();
        Arc::new(FontTableImpl::new())
    }
}
