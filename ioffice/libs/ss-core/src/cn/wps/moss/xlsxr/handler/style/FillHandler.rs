use super::{
    GradientFillHandler::GradientFillHandler, PatternFillHandler::PatternFillHandler, StylesShared,
};
use crate::java_runtime::{Rc, RefCell};
use kfc_kfc_alpha::kfc::xpxml::reader::{
    ElementAttr::ElementAttr, ElementHandler::ElementHandler, XmlRoAttr::XmlRoAttr,
};
// java public final class FillHandler extends DefaultElementHandler
pub struct FillHandler {
    shared: Rc<StylesShared>,
    patternHandler: RefCell<Option<std::rc::Rc<PatternFillHandler>>>,
    gradientFillHandler: RefCell<Option<std::rc::Rc<GradientFillHandler>>>,
}
impl FillHandler {
    pub fn new(shared: Rc<StylesShared>) -> Self {
        Self {
            shared,
            patternHandler: RefCell::new(None),
            gradientFillHandler: RefCell::new(None),
        }
    }
}
impl ElementHandler for FillHandler {
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn characters(&self, _: &str) {}

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/FillHandler.java:18
    // java public ElementHandler enterSubElement(int subElement) {
    // java if (subElement == 5148) {
    // java this.patternHandler = new PatternFillHandler();
    // java return new ElementAttr(this.patternHandler);
    // java } else if (subElement == 5149) {
    // java this.gradientFillHandler = new GradientFillHandler();
    // java return new ElementAttr(this.gradientFillHandler);
    // java } else {
    // java return null;
    // java }
    // java }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        // rust: if(subElement==5148) {this.patternHandler=new PatternFillHandler();return new ElementAttr(this.patternHandler);}else if(subElement==5149){this.gradientFillHandler=new GradientFillHandler();return new ElementAttr(this.gradientFillHandler);}else return null;
        match e {
            5148 => {
                let h = std::rc::Rc::new(PatternFillHandler::new());
                *self.patternHandler.borrow_mut() = Some(h.clone());
                Some(Box::new(ElementAttr::new(h)))
            }
            5149 => {
                let h = std::rc::Rc::new(GradientFillHandler::new());
                *self.gradientFillHandler.borrow_mut() = Some(h.clone());
                Some(Box::new(ElementAttr::new(h)))
            }
            _ => None,
        }
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/FillHandler.java:30
    // java public void endElement(int element) {
    // java if (null != this.patternHandler) {
    // java this.stylesTable.addPatternFill(this.patternHandler.getPatternFormatting());
    // java this.patternHandler = null;
    // java } else if (null != this.gradientFillHandler) {
    // java this.stylesTable.addPatternFill(this.gradientFillHandler.getPatternFormatting());
    // java this.gradientFillHandler = null;
    // java }
    // java
    // java }
    fn endElement(&self, _: i32) {
        // rust: if(null!=this.patternHandler){this.stylesTable.addPatternFill(this.patternHandler.getPatternFormatting());this.patternHandler=null;}else if(null!=this.gradientFillHandler){...;this.gradientFillHandler=null;}
        if let Some(h) = self.patternHandler.borrow_mut().take() {
            self.shared
                .table
                .borrow()
                .addPatternFill(h.pattern.borrow().clone());
        } else if let Some(h) = self.gradientFillHandler.borrow_mut().take() {
            self.shared
                .table
                .borrow()
                .addPatternFill(h.fill.borrow().clone());
        }
    }
}
