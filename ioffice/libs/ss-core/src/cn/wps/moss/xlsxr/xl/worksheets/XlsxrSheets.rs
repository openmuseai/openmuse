// java package cn.wps.moss.xlsxr.xl.worksheets;

// java import cn.wps.kfc.ooxfr.OxfXlRelations;
// java import cn.wps.kfc.opcr.OpcrRelationship;
// java import cn.wps.kfc.opcr.OpcrRelationships;
// java import cn.wps.moss.app.KmoBook;
// java import cn.wps.moss.app.KmoSheet;
use crate::java_runtime::{Cell, RefCell};
use std::collections::HashMap;
use std::io;
use std::panic::{catch_unwind, AssertUnwindSafe};
use crate::java_runtime::Rc;

use kfc_kfc_alpha::kfc::opcr::OpcrRelationships::OpcrRelationships;
use kfc_kfc_alpha::kfc::opcr::ZipFile::ZipFile;

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use crate::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use crate::cn::wps::moss::xlsxr::xl::worksheets::XlsxrWorkSheet::XlsxrWorkSheet;

// OxfXlRelations.WORKSHEET / CHARTSHEET 的 getRelsType()。类本身还不在 kfc 编译面。
const WORKSHEET_RELS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
const CHARTSHEET_RELS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet";

#[derive(Clone)]
struct RelSnap {
    id: Option<String>,
    type_: Option<String>,
    part_name: String,
}

struct SheetReaderRec {
    kind: i8,
    rid: String,
    sheet_index: i32,
    sheet: Rc<RefCell<KmoSheet>>,
    worksheet: Option<Rc<XlsxrWorkSheet>>,
    bFinish: Cell<bool>,
    bActive: Cell<bool>,
}

// java public final class XlsxrSheets
pub struct XlsxrSheets {
    mBook: Rc<KmoBook>,
    mRelses: Vec<RelSnap>,
    // java private XlsxrWorkbook mBookReader;
    mBookReader:crate::java_runtime::Weak<super::super::XlsxrWorkbook::XlsxrWorkbook>,
    mSheetReaders: RefCell<HashMap<i32, SheetReaderRec>>,
    mStylesTable: Option<Rc<RefCell<StylesTable>>>,
    pivotShips: RefCell<HashMap<i32, String>>,
    mZip: Rc<RefCell<Option<ZipFile>>>,
    mSelectUid: RefCell<Option<i32>>,
    mOpenActiveSheetFailed: Cell<bool>,
    bHasUnReadSheetData: Cell<bool>,
    importOrder: RefCell<Vec<i32>>,
}

fn snap_rels(relses: &OpcrRelationships) -> Vec<RelSnap> {
    let mut out = Vec::new();
    let mut i = 0i32;
    let size = relses.size();
    while i < size {
        let rels = relses.getRelationship(i);
        out.push(RelSnap {
            id: rels.getId(),
            type_: rels.getType(),
            part_name: rels.getPartName(),
        });
        i += 1;
    }
    out
}

// java public final class XlsxrSheets
impl XlsxrSheets {
    // java public XlsxrSheets(KmoBook book, OpcrRelationships relses, XlsxrWorkbook bookReader, StylesTable stylesTable) {
    pub fn XlsxrSheets(
        book: Rc<KmoBook>,
        relses: &OpcrRelationships,
        bookReader:crate::java_runtime::Weak<super::super::XlsxrWorkbook::XlsxrWorkbook>,
        styles_table: Option<Rc<RefCell<StylesTable>>>,
    ) -> Rc<RefCell<Self>> {
        // java this.mBook = book; this.mRelses = relses; this.mBookReader = bookReader; this.mSheetReaders = new HashMap(); this.mStylesTable = stylesTable; this.mOpenActiveSheetFailed = false; this.pivotTableReader = new XlsxPivotTableReader(this.mStylesTable);
        let mBook = book;
        let mStylesTable = styles_table;
        // Native weak back-reference retains the same workbook reader without an Arc cycle.
        Rc::new(RefCell::new(Self {
            mBook,
            mRelses: snap_rels(relses),
            mBookReader:bookReader,
            mSheetReaders: RefCell::new(HashMap::new()),
            mStylesTable,
            pivotShips: RefCell::new(HashMap::new()),
            mZip: Rc::new(RefCell::new(None)),
            mSelectUid: RefCell::new(None),
            mOpenActiveSheetFailed: Cell::new(false),
            bHasUnReadSheetData: Cell::new(false),
            importOrder: RefCell::new(Vec::new()),
        }))
    }

