// java package cn.wps.moss.xlsxr.handler;

// java import cn.wps.kfc.xpxml.reader.ElementHandler;
// java import cn.wps.kfc.xpxml.reader.XmlRoAttr;
// java import cn.wps.moss.app.KmoBook;
// java import cn.wps.moss.xlsxr.xl.XlsxrWorkbook;
use crate::java_runtime::Cell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use crate::java_runtime::Rc;

use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::core::book::Window1::Window1;
use io_poi_compact::cn::wps::encrypt::ModifyVerifier::ModifyVerifier;
use io_poi_compact::cn::wps::encrypt::CryptTool::CryptTool;
use crate::cn::wps::moss::filefmt::biff8::record::FileSharingRecord::FileSharingRecord;
use super::super::util::ValueHelper::ValueHelper;
use crate::cn::wps::moss::filefmt::biff8::record::WindowOneRecord::WindowOneRecord;
use crate::cn::wps::moss::xlsxr::xl::XlsxrWorkbook::XlsxrWorkbook;
use super::WorkBookExtLstHandler::WorkBookExtLstHandler;
use super::FileVersionHandler::FileVersionHandler;
use crate::cn::wps::moss::app::FileVersion::FileVersion;
use crate::java_runtime::RefCell;

// java private static final String TAG = null;
const TAG: &str = "";
#[cfg(test)]
#[path="WorkbookHandler.review.rs"]
mod review;

struct ParkHandler;

impl ElementHandler for ParkHandler {
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

// java public final class WorkbookHandler extends DefaultElementHandler
pub struct WorkbookHandler {
    mBook: Rc<KmoBook>,
    mReader: Rc<XlsxrWorkbook>,
    mActiveTab: Rc<Cell<i32>>,
}

impl WorkbookHandler {
    // java public WorkbookHandler(KmoBook book, XlsxrWorkbook reader) {
    pub fn WorkbookHandler(book: Rc<KmoBook>, reader: Rc<XlsxrWorkbook>) -> Self {
        // java this.mBook = book;
        let mBook = book;
        // java this.mReader = reader;
        let mReader = reader;
        Self {
            mBook,
            mReader,
            mActiveTab: Rc::new(Cell::new(0)),
        }
    }

    // java public void endElement(int element) {
    pub fn endElement(&self, element: i32) {
        let _ = element;
        // java try {
        let caught = catch_unwind(AssertUnwindSafe(|| {
            // java if (this.mActiveTab > 0) {
            if self.mActiveTab.get() > 0 {
                // java this.mBook.activateSheet(this.mActiveTab);
                self.mBook.activateSheet(self.mActiveTab.get());
            }
        }));
        // java } catch (Throwable e) {
        if caught.is_err() {
            // java IOLog.e(TAG, "Throwable", e);
            let _ = TAG;
        }
    }

    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java switch (subElement) {
        match subElement {
            // java case 4099:
            4099 => {
                // java return new WorkBookExtLstHandler(this.mBook.getWorkbookPr());
                Some(Box::new(WorkBookExtLstHandler::WorkBookExtLstHandler(self.mBook.getWorkbookPr())))
            }
            // java case 5199:
            5199 => {
                // java this.mReader.setHasDefinedNames(true);
                self.mReader.setHasDefinedNames(true);
                // java return null;
                None
            }
            // java case 5253:
            5253 => {
                // java FileVersion fileVersion = new FileVersion();
                let file_version = Rc::new(RefCell::new(FileVersion::default()));
                // java this.mBook.setFileVersion(fileVersion);
                self.mBook.setFileVersion(Rc::clone(&file_version));
                // java return new FileVersionHandler(fileVersion);
                Some(Box::new(FileVersionHandler::FileVersionHandler(file_version)))
            }
            // java case 5254:
            5254 => {
                // java return new FileSharingHandler();
                Some(Box::new(FileSharingHandler::FileSharingHandler(self.mBook.clone(),self.mReader.clone())))
            }
            // java case 5255:
            5255 => {
                // java return new BookPrHandler();
                Some(Box::new(BookPrHandler { book: Rc::clone(&self.mBook) }))
            }
            // java case 5256:
            5256 => {
                // java return new BookProtectionHandler();
                Some(Box::new(BookProtectionHandler{book:self.mBook.clone()}))
            }
            // java case 5257:
            5257 => {
                // java return new BookViewsHandler();
                Some(Box::new(BookViewsHandler {
                    book:Rc::clone(&self.mBook),
                    activeTab: Rc::clone(&self.mActiveTab),
                }))
            }
            // java case 5258:
            5258 => {
                // java return new SheetsHandler();
                Some(Box::new(SheetsHandler::SheetsHandler(
                    Rc::clone(&self.mBook),
                    Rc::clone(&self.mReader),
                )))
            }
            // java case 5260:
            5260 => {
                // java return new ExternalReferencesHandler(this.mReader);
                Some(super::book::ExternalReferencesHandler::ExternalReferencesHandler::ExternalReferencesHandler(self.mReader.clone()).asElementHandler())
            }
            // java case 5261:
            5261 => {
                // java return new CalcPrHandler();
                Some(Box::new(CalcPrHandler { book: Rc::clone(&self.mBook) }))
            }
            // java case 5264:
            5264 => {
                // java return new PivotCachesHandler(this.mReader);
                // rust: no-stmt — PivotCachesHandler 不在本 function。登记 §1.7.1。
                Some(Box::new(ParkHandler))
            }
            // java default:
            _ => {
                // java return null;
                None
            }
        }
    }
}

impl ElementHandler for WorkbookHandler {
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {}
    fn endElement(&self, element: i32) {
        WorkbookHandler::endElement(self, element);
    }
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        WorkbookHandler::enterSubElement(self, subElement)
    }
}

