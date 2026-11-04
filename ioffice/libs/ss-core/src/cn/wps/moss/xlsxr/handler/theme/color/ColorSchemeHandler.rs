use crate::cn::wps::moss::core::theme::ColorElement::ColorName;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler, XmlRoAttr::XmlRoAttr};
// java public class ColorSchemeHandler extends DefaultElementHandler
pub struct ColorSchemeHandler {
    handlers: std::cell::RefCell<
        std::collections::HashMap<i32, std::rc::Rc<super::ColorHandler::ColorHandler>>,
    >,
}
impl ColorSchemeHandler {
    pub fn new() -> Self {
        Self {
            handlers: Default::default(),
        }
    }
}
struct HandlerRef(std::rc::Rc<super::ColorHandler::ColorHandler>);
impl ElementHandler for HandlerRef {
    fn startElement(&self, e: i32, a: &dyn XmlRoAttr) {
        self.0.startElement(e, a);
    }
    fn endElement(&self, e: i32) {
        self.0.endElement(e);
    }
    fn characters(&self, s: &str) {
        self.0.characters(s);
    }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        self.0.enterSubElement(e)
    }
}
impl ElementHandler for ColorSchemeHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/theme/color/ColorSchemeHandler.java:31
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java this.colorScheme = ColorScheme.create();
    // java XmlRoAttr tmp;
    // java if ((tmp = attrsNotCopy.getByName(1114136)) != null) {
    // java this.colorScheme.putName(tmp.toString());
    // java }
    // java
    // java }
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn characters(&self, _: &str) {}
    fn endElement(&self, _: i32) {}

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/theme/color/ColorSchemeHandler.java:40
    // java public ElementHandler enterSubElement(int subElement) {
    // java switch (subElement) {
    // java case 1114124:
    // java if (null == this.dk1Handler) {
    // java this.dk1Handler = new ColorHandler(ColorElement.ColorName.dk1, this.mHandlerCallback);
    // java }
    // java
    // java return this.dk1Handler;
    // java case 1114125:
    // java if (null == this.lt1Handler) {
    // java this.lt1Handler = new ColorHandler(ColorElement.ColorName.lt1, this.mHandlerCallback);
    // java }
    // java
    // java return this.lt1Handler;
    // java case 1114126:
    // java if (null == this.dk2Handler) {
    // java this.dk2Handler = new ColorHandler(ColorElement.ColorName.dk2, this.mHandlerCallback);
    // java }
    // java
    // java return this.dk2Handler;
    // java case 1114127:
    // java if (null == this.lt2Handler) {
    // java this.lt2Handler = new ColorHandler(ColorElement.ColorName.lt2, this.mHandlerCallback);
    // java }
    // java
    // java return this.lt2Handler;
    // java case 1114128:
    // java if (null == this.accent1Handler) {
    // java this.accent1Handler = new ColorHandler(ColorElement.ColorName.accent1, this.mHandlerCallback);
    // java }
    // java
    // java return this.accent1Handler;
    // java case 1114129:
    // java if (null == this.accent2Handler) {
    // java this.accent2Handler = new ColorHandler(ColorElement.ColorName.accent2, this.mHandlerCallback);
    // java }
    // java
    // java return this.accent2Handler;
    // java case 1114130:
    // java if (null == this.accent3Handler) {
    // java this.accent3Handler = new ColorHandler(ColorElement.ColorName.accent3, this.mHandlerCallback);
    // java }
    // java
    // java return this.accent3Handler;
    // java case 1114131:
    // java if (null == this.accent4Handler) {
    // java this.accent4Handler = new ColorHandler(ColorElement.ColorName.accent4, this.mHandlerCallback);
    // java }
    // java
    // java return this.accent4Handler;
    // java case 1114132:
    // java if (null == this.accent5Handler) {
    // java this.accent5Handler = new ColorHandler(ColorElement.ColorName.accent5, this.mHandlerCallback);
    // java }
    // java
    // java return this.accent5Handler;
    // java case 1114133:
    // java if (null == this.accent6Handler) {
    // java this.accent6Handler = new ColorHandler(ColorElement.ColorName.accent6, this.mHandlerCallback);
    // java }
    // java
    // java return this.accent6Handler;
    // java case 1114134:
    // java if (null == this.hlinkHandler) {
    // java this.hlinkHandler = new ColorHandler(ColorElement.ColorName.hlink, this.mHandlerCallback);
    // java }
    // java
    // java return this.hlinkHandler;
    // java case 1114135:
    // java if (null == this.folHlinkHandler) {
    // java this.folHlinkHandler = new ColorHandler(ColorElement.ColorName.folHlink, this.mHandlerCallback);
    // java }
    // java
    // java return this.folHlinkHandler;
    // java default:
    // java return null;
    // java }
    // java }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        // rust: switch (subElement) { case 1114124: ... new ColorHandler(ColorElement.ColorName.dk1, this.mHandlerCallback); ... }
        let name = match e {
            1114124 => ColorName::dk1,
            1114125 => ColorName::lt1,
            1114126 => ColorName::dk2,
            1114127 => ColorName::lt2,
            1114128 => ColorName::accent1,
            1114129 => ColorName::accent2,
            1114130 => ColorName::accent3,
            1114131 => ColorName::accent4,
            1114132 => ColorName::accent5,
            1114133 => ColorName::accent6,
            1114134 => ColorName::hlink,
            1114135 => ColorName::folHlink,
            _ => return None,
        };
        let handler = self
            .handlers
            .borrow_mut()
            .entry(e)
            .or_insert_with(|| std::rc::Rc::new(super::ColorHandler::ColorHandler::new(name)))
            .clone();
        Some(Box::new(HandlerRef(handler)))
    }
}