    pub fn stylesTable(&self) -> Option<Rc<RefCell<StylesTable>>> {
        self.mStylesTable.clone()
    }

    pub fn bindZip(&self, zip: ZipFile) {
        *self.mZip.borrow_mut() = Some(zip);
    }

    // java public int sheetReaderSheetCount() {
    pub fn sheetReaderSheetCount(&self) -> i32 {
        // java return this.mSheetReaders.size();
        self.mSheetReaders.borrow().len() as i32
    }

    pub fn rid_of(&self, sheet_uid: i32) -> Option<String> {
        self.mSheetReaders
            .borrow()
            .get(&sheet_uid)
            .map(|rec| rec.rid.clone())
    }

    // java public void currentGridRange(KmoSheet sheet, int beginRow, int endRow) {
    pub fn currentGridRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (!XlsxrPackage.isLargestFile()) {
        // java this.notifySheet(sheet, beginRow, endRow)
        // java } else {
        // java try {
        // java this.readSheet(sheet, beginRow, endRow)
        // java } catch (IOException e) {
        // java e.printStackTrace()
        */
    }

    // java public KmoSheet addSheetRid(String sheetName, String rid, int sheetId) {
    pub fn addSheetRid(
        &self,
        sheetName: &str,
        rid: &str,
        sheetId: i32,
    ) -> Rc<RefCell<KmoSheet>> {
        // java OpcrRelationship rels = this.mRelses.getRelationship(rid);
        let rels = self
            .mRelses
            .iter()
            .find(|rel| rel.id.as_deref() == Some(rid))
            .cloned();
        // java if (null == rels) {
        if rels.is_none() {
            // java return this.mBook.appendSheet(sheetId, sheetName, (byte)0);
            return self
                .mBook
                .appendSheet__5(sheetId, Some(sheetName.to_string()), 0);
        }
        // java } else {
        let rels = rels.unwrap();
        // java KmoSheet sheet;
        // java XlsxrSheet reader;
        let rel_type = rels.type_.as_deref();
        // java if (OxfXlRelations.WORKSHEET.getRelsType().equals(rels.getType())) {
        let (sheet, kind) = if rel_type == Some(WORKSHEET_RELS) {
            // java sheet = this.mBook.appendSheet(sheetId, sheetName, (byte)0);
            let sheet = self
                .mBook
                .appendSheet__5(sheetId, Some(sheetName.to_string()), 0);
            // java reader = new XlsxrWorkSheet(sheet, rels, this.mBookReader, this.mBook.sheetCount() - 1, this.mStylesTable);
            let _sheet_index = self.mBook.sheetCount() - 1;
            sheet.borrow().setSheetType(0);
            (sheet, 0i8)
        // java } else if (OxfXlRelations.CHARTSHEET.getRelsType().equals(rels.getType())) {
        } else if rel_type == Some(CHARTSHEET_RELS) {
            // java sheet = this.mBook.appendSheet(sheetId, sheetName, (byte)2);
            let sheet = self
                .mBook
                .appendSheet__5(sheetId, Some(sheetName.to_string()), 2);
            // java reader = new XlsxrChartSheet(sheet, rels, this.mBookReader, this.mBook.sheetCount() - 1);
            let _sheet_index = self.mBook.sheetCount() - 1;
            sheet.borrow().setSheetType(2);
            (sheet, 2i8)
        // java } else {
        } else {
            // java sheet = this.mBook.appendSheet(sheetId, sheetName, (byte)1);
            let sheet = self
                .mBook
                .appendSheet__5(sheetId, Some(sheetName.to_string()), 1);
            // java reader = new XlsxrMacroSheet(sheet, rels, this.mBookReader, this.mBook.sheetCount() - 1);
            let _sheet_index = self.mBook.sheetCount() - 1;
            sheet.borrow().setSheetType(1);
            (sheet, 1i8)
        };
        let uid = sheet.borrow().getSheetUid();
        let sheet_index = self.mBook.sheetCount() - 1;
        let part_name = rels.part_name.clone();
        let worksheet = if kind == 0 {
            Some(XlsxrWorkSheet::XlsxrWorkSheet(
                Rc::clone(&sheet),
                part_name,
                Rc::clone(&self.mZip),
                self.mBookReader.clone(),
                sheet_index,
                self.mStylesTable.clone(),
            ))
        } else {
            None
        };
        // java this.mSheetReaders.put(sheet, reader);
        self.mSheetReaders.borrow_mut().insert(
            uid,
            SheetReaderRec {
                kind,
                rid: rid.to_string(),
                sheet_index,
                sheet: Rc::clone(&sheet),
                worksheet,
                bFinish: Cell::new(false),
                bActive: Cell::new(false),
            },
        );
        // java this.pivotTableReader.addSheetShips(sheet, rels);
        self.pivotShips.borrow_mut().insert(uid, rid.to_string());
        // java return sheet;
        sheet
    }