struct SheetsHandler {
    book:Rc<KmoBook>,
    reader: Rc<XlsxrWorkbook>,
    stHandler: SheetHandler,
}

impl SheetsHandler {
    // java private SheetsHandler() {
    fn SheetsHandler(book: Rc<KmoBook>, reader: Rc<XlsxrWorkbook>) -> Self {
        // java this.stHandler = WorkbookHandler.this.new SheetHandler();
        let stHandler = SheetHandler::SheetHandler(book.clone(), Rc::clone(&reader));
        Self { book,reader, stHandler }
    }

    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java return subElement == 4329 ? this.stHandler : null;
        if subElement == 4329 {
            Some(Box::new(self.stHandler.clone()))
        } else {
            None
        }
    }

    // java public void endElement(int element) {
    pub fn endElement(&self, element: i32) {
        let _ = element;
        // java WorkbookHandler.this.mReader.getBookOpenListener().bookDidFinishInitializing(WorkbookHandler.this.mBook);
        self.reader.getBookOpenListener().expect("NullPointerException").bookDidFinishInitializing(&self.book);
        // java WorkbookHandler.this.mBook.changeListenerGroup().bookDidFinishInitializing();
        self.book.changeListenerGroup().bookDidFinishInitializing();
    }
}

impl ElementHandler for SheetsHandler {
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {}
    fn endElement(&self, element: i32) {
        SheetsHandler::endElement(self, element);
    }
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        SheetsHandler::enterSubElement(self, subElement)
    }
}

#[derive(Clone)]
struct SheetHandler {
    reader: Rc<XlsxrWorkbook>,
}

