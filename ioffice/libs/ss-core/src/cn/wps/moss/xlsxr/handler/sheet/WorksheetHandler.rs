// java package cn.wps.moss.xlsxr.handler.sheet;
// rust: no-stmt
use crate::java_runtime::{Cell, RefCell};
use crate::java_runtime::Rc;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;

use super::SheetDataHandler::SheetDataHandler;
use super::ColumnsHandler::ColumnsHandler;
use super::MergeCellsHandler::MergeCellsHandler;
use super::SheetFormatPrHandler::SheetFormatPrHandler;

#[derive(Clone)]
struct SheetPrHandler {
    sheet: Rc<RefCell<KmoSheet>>,
    isOutlinePrHandle: Rc<Cell<bool>>,
}

impl ElementHandler for SheetPrHandler {
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {}
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java if (subElement == 4830) return tabColorHandler;
        // rust: no-stmt — TabColorHandler 仍停车。登记 §1.7.1。
        if subElement == 4831 {
            // java return new OutlinePrHandle();
            return Some(Box::new(OutlinePrHandle {
                sheet: Rc::clone(&self.sheet),
                isOutlinePrHandle: Rc::clone(&self.isOutlinePrHandle),
            }));
        }
        // java return subElement == 4832 ? new PageSetupPrHandle() : null;
        // rust: no-stmt — 打印设置仍停车。登记 §1.7.1。
        None
    }
}

#[derive(Clone)]
struct OutlinePrHandle {
    sheet: Rc<RefCell<KmoSheet>>,
    isOutlinePrHandle: Rc<Cell<bool>>,
}

impl ElementHandler for OutlinePrHandle {
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // java WorksheetHandler.this.isOutlinePrHandle = true;
        self.isOutlinePrHandle.set(true);
        // java boolean summaryBelow = true;
        let mut summaryBelow = true;
        // java boolean summaryRight = true;
        let mut summaryRight = true;
        // java if ((tmp = attrsNotCopy.getByName(4884)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(4884) {
            // java summaryBelow = tmp.toBoolean();
            summaryBelow = tmp.to_boolean();
            // java if (!summaryBelow)
            if !summaryBelow {
                // java summaryBelow = false;
                summaryBelow = false;
            }
        }
        // java if ((tmp = attrsNotCopy.getByName(4885)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(4885) {
            // java summaryRight = tmp.toBoolean();
            summaryRight = tmp.to_boolean();
            // java if (!summaryRight)
            if !summaryRight {
                // java summaryRight = false;
                summaryRight = false;
            }
        }
        // java WSBoolRecord wSBoolRecord = WorksheetHandler.this.sheet.getWSBool();
        let sheet = self.sheet.borrow();
        let mut wSBoolRecord = sheet.getWSBool().borrow_mut();
        // java wSBoolRecord.setRowSumsBelow(summaryBelow);
        wSBoolRecord.setRowSumsBelow(summaryBelow);
        // java wSBoolRecord.setRowSumsRight(summaryRight);
        wSBoolRecord.setRowSumsRight(summaryRight);
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

// java private final class SheetViewsHandler extends DefaultElementHandler
struct SheetViewsHandler{sheet:Rc<RefCell<KmoSheet>>}
impl ElementHandler for SheetViewsHandler{fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}fn endElement(&self,_e:i32){}fn characters(&self,_s:&str){}
// java return subElement == 4841 ? new SheetViewHandler(WorksheetHandler.this.reader) : null;
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{if e==4841{Some(Box::new(super::SheetViewHandler::SheetViewHandler::new(self.sheet.clone())))}else{None}}}
struct NopHandler;