    // java public void readPivotTable() {
    pub fn readPivotTable(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.pivotTableReader.read()
        */
    }

    // java public void open() throws IOException {
    pub fn open(&self) -> io::Result<()> {
        // java this.openActiveSheet()
        self.openActiveSheet()?;
        // java this.mBook.getXFManager().iniXFOption()
        // rust: no-stmt — iniXFOption 仍是 todo。登记 §1.7.1。
        // java if (!XlsxrPackage.isLargestFile()) {
        if !XlsxrPackage::isLargestFile() {
            // java this.openAllSheets()
            self.openAllSheets()?;
            // java if (XlsxrPackage.isLargestFile()) {
            if XlsxrPackage::isLargestFile() {
                // java this.cleanSheetsMemory()
                // rust: no-stmt
            }
        }
        // java if (XlsxrPackage.isLargestFile()) {
        if XlsxrPackage::isLargestFile() {
            // java this.indexAllSheets()
            // rust: no-stmt
        }
        // java try {
        // rust: no-stmt
        // java if (this.mBook.isReadTable()) {
        // rust: no-stmt
        // java this.delayImportTables()
        // rust: no-stmt
        // java this.delayedImportDrawing()
        // rust: no-stmt
        // java this.dumpImportOle()
        // rust: no-stmt
        // java } catch (OutOfMemoryError e) {
        // rust: no-stmt
        // java this.cleanDrawingsMemory()
        // rust: no-stmt
        // java XlsxrPackage.setLargestFile()
        // rust: no-stmt
        // java System.gc()
        // rust: no-stmt
        // java MemoryToolKit.dumpOutOfMemory(e, "XlsxrBook::delayedImportDrawing()")
        // rust: no-stmt
        // java throw new OutOfMemoryError("Out Of Memory Error" + e.getMessage())
        // rust: no-stmt
        Ok(())
    }

    fn rec_has_mindex(rec: &SheetReaderRec) -> bool {
        if rec.kind == 0 {
            rec.worksheet.as_ref().is_some_and(|ws| ws.hasMIndex())
        } else {
            rec.bFinish.get()
        }
    }

    fn rec_has_finish(rec: &SheetReaderRec) -> bool {
        if rec.kind == 0 {
            rec.worksheet.as_ref().is_some_and(|ws| ws.hasFinish())
        } else {
            rec.bFinish.get()
        }
    }

    fn first_reader(&self, pending: fn(&SheetReaderRec) -> bool) -> Option<i32> {
        let selected = *self.mSelectUid.borrow();
        let readers = self.mSheetReaders.borrow();
        if let Some(uid) = selected {
            if readers.get(&uid).is_some_and(pending) {
                return Some(uid);
            }
        }
        let mut order: Vec<(i32, i32)> = readers
            .iter()
            .map(|(uid, rec)| (rec.sheet_index, *uid))
            .collect();
        order.sort_by_key(|item| item.0);
        for (_, uid) in order {
            if readers.get(&uid).is_some_and(pending) {
                return Some(uid);
            }
        }
        None
    }

    fn set_reader_active(&self, uid: i32, active: bool) {
        if let Some(rec) = self.mSheetReaders.borrow().get(&uid) {
            rec.bActive.set(active);
            if let Some(ws) = &rec.worksheet {
                ws.setActiveSheet(active);
            }
        }
    }