impl SheetHandler {
    // java private SheetHandler() {
    fn SheetHandler(_book: Rc<KmoBook>, reader: Rc<XlsxrWorkbook>) -> Self {
        Self { reader }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _ = element;
        // java XmlRoAttr tmp = attrsNotCopy.getByName(4259841);
        let mut rid = attrsNotCopy.get_by_name(4259841).map(|tmp| tmp.to_string());
        // java if (tmp == null) {
        if rid.is_none() {
            // java tmp = attrsNotCopy.getByName(4212);
            rid = attrsNotCopy.get_by_name(4212).map(|tmp| tmp.to_string());
            // java if (null == tmp) {
            if rid.is_none() {
                // java return;
                return;
            }
        }
        // java String rId = tmp.toString();
        let rId = rid.unwrap();
        // java tmp = attrsNotCopy.getByName(4217);
        let name_attr = attrsNotCopy.get_by_name(4217);
        // java if (tmp != null) {
        if let Some(tmp) = name_attr {
            // java String sheetName = tmp.toString();
            let sheetName = tmp.to_string();
            // java int sheetId = -1;
            let mut sheetId: i32 = -1;
            // java if ((tmp = attrsNotCopy.getByName(4704)) != null) {
            if let Some(tmp) = attrsNotCopy.get_by_name(4704) {
                // java sheetId = tmp.toInt();
                sheetId = tmp.to_int();
            }
            // java KmoSheet sheet = WorkbookHandler.this.mReader.addSheetRid(sheetName, rId, sheetId);
            let sheet = self.reader.addSheetRid(&sheetName, &rId, sheetId);
            // java if (-1 != sheetId) {
            if sheetId != -1 {
                // java WorkbookHandler.this.mReader.addSheetId2Sheet(sheetId, sheet);
                self.reader.addSheetId2Sheet(sheetId, &sheet);
            }
            // java if ((tmp = attrsNotCopy.getByName(4867)) != null) {
            if let Some(tmp) = attrsNotCopy.get_by_name(4867) {
                // java String state = tmp.toString();
                let state = tmp.to_string();
                // java if (state.equals("hidden")) {
                if state == "hidden" {
                    // java sheet.setSheetHidden(true);
                    sheet.borrow().setSheetHidden(true);
                // java } else if (state.equals("veryHidden")) {
                } else if state == "veryHidden" {
                    // java sheet.setSheetVeryHidden(true);
                    sheet.borrow().setSheetVeryHidden(true);
                }
            }
        }
    }
}

impl ElementHandler for SheetHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        SheetHandler::startElement(self, element, attrsNotCopy);
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

struct FileSharingHandler{book:Rc<KmoBook>,reader:Rc<XlsxrWorkbook>}

impl FileSharingHandler {
    // java public FileSharingHandler() {
    pub fn FileSharingHandler(book:Rc<KmoBook>,reader:Rc<XlsxrWorkbook>) -> Self {
        // rust: captures of the original non-static inner class's outer workbook.
        Self{book,reader}
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _=element;
        // java short hash = 0;
        let mut hash=0i16;
        // java FileSharingRecord sharingRec = new FileSharingRecord();
        let sharingRec=Rc::new(FileSharingRecord::new());
        // java XmlRoAttr tmp;
        // java if ((tmp = attrsNotCopy.getByName(5344)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5344){
            // java hash = (short)ValueHelper.hexStringToInt(tmp.toString());
            hash=ValueHelper::hexStringToInt(Some(&tmp.to_string()))as i16;
            // java KmoBookOpenPasswordProvider passProvider = WorkbookHandler.this.mReader.getPasswordProvider();
            let passProvider=self.reader.getPasswordProvider();
            // java for(boolean bVerify = false; !bVerify; passProvider.verifyWritePassword(bVerify)) {
            let mut bVerify=false;while !bVerify{
                // java String str = passProvider.getWritePassword(false);
                let str=passProvider.as_ref().expect("NullPointerException").getWritePassword(false);
                // java if (str == null) { bVerify = true; break; }
                if str.is_none(){bVerify=true;break;}
                // java short inputHash = FileSharingRecord.hashPassword(str);
                let inputHash=FileSharingRecord::hashPassword(str.as_deref());
                // java if (hash == inputHash) { bVerify = true; }
                if hash==inputHash{bVerify=true;}
                // java passProvider.verifyWritePassword(bVerify)
                passProvider.as_ref().expect("NullPointerException").verifyWritePassword(bVerify);
            }
            // java sharingRec.setPassword(hash);
            sharingRec.setPassword(hash);
        }else{
        // java if (attrsNotCopy.getByName(5013) == null || attrsNotCopy.getByName(5014) == null || attrsNotCopy.getByName(5015) == null) {
            if attrsNotCopy.get_by_name(5013).is_none()||attrsNotCopy.get_by_name(5014).is_none()||attrsNotCopy.get_by_name(5015).is_none(){
                // java return;
                return;
            }
            // java String hashValue = attrsNotCopy.getByName(5013).toString();
            let hashValue=attrsNotCopy.get_by_name(5013).unwrap().to_string();
            // java String saltValue = attrsNotCopy.getByName(5014).toString();
            let saltValue=attrsNotCopy.get_by_name(5014).unwrap().to_string();
            // java int spinCount = attrsNotCopy.getByName(5015).toInt();
            let spinCount=attrsNotCopy.get_by_name(5015).unwrap().to_int();
            // java ModifyVerifier verifier = new ModifyVerifier(hashValue, saltValue, spinCount, 14);
            let verifier=Rc::new(RefCell::new(ModifyVerifier::new_with_algorithm(&hashValue,&saltValue,spinCount,14)));
            // java KmoBookOpenPasswordProvider passProvider = WorkbookHandler.this.mReader.getPasswordProvider();
            let passProvider=self.reader.getPasswordProvider();
            // java for(boolean bVerify = false; !bVerify; passProvider.verifyWritePassword(bVerify)) {
            let mut bVerify=false;while !bVerify{
                // java String str = passProvider.getWritePassword(true);
                let str=passProvider.as_ref().expect("NullPointerException").getWritePassword(true);
                // java if (str == null) { bVerify = true; break; }
                if str.is_none(){bVerify=true;break;}
                // java if (CryptTool.checkModifyPassword(verifier, str)) {
                if CryptTool::checkModifyPassword__nullable(Some(&verifier.borrow()),str.as_deref()){
                    // java bVerify = true;
                    bVerify=true;
                    // java sharingRec.setPassword(verifier);
                    sharingRec.setPassword__verifier(Some(verifier.clone()));
                    // java sharingRec.setPassword(FileSharingRecord.hashPassword(str));
                    sharingRec.setPassword(FileSharingRecord::hashPassword(str.as_deref()));
                }
                // java passProvider.verifyWritePassword(bVerify)
                passProvider.as_ref().expect("NullPointerException").verifyWritePassword(bVerify);
            }
        }
        // java if ((tmp = attrsNotCopy.getByName(5343)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5343){
            // java sharingRec.setReadOnly(tmp.toShort());
            sharingRec.setReadOnly(tmp.to_short());
        }
        // java if ((tmp = attrsNotCopy.getByName(4701)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(4701){
            // java sharingRec.setUsername(tmp.toString());
            sharingRec.setUsername(Some(&tmp.to_string()));
        }else{
            // java sharingRec.setUsername("");
            sharingRec.setUsername(Some(""));
        }
        // java WorkbookHandler.this.mBook.getCryptInfo().setFileSharing(sharingRec);
        self.book.getCryptInfo().setFileSharing(Some(sharingRec));
    }
}

