use crate::java_runtime::Rc;
use crate::java_runtime::RefCell;

use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;

use super::StylesShared;
use super::XFormatHandler::XFormatHandler;

// java package cn.wps.moss.xlsxr.handler.style;
// java import cn.wps.kfc.xpxml.reader.DefaultElementHandler;
// java import cn.wps.kfc.xpxml.reader.ElementAttr;
// java import cn.wps.kfc.xpxml.reader.ElementHandler;
// java import cn.wps.kfc.xpxml.reader.XmlRoAttr;
// java import cn.wps.moss.app.KmoBook;
// java import cn.wps.moss.app.xf.KmoXFManager;
// java import cn.wps.moss.xlsxr.model.StylesTable;

// java public final class StylesHandler implements ElementHandler {
pub struct StylesHandler {
    shared: Rc<StylesShared>,
}

impl StylesHandler {
    // java public StylesHandler(KmoBook book, StylesTable stylesTable) {
    pub fn StylesHandler(book: Rc<KmoBook>, stylesTable: Rc<RefCell<StylesTable>>) -> Self {
        // java this.kmoBook = book;
        // java this.mStylesTable = stylesTable;
        // java this.kmoXFManager = this.kmoBook.getXFManager();
        // java this.cellXfsHandler = new CellXFormatsHandler();
        // java this.styleXfsHandler = new StyleXFormatsHandler();
        // java this.xfHandler = new XFormatHandler(this.kmoBook, this.mStylesTable);
        // java this.fontsHandler = new FontsHandler();
        // java this.fontHandler = new FontHandler(this.kmoBook, this.kmoXFManager, this.mStylesTable);
        // java this.fillsHandler = new FillsHandler();
        // java this.fillHandler = new FillHandler(this.kmoBook, this.mStylesTable);
        // java this.numFmtsHandler = new NumberFormatsHandler();
        // java this.numFmtHandler = new NumberFormatHandler(this.kmoBook, this.mStylesTable);
        // java this.bordersHandler = new BordersHandler();
        // java this.borderHandler = new BorderHandler(this.kmoBook, this.mStylesTable);
        // java this.dxfsHandler = new DxfsHandler();
        // java this.dxfHandler = new DxfHandler(this.kmoBook, this.mStylesTable);
        // java this.colorsHandler = new ColorsHandler(this.kmoBook);
        // java this.cellStylesHandler = new CellStylesHandler(this.kmoXFManager, this.mStylesTable);
        let shared = StylesShared::new(book, stylesTable);
        let _xf = XFormatHandler::XFormatHandler(Rc::clone(&shared));
        Self { shared }
    }

    // java public CellXFormatsHandler getCellXFormatsHandler() {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/StylesHandler.java:52
    // java public CellXFormatsHandler getCellXFormatsHandler() {
    // java return this.cellXfsHandler;
    // java }
    pub fn getCellXFormatsHandler(&self) -> CellXFormatsHandler {
        // java return this.cellXfsHandler;
        CellXFormatsHandler {
            shared: Rc::clone(&self.shared),
        }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _ = (element, attrsNotCopy);
    }

    // java public void endElement(int element) {
    pub fn endElement(&self, element: i32) {
        let _ = element;
    }

    // java public void characters(String chars) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/StylesHandler.java:62
    // java public void characters(String chars) {
    // java }
    pub fn characters(&self, chars: &str) {
        let _ = chars;
    }

    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java switch (subElement) {
        // java case 5123:
        // java return this.numFmtsHandler;
        // java case 5124:
        // java return this.fontsHandler;
        // java case 5125:
        // java return this.fillsHandler;
        // java case 5126:
        // java return this.bordersHandler;
        // java case 5127:
        // java return this.styleXfsHandler;
        // java case 5128:
        // java return this.cellXfsHandler;
        // java case 5129:
        // java return this.cellStylesHandler;
        // java case 5130:
        // java return this.dxfsHandler;
        // java case 5131:
        // java return new TableStylesHandler(this.kmoBook, this.mStylesTable);
        // java case 5132:
        // java return this.colorsHandler;
        // java default:
        // java return null;
        match subElement {
5131=>Some(Box::new(super::TableStylesHandler::TableStylesHandler::new(self.shared.book.clone(),self.shared.table.clone()))),
            5127 => Some(Box::new(StyleXFormatsHandler {
                shared: Rc::clone(&self.shared),
            })),
            5128 => Some(Box::new(CellXFormatsHandler {
                shared: Rc::clone(&self.shared),
            })),
            5123 => Some(Box::new(NumberFormatsHandler {
                shared: self.shared.clone(),
            })),
            5124 => Some(Box::new(FontsHandler {
                shared: self.shared.clone(),
            })),
            5125 => Some(Box::new(FillsHandler {
                shared: self.shared.clone(),
            })),
            5126 => Some(Box::new(BordersHandler {
                shared: self.shared.clone(),
            })),
            5129 => Some(Box::new(super::CellStylesHandler::CellStylesHandler {
                shared: self.shared.clone(),
            })),
            5132 => Some(Box::new(super::ColorsHandler::ColorsHandler::new(self.shared.book.clone()))),
            5130 => Some(Box::new(DxfsHandler::new(self.shared.clone()))),

            _ => None,
        }
    }
}

// java public class CellXFormatsHandler extends DefaultElementHandler {
pub struct CellXFormatsHandler {
    shared: Rc<StylesShared>,
}