impl ElementHandler for NopHandler {
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {}
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

// java public final class WorksheetHandler implements ElementHandler
#[derive(Clone)]
pub struct WorksheetHandler {
    sheet: Rc<RefCell<KmoSheet>>,
    // java private boolean isOutlinePrHandle;
    isOutlinePrHandle: Rc<Cell<bool>>,
    sheetDataHandler: SheetDataHandler,
    colsHandler: ColumnsHandler,
    mergeCellsHandler: MergeCellsHandler,
    sheetFormatPRHandler: SheetFormatPrHandler,
    styles:Option<Rc<RefCell<StylesTable>>>,active:Rc<Cell<bool>>,hasCF:Rc<Cell<bool>>,
}

impl ElementHandler for WorksheetHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _ = (element, attrsNotCopy);
        self.startElement__body();
    }
    fn endElement(&self, element: i32) {
        let _ = element;
        self.endElement__body();
    }
    fn characters(&self, chars: &str) {
        let _ = chars;
        self.characters__body();
    }
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        self.enterSubElement__body(subElement)
    }
}

impl WorksheetHandler {
    // java public WorksheetHandler(XlsxrWorkSheet reader) {
    pub fn WorksheetHandler(
        sheet: Rc<RefCell<KmoSheet>>,
        styles: Option<Rc<RefCell<StylesTable>>>,
    ) -> Self {
        // java this.reader = reader;
        // rust: no-stmt — 不保存 XlsxrWorkSheet。登记 §1.7.1。
        // java this.sheet = reader.getSheet();
        // rust: no-stmt
        // java this.sheetPrHandler = new SheetPrHandler();
        // rust: no-stmt
        // java this.colsHandler = new ColumnsHandler(reader);
        // rust: no-stmt
        // java this.sheetFormatPRHandler = new SheetFormatPrHandler(this.sheet);
        // rust: no-stmt
        // java this.sheetProtectionHandler = new SheetProtectionHandler(this.sheet);
        // rust: no-stmt
        // java this.mergeCellsHandler = new MergeCellsHandler(this.sheet);
        // rust: no-stmt
        // java this.hyperlinksHandler = new HyperlinksHandler();
        // rust: no-stmt
        // java this.dimensionHandler = new DimensionHandler();
        // rust: no-stmt
        // java this.tabColorHandler = new TabColorHandler();
        // rust: no-stmt
        // java this.isOutlinePrHandle = false;
        let isOutlinePrHandle = Rc::new(Cell::new(false));
        // java this.sheetDataHandler = new SheetDataHandler(reader);
        let colsHandler = ColumnsHandler::ColumnsHandler(Rc::clone(&sheet), styles.clone());
        let mergeCellsHandler = MergeCellsHandler::MergeCellsHandler(Rc::clone(&sheet));
        let sheetFormatPRHandler = SheetFormatPrHandler::SheetFormatPrHandler(Rc::clone(&sheet));
        let sheetDataHandler = SheetDataHandler::SheetDataHandler(Rc::clone(&sheet), styles.clone());
        Self {
            sheet,
            isOutlinePrHandle,
            sheetDataHandler,
            colsHandler,
            mergeCellsHandler,
            sheetFormatPRHandler,styles,active:Rc::new(Cell::new(false)),hasCF:Rc::new(Cell::new(false)),
        }
    }

    // rust: Reader ownership adapters for the original getActiveSheet/setHasConditionalFormatting calls.
    pub fn setActiveSheet(&self,active:bool){self.active.set(active);}
    pub fn hasConditionalFormatting(&self)->bool{self.hasCF.get()}
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    fn startElement__body(&self) {
    }

    // java public void endElement(int element) {
    fn endElement__body(&self) {
        // java int width = this.sheet.getDefColWidth();
        // rust: no-stmt
        // java if (width == 0) {
        // rust: no-stmt
        // java int colInfoLast = this.sheet.getColInfos().getLastCol() + 1;
        // rust: no-stmt
        // java int maxColumns = this.sheet.getMaxColumns();
        // rust: no-stmt
        // java if (colInfoLast < 0) {
        // rust: no-stmt
        // java colInfoLast = 0;
        let _ = stringify!(colInfoLast);
        // rust: no-stmt
        // java if (colInfoLast <= maxColumns - 1) {
        // rust: no-stmt
        // java this.sheet.getColInfos().initFillColInfo(colInfoLast, maxColumns - 1, true);
        // rust: no-stmt
    }