impl ElementHandler for FileSharingHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        FileSharingHandler::startElement(self, element, attrsNotCopy);
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

struct BookViewsHandler {
    book:Rc<KmoBook>,
    activeTab: Rc<Cell<i32>>,
}

impl BookViewsHandler {
    // java private BookViewsHandler() {
    fn BookViewsHandler() {}

    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java return subElement == 5275 ? WorkbookHandler.this.new WorkbookViewHandler() : null;
        if subElement == 5275 {
            Some(Box::new(WorkbookViewHandler {
                book:Rc::clone(&self.book),
                activeTab: Rc::clone(&self.activeTab),
            }))
        } else {
            None
        }
    }
}

impl ElementHandler for BookViewsHandler {
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {}
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        BookViewsHandler::enterSubElement(self, subElement)
    }
}

struct WorkbookViewHandler {
    book:Rc<KmoBook>,
    activeTab: Rc<Cell<i32>>,
}

impl WorkbookViewHandler {
    // java private WorkbookViewHandler() {
    fn WorkbookViewHandler() {}

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _ = element;
        // java XmlRoAttr tmp;
        // java if ((tmp = attrsNotCopy.getByName(5285)) != null) {
        if let Some(tmp) = attrsNotCopy.get_by_name(5285) {
            // java WorkbookHandler.this.mActiveTab = tmp.toInt();
            self.activeTab.set(tmp.to_int());
        }
        // java WindowOneRecord win = new WindowOneRecord();
        let win=WindowOneRecord::new();
        // java if ((tmp = attrsNotCopy.getByName(5284)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5284){
            // java win.setFirstVisibleTab(tmp.toInt());
            win.setFirstVisibleTab(tmp.to_int());
        }
        // java if ((tmp = attrsNotCopy.getByName(5282)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5282){
            // java win.setHeight(tmp.toShort());
            win.setHeight(tmp.to_short());
        }
        // java if ((tmp = attrsNotCopy.getByName(5281)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5281){
            // java win.setWidth(tmp.toShort());
            win.setWidth(tmp.to_short());
        }
        // java if ((tmp = attrsNotCopy.getByName(4931)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(4931){
            // java win.setHorizontalHold(tmp.toShort());
            win.setHorizontalHold(tmp.to_short());
        }
        // java if ((tmp = attrsNotCopy.getByName(4932)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(4932){
            // java win.setVerticalHold(tmp.toShort());
            win.setVerticalHold(tmp.to_short());
        }
        // java if ((tmp = attrsNotCopy.getByName(5280)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5280){
            // java win.setDisplayTabs(tmp.toBoolean());
            win.setDisplayTabs(tmp.to_boolean());
        }else{
            // java win.setDisplayTabs(true);
            win.setDisplayTabs(true);
        }
        // java if ((tmp = attrsNotCopy.getByName(5279)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5279){
            // java win.setDisplayVerticalScrollbar(tmp.toBoolean());
            win.setDisplayVerticalScrollbar(tmp.to_boolean());
        }else{
            // java win.setDisplayVerticalScrollbar(true);
            win.setDisplayVerticalScrollbar(true);
        }
        // java if ((tmp = attrsNotCopy.getByName(5278)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5278){
            // java win.setDisplayHorizonalScrollbar(tmp.toBoolean());
            win.setDisplayHorizonalScrollbar(tmp.to_boolean());
        }else{
            // java win.setDisplayHorizonalScrollbar(true);
            win.setDisplayHorizonalScrollbar(true);
        }
        // java if ((tmp = attrsNotCopy.getByName(5283)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5283){
            // java win.setTabWidthRatio(tmp.toShort());
            win.setTabWidthRatio(tmp.to_short());
        }else{
            // java win.setTabWidthRatio((short)600);
            win.setTabWidthRatio(600);
        }
        // java WorkbookHandler.this.mBook.setWindow1(new Window1(win));
        self.book.setWindow1(Some(Rc::new(Window1::new__record(Some(&win)))));
    }
}