    fn open_uid(&self, uid: i32) -> io::Result<()> {
        let kind = self.mSheetReaders.borrow().get(&uid).map(|rec| rec.kind);
        let Some(kind) = kind else {
            return Ok(());
        };
        if kind == 0 {
            let ws = self
                .mSheetReaders
                .borrow()
                .get(&uid)
                .and_then(|rec| rec.worksheet.clone());
            if let Some(ws) = ws {
                ws.open()?;
            }
            return Ok(());
        }
        if kind == 2 {
            let sheet = self
                .mSheetReaders
                .borrow()
                .get(&uid)
                .map(|rec| Rc::clone(&rec.sheet));
            if let Some(sheet) = sheet {
                // java this.mSheet.setHasDrawing(true);
                sheet.borrow().setHasDrawing(true);
            }
            // rust: no-stmt — WorksheetDrawingHandler 不在本 function。登记 §1.7.1。
        }
        if let Some(rec) = self.mSheetReaders.borrow().get(&uid) {
            rec.bFinish.set(true);
        }
        Ok(())
    }

    fn panic_text(payload: &(dyn std::any::Any + Send)) -> String {
        if let Some(text) = payload.downcast_ref::<&str>() {
            return (*text).to_string();
        }
        if let Some(text) = payload.downcast_ref::<String>() {
            return text.clone();
        }
        String::new()
    }

    // java private void openActiveSheet() throws IOException {
    pub fn openActiveSheet(&self) -> io::Result<()> {
        // java if (this.mSelectSheet == null) {
        if self.mSelectUid.borrow().is_none() {
            // java this.mSelectSheet = this.mBook.getActiveSheet()
            let active = self.mBook.getActiveSheet();
            *self.mSelectUid.borrow_mut() = Some(active.borrow().getSheetUid());
        }
        // java XlsxrSheet reader = (XlsxrSheet)this.mSheetReaders.get(this.mSelectSheet)
        let uid = *self.mSelectUid.borrow();
        let present = uid.is_some_and(|id| self.mSheetReaders.borrow().contains_key(&id));
        // java if (reader != null) {
        if let Some(uid) = uid.filter(|_| present) {
            // java reader.setActiveSheet(true)
            self.set_reader_active(uid, true);
            // java try {
            // java reader.open()
            let opened = catch_unwind(AssertUnwindSafe(|| self.open_uid(uid)));
            match opened {
                Ok(Ok(())) => {}
                Ok(Err(_)) => {
                    // java } catch (Throwable e) {
                    // java IOLog.d(TAG, "Throwable", e)
                    // java this.removeBadSheet_SheetReaders(reader)
                    self.remove_uid(uid);
                    // java this.mOpenActiveSheetFailed = true
                    self.mOpenActiveSheetFailed.set(true);
                }
                Err(payload) => {
                    let text = Self::panic_text(&*payload);
                    // java } catch (ForceQuitException e) {
                    if text.starts_with("ForceQuitException") {
                        // java IOLog.d(TAG, "ForceQuitException", e)
                        // java throw new ForceQuitException()
                        panic!("ForceQuitException");
                    }
                    // java } catch (Throwable e) {
                    // java IOLog.d(TAG, "Throwable", e)
                    // java this.removeBadSheet_SheetReaders(reader)
                    self.remove_uid(uid);
                    // java this.mOpenActiveSheetFailed = true
                    self.mOpenActiveSheetFailed.set(true);
                }
            }
        // java } else if (this.mSelectSheet != null) {
        } else if uid.is_some() {
            // java this.mBookReader.onFinishActiveSheet(this.mSelectSheet.getSheetIndex())
            let reader=self.mBookReader.upgrade().expect("NullPointerException");
            let sheet=self.mBook.getActiveSheet();let index=sheet.borrow().getSheetIndex();
            reader.onFinishActiveSheet(index)?;
            // java this.mSelectSheet.changeListenerGroup().sheetDidFinishInitializing()
            sheet.borrow().changeListenerGroup().sheetDidFinishInitializing();
            // java this.mBookReader.getBookOpenListener().sheetDidFinishReadingEverything(this.mSelectSheet.getSheetIndex())
            reader.getBookOpenListener().expect("NullPointerException").sheetDidFinishReadingEverything(index);
        }
        Ok(())
    }

