use crate::{
    cn::wps::moss::core::theme::{
        ColorElement::{ColorElement, ColorName},
        ColorScheme::ColorScheme,
    },
    java_runtime::Rc,
};
use kfc_kfc_alpha::kfc::xpxml::reader::{
    ElementAttr::ElementAttr, ElementHandler::ElementHandler, XmlRoAttr::XmlRoAttr,
};
// java public class ColorHandler extends DefaultElementHandler
pub struct ColorHandler {
    colorElement: Rc<ColorElement>,
}
impl ColorHandler {
    pub fn new(name: ColorName) -> Self {
        Self {
            colorElement: Rc::new(ColorElement::new(name)),
        }
    }
}
impl ElementHandler for ColorHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/theme/color/ColorHandler.java:31
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java this.color = Color.create();
    // java }
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn characters(&self, _: &str) {}
    // java ColorScheme.addColor(this.colorElement);

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/theme/color/ColorHandler.java:51
    // java public void endElement(int element) {
    // java if (null != this.mHandlerCallback && this.mHandlerCallback.isGVml()) {
    // java ColorScheme4GVml.addColor(this.colorElement);
    // java } else {
    // java ColorScheme.addColor(this.colorElement);
    // java }
    // java
    // java }
    fn endElement(&self, _: i32) {
        ColorScheme::addColor(self.colorElement.clone());
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/theme/color/ColorHandler.java:35
    // java public ElementHandler enterSubElement(int subElement) {
    // java switch (subElement) {
    // java case 1114215:
    // java case 1114217:
    // java case 1114219:
    // java case 1114220:
    // java return ColorHandlerFactory.createColorHandler(subElement, this.color);
    // java case 1114216:
    // java return new ElementAttr(new SrgbClrHandler(this.colorElement, this.color));
    // java case 1114218:
    // java return new ElementAttr(new SysColorHandler(this.colorElement, this.color));
    // java default:
    // java return null;
    // java }
    // java }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        // java case 1114216: return new ElementAttr(new SrgbClrHandler(this.colorElement, this.color));
        // java case 1114218: return new ElementAttr(new SysColorHandler(this.colorElement, this.color));
        match e {
            1114216 => Some(Box::new(ElementAttr::new(std::rc::Rc::new(
                super::SrgbClrHandler::SrgbClrHandler::new(self.colorElement.clone()),
            )))),
            1114218 => Some(Box::new(ElementAttr::new(std::rc::Rc::new(
                super::SysColorHandler::SysColorHandler::new(self.colorElement.clone()),
            )))),
            _ => None,
        }
    }
}