impl ElementHandler for WorkbookViewHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        WorkbookViewHandler::startElement(self, element, attrsNotCopy);
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

struct BookProtectionHandler{book:Rc<KmoBook>}

impl BookProtectionHandler {
    // java private BookProtectionHandler() {
    fn BookProtectionHandler() {}

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _=element;
        // java KmoBookProtect protect = WorkbookHandler.this.mBook.getBookProtect();
        let protect=self.book.getBookProtect();
        // java XmlRoAttr tmp;
        // java if ((tmp = attrsNotCopy.getByName(5345)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5345){
            // java protect.setPassword(ValueHelper.hexStringToInt(tmp.toString()));
            protect.setPassword(ValueHelper::hexStringToInt(Some(&tmp.to_string())));
        }
        // java if ((tmp = attrsNotCopy.getByName(5349)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5349){
            // java protect.setLockStructure(tmp.toBoolean());
            protect.setLockStructure(tmp.to_boolean());
        }
        // java if ((tmp = attrsNotCopy.getByName(5350)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5350){
            // java protect.setLockWindow(tmp.toBoolean());
            protect.setLockWindow(tmp.to_boolean());
        }
        // java if ((tmp = attrsNotCopy.getByName(5351)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5351){
            // java protect.setLockRevion(tmp.toBoolean());
            protect.setLockRevion(tmp.to_boolean());
        }
        // java if ((tmp = attrsNotCopy.getByName(5347)) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5347){
            // java protect.setRevisionPassword(ValueHelper.hexStringToInt(tmp.toString()));
            protect.setRevisionPassword(ValueHelper::hexStringToInt(Some(&tmp.to_string())));
        }
        // java if ((tmp = attrsNotCopy.getByName(5356)) != null && "SHA-512".endsWith(tmp.toString()) && attrsNotCopy.getByName(5357) != null && attrsNotCopy.getByName(5358) != null && attrsNotCopy.getByName(5359) != null) {
        if let Some(tmp)=attrsNotCopy.get_by_name(5356){if "SHA-512".ends_with(&tmp.to_string())&&attrsNotCopy.get_by_name(5357).is_some()&&attrsNotCopy.get_by_name(5358).is_some()&&attrsNotCopy.get_by_name(5359).is_some(){
            // java String hashValue = attrsNotCopy.getByName(5357).toString();
            let hashValue=attrsNotCopy.get_by_name(5357).unwrap().to_string();
            // java String saltValue = attrsNotCopy.getByName(5358).toString();
            let saltValue=attrsNotCopy.get_by_name(5358).unwrap().to_string();
            // java int spinCount = attrsNotCopy.getByName(5359).toInt();
            let spinCount=attrsNotCopy.get_by_name(5359).unwrap().to_int();
            // java ModifyVerifier verifier = new ModifyVerifier(hashValue, saltValue, spinCount, 14);
            let verifier=Rc::new(RefCell::new(ModifyVerifier::new_with_algorithm(&hashValue,&saltValue,spinCount,14)));
            // java protect.setPassword(verifier);
            protect.setPassword__verifier(Some(verifier));
        }}
    }
}