impl CellXFormatsHandler {
    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java if (subElement == 5162) {
        // java StylesHandler.this.xfHandler.init(false);
        // java if (this.mCurrentIndex != -1) {
        // java StylesHandler.this.xfHandler.setXFIndex(this.mCurrentIndex);
        // java ++this.mCurrentIndex;
        // java return StylesHandler.this.xfHandler;
        // java return null;
        if subElement == 5162 {
            let handler = XFormatHandler {
                shared: Rc::clone(&self.shared),
            };
            handler.init(false);
            let cur = self.shared.mCurrentIndex.get();
            if cur != -1 {
                handler.setXFIndex(cur);
                self.shared.mCurrentIndex.set(cur + 1);
            }
            return Some(Box::new(handler));
        }
        None
    }

    // java public void setCurrentXF(int index) {

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/StylesHandler.java:148
    // java public void setCurrentXF(int index) {
    // java this.mCurrentIndex = index;
    // java }
    pub fn setCurrentXF(&self, index: i32) {
        // java this.mCurrentIndex = index;
        self.shared.mCurrentIndex.set(index);
    }
}

// java private class StyleXFormatsHandler extends DefaultElementHandler {
pub struct StyleXFormatsHandler {
    shared: Rc<StylesShared>,
}

impl StyleXFormatsHandler {
    // java private StyleXFormatsHandler() {
    pub fn StyleXFormatsHandler(shared:Rc<StylesShared>)->Self {
        Self{shared}
    }

    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java if (subElement == 5162) {
        // java StylesHandler.this.xfHandler.init(true);
        // java return StylesHandler.this.xfHandler;
        // java return null;
        if subElement == 5162 {
            let handler = XFormatHandler {
                shared: Rc::clone(&self.shared),
            };
            handler.init(true);
            return Some(Box::new(handler));
        }
        None
    }

    // java public void endElement(int element) {
    pub fn endElement(&self, element: i32) {
        // java StylesHandler.this.mStylesTable.initStartCoreXF(StylesHandler.this.kmoXFManager);
        let _ = element;
        self.shared
            .table
            .borrow()
            .initStartCoreXF(&self.shared.book.getXFManager().borrow());
    }
}

struct SkipHandler;

impl ElementHandler for SkipHandler {
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {}
    fn endElement(&self, _element: i32) {}

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/StylesHandler.java:62
    // java public void characters(String chars) {
    // java }
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

impl ElementHandler for StylesHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        StylesHandler::startElement(self, element, attrsNotCopy);
    }
    fn endElement(&self, element: i32) {
        StylesHandler::endElement(self, element);
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/StylesHandler.java:62
    // java public void characters(String chars) {
    // java }
    fn characters(&self, chars: &str) {
        StylesHandler::characters(self, chars);
    }
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        StylesHandler::enterSubElement(self, subElement)
    }
}

impl ElementHandler for CellXFormatsHandler {
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {}
    fn endElement(&self, _element: i32) {}

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/StylesHandler.java:62
    // java public void characters(String chars) {
    // java }
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        CellXFormatsHandler::enterSubElement(self, subElement)
    }
}

impl ElementHandler for StyleXFormatsHandler {
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {}
    fn endElement(&self, element: i32) {
        StyleXFormatsHandler::endElement(self, element);
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/StylesHandler.java:62
    // java public void characters(String chars) {
    // java }
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        StyleXFormatsHandler::enterSubElement(self, subElement)
    }
}

// java private class FontsHandler extends DefaultElementHandler
pub struct FontsHandler {
    shared: Rc<StylesShared>,
}
impl ElementHandler for FontsHandler {
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    // java return subElement==5146?new ElementAttr(StylesHandler.this.fontHandler):null;
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        if e == 5146 {
            Some(Box::new(
                kfc_kfc_alpha::kfc::xpxml::reader::ElementAttr::ElementAttr::new(std::rc::Rc::new(
                    super::FontHandler::FontHandler::new(self.shared.clone(), false),
                )),
            ))
        } else {
            None
        }
    }
}

// java private class FillsHandler extends DefaultElementHandler
pub struct FillsHandler {
    shared: Rc<StylesShared>,
}
impl ElementHandler for FillsHandler {
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    // java return subElement==5147?StylesHandler.this.fillHandler:null;
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        if e == 5147 {
            Some(Box::new(super::FillHandler::FillHandler::new(
                self.shared.clone(),
            )))
        } else {
            None
        }
    }
}

// java private class NumberFormatsHandler extends DefaultElementHandler
pub struct NumberFormatsHandler {
    shared: Rc<StylesShared>,
}
impl ElementHandler for NumberFormatsHandler {
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    // rust: return subElement==5160?StylesHandler.this.numberFormatHandler:null;
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        if e == 5160 {
            Some(Box::new(
                super::NumberFormatHandler::NumberFormatHandler::new(self.shared.clone()),
            ))
        } else {
            None
        }
    }
}

// java private class BordersHandler extends DefaultElementHandler
pub struct BordersHandler {
    shared: Rc<StylesShared>,
}
impl ElementHandler for BordersHandler {
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    // java return subElement==5141?StylesHandler.this.borderHandler:null;
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        if e == 5141 {
            Some(Box::new(super::BorderHandler::BorderHandler::new(
                self.shared.clone(),
            )))
        } else {
            None
        }
    }
}

// java private class DxfsHandler extends DefaultElementHandler
pub struct DxfsHandler {
    handler: std::rc::Rc<super::DxfHandler::DxfHandler>,
}
impl DxfsHandler {
    pub fn new(shared: Rc<StylesShared>) -> Self {
        Self {
            handler: std::rc::Rc::new(super::DxfHandler::DxfHandler::new(shared)),
        }
    }
}
impl ElementHandler for DxfsHandler {
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    // java return subElement == 4729 ? StylesHandler.this.dxfHandler : null;
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        if e == 4729 {
            Some(Box::new(super::DxfHandler::DxfHandlerRef(
                self.handler.clone(),
            )))
        } else {
            None
        }
    }
}