    // java private void openAllSheets() throws IOException {
    pub fn openAllSheets(&self) -> io::Result<()> {
        // java while(true) {
        loop {
            // java XlsxrSheet reader = null
            let mut reader: Option<i32> = None;
            // java if (this.bHasUnReadSheetData) {
            if self.bHasUnReadSheetData.get() {
                // java reader = (XlsxrSheet)this.mSheetReaders.get(this.mSelectSheet)
                reader = *self.mSelectUid.borrow();
                // java this.bHasUnReadSheetData = false
                self.bHasUnReadSheetData.set(false);
            // java } else {
            } else {
                // java reader = this.getUnMIndexPart()
                reader = self.getUnMIndexPart();
                // java if (reader == null) {
                if reader.is_none() {
                    // java reader = this.getUnReadPart()
                    reader = self.getUnReadPart();
                }
            }
            // java if (reader != null) {
            if let Some(uid) = reader {
                // java try {
                let opened = catch_unwind(AssertUnwindSafe(|| {
                    // java if (this.mOpenActiveSheetFailed) {
                    if self.mOpenActiveSheetFailed.get() {
                        // java this.mOpenActiveSheetFailed = false
                        self.mOpenActiveSheetFailed.set(false);
                        // java reader.setActiveSheet(true)
                        self.set_reader_active(uid, true);
                    }
                    // java reader.open()
                    self.open_uid(uid)
                }));
                match opened {
                    Ok(Ok(())) => {}
                    Ok(Err(err)) => return Err(err),
                    Err(payload) => {
                        let text = Self::panic_text(&*payload);
                        // java } catch (OutOfMemoryError e) {
                        if text.contains("OutOfMemoryError") {
                            // java XlsxrPackage.setLargestFile()
                            XlsxrPackage::setLargestFile();
                            // java reader.restore()
                            // rust: no-stmt
                            // java MemoryToolKit.dumpOutOfMemory(e, "XlsxrSheetHelper::readAllSheets()")
                            // rust: no-stmt
                            // java throw new OutOfMemoryError("Out Of Memory Error" + e.getMessage())
                            panic!("OutOfMemoryError");
                        }
                        // java } catch (RuntimeException e1) {
                        // java IOLog.d(TAG, "RuntimeException", e1)
                        // java this.removeBadSheet_SheetReaders(reader)
                        self.remove_uid(uid);
                        // java this.mOpenActiveSheetFailed = true
                        self.mOpenActiveSheetFailed.set(true);
                    }
                }
            }
            // java if (MemoryToolKit.getMemoryPercent() > 90) {
            // rust: no-stmt — getMemoryPercent 没有类文件，按 0，不进入最大文件退出。登记 §1.7.1。
            // java XlsxrPackage.setLargestFile()
            // rust: no-stmt
            // java } else if (reader != null) {
            if reader.is_some() {
                // java continue
                continue;
            }
            // java return
            return Ok(());
        }
    }

    fn remove_uid(&self, uid: i32) {
        self.mSheetReaders.borrow_mut().remove(&uid);
    }

    // java private void removeBadSheet_SheetReaders(XlsxrSheet target) {
    pub fn removeBadSheet_SheetReaders(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java for(Map.Entry<KmoSheet, XlsxrSheet> next : this.mSheetReaders.entrySet()) {
        // java if (next.getValue() == target) {
        // java this.mSheetReaders.remove(next.getKey())
        // java break
        */
    }

    // java private void indexAllSheets() throws IOException {
    pub fn indexAllSheets(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java XlsxrSheet reader
        // java while((reader = this.getUnMIndexPart()) != null) {
        // java reader.open()
        */
    }

    pub fn delayedImportOrder(&self) -> Vec<i32> {
        self.importOrder.borrow().clone()
    }

    pub fn formulaImportKinds(&self, sheet_index: i32) -> Vec<&'static str> {
        let sheet = self.mBook.sheetAt(sheet_index);
        let uid = sheet.borrow().getSheetUid();
        self.mSheetReaders
            .borrow()
            .get(&uid)
            .and_then(|rec| rec.worksheet.clone())
            .map(|worksheet| worksheet.formulaImportKinds())
            .unwrap_or_default()
    }