impl ElementHandler for BookProtectionHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        BookProtectionHandler::startElement(self, element, attrsNotCopy);
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

struct BookPrHandler { book: Rc<KmoBook> }

impl BookPrHandler {
    // java private BookPrHandler() {
    fn BookPrHandler() {}

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _ = element;
        // java if ((tmp = attrsNotCopy.getByName(5301)) != null) {
        if let Some(tmp) = attrsNotCopy.get_by_name(5301) {
            // java mBook.setUsing1904(tmp.toBoolean());
            self.book.setUsing1904(tmp.to_boolean());
        }
        // java tmp = attrsNotCopy.getByName(5317);
        if let Some(tmp) = attrsNotCopy.get_by_name(5317) {
            // java mBook.getWorkbookPr().setDefaultThemeVersion(tmp.toString());
            self.book.getWorkbookPr().borrow_mut().setDefaultThemeVersion(tmp.to_string());
        }
    }
}

impl ElementHandler for BookPrHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        BookPrHandler::startElement(self, element, attrsNotCopy);
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

struct CalcPrHandler { book: Rc<KmoBook> }

impl CalcPrHandler {
    // java private CalcPrHandler() {
    fn CalcPrHandler() {}

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _ = element;
        let calc = self.book.getCalcPr();
        // Java: each present calcPr attribute updates the matching CalcPr property.
        if let Some(x) = attrsNotCopy.get_by_name(5332) { calc.borrow_mut().setCalcCompleted(x.to_boolean()); }
        if let Some(x) = attrsNotCopy.get_by_name(5325) { calc.borrow_mut().setCalcId(x.to_int()); }
        if let Some(x) = attrsNotCopy.get_by_name(5326) { calc.borrow_mut().setCalcMode(x.to_string()); }
        if let Some(x) = attrsNotCopy.get_by_name(5333) { calc.borrow_mut().setCalcOnSave(x.to_boolean()); }
        if let Some(x) = attrsNotCopy.get_by_name(5334) { calc.borrow_mut().setConcurrentCalc(x.to_boolean()); }
        if let Some(x) = attrsNotCopy.get_by_name(5335) { calc.borrow_mut().setConcurrentManualcCount(x.to_int()); }
        if let Some(x) = attrsNotCopy.get_by_name(5336) { calc.borrow_mut().setForceFullCalc(x.to_boolean()); }
        if let Some(x) = attrsNotCopy.get_by_name(4803) { calc.borrow_mut().setFullCalcOnLoad(x.to_boolean()); }
        if let Some(x) = attrsNotCopy.get_by_name(5331) { calc.borrow_mut().setFullPrecision(x.to_boolean()); }
        if let Some(x) = attrsNotCopy.get_by_name(5328) { calc.borrow_mut().setIterate(x.to_boolean()); }
        if let Some(x) = attrsNotCopy.get_by_name(5329) { calc.borrow_mut().setIterateCount(x.to_int()); }
        if let Some(x) = attrsNotCopy.get_by_name(5330) { calc.borrow_mut().setIterateDelta(x.to_double()); }
        if let Some(x) = attrsNotCopy.get_by_name(5327) { calc.borrow_mut().setRefMode(x.to_string()); }
    }
}

impl ElementHandler for CalcPrHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        CalcPrHandler::startElement(self, element, attrsNotCopy);
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}