    // java public void characters(String chars) {
    fn characters__body(&self) {
    }

    // java public ElementHandler enterSubElement(int subElement) {
    fn enterSubElement__body(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java switch (subElement) {
        match subElement {
            // java case 4769: return new SheetViewsHandler();
            4769=>return Some(Box::new(SheetViewsHandler{sheet:self.sheet.clone()})),
            4776=>{if self.active.get(){self.hasCF.set(true);return None;}return self.styles.as_ref().map(|t|Box::new(super::ConditionalFormattingHandler::ConditionalFormattingHandler::new(self.sheet.clone(),t.clone(),false))as Box<dyn ElementHandler>);},
            4099=>{return self.styles.as_ref().map(|t|Box::new(super::ExtListHandler::ExtListHandler::new(self.sheet.clone(),t.clone()))as Box<dyn ElementHandler>);},
            // java case 4772:
            4772 => {
                // java return this.sheetDataHandler;
                return Some(Box::new(self.sheetDataHandler.clone()));
            }
            4771 => {
                // Java: this.colsHandler.init(this.sheet); return this.colsHandler;
                return Some(Box::new(self.colsHandler.clone()));
            }
            4793 => {
                // Java: return this.mergeCellsHandler;
                return Some(Box::new(self.mergeCellsHandler.clone()));
            }
            4768 => {
                // java return this.sheetPrHandler;
                return Some(Box::new(SheetPrHandler {
                    sheet: Rc::clone(&self.sheet),
                    isOutlinePrHandle: Rc::clone(&self.isOutlinePrHandle),
                }));
            }
            4770 => {
                // java this.sheetFormatPRHandler.init(this.isOutlinePrHandle);
                self.sheetFormatPRHandler.init(self.isOutlinePrHandle.get());
                // java return this.sheetFormatPRHandler;
                return Some(Box::new(self.sheetFormatPRHandler.clone()));
            }
            // java case 4594:
            4594 => {
                // java return this.dimensionHandler;
                return Some(Box::new(DimensionHandler{sheet:self.sheet.clone()}));
            }
            _ => {}
        }
        // java case 4099:
        // rust: no-stmt
        // java ExtListHandler extListHandle = new ExtListHandler(this.reader);
        let _ = stringify!(extListHandle);
        // rust: no-stmt
        // java extListHandle.Init(false);
        // rust: no-stmt
        // java return extListHandle;
        // rust: no-stmt
        // java case 4594:
        // rust: no-stmt
        // java return this.dimensionHandler;
        // rust: no-stmt
        // java case 4601:
        // rust: no-stmt
        // java return new AutoFilterHandler(this.sheet, this.reader.getStylesTable());
        // rust: no-stmt
        // java case 4768:
        // rust: no-stmt
        // java return this.sheetPrHandler;
        // rust: no-stmt
        // java case 4769:
        // rust: no-stmt
        // java return new SheetViewsHandler();
        // rust: no-stmt
        // java case 4770:
        // rust: no-stmt
        // java this.sheetFormatPRHandler.init(this.isOutlinePrHandle);
        // rust: no-stmt
        // java return this.sheetFormatPRHandler;
        // rust: no-stmt
        // java case 4771:
        // rust: no-stmt
        // java this.colsHandler.init(this.sheet);
        // rust: no-stmt
        // java return this.colsHandler;
        // rust: no-stmt
        // java case 4772:
        // rust: no-stmt
        // java return this.sheetDataHandler;
        // rust: no-stmt
        // java case 4773:
        // rust: no-stmt
        // java return this.sheetProtectionHandler;
        // rust: no-stmt
        // java case 4776:
        // rust: no-stmt
        // java if (this.reader.getActiveSheet()) {
        // rust: no-stmt
        // java this.reader.setHasConditionalFormatting(true);
        // rust: no-stmt
        // java return null;
        // rust: no-stmt
        // java return new ConditionalFormattingHandler(this.sheet, this.reader);
        // rust: no-stmt
        // java case 4777:
        // rust: no-stmt
        // java return new PrintOptionHandler();
        // rust: no-stmt
        // java case 4778:
        // rust: no-stmt
        // java return new PageMarginsHandler();
        // rust: no-stmt
        // java case 4779:
        // rust: no-stmt
        // java return new PageSetupHandler();
        // rust: no-stmt
        // java case 4780:
        // rust: no-stmt
        // java return new HeaderFooterHandler(this.sheet);
        // rust: no-stmt
        // java case 4788:
        // rust: no-stmt
        // java return new BackgroundPictureHandler();
        // rust: no-stmt
        // java case 4789:
        // rust: no-stmt
        // java return new OleObjectsHandler(this.reader);
        // rust: no-stmt
        // java case 4791:
        // rust: no-stmt
        // java return new SheetProtectionRangesHandler(this.sheet);
        // rust: no-stmt
        // java case 4793:
        // rust: no-stmt
        // java return this.mergeCellsHandler;
        // rust: no-stmt
        // java case 4794:
        // rust: no-stmt
        // java if (this.reader.getActiveSheet()) {
        // rust: no-stmt
        // java this.reader.setHasDataValidations(true);
        // rust: no-stmt
        // java return null;
        // rust: no-stmt
        // java return new DataValidationsHandler(this.reader.getSheet());
        // rust: no-stmt
        // java case 4795:
        // rust: no-stmt
        // java return this.hyperlinksHandler;
        // rust: no-stmt
        // java case 4797:
        // rust: no-stmt
        // java return new IgnoredErrorsHandler(this.sheet);
        // rust: no-stmt
        // java case 4801:
        // rust: no-stmt
        // java return new TablePartsHandler(this.reader);
        // rust: no-stmt
        // java default:
        // rust: no-stmt
        // java return null;
        // rust: no-stmt
        None
    }

    // java public int getMaxValueIndex() {
    pub fn getMaxValueIndex(&self) -> i32 {
        // java return this.sheetDataHandler.getMaxValueIndex();
        self.sheetDataHandler.getMaxValueIndex()
    }

    // java public SheetDataHandler getSheetDataHandler() {
    pub fn getSheetDataHandler(&self) -> &SheetDataHandler {
        // java return this.sheetDataHandler;
        &self.sheetDataHandler
    }

    // java private SheetViewsHandler() {
    pub fn SheetViewsHandler(&self) {
    }

    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement__2(&self) {
        // java return subElement == 4841 ? new SheetViewHandler(WorksheetHandler.this.reader) : null;
        // rust: no-stmt
    }

    // java private OutlinePrHandle() {
    pub fn OutlinePrHandle(&self) {
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement__2(&self) {
        // java WorksheetHandler.this.isOutlinePrHandle = true;
        let _ = stringify!(isOutlinePrHandle);
        // rust: no-stmt
        // java boolean summaryBelow = true;
        let _ = stringify!(summaryBelow);
        // rust: no-stmt
        // java boolean summaryRight = true;
        let _ = stringify!(summaryRight);
        // rust: no-stmt
        // java XmlRoAttr tmp;
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4884)) != null) {
        // rust: no-stmt
        // java summaryBelow = tmp.toBoolean();
        // rust: no-stmt
        // java if (!summaryBelow) {
        // rust: no-stmt
        // java summaryBelow = false;
        let _ = stringify!(summaryBelow);
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4885)) != null) {
        // rust: no-stmt
        // java summaryRight = tmp.toBoolean();
        // rust: no-stmt
        // java if (!summaryRight) {
        // rust: no-stmt
        // java summaryRight = false;
        let _ = stringify!(summaryRight);
        // rust: no-stmt
        // java WSBoolRecord wSBoolRecord = WorksheetHandler.this.sheet.getWSBool();
        // rust: no-stmt
        // java wSBoolRecord.setRowSumsBelow(summaryBelow);
        // rust: no-stmt
        // java wSBoolRecord.setRowSumsRight(summaryRight);
        // rust: no-stmt
    }

    // java private PageSetupPrHandle() {
    pub fn PageSetupPrHandle(&self) {
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement__3(&self) {
        // java KmoPrintSetting pSetting = WorksheetHandler.this.sheet.getPrintSettings();
        // rust: no-stmt
        // java PrintSetupRecord pSetup = pSetting.getPrintSetup();
        // rust: no-stmt
        // java XmlRoAttr tmp;
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4887)) != null && tmp.toBoolean()) {
        // rust: no-stmt
        // java WorksheetHandler.this.sheet.getWSBool().setFitToPage(true);
        // rust: no-stmt
        // java pSetup.setFitWidth((short)1);
        // rust: no-stmt
        // java pSetup.setFitHeight((short)1);
        // rust: no-stmt
        // java if (attrsNotCopy.getByName(4886) != null) {
        // rust: no-stmt
    }

    // java private SheetPrHandler() {
    pub fn SheetPrHandler(&self) {
    }

    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement__3(&self) {
        // java if (subElement == 4830) {
        // rust: no-stmt
        // java return WorksheetHandler.this.tabColorHandler;
        // rust: no-stmt
        // java } else if (subElement == 4831) {
        // rust: no-stmt
        // java return WorksheetHandler.this.new OutlinePrHandle();
        // rust: no-stmt
        // java } else {
        // java return subElement == 4832 ? WorksheetHandler.this.new PageSetupPrHandle() : null;
        // rust: no-stmt
    }

    // java private TabColorHandler() {
    pub fn TabColorHandler(&self) {
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement__4(&self) {
        // java int index = ColorHelper.getColor(attrsNotCopy, 127);
        // rust: no-stmt
        // java if (index != 64) {
        // rust: no-stmt
        // java WorksheetHandler.this.sheet.setSheetTabColor(index);
        // rust: no-stmt
    }

    // java private HyperlinksHandler() {
    pub fn HyperlinksHandler(&self) {
    }

    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement__4(&self) {
        // java if (subElement == 4955) {
        // rust: no-stmt
        // java if (WorksheetHandler.this.hyperlinkHandler == null) {
        // rust: no-stmt
        // java WorksheetHandler.this.hyperlinkHandler = new HyperlinkHandler(WorksheetHandler.this.sheet, WorksheetHandler.this.reader.getHyperTargetMap());
        // rust: no-stmt
        // java return WorksheetHandler.this.hyperlinkHandler;
        // rust: no-stmt
        // java } else {
        // java return null;
        // rust: no-stmt
    }

    // java private DimensionHandler() {
    pub fn DimensionHandler(&self) {
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement__5(&self) {
        // java XmlRoAttr tmp;
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4100)) != null) {
        // rust: no-stmt
        // java String str = tmp.toString();
        // rust: no-stmt
        // java if (str.contains(":")) {
        // rust: no-stmt
        // java CellRangeAddress address = CellRangeAddress.valueOf(tmp.toString(), WorksheetHandler.this.sheet.getVersion());
        // rust: no-stmt
        // java int maxRow = address.getLastRow() > WorksheetHandler.this.sheet.getMaxRows() - 1 ? WorksheetHandler.this.sheet.getMaxRows() - 1 : address.getLastRow();
        // rust: no-stmt
        // java int maxCol = address.getLastColumn() > WorksheetHandler.this.sheet.getMaxColumns() - 1 ? WorksheetHandler.this.sheet.getMaxColumns() - 1 : address.getLastColumn();
        // rust: no-stmt
        // java WorksheetHandler.this.sheet.setUsedRange(address.getFirstRow(), address.getFirstColumn(), maxRow + 1, maxCol + 1);
        // rust: no-stmt
        // java } else {
        // java int y = ValueHelper.getRowIndex(str);
        // rust: no-stmt
        // java int x = ValueHelper.getColumnIndex(str);
        // rust: no-stmt
        // java WorksheetHandler.this.sheet.setUsedRange(y, x, y + 1, x + 1);
        // rust: no-stmt
    }

    // java private BackgroundPictureHandler() {
    pub fn BackgroundPictureHandler(&self) {
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement__6(&self) {
        // java XmlRoAttr tmp;
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4259841)) != null) {
        // rust: no-stmt
        // java WorksheetHandler.this.reader.setBackgroundRId(tmp.toString());
        // rust: no-stmt
    }

    // java private PrintOptionHandler() {
    pub fn PrintOptionHandler(&self) {
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement__7(&self) {
        // java KmoPrintSetting pSetting = WorksheetHandler.this.sheet.getPrintSettings();
        // rust: no-stmt
        // java PrintSetupRecord pSetup = pSetting.getPrintSetup();
        // rust: no-stmt
        // java XmlRoAttr tmp;
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4979)) != null) {
        // rust: no-stmt
        // java pSetting.setPrintGridLines(tmp.toBoolean());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4978)) != null) {
        // rust: no-stmt
        // java pSetting.setPrintHeaders(tmp.toBoolean());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4995)) != null) {
        // rust: no-stmt
        // java pSetup.setHResolution(tmp.toShort());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4996)) != null) {
        // rust: no-stmt
        // java pSetup.setVResolution(tmp.toShort());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4976)) != null) {
        // rust: no-stmt
        // java pSetting.setHCenter(tmp.toBoolean());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4977)) != null) {
        // rust: no-stmt
        // java pSetting.setVCenter(tmp.toBoolean());
        // rust: no-stmt
    }

    // java private PageMarginsHandler() {
    pub fn PageMarginsHandler(&self) {
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement__8(&self) {
        // java KmoPrintSetting pSetting = WorksheetHandler.this.sheet.getPrintSettings();
        // rust: no-stmt
        // java PrintSetupRecord pSetup = pSetting.getPrintSetup();
        // rust: no-stmt
        // java XmlRoAttr tmp;
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4670)) != null) {
        // rust: no-stmt
        // java pSetup.setHeaderMargin(tmp.toDouble());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4975)) != null) {
        // rust: no-stmt
        // java pSetup.setFooterMargin(tmp.toDouble());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4950)) != null) {
        // rust: no-stmt
        // java pSetting.setBottomMargin(tmp.toDouble());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4118)) != null) {
        // rust: no-stmt
        // java pSetting.setTopMargin(tmp.toDouble());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4974)) != null) {
        // rust: no-stmt
        // java pSetting.setRightMargin(tmp.toDouble());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4973)) != null) {
        // rust: no-stmt
        // java pSetting.setLeftMargin(tmp.toDouble());
        // rust: no-stmt
    }

    // java private PageSetupHandler() {
    pub fn PageSetupHandler(&self) {
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement__9(&self) {
        // java KmoPrintSetting pSetting = WorksheetHandler.this.sheet.getPrintSettings();
        // rust: no-stmt
        // java PrintSetupRecord pSetup = pSetting.getPrintSetup();
        // rust: no-stmt
        // java XmlRoAttr tmp;
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4988)) != null) {
        // rust: no-stmt
        // java String orientation = tmp.toString();
        // rust: no-stmt
        // java if (orientation.equalsIgnoreCase("landscape")) {
        // rust: no-stmt
        // java pSetup.setLandscape(false);
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4981)) != null) {
        // rust: no-stmt
        // java pSetup.setPaperSize(tmp.toShort());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4985)) != null) {
        // rust: no-stmt
        // java pSetup.setFitWidth(tmp.toShort());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4986)) != null) {
        // rust: no-stmt
        // java pSetup.setFitHeight(tmp.toShort());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4987)) != null) {
        // rust: no-stmt
        // java String order = tmp.toString();
        // rust: no-stmt
        // java pSetup.setLeftToRight(order.equalsIgnoreCase("overThenDown"));
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4919)) != null) {
        // rust: no-stmt
        // java pSetup.setScale(tmp.toShort());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4990)) != null) {
        // rust: no-stmt
        // java pSetup.setNoColor(tmp.toBoolean());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4997)) != null) {
        // rust: no-stmt
        // java long copies = tmp.toLong();
        // rust: no-stmt
        // java if (copies < 1L || copies > 32767L) {
        // rust: no-stmt
        // java copies = 1L;
        let _ = stringify!(copies);
        // rust: no-stmt
        // java pSetup.setCopies((short)((int)copies));
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4984)) != null) {
        // rust: no-stmt
        // java long firstPageNumber = tmp.toLong();
        // rust: no-stmt
        // java if (firstPageNumber < 1L || firstPageNumber > 32767L) {
        // rust: no-stmt
        // java firstPageNumber = 1L;
        let _ = stringify!(firstPageNumber);
        // rust: no-stmt
        // java pSetup.setPageStart((short)((int)firstPageNumber));
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4991)) != null) {
        // rust: no-stmt
        // java pSetup.setDraft(tmp.toBoolean());
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4994)) != null) {
        // rust: no-stmt
        // java pSetup.setErrors(PrintErrorMap.getError(tmp.toString()));
        // rust: no-stmt
        // java if ((tmp = attrsNotCopy.getByName(4992)) != null) {
        // rust: no-stmt
        // java String str = tmp.toString();
        // rust: no-stmt
        // java if (str.equals("asDisplayed")) {
        // rust: no-stmt
        // java pSetup.setNotes(true);
        // rust: no-stmt
        // java } else if (str.equals("atEnd")) {
        // rust: no-stmt
        // java pSetup.setNotes(true);
        // rust: no-stmt
        // java pSetup.setEndNotes(true);
        // rust: no-stmt
    }

}

