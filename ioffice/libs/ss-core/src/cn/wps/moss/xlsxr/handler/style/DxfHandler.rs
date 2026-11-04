use super::{
    BorderHandler::BorderHandler, FillHandler::FillHandler, FontHandler::FontHandler,
    NumberFormatHandler::NumberFormatHandler, StylesShared,
};
use crate::{
    cn::wps::moss::{
        filefmt::biff8::record::FontRecord::FontRecord, xlsxr::model::DxfPropertyId::DxfPropertyId,
    },
    java_runtime::{Cell, Rc, RefCell},
};
use kfc_kfc_alpha::kfc::xpxml::reader::{
    ElementAttr::ElementAttr, ElementHandler::ElementHandler, XmlRoAttr::XmlRoAttr,
};
// java public class DxfHandler extends DefaultElementHandler
pub struct DxfHandler {
    shared: Rc<StylesShared>,
    fillHandler: FillHandler,
    borderHandler: BorderHandler,
    fontHandler: std::rc::Rc<FontHandler>,
    numberFormatHandler: NumberFormatHandler,
    usedbord: Cell<bool>,
    usedfill: Cell<bool>,
    usedfont: Cell<bool>,
    usedNumFmt: Cell<bool>,
}
impl DxfHandler {
    pub fn new(shared: Rc<StylesShared>) -> Self {
        Self {
            fillHandler: FillHandler::new(shared.clone()),
            borderHandler: BorderHandler::new(shared.clone()),
            fontHandler: std::rc::Rc::new(FontHandler::new(shared.clone(), true)),
            numberFormatHandler: NumberFormatHandler::new(shared.clone()),
            shared,
            usedbord: Cell::new(false),
            usedfill: Cell::new(false),
            usedfont: Cell::new(false),
            usedNumFmt: Cell::new(false),
        }
    }
}
// rust: Java handler references are returned without copying the enclosing instance.
pub struct DxfHandlerRef(pub std::rc::Rc<DxfHandler>);
pub struct FillRef(std::rc::Rc<DxfHandler>);
pub struct BorderRef(std::rc::Rc<DxfHandler>);
pub struct NumberRef(std::rc::Rc<DxfHandler>);
impl ElementHandler for DxfHandlerRef {
    // java this.usedbord = false; this.usedfill = false; this.usedfont = false; this.usedNumFmt = false;

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:31
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java this.usedbord = false;
    // java this.usedfill = false;
    // java this.usedfont = false;
    // java this.usedNumFmt = false;
    // java }
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {
        let s = &self.0;
        s.usedbord.set(false);
        s.usedfill.set(false);
        s.usedfont.set(false);
        s.usedNumFmt.set(false);
    }
    fn characters(&self, _: &str) {}

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:38
    // java public ElementHandler enterSubElement(int subElement) {
    // java switch (subElement) {
    // java case 5141:
    // java this.usedbord = true;
    // java return this.borderHandler;
    // java case 5146:
    // java this.usedfont = true;
    // java FontRecord font = new FontRecord();
    // java this.stylesTable.addFontRecord(font);
    // java this.fontHandler.init(font);
    // java return new ElementAttr(this.fontHandler);
    // java case 5147:
    // java this.usedfill = true;
    // java return this.fillHandler;
    // java case 5160:
    // java this.usedNumFmt = true;
    // java return this.numberFormatHandler;
    // java default:
    // java return null;
    // java }
    // java }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        let s = &self.0;
        match e {
            // java case 5141: this.usedbord = true; return this.borderHandler;
            5141 => {
                s.usedbord.set(true);
                Some(Box::new(BorderRef(s.clone())))
            }
            // java case 5146: this.usedfont = true; FontRecord font = new FontRecord(); this.stylesTable.addFontRecord(font); this.fontHandler.init(font); return new ElementAttr(this.fontHandler);
            5146 => {
                s.usedfont.set(true);
                let font = Rc::new(RefCell::new(FontRecord::FontRecord()));
                s.shared.table.borrow().addFontRecord(font.clone());
                s.fontHandler.init(font);
                Some(Box::new(ElementAttr::new(s.fontHandler.clone())))
            }
            // java case 5147: this.usedfill = true; return this.fillHandler;
            5147 => {
                s.usedfill.set(true);
                Some(Box::new(FillRef(s.clone())))
            }
            // java case 5160: this.usedNumFmt = true; return this.numberFormatHandler;
            5160 => {
                s.usedNumFmt.set(true);
                Some(Box::new(NumberRef(s.clone())))
            }
            _ => None,
        }
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:60
    // java public void endElement(int element) {
    // java DxfPropertyId dxf = new DxfPropertyId();
    // java if (this.usedNumFmt) {
    // java int numFmtId = this.numberFormatHandler.kmoFmtid();
    // java dxf.setNumFmtId(numFmtId);
    // java }
    // java
    // java if (!this.usedbord) {
    // java dxf.setBordid(-1);
    // java } else {
    // java dxf.setBordid(this.stylesTable.getborderListsize() - 1);
    // java }
    // java
    // java if (!this.usedfill) {
    // java dxf.setPatid(-1);
    // java } else {
    // java dxf.setPatid(this.stylesTable.getpatternFillListsize() - 1);
    // java }
    // java
    // java if (!this.usedfont) {
    // java dxf.setFontid(-1);
    // java } else {
    // java dxf.setFontid(this.stylesTable.getFontListsize() - 1);
    // java }
    // java
    // java this.stylesTable.adddxf(dxf);
    // java }
    fn endElement(&self, _: i32) {
        let s = &self.0;
        let dxf = Rc::new(DxfPropertyId::new());
        let table = s.shared.table.borrow();
        // java if (this.usedNumFmt) { int numFmtId = this.numberFormatHandler.kmoFmtid(); dxf.setNumFmtId(numFmtId); }
        if s.usedNumFmt.get() {
            dxf.setNumFmtId(s.numberFormatHandler.kmoFmtid() as i32);
        }
        // rust: dxf.setBordid(this.usedbord ? this.stylesTable.getborderListsize() - 1 : -1);
        dxf.setBordid(if s.usedbord.get() {
            table.getborderListsize() - 1
        } else {
            -1
        });
        dxf.setPatid(if s.usedfill.get() {
            table.getpatternFillListsize() - 1
        } else {
            -1
        });
        dxf.setFontid(if s.usedfont.get() {
            table.getFontListsize() - 1
        } else {
            -1
        });
        table.adddxf(dxf);
    }
}
impl ElementHandler for FillRef {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:31
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java this.usedbord = false;
    // java this.usedfill = false;
    // java this.usedfont = false;
    // java this.usedNumFmt = false;
    // java }
    fn startElement(&self, e: i32, a: &dyn XmlRoAttr) {
        self.0.fillHandler.startElement(e, a);
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:60
    // java public void endElement(int element) {
    // java DxfPropertyId dxf = new DxfPropertyId();
    // java if (this.usedNumFmt) {
    // java int numFmtId = this.numberFormatHandler.kmoFmtid();
    // java dxf.setNumFmtId(numFmtId);
    // java }
    // java
    // java if (!this.usedbord) {
    // java dxf.setBordid(-1);
    // java } else {
    // java dxf.setBordid(this.stylesTable.getborderListsize() - 1);
    // java }
    // java
    // java if (!this.usedfill) {
    // java dxf.setPatid(-1);
    // java } else {
    // java dxf.setPatid(this.stylesTable.getpatternFillListsize() - 1);
    // java }
    // java
    // java if (!this.usedfont) {
    // java dxf.setFontid(-1);
    // java } else {
    // java dxf.setFontid(this.stylesTable.getFontListsize() - 1);
    // java }
    // java
    // java this.stylesTable.adddxf(dxf);
    // java }
    fn endElement(&self, e: i32) {
        self.0.fillHandler.endElement(e);
    }
    fn characters(&self, s: &str) {
        self.0.fillHandler.characters(s);
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:38
    // java public ElementHandler enterSubElement(int subElement) {
    // java switch (subElement) {
    // java case 5141:
    // java this.usedbord = true;
    // java return this.borderHandler;
    // java case 5146:
    // java this.usedfont = true;
    // java FontRecord font = new FontRecord();
    // java this.stylesTable.addFontRecord(font);
    // java this.fontHandler.init(font);
    // java return new ElementAttr(this.fontHandler);
    // java case 5147:
    // java this.usedfill = true;
    // java return this.fillHandler;
    // java case 5160:
    // java this.usedNumFmt = true;
    // java return this.numberFormatHandler;
    // java default:
    // java return null;
    // java }
    // java }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        self.0.fillHandler.enterSubElement(e)
    }
}
impl ElementHandler for BorderRef {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:31
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java this.usedbord = false;
    // java this.usedfill = false;
    // java this.usedfont = false;
    // java this.usedNumFmt = false;
    // java }
    fn startElement(&self, e: i32, a: &dyn XmlRoAttr) {
        self.0.borderHandler.startElement(e, a);
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:60
    // java public void endElement(int element) {
    // java DxfPropertyId dxf = new DxfPropertyId();
    // java if (this.usedNumFmt) {
    // java int numFmtId = this.numberFormatHandler.kmoFmtid();
    // java dxf.setNumFmtId(numFmtId);
    // java }
    // java
    // java if (!this.usedbord) {
    // java dxf.setBordid(-1);
    // java } else {
    // java dxf.setBordid(this.stylesTable.getborderListsize() - 1);
    // java }
    // java
    // java if (!this.usedfill) {
    // java dxf.setPatid(-1);
    // java } else {
    // java dxf.setPatid(this.stylesTable.getpatternFillListsize() - 1);
    // java }
    // java
    // java if (!this.usedfont) {
    // java dxf.setFontid(-1);
    // java } else {
    // java dxf.setFontid(this.stylesTable.getFontListsize() - 1);
    // java }
    // java
    // java this.stylesTable.adddxf(dxf);
    // java }
    fn endElement(&self, e: i32) {
        self.0.borderHandler.endElement(e);
    }
    fn characters(&self, s: &str) {
        self.0.borderHandler.characters(s);
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:38
    // java public ElementHandler enterSubElement(int subElement) {
    // java switch (subElement) {
    // java case 5141:
    // java this.usedbord = true;
    // java return this.borderHandler;
    // java case 5146:
    // java this.usedfont = true;
    // java FontRecord font = new FontRecord();
    // java this.stylesTable.addFontRecord(font);
    // java this.fontHandler.init(font);
    // java return new ElementAttr(this.fontHandler);
    // java case 5147:
    // java this.usedfill = true;
    // java return this.fillHandler;
    // java case 5160:
    // java this.usedNumFmt = true;
    // java return this.numberFormatHandler;
    // java default:
    // java return null;
    // java }
    // java }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        self.0.borderHandler.enterSubElement(e)
    }
}
impl ElementHandler for NumberRef {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:31
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java this.usedbord = false;
    // java this.usedfill = false;
    // java this.usedfont = false;
    // java this.usedNumFmt = false;
    // java }
    fn startElement(&self, e: i32, a: &dyn XmlRoAttr) {
        self.0.numberFormatHandler.startElement(e, a);
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:60
    // java public void endElement(int element) {
    // java DxfPropertyId dxf = new DxfPropertyId();
    // java if (this.usedNumFmt) {
    // java int numFmtId = this.numberFormatHandler.kmoFmtid();
    // java dxf.setNumFmtId(numFmtId);
    // java }
    // java
    // java if (!this.usedbord) {
    // java dxf.setBordid(-1);
    // java } else {
    // java dxf.setBordid(this.stylesTable.getborderListsize() - 1);
    // java }
    // java
    // java if (!this.usedfill) {
    // java dxf.setPatid(-1);
    // java } else {
    // java dxf.setPatid(this.stylesTable.getpatternFillListsize() - 1);
    // java }
    // java
    // java if (!this.usedfont) {
    // java dxf.setFontid(-1);
    // java } else {
    // java dxf.setFontid(this.stylesTable.getFontListsize() - 1);
    // java }
    // java
    // java this.stylesTable.adddxf(dxf);
    // java }
    fn endElement(&self, e: i32) {
        self.0.numberFormatHandler.endElement(e);
    }
    fn characters(&self, s: &str) {
        self.0.numberFormatHandler.characters(s);
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/DxfHandler.java:38
    // java public ElementHandler enterSubElement(int subElement) {
    // java switch (subElement) {
    // java case 5141:
    // java this.usedbord = true;
    // java return this.borderHandler;
    // java case 5146:
    // java this.usedfont = true;
    // java FontRecord font = new FontRecord();
    // java this.stylesTable.addFontRecord(font);
    // java this.fontHandler.init(font);
    // java return new ElementAttr(this.fontHandler);
    // java case 5147:
    // java this.usedfill = true;
    // java return this.fillHandler;
    // java case 5160:
    // java this.usedNumFmt = true;
    // java return this.numberFormatHandler;
    // java default:
    // java return null;
    // java }
    // java }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        self.0.numberFormatHandler.enterSubElement(e)
    }
}