    // java public void delayedImportFormula() {
    pub fn delayedImportFormula(&self) {
        self.importOrder.borrow_mut().clear();
        // java int activeSheet = this.mBook.whichSheetIsActivated();
        let activeSheet = self.mBook.whichSheetIsActivated();
        // java KmoSheet sheet = this.mBook.sheetAt(activeSheet);
        let sheet = self.mBook.sheetAt(activeSheet);
        // java this.delayedImportFormula(sheet);
        self.delayedImportFormula__2(&sheet);
        // java int i = 0;
        let mut i = 0;
        // java for(int count = this.mBook.sheetCount(); i < count; ++i) {
        let count = self.mBook.sheetCount();
        while i < count {
            // java if (i != activeSheet) {
            if i != activeSheet {
                // java sheet = this.mBook.sheetAt(i);
                let sheet = self.mBook.sheetAt(i);
                // java this.delayedImportFormula(sheet);
                self.delayedImportFormula__2(&sheet);
            }
            i += 1;
        }
    }

    // java private void delayedImportFormula(KmoSheet sheet) {
    pub fn delayedImportFormula__2(&self, sheet: &Rc<RefCell<KmoSheet>>) {
        // java XlsxrSheet reader = (XlsxrSheet)this.mSheetReaders.get(sheet);
        let uid = sheet.borrow().getSheetUid();
        let worksheet = self
            .mSheetReaders
            .borrow()
            .get(&uid)
            .and_then(|rec| rec.worksheet.clone());
        // java if (reader instanceof XlsxrWorkSheet) {
        if let Some(stReader) = worksheet {
            // java if (stReader != null && stReader.hasFinish()) {
            if stReader.hasFinish() {
                self.importOrder.borrow_mut().push(sheet.borrow().getSheetIndex());
                // java stReader.delayedImportFormula();
                stReader.delayedImportFormula();
            }
        }
    }

    // java private void dumpImportOle() {
    pub fn dumpImportOle(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int i = 0
        // java for(int count = this.mBook.sheetCount(); i < count; ++i) {
        // java KmoSheet sheet = this.mBook.sheetAt(i)
        // java XlsxrSheet reader = (XlsxrSheet)this.mSheetReaders.get(sheet)
        // java if (reader instanceof XlsxrWorkSheet) {
        // java XlsxrWorkSheet stReader = (XlsxrWorkSheet)reader
        // java stReader.importOle()
        */
    }

    // java private void delayedImportDrawing() {
    pub fn delayedImportDrawing(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int activeSheet = this.mBook.whichSheetIsActivated()
        // java KmoSheet sheet = this.mBook.sheetAt(activeSheet)
        // java this.delayedImportDrawing(sheet)
        // java int i = 0
        // java for(int count = this.mBook.sheetCount(); i < count; ++i) {
        // java if (i != activeSheet) {
        // java sheet = this.mBook.sheetAt(i)
        // java this.delayedImportDrawing(sheet)
        // java this.mBook.initDisimageMap()
        */
    }

