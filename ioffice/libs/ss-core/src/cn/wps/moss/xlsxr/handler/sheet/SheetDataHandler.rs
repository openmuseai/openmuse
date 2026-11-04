// java package cn.wps.moss.xlsxr.handler.sheet;

// java import cn.wps.kfc.xpxml.reader.ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
// java import cn.wps.kfc.xpxml.reader.XmlRoAttr;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// java import cn.wps.moss.app.KmoSheet;
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
// java import cn.wps.moss.xlsxr.xl.worksheets.XlsxrWorkSheet;
// rust: no-stmt
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;

use super::CellFmlBackupHandler::CellFmlBackupHandler;
use super::CellHandler::CellHandler;
use super::RowHandler::RowHandler;

// java public final class SheetDataHandler implements ElementHandler
#[derive(Clone)]
pub struct SheetDataHandler {
    // java private RowHandler rowHandler;
    rowHandler: RowHandler,
    // java private CellHandler cellHandler;
    cellHandler: CellHandler,
    // java private CellFmlBackupHandler fmlHandler = new CellFmlBackupHandler();
    fmlHandler: CellFmlBackupHandler,
}

impl ElementHandler for SheetDataHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _ = (element, attrsNotCopy);
        self.startElement__body();
    }
    fn endElement(&self, element: i32) {
        let _ = element;
    }
    fn characters(&self, chars: &str) {
        let _ = chars;
    }
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        self.enterSubElement__body(subElement)
    }
}

impl SheetDataHandler {
    // java public SheetDataHandler(XlsxrWorkSheet reader)
    pub fn SheetDataHandler(
        sheet: Rc<RefCell<KmoSheet>>,
        styles: Option<Rc<RefCell<StylesTable>>>,
    ) -> Self {
        // java this.fmlHandler = new CellFmlBackupHandler();
        let fmlHandler = CellFmlBackupHandler::CellFmlBackupHandler();
        // java this.cellHandler = new CellHandler(reader, this.fmlHandler);
        let cellHandler = CellHandler::CellHandler(Rc::clone(&sheet), styles.clone(), fmlHandler.clone());
        // java this.rowHandler = new RowHandler(reader, this.cellHandler);
        let rowHandler = RowHandler::RowHandler(sheet, styles, cellHandler.clone());
        // java this.rowHandler.init();
        rowHandler.init();
        Self {
            rowHandler,
            cellHandler,
            fmlHandler,
        }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement__body(&self) {
        // java this.rowHandler.init();
        self.rowHandler.init();
    }

    // java public void endElement(int element)
    pub fn endElement__2(&self) {}

    // java public void characters(String chars)
    pub fn characters__2(&self) {}

    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement__body(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java return subElement == 4802 ? this.rowHandler : null;
        if subElement == 4802 {
            Some(Box::new(self.rowHandler.clone()))
        } else {
            None
        }
    }

    // java public int getMaxValueIndex()
    pub fn getMaxValueIndex(&self) -> i32 {
        // java return this.cellHandler.getMaxValueIndex();
        // rust: no-stmt — 格子值留给 13，索引保持 0。登记 §1.7.1。
        let _ = &self.cellHandler;
        0
    }

    // java public void delayedImportFormula(KmoSheet sheet)
    pub fn delayedImportFormula(&self, sheet: &KmoSheet) {
        // java this.fmlHandler.delayImportAllSharedFml(sheet);
        self.fmlHandler.delayImportAllSharedFml(sheet);
        // java this.fmlHandler.delayImportAllArrayFml(sheet);
        self.fmlHandler.delayImportAllArrayFml(sheet);
        // java this.fmlHandler.delayImportAllNormalFml(sheet);
        self.fmlHandler.delayImportAllNormalFml(sheet);
    }

    pub fn fmlHandler(&self) -> &CellFmlBackupHandler {
        &self.fmlHandler
    }
}
