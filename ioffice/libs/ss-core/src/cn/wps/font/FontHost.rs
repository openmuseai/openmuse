use super::r#impl::FontTableImpl::FontTableImpl;
use crate::java_runtime::RefCell;
// java public class FontHost
pub struct FontHost;
static FONT_TABLE: std::sync::LazyLock<RefCell<FontTableImpl>> =
    std::sync::LazyLock::new(|| RefCell::new(FontTableImpl::new(true)));
impl FontHost {
    // java public static IFontTable getFontTable() {
    // java IFontTable ret = sFontTable;
    // java if (ret == null) {
    // java ret = FontTableFactory.createFontTable();
    // java setFontTable(ret);
    // java }
    // java
    // java return ret;
    // java }
    pub fn getFontTable() -> &'static RefCell<FontTableImpl> {
        &FONT_TABLE
    }
}
