// java package cn.wps.moss.xlsxr.xl;

// java import cn.wps.kfc.opcr.OpcrRelationships;
// java import cn.wps.moss.app.KmoBook;
// java import cn.wps.moss.app.KmoSheet;
// java import cn.wps.moss.xlsxr.handler.WorkbookHandler;
// java import cn.wps.moss.xlsxr.util.UnifiedXmlReader;
// java import cn.wps.moss.xlsxr.xl.worksheets.XlsxrSheets;
use crate::java_runtime::{Cell, RefCell};
use std::collections::HashMap;
use std::io::{self, Read};
use std::panic::{catch_unwind, AssertUnwindSafe};
use crate::java_runtime::Rc;

use kfc_kfc_alpha::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
use kfc_kfc_alpha::kfc::opcr::OpcrRelationships::OpcrRelationships;

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::xlsxr::handler::WorkbookHandler::WorkbookHandler;
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use crate::cn::wps::moss::xlsxr::util::UnifiedXmlReader::UnifiedXmlReader;
use crate::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use crate::cn::wps::moss::xlsxr::xl::XlsxrSharedStrings::XlsxrSharedStrings;
use crate::cn::wps::moss::xlsxr::xl::XlsxrStyles::XlsxrStyles;
use crate::cn::wps::moss::xlsxr::xl::worksheets::XlsxrSheets::XlsxrSheets;
use crate::cn::wps::moss::{app::event::{KmoBookOpenEventListener::KmoBookOpenEventListener,KmoBookOpenPasswordProvider::KmoBookOpenPasswordProvider},xlsxr::{xl::externalLinks::XlsxrExternalLinks::XlsxrExternalLinks,handler::WorkbookDelayHandler::WorkbookDelayHandler}};
pub type RidList=Rc<RefCell<Vec<Option<String>>>>;

fn panic_text(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(text) = payload.downcast_ref::<&str>() {
        return (*text).to_string();
    }
    if let Some(text) = payload.downcast_ref::<String>() {
        return text.clone();
    }
    String::new()
}

#[cfg(test)]
#[path="XlsxrWorkbook.review.rs"]
mod review;

// OxfXlRelations 的 getRelsType()。类本身还不在编译面。
const THEME_RELS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
const STYLES_RELS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
const SHARED_STRINGS_RELS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings";
const CELL_IMAGES_RELS: &str = "http://www.wps.cn/officeDocument/2020/cellImage";

