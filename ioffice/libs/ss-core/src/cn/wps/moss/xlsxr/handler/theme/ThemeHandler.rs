use crate::cn::wps::moss::core::theme::ColorScheme::ColorScheme;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler, XmlRoAttr::XmlRoAttr};
// java public class ThemeHandler extends DefaultElementHandler
// rust: Spreadsheet theme color value closure. Drawing Theme protocol/font/style matrix writes remain in M3.
pub struct ThemeHandler;
impl ElementHandler for ThemeHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/theme/ThemeHandler.java:31
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java XmlRoAttr tmp;
    // java if ((tmp = attrsNotCopy.getByName(1114136)) != null) {
    // java this.mTheme.putName(tmp.toString());
    // java }
    // java
    // java }
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn characters(&self, _: &str) {}
    fn endElement(&self, _: i32) {}
    // java case 1114461: return new ThemeElementsHandler();
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        if e == 1114461 {
            Some(Box::new(ThemeElementsHandler))
        } else {
            None
        }
    }
}
// java class ThemeElementsHandler extends DefaultElementHandler
pub struct ThemeElementsHandler;
impl ElementHandler for ThemeElementsHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/theme/ThemeHandler.java:31
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java XmlRoAttr tmp;
    // java if ((tmp = attrsNotCopy.getByName(1114136)) != null) {
    // java this.mTheme.putName(tmp.toString());
    // java }
    // java
    // java }
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn characters(&self, _: &str) {}
    fn endElement(&self, _: i32) {}
    // rust: case 1114154: ColorScheme.clean(); ColorScheme4GVml.clean(); ... return this.mColorSchemeHandler;
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        if e == 1114154 {
            ColorScheme::clean();
            Some(Box::new(
                super::color::ColorSchemeHandler::ColorSchemeHandler::new(),
            ))
        } else {
            None
        }
    }
}