// java private final class DimensionHandler extends DefaultElementHandler {
// java private DimensionHandler() {
// java }
// java
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(4100)) != null) {
// java String str = tmp.toString();
// java if (str.contains(":")) {
// java CellRangeAddress address = CellRangeAddress.valueOf(tmp.toString(), WorksheetHandler.this.sheet.getVersion());
// java int maxRow = address.getLastRow() > WorksheetHandler.this.sheet.getMaxRows() - 1 ? WorksheetHandler.this.sheet.getMaxRows() - 1 : address.getLastRow();
// java int maxCol = address.getLastColumn() > WorksheetHandler.this.sheet.getMaxColumns() - 1 ? WorksheetHandler.this.sheet.getMaxColumns() - 1 : address.getLastColumn();
// java WorksheetHandler.this.sheet.setUsedRange(address.getFirstRow(), address.getFirstColumn(), maxRow + 1, maxCol + 1);
// java } else {
// java int y = ValueHelper.getRowIndex(str);
// java int x = ValueHelper.getColumnIndex(str);
// java WorksheetHandler.this.sheet.setUsedRange(y, x, y + 1, x + 1);
// java }
// java }
// java
// java }
// java }
// java
struct DimensionHandler{sheet:Rc<RefCell<KmoSheet>>}
impl ElementHandler for DimensionHandler{
fn startElement(&self,_element:i32,a:&dyn XmlRoAttr){if let Some(v)=a.get_by_name(4100){let text=v.to_string();let s=self.sheet.borrow();if text.contains(':'){let address=crate::cn::wps::moss::filefmt::biff8::util::CellRangeAddress::CellRangeAddress::valueOf(&text,s.getVersion()).expect("Invalid cell range");let maxRow=address.getLastRow().min(s.getMaxRows()-1);let maxCol=address.getLastColumn().min(s.getMaxColumns()-1);s.setUsedRange(address.getFirstRow(),address.getFirstColumn(),maxRow+1,maxCol+1);}else{use crate::cn::wps::moss::xlsxr::util::ValueHelper::ValueHelper;let y=ValueHelper::getRowIndex(&text);let x=ValueHelper::getColumnIndex(&text);s.setUsedRange(y,x,y+1,x+1);}}}
fn endElement(&self,_element:i32){}fn characters(&self,_chars:&str){}fn enterSubElement(&self,_sub:i32)->Option<Box<dyn ElementHandler>>{None}}