fn index_of(relses: &OpcrRelationships, rels_type: &str) -> Option<i32> {
    let mut i = 0i32;
    let size = relses.size();
    while i < size {
        if relses.getRelationship(i).getType().as_deref().expect("NullPointerException: relationship Type") == rels_type {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn is_zip_exception(err: &io::Error) -> bool {
    let msg = err.to_string();
    msg.contains("ZipException") || msg.contains("zip error")
}

// rust: MemoryToolKit.getCpuNumber。目录不存在时返回 0，和 Java 在 macOS 上一样走 openGlobalData。
pub fn memoryToolKitCpuNumber() -> i32 {
    let dir = std::path::Path::new("/sys/devices/system/cpu/");
    if !dir.is_dir() {
        return 0;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(rd) => rd.filter_map(|ent| ent.ok()).collect::<Vec<_>>(),
        Err(_) => return 0,
    };
    if entries.is_empty() {
        return 0;
    }
    let mut ret = 0i32;
    for ent in entries {
        let name = ent.file_name();
        let name = name.to_string_lossy();
        if let Some(rest) = name.strip_prefix("cpu") {
            if let Ok(n) = rest.parse::<i32>() {
                if n > ret {
                    ret = n;
                }
            }
        }
    }
    ret + 1
}

// java public final class XlsxrWorkbook
pub struct XlsxrWorkbook {
    mBook: Rc<KmoBook>,
    mBookPart: Rc<RefCell<Option<OpcrPackagePart>>>,
    // java private KmoBookOpenEventListener mOpenListener;
    mOpenListener:Option<Rc<dyn KmoBookOpenEventListener>>,
    // java private KmoBookOpenPasswordProvider mPassProvider;
    mPassProvider:Option<Rc<dyn KmoBookOpenPasswordProvider>>,
    mSheetsReader: RefCell<Option<Rc<RefCell<XlsxrSheets>>>>,
    bHasDefinedNames: Cell<bool>,
    // java private List<String> mRids;
    mRids:RefCell<Option<RidList>>,
    // java private XlsxrExternalLinks mExtLinksReader;
    mExtLinksReader:RefCell<Option<XlsxrExternalLinks<'static>>>,
    mStylesTable: RefCell<Option<Rc<RefCell<StylesTable>>>>,
    mSheetId2Sheet: RefCell<HashMap<i32, Rc<RefCell<KmoSheet>>>>,
}

impl XlsxrWorkbook {
    // java public XlsxrWorkbook(KmoBook book, OpcrPackagePart part, KmoBookOpenEventListener openListener, KmoBookOpenPasswordProvider passProvider) {
    pub fn XlsxrWorkbook(book: Rc<KmoBook>,part:Option<OpcrPackagePart>,openListener:Option<Rc<dyn KmoBookOpenEventListener>>,passProvider:Option<Rc<dyn KmoBookOpenPasswordProvider>>) -> Rc<Self> {
        // java this.mBook = book; this.mBookPart = part; this.mOpenListener = openListener; this.mPassProvider = passProvider;
        let mBook = book;
        Rc::new(Self {
            mBook,
            mBookPart: Rc::new(RefCell::new(part)),
            mOpenListener:openListener,
            mPassProvider:passProvider,
            mSheetsReader: RefCell::new(None),
            mStylesTable: RefCell::new(None),
            bHasDefinedNames: Cell::new(false),
            mRids:RefCell::new(None),
            mExtLinksReader:RefCell::new(None),
            mSheetId2Sheet: RefCell::new(HashMap::new()),
        })
    }

    // Native low-level review adapter; normal opens pass the part to the constructor.
    pub fn setBookPart(&self, part: OpcrPackagePart) {
        *self.mBookPart.borrow_mut() = Some(part);
    }

    pub fn stylesTable(&self) -> Option<Rc<RefCell<StylesTable>>> {
        self.mStylesTable.borrow().clone()
    }

    // rust: open() 创建 mSheetsReader 之后，openGlobalData 用 WorkbookHandler 读 workbook 流。09 再把样式和共享字符串接回 open。
    pub fn readDeclaredSheets(
        this: &Rc<Self>,
        relses: &OpcrRelationships,
        stream: &mut dyn Read,
    ) -> std::io::Result<()> {
        // java this.mSheetsReader = new XlsxrSheets(this.mBook, relses, this, this.mStylesTable);
        let sheets = XlsxrSheets::XlsxrSheets(Rc::clone(&this.mBook), relses, Rc::downgrade(this),None);
        *this.mSheetsReader.borrow_mut() = Some(sheets);
        // java WorkbookHandler handler = new WorkbookHandler(this.mBook, this);
        let handler = WorkbookHandler::WorkbookHandler(Rc::clone(&this.mBook), Rc::clone(this));
        // java UnifiedXmlReader.read((InputStream)stream, handler);
        UnifiedXmlReader::read(stream, Box::new(handler))
    }

    pub fn hasDefinedNames(&self) -> bool {
        self.bHasDefinedNames.get()
    }

    pub fn relationshipId(&self, index: i32) -> Option<String> {
        let sheet = self.mBook.sheetAt(index);
        let uid = sheet.borrow().getSheetUid();
        self.mSheetsReader
            .borrow()
            .as_ref()
            .and_then(|sheets| sheets.borrow().rid_of(uid))
    }

    pub fn formulaImportKinds(&self, sheet_index: i32) -> Vec<&'static str> {
        self.mSheetsReader
            .borrow()
            .as_ref()
            .map(|sheets| sheets.borrow().formulaImportKinds(sheet_index))
            .unwrap_or_default()
    }

    pub fn delayedImportOrder(&self) -> Vec<i32> {
        self.mSheetsReader
            .borrow()
            .as_ref()
            .map(|sheets| sheets.borrow().delayedImportOrder())
            .unwrap_or_default()
    }

    pub fn sheetReaderSheetCount(&self) -> i32 {
        self.mSheetsReader
            .borrow()
            .as_ref()
            .map(|sheets| sheets.borrow().sheetReaderSheetCount())
            .unwrap_or(0)
    }

    // java public void open() throws IOException {
    pub fn open(this: &Rc<Self>) -> io::Result<()> {
        // rust: no-stmt — XlsxrImages.initialize 不在本 function。登记 §1.7.1。
        let table = Rc::new(RefCell::new(StylesTable::StylesTable()));
        *this.mStylesTable.borrow_mut() = Some(Rc::clone(&table));
        let sheets = {
            let mut slot = this.mBookPart.borrow_mut();
            let part = slot.as_mut().expect("NullPointerException");
            let relses = part
                .getRelationships_mut()?
                .expect("NullPointerException");
            XlsxrSheets::XlsxrSheets(
                Rc::clone(&this.mBook),
                relses,
                Rc::downgrade(this),
                Some(Rc::clone(&table)),
            )
        };
        *this.mSheetsReader.borrow_mut() = Some(sheets);
        // rust: no-stmt — PivotCachesReader 不在本 function。登记 §1.7.1。
        if memoryToolKitCpuNumber() < 4 {
            Self::openGlobalData(this)?;
        } else {
            Self::openGlobalDataMulti(this)?;
        }
        {
            let mut slot = this.mBookPart.borrow_mut();
            let part = slot.as_mut().expect("NullPointerException");
            let relses = part
                .getRelationships_mut()?
                .expect("NullPointerException");
            let _cell_images = index_of(relses, CELL_IMAGES_RELS);
            // rust: no-stmt — XlsxrCellImage.open 不在本 function。Throwable 在 Java 里被吃掉。登记 §1.7.1。
        }
        let sheet_count = this.mBook.sheetCount();
        let reader_count = this.sheetReaderSheetCount();
        /* DEFERRED_NON_FUNCTION
        // java XlsxrImages.initialize()
        // java this.mStylesTable = new StylesTable()
        // java OpcrRelationships relses = this.mBookPart.getRelationships()
        // java this.mSheetsReader = new XlsxrSheets(this.mBook, relses, this, this.mStylesTable)
        // java this.mPivotCacheReader = new PivotCachesReader(this.mBookPart, this.mBook)
        // java if (MemoryToolKit.getCpuNumber() < 4) {
        // java this.openGlobalData(relses)
        // java } else {
        // java XlsxrGlobalDataMultiThreads globalReader = new XlsxrGlobalDataMultiThreads(this.mBook, this.mBookPart, this, this.mStylesTable)
        // java globalReader.open()
        // java OpcrRelationship cellImageRels = relses.getRelationshipByType(OxfXlRelations.CELL_IMAGES.getRelsType())
        // java try {
        // java XlsxrCellImage xlsxrCellImage = new XlsxrCellImage(this.mBook, cellImageRels)
        // java xlsxrCellImage.open()
        // java } catch (Throwable e) {
        // java IOLog.d(TAG, "Throwable", e)
        // java if (this.mBook.sheetCount() != 0 && this.mSheetsReader.sheetReaderSheetCount() != 0) {
        // java int idx = 0
        // java for(int count = this.mBook.sheetCount(); idx < count; ++idx) {
        // java KmoSheet sheet = this.mBook.sheetAt(idx)
        // java sheet.changeListenerGroup().startMergeEvents()
        // java if (sheet.getGrid() != null && sheet.getGrid().getMatrix() != null) {
        // java sheet.getGrid().getMatrix().startCmdMode()
        // java boolean var13 = false
        // java try {
        // java var13 = true
        // java this.mSheetsReader.open()
        // java if (this.mBook.getPivotTable().getCacheDefines().size() > 0) {
        // java this.readConnections(relses)
        // java this.mPivotCacheReader.open()
        // java this.mSheetsReader.readPivotTable()
        // java this.mBook.getKmoConnections().clearOlapIds()
        // java if (!XlsxrPackage.isLargestFile()) {
        // java try {
        // java this.mSheetsReader.delayedImportFormula()
        // java var13 = false
        // java } catch (OutOfMemoryError e) {
        // java XlsxrPackage.setLargestFile()
        // java this.mSheetsReader.cleanSheetsMemory()
        // java System.gc()
        // java MemoryToolKit.dumpOutOfMemory(e, "XlsxrBook::delayedImportFormula()")
        // java throw new OutOfMemoryError("Out Of Memory Error" + e.getMessage())
        // java } else {
        // java var13 = false
        // java } finally {
        // java if (var13) {
        // java for (int fi = 0, fcnt = this.mBook.sheetCount(); fi < fcnt; ++fi) {
        // java KmoSheet sheet = this.mBook.sheetAt(fi)
        // java sheet.changeListenerGroup().finishMergeEvents()
        // java if (sheet.getGrid() != null && sheet.getGrid().getMatrix() != null) {
        // java sheet.getGrid().getMatrix().endCmdMode()
        // java idx = 0
        // java for(int count = this.mBook.sheetCount(); idx < count; ++idx) {
        // java KmoSheet sheet = this.mBook.sheetAt(idx)
        // java sheet.changeListenerGroup().finishMergeEvents()
        // java if (sheet.getGrid() != null && sheet.getGrid().getMatrix() != null) {
        // java sheet.getGrid().getMatrix().endCmdMode()
        // java if (relses.getRelationshipByType(OxfXlRelations.VBA_MACROS.getRelsType()) != null) {
        // java this.mBook.getVBAProject().setOOXMLHasMacro(true)
        // java for (int ri = 0, rn = relses.size(); ri < rn; ++ri) {
        // java OpcrRelationship rel = relses.getRelationship(ri)
        // java if (rel != null && rel.getType() != null && rel.getType().equals(OxfXlRelations.ITEM.getRelsType())) {
        // java XlsxrItem itemReader = new XlsxrItem(this.mBook, rel, this)
        // java itemReader.open()
        // java this.mBook.getBookDrawingAgg().convertDrawingGroup()
        // java } else {
        // java throw new FileDamagedException()
        */
        // rust: no-stmt — changeListenerGroup、公式、透视、VBA、customXml、绘图仍停车。mSheetsReader.open 在本轮读小表 sheetData。登记 §1.7.1。
        if sheet_count != 0 && reader_count != 0 {
            // java for(int idx=0,count=this.mBook.sheetCount();idx<count;++idx){KmoSheet sheet=this.mBook.sheetAt(idx);sheet.changeListenerGroup().startMergeEvents();if(sheet.getGrid()!=null&&sheet.getGrid().getMatrix()!=null){sheet.getGrid().getMatrix().startCmdMode();}}
            for idx in 0..this.mBook.sheetCount(){let sheet=this.mBook.sheetAt(idx);sheet.borrow().changeListenerGroup().startMergeEvents();sheet.borrow().getGrid().getMatrix().borrow_mut().startCmdMode();}
            // Native try/finally transport: flush the same event groups on both
            // normal return and a propagated Java exception.
            let opened=catch_unwind(AssertUnwindSafe(||->io::Result<()>{
            let zip = {
                let slot = this.mBookPart.borrow();
                slot.as_ref().expect("NullPointerException").getZipFile()
            };
            let sheets = this.mSheetsReader.borrow();
            if let Some(sheets) = sheets.as_ref() {
                sheets.borrow().bindZip(zip);
                sheets.borrow().open()?;
                // java if (!XlsxrPackage.isLargestFile()) {
                if !XlsxrPackage::isLargestFile() {
                    // java try {
                    let imported = catch_unwind(AssertUnwindSafe(|| {
                        // java this.mSheetsReader.delayedImportFormula();
                        sheets.borrow().delayedImportFormula();
                    }));
                    // java } catch (OutOfMemoryError e) {
                    if let Err(payload) = imported {
                        let text = panic_text(&*payload);
                        if text.contains("OutOfMemory") || text.contains("WillOutOfMemory") {
                            // java XlsxrPackage.setLargestFile();
                            XlsxrPackage::setLargestFile();
                            // java this.mSheetsReader.cleanSheetsMemory();
                            // rust: no-stmt — cleanSheetsMemory 仍停车。登记 §1.7.1。
                            // java System.gc();
                            // rust: no-stmt
                            // java MemoryToolKit.dumpOutOfMemory(e, "XlsxrBook::delayedImportFormula()");
                            // rust: no-stmt
                            // java throw new OutOfMemoryError("Out Of Memory Error" + e.getMessage());
                            panic!("OutOfMemoryError: Out Of Memory Error{text}");
                        }
                        std::panic::resume_unwind(payload);
                    }
                }
            }
            Ok(())
            }));
            // java finally/on normal return: for(int idx=0,count=this.mBook.sheetCount();idx<count;++idx){KmoSheet sheet=this.mBook.sheetAt(idx);sheet.changeListenerGroup().finishMergeEvents();if(sheet.getGrid()!=null&&sheet.getGrid().getMatrix()!=null){sheet.getGrid().getMatrix().endCmdMode();}}
            for idx in 0..this.mBook.sheetCount(){let sheet=this.mBook.sheetAt(idx);sheet.borrow().changeListenerGroup().finishMergeEvents();sheet.borrow().getGrid().getMatrix().borrow_mut().endCmdMode();}
            match opened{Ok(result)=>result,Err(payload)=>std::panic::resume_unwind(payload)}
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FileDamagedException",
            ))
        }
    }

    // java public void currentGridRange(KmoSheet sheet, int beginRow, int endRow) {
    pub fn currentGridRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.mSheetsReader != null) {
        // java this.mSheetsReader.currentGridRange(sheet, beginRow, endRow)
        */
    }

    // java public KmoBookOpenEventListener getBookOpenListener() {
    pub fn getBookOpenListener(&self)->Option<Rc<dyn KmoBookOpenEventListener>> {
        // java return this.mOpenListener;
        self.mOpenListener.clone()
    }

    // java public KmoBookOpenPasswordProvider getPasswordProvider() {
    pub fn getPasswordProvider(&self)->Option<Rc<dyn KmoBookOpenPasswordProvider>> {
        // java return this.mPassProvider;
        self.mPassProvider.clone()
    }

    // java private void readConnections(OpcrRelationships relses) {
    pub fn readConnections(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java OpcrRelationship connectionsRels = relses.getRelationshipByType(OxfXlRelations.CONNECTIONS.getRelsType())
        // java try {
        // java XlsxrConnections.open(this.mBook, connectionsRels)
        // java } catch (Throwable e) {
        // java IOLog.d(TAG, "Throwable", e)
        */
    }

    // java public void openGlobalData(OpcrRelationships relses) throws IOException {
    pub fn openGlobalData(this: &Rc<Self>) -> io::Result<()> {
        let read_result = Self::read_workbook(this);
        if let Err(e) = read_result {
            if !is_zip_exception(&e) {
                return Err(e);
            }
        }
        if let Err(e)=Self::open_theme(this){if !is_zip_exception(&e){return Err(e);}}
        if let Err(_e) = Self::open_styles(this) {
            // rust: Throwable 被吃掉，继续读共享字符串。
        }
        if let Err(_e) = Self::open_strings(this) {
            // rust: Throwable 被吃掉。
        }
        /* DEFERRED_NON_FUNCTION
        // java try {
        // java InputStream stream = this.mBookPart.getInputStream()
        // java WorkbookHandler handler = new WorkbookHandler(this.mBook, this)
        // java UnifiedXmlReader.read((InputStream)stream, handler)
        // java } catch (ZipException e) {
        // java IOLog.d(TAG, "ZipException", e)
        // java OpcrRelationship themeRels = relses.getRelationshipByType(OxfXlRelations.theme().getRelsType())
        // java if (themeRels != null) {
        // java XlsxrTheme themeReader = new XlsxrTheme(this.mBook.theme(), this.mBook, themeRels)
        // java try {
        // java themeReader.open()
        // java } catch (ZipException e) {
        // java IOLog.d(TAG, "ZipException", e)
        // java OpcrRelationship stylesRels = relses.getRelationshipByType(OxfXlRelations.STYLES.getRelsType())
        // java try {
        // java XlsxrStyles.open(this.mBook, stylesRels, this.mStylesTable)
        // java } catch (Throwable e) {
        // java IOLog.d(TAG, "Throwable", e)
        // java OpcrRelationship stringsRels = relses.getRelationshipByType(OxfXlRelations.SHARED_STRINGS.getRelsType())
        // java try {
        // java XlsxrSharedStrings.open(this.mBook, stringsRels)
        // java } catch (Throwable e) {
        // java IOLog.d(TAG, "Throwable", e)
        */
        Ok(())
    }

    pub(super) fn read_workbook(this: &Rc<Self>) -> io::Result<()> {
        let mut stream = {
            let slot = this.mBookPart.borrow();
            let part = slot.as_ref().expect("NullPointerException");
            part.getInputStream_default()?
        };
        let handler = WorkbookHandler::WorkbookHandler(Rc::clone(&this.mBook), Rc::clone(this));
        UnifiedXmlReader::read(&mut *stream, Box::new(handler))
    }

    fn relationship_present(this: &Rc<Self>, rels_type: &str) -> io::Result<bool> {
        let mut slot = this.mBookPart.borrow_mut();
        let part = slot.as_mut().expect("NullPointerException");
        let relses = part
            .getRelationships_mut()?
            .expect("NullPointerException");
        Ok(index_of(relses, rels_type).is_some())
    }

    // java XlsxrTheme themeReader = new XlsxrTheme(this.mBook.theme(), this.mBook, themeRels); themeReader.open();
    pub(super) fn open_theme(this:&Rc<Self>)->io::Result<()> {
        let mut slot=this.mBookPart.borrow_mut();let part=slot.as_mut().expect("NullPointerException");let relses=part.getRelationships_mut()?.expect("NullPointerException");
        let index=index_of(relses,THEME_RELS);if let Some(index)=index {super::theme::XlsxrTheme::XlsxrTheme::open(Some(relses.getRelationship_mut(index)))}else{Ok(())}
    }
    fn open_styles(this: &Rc<Self>) -> io::Result<()> {
        let table = this
            .mStylesTable
            .borrow()
            .clone()
            .expect("NullPointerException");
        let mut slot = this.mBookPart.borrow_mut();
        let part = slot.as_mut().expect("NullPointerException");
        let relses = part
            .getRelationships_mut()?
            .expect("NullPointerException");
        let idx = index_of(relses, STYLES_RELS);
        if let Some(i) = idx {
            let rel = relses.getRelationship_mut(i);
            XlsxrStyles::open(&this.mBook, Some(rel), &table)
        } else {
            XlsxrStyles::open(&this.mBook, None, &table)
        }
    }

    fn open_strings(this: &Rc<Self>) -> io::Result<()> {
        let mut slot = this.mBookPart.borrow_mut();
        let part = slot.as_mut().expect("NullPointerException");
        let relses = part
            .getRelationships_mut()?
            .expect("NullPointerException");
        let idx = index_of(relses, SHARED_STRINGS_RELS);
        if let Some(i) = idx {
            let rel = relses.getRelationship_mut(i);
            XlsxrSharedStrings::open(&this.mBook, Some(rel))
        } else {
            XlsxrSharedStrings::open(&this.mBook, None)
        }
    }

    // java XlsxrGlobalDataMultiThreads globalReader = new XlsxrGlobalDataMultiThreads(this.mBook, this.mBookPart, this, this.mStylesTable);
    // java globalReader.open();
    fn openGlobalDataMulti(this:&Rc<Self>)->io::Result<()>{super::XlsxrGlobalDataMultiThreads::XlsxrGlobalDataMultiThreads::open(this)}
    // rust: Independent read handles release the Java reference-list ownership borrow before starting workers.
    pub(super) fn globalReaderInputs(&self)->io::Result<(Option<kfc_kfc_alpha::kfc::opcr::OpcrRelationship::OpcrRelationship>,Option<kfc_kfc_alpha::kfc::opcr::OpcrRelationship::OpcrRelationship>,Rc<RefCell<StylesTable>>,Rc<KmoBook>)>{
      let mut slot=self.mBookPart.borrow_mut();let rels=slot.as_mut().expect("NullPointerException").getRelationships_mut()?.expect("NullPointerException");if rels.size()==0{return Err(io::Error::new(io::ErrorKind::InvalidData,"FileDamagedException"));}
      let strings=index_of(rels,SHARED_STRINGS_RELS).map(|i|rels.getRelationship(i).readerHandle());let styles=index_of(rels,STYLES_RELS).map(|i|rels.getRelationship(i).readerHandle());Ok((strings,styles,self.mStylesTable.borrow().as_ref().unwrap().clone(),self.mBook.clone()))
    }

    // java public void setHasDefinedNames(boolean b) {
    pub fn setHasDefinedNames(&self, b: bool) {
        // java this.bHasDefinedNames = b;
        self.bHasDefinedNames.set(b);
    }

    // java public void setExternalReferences(List<String> rids) {
    pub fn setExternalReferences(&self,rids:Option<RidList>) {
        // java this.mRids=rids;
        *self.mRids.borrow_mut()=rids;
    }

    // java public KmoSheet addSheetRid(String sheetName, String rid, int sheetId) {
    pub fn addSheetRid(
        &self,
        sheetName: &str,
        rid: &str,
        sheetId: i32,
    ) -> Rc<RefCell<KmoSheet>> {
        // java return this.mSheetsReader.addSheetRid(sheetName, rid, sheetId);
        self.mSheetsReader
            .borrow()
            .as_ref()
            .expect("NullPointerException")
            .borrow()
            .addSheetRid(sheetName, rid, sheetId)
    }

    // java public void addPivotCacheRid(int cacheId, String rid) {
    pub fn addPivotCacheRid(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mPivotCacheReader.addPivotCacheRid(cacheId, rid)
        */
    }

    // java public void onFinishActiveSheet(int maxValueIndex) throws IOException {
    pub fn onFinishActiveSheet(&self,_maxValueIndex:i32)->io::Result<()> {
        // java XlsxrSharedStrings.onActiveSheetBefore(this.mBook);
        XlsxrSharedStrings::onActiveSheetBefore(&self.mBook)?;
        // java this.mOpenListener.activeSheetDidFinishInitializing();
        self.mOpenListener.as_ref().expect("NullPointerException").activeSheetDidFinishInitializing();
        // java try{Thread.sleep(20L);}catch(InterruptedException var5){}
        std::thread::sleep(std::time::Duration::from_millis(20));
        // java XlsxrSharedStrings.onActiveSheetAfter(this.mBook);
        XlsxrSharedStrings::onActiveSheetAfter(&self.mBook)?;
        // java XlsxrStyles.onActiveSheetAfter(this.mStylesTable);
        XlsxrStyles::onActiveSheetAfter(&self.mStylesTable.borrow().as_ref().expect("NullPointerException").borrow())?;
        // java try{
        let external=catch_unwind(AssertUnwindSafe(||->io::Result<()>{
            // java if(this.mRids!=null&&this.mRids.size()>0){
            let rids=self.mRids.borrow().clone();
            if rids.as_ref().is_some_and(|r|!r.borrow().is_empty()){
                // java OpcrRelationships relses=this.mBookPart.getRelationships(); this.mExtLinksReader=new XlsxrExternalLinks(this.mBook,relses,this.mRids);
                let reader=XlsxrExternalLinks::fromPart(self.mBook.clone(),self.mBookPart.clone(),rids)?;
                *self.mExtLinksReader.borrow_mut()=Some(reader);
                // java this.mExtLinksReader.open();
                self.mExtLinksReader.borrow_mut().as_mut().unwrap().open()?;
            }Ok(())
        }));
        // java catch(Throwable e){IOLog.e(TAG,"throwable",e);}
        if !matches!(external,Ok(Ok(()))){eprintln!("XlsxrWorkbook external links caught Throwable");}
        // java if(this.bHasDefinedNames){
        if self.bHasDefinedNames.get(){
            // java InputStream stream=this.mBookPart.getInputStream();
            let mut stream=self.mBookPart.borrow().as_ref().expect("NullPointerException").getInputStream_default()?;
            // java WorkbookDelayHandler handler=new WorkbookDelayHandler(this.mBook); UnifiedXmlReader.read((InputStream)stream,handler);
            UnifiedXmlReader::read(&mut stream,Box::new(WorkbookDelayHandler::WorkbookDelayHandler(self.mBook.clone())))?;
        }Ok(())
    }

    // java public void addSheetId2Sheet(int sheetId, KmoSheet sheet) {
    pub fn addSheetId2Sheet(&self, sheetId: i32, sheet: &Rc<RefCell<KmoSheet>>) {
        // java if (null == this.mSheetId2Sheet) {
        if self.mSheetId2Sheet.borrow().is_empty() {
            // java this.mSheetId2Sheet = new IntObjectHashMap<KmoSheet>();
            let created = &self.mSheetId2Sheet;
            let _ = created;
        }
        // java this.mSheetId2Sheet.put(sheetId, sheet);
        self.mSheetId2Sheet
            .borrow_mut()
            .insert(sheetId, Rc::clone(sheet));
    }

    // java public KmoSheet getSheetBySheetId(int sheetId) {
    pub fn getSheetBySheetId(&self, sheetId: i32) -> Option<Rc<RefCell<KmoSheet>>> {
        // java return null == this.mSheetId2Sheet ? null : (KmoSheet)this.mSheetId2Sheet.get(sheetId);
        self.mSheetId2Sheet.borrow().get(&sheetId).cloned()
    }

}