    // java private void delayedImportDrawing(KmoSheet sheet) {
    pub fn delayedImportDrawing__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java XlsxrSheet reader = (XlsxrSheet)this.mSheetReaders.get(sheet)
        // java if (reader instanceof XlsxrWorkSheet) {
        // java XlsxrWorkSheet stReader = (XlsxrWorkSheet)reader
        // java if (stReader != null && stReader.hasFinish()) {
        // java try {
        // java stReader.openSparkLines()
        // java } catch (IOException e) {
        // java throw new RuntimeException(e)
        // java try {
        // java stReader.openBackgroundImage()
        // java } catch (IOException e) {
        // java e.printStackTrace()
        // java try {
        // java stReader.openDrawingPart()
        // java } catch (Throwable e) {
        // java e.printStackTrace()
        // java IOLog.d(TAG, "Throwable", e)
        // java } else if (reader instanceof XlsxrChartSheet) {
        // java XlsxrChartSheet stReader = (XlsxrChartSheet)reader
        // java try {
        // java stReader.openDrawingPart()
        // java } catch (IOException e) {
        // java e.printStackTrace()
        */
    }

    // java private void delayImportTables() {
    pub fn delayImportTables(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int activeSheet = this.mBook.whichSheetIsActivated()
        // java KmoSheet sheet = this.mBook.sheetAt(activeSheet)
        // java this.delayImportTables(sheet)
        // java int i = 0
        // java for(int count = this.mBook.sheetCount(); i < count; ++i) {
        // java if (i != activeSheet) {
        // java sheet = this.mBook.sheetAt(i)
        // java this.delayImportTables(sheet)
        */
    }

    // java private void delayImportTables(KmoSheet sheet) {
    pub fn delayImportTables__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java XlsxrSheet reader = (XlsxrSheet)this.mSheetReaders.get(sheet)
        // java if (reader instanceof XlsxrWorkSheet) {
        // java XlsxrWorkSheet stReader = (XlsxrWorkSheet)reader
        // java if (stReader != null && stReader.hasFinish()) {
        // java stReader.openTables()
        */
    }

    // java public void cleanSheetsMemory() {
    pub fn cleanSheetsMemory(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int maxStIndex = MemoryToolKit.getMaxMemorySheet(this.mBook)
        // java if (maxStIndex != -1) {
        // java this.cleanSheetMemory(this.mBook.sheetAt(maxStIndex))
        */
    }

    // java private void cleanSheetMemory(KmoSheet sheet) {
    pub fn cleanSheetMemory(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java XlsxrSheet reader = (XlsxrSheet)this.mSheetReaders.get(sheet)
        // java if (reader != null && reader instanceof XlsxrWorkSheet) {
        // java XlsxrWorkSheet stReader = (XlsxrWorkSheet)reader
        // java long ret = stReader.cleanBlockByGridRange(sheet.getWindow2().getTopRow())
        // java int beginRow = (int)(ret & 1048575L)
        // java int endRow = (int)(ret >> 21 & 1048575L)
        // java if (beginRow < endRow) {
        // java sheet.getGrid().getMatrix().clearRowsMemory(beginRow, endRow)
        */
    }

    // java private void cleanDrawingsMemory() {
    pub fn cleanDrawingsMemory(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int i = 0
        // java for(int count = this.mBook.sheetCount(); i < count; ++i) {
        // java KmoSheet sheet = this.mBook.sheetAt(i)
        // java SheetDrawingAgg agg = sheet.getSheetDrawingAgg()
        // java agg.destroy()
        // java sheet.noteMgr().destory()
        */
    }

    // java private XlsxrSheet getUnMIndexPart() {
    pub fn getUnMIndexPart(&self) -> Option<i32> {
        // java XlsxrSheet selReader = (XlsxrSheet)this.mSheetReaders.get(this.mSelectSheet)
        // java if (null != selReader && !selReader.hasMIndex()) {
        // java return selReader
        // java } else {
        // java for(XlsxrSheet reader : this.mSheetReaders.values()) {
        // rust: 非活动表按 sheet_index 排序。登记 §1.7.1。
        // java if (!reader.hasMIndex()) {
        // java return reader
        // java return null
        self.first_reader(|rec| !Self::rec_has_mindex(rec))
    }

    // java private XlsxrSheet getUnReadPart() {
    pub fn getUnReadPart(&self) -> Option<i32> {
        // java XlsxrSheet selReader = (XlsxrSheet)this.mSheetReaders.get(this.mSelectSheet)
        // java if (null != selReader && !selReader.hasFinish()) {
        // java return selReader
        // java } else {
        // java for(XlsxrSheet reader : this.mSheetReaders.values()) {
        // java if (!reader.hasFinish()) {
        // java return reader
        // java return null
        self.first_reader(|rec| !Self::rec_has_finish(rec))
    }

    // java public void notifySheet(KmoSheet sheet, int beginRow, int endRow) {
    pub fn notifySheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java XlsxrSheet reader = (XlsxrSheet)this.mSheetReaders.get(sheet)
        // java if (reader != null && !reader.hasFinish()) {
        // java this.mSelectSheet = sheet
        // java this.bHasUnReadSheetData = reader.notifyGridRange(beginRow, endRow)
        */
    }

    // java public void readSheet(KmoSheet sheet, int beginRow, int endRow) throws IOException {
    pub fn readSheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java XlsxrSheet reader = (XlsxrSheet)this.mSheetReaders.get(sheet)
        // java if (reader != null && !reader.hasFinish()) {
        // java this.mSelectSheet = sheet
        // java boolean hasRead = reader.readGridRange(beginRow, endRow)
        // java if (hasRead) {
        // java int maxStIndex = MemoryToolKit.getMaxMemorySheet(this.mBook)
        // java if (maxStIndex != -1) {
        // java this.cleanSheetMemory(this.mBook.sheetAt(maxStIndex))
        */
    }

}
