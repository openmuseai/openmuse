// java package cn.wps.moss.app;

// java import cn.wps.moffice.io.ForceQuitException;
// java import cn.wps.moss.engine.grid.KmoCellData;
// java import cn.wps.moss.engine.grid.KmoCellValueStore;
// java import cn.wps.moss.engine.grid.KmoGrid;
// java import cn.wps.moss.util.KmoRect;
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::java_runtime::{Cell, RefCell};
use std::collections::HashMap;
use crate::java_runtime::{Rc, Weak};
use std::sync::atomic::{AtomicI32, Ordering};

use util::org::apache::poi::util::BitFieldFactory::BitFieldFactory;

use crate::cn::wps::et::ss::formula::parser::FormulaParser;
use crate::cn::wps::et::ss::formula::parser::ParserOption::ParserOption;
use crate::cn::wps::et::ss::formula::ptg::Ptg::{Ptg, PtgKind};
use crate::cn::wps::moss::core::drawing::KmoCellPic::KmoCellPic;
use crate::cn::wps::moss::engine::eval::EvaluationExtSheet::EvaluationExtSheet;
use crate::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use crate::cn::wps::moss::engine::grid::ExpFmlManager::ExpFmlManager;
use crate::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;
use crate::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use crate::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
use crate::cn::wps::moss::engine::grid::KmoGrid::KmoGrid;
use crate::cn::wps::moss::engine::grid::KmoMatrix::KmoMatrix;
use crate::cn::wps::moss::engine::grid::griditerator::KmoMatrixCellIterator::KmoMatrixCellIterator;
use crate::cn::wps::moss::engine::grid::KmoMatrixToolkit::KmoMatrixToolkit;
use crate::cn::wps::moss::core::sheet::KmoColInfos::KmoColInfos;
use crate::cn::wps::moss::filefmt::biff8::record::GutsRecord::GutsRecord;
use crate::cn::wps::moss::filefmt::biff8::record::WSBoolRecord::WSBoolRecord;
use crate::cn::wps::moss::util::KmoRect::KmoRect;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

use super::GridSheetData::GridSheetData;
use super::KmoBook::KmoBook;

static NEXT_UID: AtomicI32 = AtomicI32::new(0);
// java this.changeListeners.add(new KmoSheetChangeEventsListener() { ... });
// rust: the anonymous listener holds the constructor's same parent reference.
struct SheetBookChangeListener(Weak<KmoBook>);
impl super::event::KmoSheetChangeListenerGroup::KmoSheetChangeEventsListener for SheetBookChangeListener {
    fn sheetSettingsDidFinishChanging(&self) {
        // java KmoSheet.this.parent.changeListenerGroup().bookSettingsDidFinishChanging();
        self.0.upgrade().expect("KmoBook").changeListenerGroup().bookSettingsDidFinishChanging();
    }
    fn sheetSelectionChanged(&self) {
        // java KmoSheet.this.parent.changeListenerGroup().bookSettingsDidFinishChanging();
        self.0.upgrade().expect("KmoBook").changeListenerGroup().bookSettingsDidFinishChanging();
    }
    fn sheetContentsDidFinishChanging(&self) {
        // java KmoSheet.this.parent.changeListenerGroup().bookSettingsDidFinishChanging();
        let book=self.0.upgrade().expect("KmoBook");
        book.changeListenerGroup().bookSettingsDidFinishChanging();
        // java KmoSheet.this.parent.changeListenerGroup().bookContentsDidFinishChanging();
        book.changeListenerGroup().bookContentsDidFinishChanging();
    }
}
// java private static final ConstRecognisor constRecognisor=new ConstRecognisor();
static SHEET_CONST_RECOGNISOR:std::sync::LazyLock<std::sync::Mutex<crate::cn::wps::et::ss::constant::ConstRecognisor::ConstRecognisor>>=std::sync::LazyLock::new(||std::sync::Mutex::new(crate::cn::wps::et::ss::constant::ConstRecognisor::ConstRecognisor::ConstRecognisor()));


// java public class KmoSheet.CellIterator
pub struct CellIterator {
    // java KmoMatrixCellIterator iter;
    iter: Box<dyn KmoMatrixCellIterator>,
    // java KmoCellData data = new KmoCellData();
    data: KmoCellData,
    // java boolean dataNeedRefresh = false;
    dataNeedRefresh: bool,
}

impl CellIterator {
    // java public CellIterator(KmoMatrixCellIterator iter)
    pub fn CellIterator(iter: Box<dyn KmoMatrixCellIterator>) -> Self {
        // java this.iter = iter;
        Self { iter, data: KmoCellData::default(), dataNeedRefresh: false }
    }

    // java public boolean hasNext()
    pub fn hasNext(&self) -> bool { self.iter.hasNext() }

    // java public void next()
    pub fn next(&mut self) {
        // java this.iter.next();
        self.iter.next();
        // java this.dataNeedRefresh = true;
        self.dataNeedRefresh = true;
    }

    // java public int row()
    pub fn row(&self) -> i32 { self.iter.row() }

    // java public int col()
    pub fn col(&self) -> i32 { self.iter.col() }

    // java public KmoCellData cellData()
    pub fn cellData(&mut self) -> &KmoCellData {
        // java if (this.dataNeedRefresh) { this.data.read(this.iter.data()); this.dataNeedRefresh = false; }
        if self.dataNeedRefresh {
            self.data.read__bytearr_int(self.iter.data(), 0);
            self.dataNeedRefresh = false;
        }
        // java return this.data;
        &self.data
    }
}

use crate::cn::wps::moss::core::sheet::KmoPaneInfo::KmoPaneInfo;
pub use crate::cn::wps::moss::core::sheet::Window2::Window2;

pub struct KmoSheetInfo {
    selection:RefCell<Rc<crate::cn::wps::moss::core::sheet::KmoSelection::KmoSelection>>,
    pub name: Option<String>,
    pub sheetId: i32,
    pub sheetType: Cell<i8>,
    pub isHidden: Cell<bool>,
    pub isVeryHidden: Cell<bool>,
    defRowHeight: Cell<i32>,
    defRowDyZero: Cell<bool>,
    defRowUnsyned: Cell<bool>,
    stdColWidth: Cell<i32>,
    defColWidth: Cell<i32>,
    window2: Window2,
    paneInfo:RefCell<KmoPaneInfo>,hasPane:Cell<bool>,
    nscl: Cell<i16>,
    dscl: Cell<i16>,
    wsbool: RefCell<WSBoolRecord>,
    guts: RefCell<GutsRecord>,
    // java boolean fullPrecision = true;
    fullPrecision: Cell<bool>,
}

impl KmoSheetInfo {
    fn new(name: Option<String>, sheet_id: i32) -> Self {
        Self {
            name,
            selection:RefCell::new(Rc::new(crate::cn::wps::moss::core::sheet::KmoSelection::KmoSelection::new())),
            sheetId: sheet_id,
            sheetType: Cell::new(0),
            isHidden: Cell::new(false),
            isVeryHidden: Cell::new(false),
            defRowHeight: Cell::new(325),
            defRowDyZero: Cell::new(false),
            defRowUnsyned: Cell::new(false),
            stdColWidth: Cell::new(-1),
            defColWidth: Cell::new(10),
            window2: Window2::Window2(),paneInfo:RefCell::new(KmoPaneInfo::default()),hasPane:Cell::new(false),
            nscl:Cell::new(1),dscl:Cell::new(1),
            wsbool: RefCell::new(WSBoolRecord::WSBoolRecord()),
            guts: RefCell::new(GutsRecord::GutsRecord()),
            fullPrecision: Cell::new(true),
        }
    }

    pub fn getDefRowHeight(&self) -> i32 {
        self.defRowHeight.get()
    }

    pub fn getDefRowDyZero(&self) -> bool {
        self.defRowDyZero.get()
    }

    pub fn getDefRowUnsyned(&self) -> bool {
        self.defRowUnsyned.get()
    }
}

pub use crate::cn::wps::moss::core::sheet::KmoRowInfo::KmoRowInfo;
use crate::cn::wps::moss::core::sheet::KmoRowInfos::KmoRowInfos;

// java public static class CellTypeInfo
pub struct CellTypeInfo {
    // java public int cellType;
    pub cellType: i32,
    // java public boolean hasRun;
    pub hasRun: bool,
    // java public byte[] cellTypeBuff = KmoMatrix.newBuff();
    pub cellTypeBuff: [u8; 7],
}

impl CellTypeInfo {
    pub fn CellTypeInfo() -> Self {
        Self {
            cellType: 0,
            hasRun: false,
            cellTypeBuff: KmoMatrixToolkit::newBuff(),
        }
    }
}

// java public class KmoSheet
pub struct KmoSheet {
    // java private KmoConditionFormatter cformatter;private KmoSheetSharedObjects shared=new KmoSheetSharedObjects();
    cformatter:super::condfmt::KmoConditionFormatter::KmoConditionFormatter,
    shared:super::KmoSheetSharedObjects::KmoSheetSharedObjects,
    uid: i32,
    // java private KmoSheetChangeListenerGroup changeListeners = new KmoSheetChangeListenerGroup();
    changeListeners: super::event::KmoSheetChangeListenerGroup::KmoSheetChangeListenerGroup,
    possibleSheetIndex: Cell<i32>,
    usedRange: Rc<RefCell<KmoRect>>,
    sheetInfo: KmoSheetInfo,
    // java private KmoRect lastSelection = new KmoRect();
    lastSelection: RefCell<KmoRect>,
    // java private List<KmoRect> uTransactionList = new ArrayList();
    uTransactionList: RefCell<Vec<KmoRect>>,
    // java private byte[] cellIdBuff;
    cellIdBuff:RefCell<[u8;7]>,
    grid: KmoGrid,
    // java private EvaluationPresent evaluationPresent;
    evaluationPresent:Rc<crate::cn::wps::moss::engine::eval::EvaluationPresent::EvaluationPresent>,
    rowInfos: Rc<KmoRowInfos>,
    // java private KmoHiddenRowsManager hiddenRowsManager;
    hiddenRowsManager:RefCell<Option<Rc<crate::cn::wps::moss::core::sheet::KmoHiddenRowsManager::KmoHiddenRowsManager>>>,
    // java private OptHiddenRowColManager optHiddenRowColManager;
    optHiddenRowColManager:RefCell<Option<Rc<super::rowcol::OptHiddenRowColManager::OptHiddenRowColManager>>>,
    colInfos: RefCell<KmoColInfos>,
    parent: Weak<KmoBook>,
    // java private boolean hasDrawing;
    hasDrawing: Cell<bool>,
    // java private ExpFmlManager expFmlManager;
    expFmlManager: RefCell<ExpFmlManager>,
    // java public HashMap<Integer, ExpFmlManager.ExpFml> sharedFormulaMap = new HashMap();
    pub sharedFormulaMap:RefCell<HashMap<i32,Rc<crate::cn::wps::moss::engine::grid::ExpFmlManager::ExpFml>>>,
}

impl KmoSheet {
    pub fn same_sheet(&self, other: &KmoSheet) -> bool {
        self.uid == other.uid
    }

    fn book(&self) -> Rc<KmoBook> {
        self.parent
            .upgrade()
            .unwrap_or_else(|| panic!("NullPointerException"))
    }

    // java KmoSheet(KmoBook book)
    pub fn create__book(book: &KmoBook) -> Rc<RefCell<Self>> {
        // java this(book, "", -1);
        Self::create(book, Some(String::new()), -1)
    }

    // java KmoSheet(KmoBook book, String sheetName, int sheetId)
    pub fn create(book: &KmoBook, sheet_name: Option<String>, sheet_id: i32) -> Rc<RefCell<Self>> {
        // java this.uid = nextUid++;
        let uid = NEXT_UID.fetch_add(1, Ordering::Relaxed);
        // java this.possibleSheetIndex = 0;
        // java this.usedRange = new KmoRect(0, 0, 0, 0);
        let used = Rc::new(RefCell::new(KmoRect::KmoRect__bounds(0, 0, 0, 0)));
        // java this.sheetInfo = new KmoSheetInfo();
        // java this.parent = book;
        // java this.rowInfos = new KmoRowInfos(this);
        // java this.gridData = new KmoGrid(this, this.usedRange, book.getSpreadsheetVersion());
        let grid = KmoGrid::KmoGrid__sheet(Rc::clone(&used), book.getSpreadsheetVersion());
        // java sheetName = this.deleteIvalidChar(sheetName);
        let cleaned = Self::deleteIvalidChar(sheet_name.as_deref());
        // java this.sheetInfo.name = sheetName;
        // java this.sheetInfo.sheetId = -1 == sheetId ? this.parent.getNextSheetId() : sheetId;
        let resolved_id = if sheet_id == -1 {
            book.getNextSheetId()
        } else {
            sheet_id
        };
        let sheet = Rc::new_cyclic(|weak_sheet|RefCell::new(Self {
            cformatter:super::condfmt::KmoConditionFormatter::KmoConditionFormatter::new(weak_sheet.clone()),
            shared:super::KmoSheetSharedObjects::KmoSheetSharedObjects::new(),
            uid,
            changeListeners: super::event::KmoSheetChangeListenerGroup::KmoSheetChangeListenerGroup::new(),
            possibleSheetIndex: Cell::new(0),
            usedRange: used,
            sheetInfo: KmoSheetInfo::new(cleaned, resolved_id),
            lastSelection: RefCell::new(KmoRect::KmoRect()),
            uTransactionList: RefCell::new(vec![]),
            // java this.cellIdBuff=KmoMatrix.newBuff();
            cellIdBuff:RefCell::new(crate::cn::wps::moss::engine::grid::KmoMatrixToolkit::KmoMatrixToolkit::newBuff()),
            grid,
            // java this.evaluationPresent = new EvaluationPresent();
            evaluationPresent:Rc::new(crate::cn::wps::moss::engine::eval::EvaluationPresent::EvaluationPresent::new()),
            rowInfos: Rc::new(KmoRowInfos::new(weak_sheet.clone(),book.getSpreadsheetVersion().getMaxRows())),
            hiddenRowsManager:RefCell::new(None),
            optHiddenRowColManager:RefCell::new(None),
            colInfos: RefCell::new(KmoColInfos::KmoColInfos()),
            parent: book.me_weak(),
            hasDrawing: Cell::new(false),
            expFmlManager: RefCell::new(ExpFmlManager::ExpFmlManager__sheet(weak_sheet.clone())),
            sharedFormulaMap:RefCell::new(HashMap::new()),
        }));
        // java this.hiddenRowsManager = new KmoHiddenRowsManager(this);
        // Native constructor-this binding after Arc publication, before content writes.
        *sheet.borrow().hiddenRowsManager.borrow_mut()=Some(crate::cn::wps::moss::core::sheet::KmoHiddenRowsManager::KmoHiddenRowsManager::new(&sheet));
        // java this.optHiddenRowColManager = new OptHiddenRowColManager(this);
        *sheet.borrow().optHiddenRowColManager.borrow_mut()=Some(super::rowcol::OptHiddenRowColManager::OptHiddenRowColManager::new(&sheet));
        // rust: bind Java's constructor `this` after Arc publication; no content is written in between.
        sheet.borrow().grid.getMatrix().borrow_mut().bindSheet(Rc::downgrade(&sheet), book, uid);
        // java this.changeListeners.add(new KmoSheetChangeEventsListener() { ... });
        sheet.borrow().changeListeners.add(Rc::new(SheetBookChangeListener(book.me_weak())));
        sheet
    }
}

impl KmoSheet {
    // java public AutoTestRangeFind getAutoTestRangeFind(KmoRect rect) {
    pub fn getAutoTestRangeFind(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return (AutoTestRangeFind)this.autoTestFindMap.get(rect)
        */
    }

    // java public void addAutoTestRangeFind(KmoRect rect, AutoTestRangeFind autoTestRangeFind) {
    pub fn addAutoTestRangeFind(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.autoTestFindMap == null)
        // java this.autoTestFindMap = new HashMap()
        // java this.autoTestFindMap.put(rect, autoTestRangeFind)
        */
    }

    // java public RangeManager getRangeManager() {
    pub fn getRangeManager(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.rangeManager
        */
    }

    // java public void setRangeManager(RangeManager rangeManager) {
    pub fn setRangeManager(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.rangeManager = rangeManager
        */
    }

    // java public static void resetConstRecognLang() {
    pub fn resetConstRecognLang() {
// java constRecognisor.setLangeuage(KmoBooks.getLanguageType());
SHEET_CONST_RECOGNISOR.lock().unwrap().setLangeuage(super::KmoBooks::KmoBooks::getLanguageType());
    }

    // java KmoSheet copy(KmoSheet srcSheet) {
    pub fn copy(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.hiddenRowsManager.copy(srcSheet.hiddenRowsManager)
        // java this.optHiddenRowColManager.copy(srcSheet.optHiddenRowColManager)
        // java this.rowInfos.copy(srcSheet.rowInfos)
        // java this.colInfos.copy(srcSheet.colInfos)
        // java this.hyperlinkMgr.copy(srcSheet.hyperlinkMgr)
        // java this.dvMgr.copy(srcSheet.dvMgr)
        // java this.copyImpl(srcSheet, false, false)
        // java this.sheetDrawingAgg.copy(srcSheet.sheetDrawingAgg)
        // java if (srcSheet.woSheetPropsItem != null)
        // java this.woSheetPropsItem = new WoSheetPropsItem()
        // java this.woSheetPropsItem.copy(srcSheet.woSheetPropsItem)
        // java this.copyFreeze(srcSheet)
        // java return this
        */
    }

    // java private void copyFreeze(KmoSheet srcSheet) {
    pub fn copyFreeze(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.freezeTopRowCol[0] = srcSheet.freezeTopRowCol[0]
        // java this.freezeTopRowCol[1] = srcSheet.freezeTopRowCol[1]
        // java this.topOffset[0] = srcSheet.topOffset[0]
        // java this.topOffset[1] = srcSheet.topOffset[1]
        // java this.mainOffset[0] = srcSheet.mainOffset[0]
        // java this.mainOffset[1] = srcSheet.mainOffset[1]
        */
    }

    // java void mergeCopy(KmoBook extBook, KmoSheet extSheet, KmoBook curBook) {
    pub fn mergeCopy(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mergeCopy(extBook, extSheet, curBook, false, false)
        */
    }

    // java void mergeCopy(KmoBook extBook, KmoSheet extSheet, KmoBook curBook, boolean ignoreFilter, boolean ignoreProtect) {
    pub fn mergeCopy__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.hiddenRowsManager.mergeCopy(extSheet)
        // java this.rowInfos.mergeCopy(extSheet, curBook)
        // java this.colInfos.mergeCopy(extSheet, curBook)
        // java this.hyperlinkMgr.mergeCopy(extSheet.hyperlinkMgr)
        // java this.dvMgr.mergeCopy(extSheet.dvMgr)
        // java this.copyImpl(extSheet, ignoreFilter, ignoreProtect)
        // java this.sheetDrawingAgg.mergeCopy(extBook, extSheet.sheetDrawingAgg, curBook)
        // java this.mergeCopyWoSheetProps(extSheet.woSheetPropsItem)
        */
    }

    // java private void mergeCopyWoSheetProps(WoSheetPropsItem extPropsItem) {
    pub fn mergeCopyWoSheetProps(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (extPropsItem == null)
        // java this.woSheetPropsItem = null
        // java this.woSheetPropsItem = new WoSheetPropsItem()
        // java this.woSheetPropsItem.mergeCopy(extPropsItem)
        */
    }

    // java void mergeCopy(int offset, int ignoreNum, int dataHeight, KmoRect extUsedRange, KmoBook extBook, KmoSheet extSheet, KmoBook curBook) {
    pub fn mergeCopy__3(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.hiddenRowsManager.mergeCopy(offset, ignoreNum, extUsedRange, extSheet)
        // java this.rowInfos.mergeCopy(offset, ignoreNum, extUsedRange, extSheet, curBook)
        // java this.hyperlinkMgr.mergeCopy(offset, ignoreNum, extUsedRange, extSheet.hyperlinkMgr)
        // java this.mergeUsedRange(dataHeight, ignoreNum, extUsedRange)
        // java this.gridData.mergeCopy(offset, ignoreNum, extUsedRange, extSheet.gridData)
        // java this.expFmlManager.mergeCopy(offset, ignoreNum, extUsedRange, extSheet.expFmlManager)
        // java this.dvMgr.mergeCopy(offset, ignoreNum, extUsedRange, extSheet.dvMgr)
        // java this.cformatter.mergeCopy(offset, ignoreNum, extUsedRange, extSheet.cformatter)
        // java this.sheetDrawingAgg.mergeCopy(offset, ignoreNum, extBook, extSheet.sheetDrawingAgg, curBook)
        */
    }

    // java private void mergeUsedRange(int dataHeight, int ignoreNum, KmoRect extUsedRange) {
    pub fn mergeUsedRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (extUsedRange.br.row > ignoreNum)
        // java if (extUsedRange.tl.col < this.usedRange.tl.col)
        // java this.usedRange.tl.col = extUsedRange.tl.col
        // java if (extUsedRange.br.col > this.usedRange.br.col)
        // java this.usedRange.br.col = extUsedRange.br.col
        // java KmoPoint var10000 = this.usedRange.br
        // java var10000.row += dataHeight
        */
    }

    // java private void copyImpl(KmoSheet srcSheet, boolean ignoreFilter, boolean ignoreProtect) {
    pub fn copyImpl(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.usedRange.copy(srcSheet.usedRange)
        // java this.gridData.copy(srcSheet.gridData)
        // java this.sheetInfo.copy(srcSheet.sheetInfo)
        // java this.expFmlManager.copy(srcSheet.expFmlManager)
        // java this.dviFlags = srcSheet.dviFlags
        // java if (srcSheet.backgroundRecord != null)
        // java this.backgroundRecord = (BitmapRecord)srcSheet.backgroundRecord.clone()
        // java if (srcSheet.backgroundByte != null)
        // java this.backgroundByte = KmoArrays.copyOf(srcSheet.backgroundByte, srcSheet.backgroundByte.length)
        // java if (srcSheet.backgroundBitmap != null)
        // java this.backgroundBitmap = srcSheet.backgroundBitmap.copy(srcSheet.backgroundBitmap.getConfig(), false)
        // java this.backgroundType = srcSheet.backgroundType
        // java this.printSettings.copy(srcSheet.printSettings)
        // java this.cformatter.copy(srcSheet.cformatter)
        // java if (!ignoreProtect)
        // java this.sheetProtect.copy(srcSheet.sheetProtect)
        // java this.sheetHasDrawingIndex = srcSheet.sheetHasDrawingIndex
        // java if (!ignoreFilter && srcSheet.sheetOp.hasAutoFilter())
        // java CellRangeAddress srcFilterRange = srcSheet.sheetOp.autoFilter().getFilterRegion()
        // java this.sheetOp.initAutoFilter(new CellRangeAddress(srcFilterRange.getFirstRow(), srcFilterRange.getLastRow(), srcFilterRange.getFirstColumn(), srcFilterRange.getLastColumn()))
        // java this.sheetOp.autoFilter().copy(srcSheet.sheetOp.autoFilter())
        */
    }

    // java public WoSheetPropsItem getWoSheetPropsItem() {
    pub fn getWoSheetPropsItem(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.woSheetPropsItem
        */
    }

    // java public void setWoSheetPropsItem(WoSheetPropsItem woSheetPropsItem) {
    pub fn setWoSheetPropsItem(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.woSheetPropsItem = woSheetPropsItem
        */
    }

    // java public boolean isValidWoProps() {
    pub fn isValidWoProps(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.woSheetPropsItem != null ? this.woSheetPropsItem.isInterlineOnOff() : false
        */
    }

    // java public List<FindResult> getFindResult() {
    pub fn getFindResult(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.getBook().getFindList()
        */
    }

    // java public void addInUTransactionList(KmoRect rect) {
    pub fn addInUTransactionList(&self, rect: &KmoRect) {
        // java if (!this.uTransactionList.contains(rect))
        if !self.uTransactionList.borrow().contains(rect) {
        // java CalculationChain calculationChain = this.getBook().getCalculationChain()
            let calculationChain = self.book().getCalculationChain();
        // java if (calculationChain != null)
        // java calculationChain.interrupt()
            if let Some(calculationChain) = calculationChain { calculationChain.interrupt(); }
        // java this.uTransactionList.add(rect)
            self.uTransactionList.borrow_mut().push(rect.clone());
        }
    }

    // java public void cleanUTransactionList() {
    pub fn cleanUTransactionList(&self) {
        // java this.uTransactionList.clear()
        self.uTransactionList.borrow_mut().clear();
    }

    // java public List<KmoRect> getUTransactionList() {
    pub fn getUTransactionList(&self) -> Vec<KmoRect> {
        // java return this.uTransactionList
        self.uTransactionList.borrow().clone()
    }

    // java public void sheetSettingsDidFinishChanging() {
    pub fn sheetSettingsDidFinishChanging(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoSheet.this.parent.changeListenerGroup().bookSettingsDidFinishChanging()
        */
    }

    // java public void sheetSelectionChanged() {
    pub fn sheetSelectionChanged(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoSheet.this.parent.changeListenerGroup().bookSettingsDidFinishChanging()
        */
    }

    // java public void sheetDidFinishInitializing() {
    pub fn sheetDidFinishInitializing(&self) {
        todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void sheetContentsDidFinishChanging() {
    pub fn sheetContentsDidFinishChanging(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoSheet.this.parent.changeListenerGroup().bookSettingsDidFinishChanging()
        // java KmoSheet.this.parent.changeListenerGroup().bookContentsDidFinishChanging()
        */
    }

    // java public void shapeContentChanged(int id) {
    pub fn shapeContentChanged(&self) {
        todo!("DEFERRED_NON_FUNCTION");
    }

    // java private String deleteIvalidChar(String sheetName) {
    pub fn deleteIvalidChar(sheet_name: Option<&str>) -> Option<String> {
        // java if (null != sheetName && sheetName.length() >= 0)
        if let Some(sheet_name) = sheet_name {
            if sheet_name.encode_utf16().count() >= 0 {
                // java StringBuilder sb = new StringBuilder();
                let mut sb: Vec<u16> = Vec::new();
                // java int i = 0;
                let mut i: i32 = 0;
                // java for(int size = sheetName.length(); i < size; ++i)
                let units: Vec<u16> = sheet_name.encode_utf16().collect();
                let size = units.len() as i32;
                while i < size {
                    // java char ch = sheetName.charAt(i);
                    let ch = units[i as usize];
                    // java if (ch != '\n' && ch != '\r' && ch != '\t')
                    if ch != 10 && ch != 13 && ch != 9 {
                        // java sb.append(ch);
                        sb.push(ch);
                    }
                    i += 1;
                }
                // java return sb.toString();
                return Some(String::from_utf16_lossy(&sb));
            }
        }
        // java return sheetName;
        sheet_name.map(|s| s.to_string())
    }

    // java public int getSheetUid() {
    pub fn getSheetUid(&self) -> i32 {
        // java return this.uid;
        self.uid
    }

    // java public void initDefRowHeight(DefaultRowHeightRecord defRowHeight) {
    pub fn initDefRowHeight(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.setDefRowDyZero(defRowHeight.isDyZero(), true)
        // java this.sheetInfo.setDefRowHeight(defRowHeight.getRowHeight(), true)
        // java this.sheetInfo.setDefRowExAsc(defRowHeight.isExAsc(), true)
        // java this.sheetInfo.setDefRowExDsc(defRowHeight.isExDsc(), true)
        // java this.sheetInfo.setDefRowUnsyned(defRowHeight.isUnsynced(), true)
        */
    }

    // java public void initDefColWidth(int colWidth) {
    pub fn initDefColWidth(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.setDefColWidth(colWidth, true)
        */
    }

    // java public void initStdColWidth(int colWidth) {
    pub fn initStdColWidth(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.setStdColWidth(colWidth, true)
        */
    }

    // java public IGridSheet sharedReadOnlyGridSheet() {
    pub fn sharedReadOnlyGridSheet(sheet: &Rc<RefCell<KmoSheet>>) -> Rc<GridSheetData> {
        // java if (this.shared.gridSheetData == null)
        // rust: no-stmt — 不把视图写回 sheet，避免 Rc 环。每次返回的都是同一张表。登记 §1.7.1。
        // java this.shared.gridSheetData = new GridSheetData(this)
        let gridSheetData = Rc::new(GridSheetData::GridSheetData(Rc::clone(sheet)));
        // java return this.shared.gridSheetData
        gridSheetData
    }

    // java public KmoSheetOp sheetOp() {
    pub fn sheetOp(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetOp
        */
    }

    // java public KmoSheetChangeListenerGroup changeListenerGroup() {
    pub fn changeListenerGroup(&self) -> &super::event::KmoSheetChangeListenerGroup::KmoSheetChangeListenerGroup {
        // java return this.changeListeners;
        &self.changeListeners
    }

    // java public void subscribeChangeEvents(KmoSheetChangeEventsListener l) {
    pub fn subscribeChangeEvents(&self, l: Rc<dyn super::event::KmoSheetChangeListenerGroup::KmoSheetChangeEventsListener>) {
        // java this.changeListeners.add(l)
        self.changeListeners.add(l);
    }

    // java public void unsubscribeChangeEvents(KmoSheetChangeEventsListener l) {
    pub fn unsubscribeChangeEvents(&self, l: &Rc<dyn super::event::KmoSheetChangeListenerGroup::KmoSheetChangeEventsListener>) {
        // java this.changeListeners.remove(l)
        self.changeListeners.remove(l);
    }

    // java public void clearChangeEvents() {
    pub fn clearChangeEvents(&self) {
        // java this.changeListeners.clear()
        self.changeListeners.clear();
    }

    // java public KmoConditionFormatter getCFormatter() {
    pub fn getCFormatter(&self)->&super::condfmt::KmoConditionFormatter::KmoConditionFormatter {
&self.cformatter
    }

    // java public String name() {
    pub fn name(&self) -> String {
        // java return this.sheetInfo != null && this.sheetInfo.name != null ? this.sheetInfo.name : "";
        match &self.sheetInfo.name {
            Some(name) => name.clone(),
            None => String::new(),
        }
    }

    // java public boolean isEmpty() {
    pub fn isEmpty(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java IndexMapIterator<KmoRowInfo> riter = this.rowInfos.iterator()
        // java while(riter.hasNext())
        // java if (riter.next() != null)
        // java return false
        // java IndexMapIterator<KmoColInfo> citer = this.colInfos.iterator()
        // java while(citer.hasNext())
        // java if (citer.next() != null)
        // java return false
        // java KmoRect usedRange = this.getUsedRange()
        // java if (usedRange.tl.row < usedRange.br.row && usedRange.tl.col <= usedRange.br.col)
        // java return false
        // java return true
        */
    }

    // java public SpreadsheetVersion getVersion() {
    pub fn getVersion(&self) -> SpreadsheetVersion {
        // java return this.getBook().getSpreadsheetVersion();
        self.book().getSpreadsheetVersion()
    }

    // java public int getMaxRows() {
    pub fn getMaxRows(&self) -> i32 {
        // java return this.getBook().getSpreadsheetVersion().getMaxRows();
        self.book().getSpreadsheetVersion().getMaxRows()
    }

    // java public int getMaxColumns() {
    pub fn getMaxColumns(&self) -> i32 {
        // java return this.getBook().getSpreadsheetVersion().getMaxColumns();
        self.book().getSpreadsheetVersion().getMaxColumns()
    }

    // java public KmoTableManager getTableManager() {
    pub fn getTableManager(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.tableManager
        */
    }

    // java public KmoTableManagerOp getTableManagerOp() {
    pub fn getTableManagerOp(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return null != this.tableManager ? this.tableManager.getTableManagerOp() : null
        */
    }

    // java public void setStdColWidth(int width) {
    pub fn setStdColWidth(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.setStdColWidth(width, false)
        // java this.changeListeners.sheetSettingsDidFinishChanging()
        */
    }

    // java public int getStdColWidth() {
    pub fn getStdColWidth(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.getStdColWidth()
        */
    }

    // java public void setDefColWidth(int width) {
    pub fn setDefColWidth(&self, width: i32) {
        // Java: this.sheetInfo.setDefColWidth(width, false).
        self.sheetInfo.defColWidth.set(width);
        // Java: this.parent.setDirty(true).
        self.book().setDirty(true);
    }

    // java public int getDefColWidth() {
    pub fn getDefColWidth(&self) -> i32 {
        self.sheetInfo.defColWidth.get()
    }

    // java public void setDefRowHeight(int height) {
    pub fn setDefRowHeight(&self, height: i32) {
        // Java: this.sheetInfo.setDefRowHeight(height, false).
        self.sheetInfo.defRowHeight.set(height);
        // Java: this.parent.setDirty(true).
        self.book().setDirty(true);
    }

    // java public int getDefRowHeight() {
    pub fn getDefRowHeight(&self) -> i32 {
        // java return this.sheetInfo.getDefRowHeight();
        self.sheetInfo.getDefRowHeight()
    }

    // java public void setDefaultRowUnsyned(boolean u) {
    pub fn setDefaultRowUnsyned(&self, u: bool) {
        // Java: this.sheetInfo.setDefRowUnsyned(u, false).
        self.sheetInfo.defRowUnsyned.set(u);
    }

    // java public boolean isDefaultRowUnsyned() {
    pub fn isDefaultRowUnsyned(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.getDefRowUnsyned()
        */
    }

    // java public void setDefaultRowDyZero(boolean z) {
    pub fn setDefaultRowDyZero(&self, z: bool) {
        // Java: this.sheetInfo.setDefRowDyZero(z, false).
        self.sheetInfo.defRowDyZero.set(z);
        // Java: this.parent.setDirty(true).
        self.book().setDirty(true);
    }

    // java public boolean isDefaultRowDyZero() {
    pub fn isDefaultRowDyZero(&self) -> bool {
        // java return this.sheetInfo.getDefRowDyZero()
        self.sheetInfo.getDefRowDyZero()
    }

    // java public void setDefaultRowExAsc(boolean a) {
    pub fn setDefaultRowExAsc(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.setDefRowExAsc(a, false)
        // java this.changeListeners.sheetSettingsDidFinishChanging()
        */
    }

    // java public boolean isDefaultRowExAsc() {
    pub fn isDefaultRowExAsc(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.getDefRowExAsc()
        */
    }

    // java public void setDefaultRowExDsc(boolean d) {
    pub fn setDefaultRowExDsc(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.setDefRowExDsc(d, false)
        // java this.changeListeners.sheetSettingsDidFinishChanging()
        */
    }

    // java public boolean isDefaultRowExDsc() {
    pub fn isDefaultRowExDsc(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.getDefRowExDsc()
        */
    }

    // java public KmoSelection getSelection() {
// java public KmoSelection getSelection() {
// java return this.sheetInfo.selection();
// java }
pub fn getSelection(&self)->Rc<crate::cn::wps::moss::core::sheet::KmoSelection::KmoSelection>{self.sheetInfo.selection.borrow().clone()}


    // java public KmoRect getSelectionRect() {
    pub fn getSelectionRect(&self) -> KmoRect {
        // java return this.sheetInfo.selection().selection()
        self.getSelection().selection()
    }

    // java public void setSelection(KmoSelection sel) {
// java public void setSelection(KmoSelection sel) {
// java this.sheetInfo.setSelection(sel, false);
// java this.changeListeners.sheetSelectionChanged();
// java }
pub fn setSelection(&self,s:Rc<crate::cn::wps::moss::core::sheet::KmoSelection::KmoSelection>)->(){*self.sheetInfo.selection.borrow_mut()=s;self.changeListeners.sheetSelectionChanged();}


    // java public void setSelection(KmoRect range) {
    pub fn setSelection__2(&self, range: &KmoRect) {
        // java this.setSelection(range, range.tl.row, range.tl.col)
        self.setSelection__3(range, range.tl.row, range.tl.col);
    }

    // java public void setSelection(KmoRect _rect, int active_row, int active_col) {
    pub fn setSelection__3(&self, rect: &KmoRect, active_row: i32, active_col: i32) {
        // java this.setSelectionIgnoreExpand(_rect, active_row, active_col, false)
        self.setSelectionIgnoreExpand__2(rect, active_row, active_col, false);
    }

    // java public void setSelectionIgnoreExpand(KmoRect range) {
    pub fn setSelectionIgnoreExpand(&self, range: &KmoRect) {
        // java this.setSelectionIgnoreExpand(range, range.tl.row, range.tl.col, true)
        self.setSelectionIgnoreExpand__2(range, range.tl.row, range.tl.col, true);
    }

    // java private void setSelectionIgnoreExpand(KmoRect _rect, int active_row, int active_col, boolean shouldIgnoreExpand) {
    pub fn setSelectionIgnoreExpand__2(&self, _rect: &KmoRect, mut active_row: i32, mut active_col: i32, shouldIgnoreExpand: bool) {
        // java KmoRect rect = new KmoRect(_rect)
        let mut rect = _rect.clone();
        // java if (!shouldIgnoreExpand)
        if !shouldIgnoreExpand {
        // java SpreadsheetVersion ss = this.parent.getSpreadsheetVersion()
            let ss = self.book().getSpreadsheetVersion();
        // java if ((rect.tl.row != 0 || rect.br.row != ss.getLastRowIndex()) && (rect.tl.col != 0 || rect.br.col != ss.getLastColumnIndex()))
        // java rect = this.gridData.mergedDele().expandBounds(rect)
            if (rect.tl.row != 0 || rect.br.row != ss.getLastRowIndex()) && (rect.tl.col != 0 || rect.br.col != ss.getLastColumnIndex()) { rect = self.grid.mergedDele().expandBounds(&rect); }
        // java if (active_row < rect.tl.row || active_row > rect.br.row)
        // java active_row = rect.tl.row
            if active_row < rect.tl.row || active_row > rect.br.row { active_row = rect.tl.row; }
        // java if (active_col < rect.tl.col || active_col > rect.br.col)
        // java active_col = rect.tl.col
            if active_col < rect.tl.col || active_col > rect.br.col { active_col = rect.tl.col; }
        // java KmoRect mrg = this.gridData.mergedDele().getMergedCells(active_row, active_col)
        // java if (mrg != null)
        // java active_row = mrg.tl.row
        // java active_col = mrg.tl.col
            if let Some(mrg) = self.grid.mergedDele().getMergedCells(active_row, active_col) { active_row = mrg.tl.row; active_col = mrg.tl.col; }
        }
        // java if (!this.getSelectionRect().equals(rect) || active_col != this.getSelection().activeCol() || active_row != this.getSelection().activeRow())
        if self.getSelectionRect() != rect || active_col != self.getSelection().activeCol() || active_row != self.getSelection().activeRow() {
        // java this.lastSelection.copy(this.getSelectionRect())
            *self.lastSelection.borrow_mut() = self.getSelectionRect();
        // java this.sheetInfo.selection().setSelection(rect, active_row, active_col)
            self.getSelection().setSelection(&rect, active_row, active_col);
        // java this.changeListeners.sheetSelectionChanged()
            self.changeListeners.sheetSelectionChanged();
        }
    }

    // java public void getPrintArea(List<KmoRect> ret) {
    pub fn getPrintArea(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java LinkTable lt = this.parent.getLinkTable()
        // java NameRecord nr = lt.getSpecificBuiltinRecord((byte)6, this.getSheetIndex() + 1)
        // java if (ret != null && nr != null)
        // java Ptg[] ptgs = nr.getNameDefinition()
        // java if (ptgs != null)
        // java for(Ptg ptg : ptgs)
        // java if (ptg.getSid() == 59)
        // java Area3DPtg area = (Area3DPtg)ptg
        // java ret.add(new KmoRect(area.getFirstRow(), area.getFirstColumn(), area.getLastRow(), area.getLastColumn()))
        */
    }

    // java public void setPrintArea(List<KmoRect> areas) {
    pub fn setPrintArea(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java LinkTable lt = this.parent.getLinkTable()
        // java int sheetIndex = this.getSheetIndex()
        // java if (areas != null && !areas.isEmpty())
        // java NameRecord nr = lt.getSpecificBuiltinRecord((byte)6, sheetIndex + 1)
        // java if (nr == null)
        // java nr = lt.createBuiltInName((byte)6, sheetIndex + 1)
        // java int extSheet = lt.checkNullSupBookExternSheet(sheetIndex)
        // java int size = areas.size()
        // java Ptg[] ptgs = new Ptg[size]
        // java for(int i = 0; i < size; ++i)
        // java KmoRect rect = (KmoRect)areas.get(i)
        // java if (this.getVersion() == SpreadsheetVersion.EXCEL97)
        // java ptgs[i] = new Area3D03Ptg(rect.tl.row, rect.br.row, rect.tl.col, rect.br.col, false, false, false, false, extSheet)
        // java ptgs[i] = new Area3D10Ptg(rect.tl.row, rect.br.row, rect.tl.col, rect.br.col, false, false, false, false, extSheet)
        // java nr.setNameDefinition(ptgs)
        // java this.parent.setSilentSaveWithoutDirty(true)
        // java lt.removeBuiltinRecord((byte)6, sheetIndex + 1)
        */
    }

    // java public KmoRect adjustRect(KmoRect _rect) {
    pub fn adjustRect(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect rect = new KmoRect(_rect)
        // java SpreadsheetVersion ss = this.parent.getSpreadsheetVersion()
        // java if ((rect.tl.row != 0 || rect.br.row != ss.getLastRowIndex()) && (rect.tl.col != 0 || rect.br.col != ss.getLastColumnIndex()))
        // java ArrayList<KmoRect> merges = new ArrayList()
        // java this.gridData.mergedDele().getIntersectMergedRanges(rect, merges)
        // java for(KmoRect mrg : merges)
        // java if (mrg.tl.row < rect.tl.row)
        // java rect.tl.row = mrg.tl.row
        // java if (mrg.tl.col < rect.tl.col)
        // java rect.tl.col = mrg.tl.col
        // java if (mrg.br.row > rect.br.row)
        // java rect.br.row = mrg.br.row
        // java if (mrg.br.col > rect.br.col)
        // java rect.br.col = mrg.br.col
        // java return rect
        */
    }

    // java public void setSheetName(String name) {
    pub fn setSheetName(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java name = this.deleteIvalidChar(name)
        // java if (!this.parent.isSheetNameUnique(this.getSheetIndex(), name))
        // java throw new SheetNameConflictException()
        // java else if (!AppLogicHelper.isValidSheetName(name))
        // java throw new InvalidSheetNameException()
        // java this.parent.checkChartLink()
        // java this.sheetInfo.name = name
        // java this.parent.setDirty(true)
        // java this.parent.areaWatcher().postSheetNameChanged(this)
        // java this.changeListeners.sheetSettingsDidFinishChangingNonBlockable()
        */
    }

    // java public int getSheetIndex() {
    pub fn getSheetIndex(&self) -> i32 {
        // java return 0 <= this.possibleSheetIndex && this.possibleSheetIndex < this.parent.sheetCount() && this.parent.sheetAt(this.possibleSheetIndex) == this ? this.possibleSheetIndex : (this.possibleSheetIndex = this.parent.sheetIndex(this));
        let possible = self.possibleSheetIndex.get();
        let book = self.book();
        if possible >= 0
            && possible < book.sheetCount()
            && std::ptr::eq(book.sheetAt(possible).as_ref().as_ptr(), self as *const KmoSheet)
        {
            possible
        } else {
            let found = book.sheetIndex(self);
            self.possibleSheetIndex.set(found);
            found
        }
    }

    // java public boolean isSheetHidden() {
    pub fn isSheetHidden(&self) -> bool {
        // java return this.sheetInfo.isHidden || this.sheetInfo.isVeryHidden;
        self.sheetInfo.isHidden.get() || self.sheetInfo.isVeryHidden.get()
    }

    // java public boolean isSheetVeryHidden() {
    pub fn isSheetVeryHidden(&self) -> bool {
        // java return this.sheetInfo.isVeryHidden;
        self.sheetInfo.isVeryHidden.get()
    }

    // java public void setSheetHiddenWithoutNotify(boolean hidden) {
    pub fn setSheetHiddenWithoutNotify(&self, hidden: bool) {
        // java this.sheetInfo.isHidden = hidden;
        self.sheetInfo.isHidden.set(hidden);
    }

    // java public void setSheetVeryHidden(boolean isVeryHidden) {
    pub fn setSheetVeryHidden(&self, isVeryHidden: bool) {
        // java if (isVeryHidden != this.sheetInfo.isVeryHidden)
        if isVeryHidden != self.sheetInfo.isVeryHidden.get() {
            // java this.parent.transaction().reset();
            self.book().transaction().reset();
        }
        // java this.sheetInfo.isVeryHidden = isVeryHidden;
        self.sheetInfo.isVeryHidden.set(isVeryHidden);
        // java this.parent.setDirty(true);
        self.book().setDirty(true);
        // java this.changeListeners.sheetSettingsDidFinishChanging();
        // rust: no-stmt — 监听器不改变 veryHidden 标志。登记 08 §1.7.1。
    }

    // java public void setSheetHidden(boolean hidden) {
    pub fn setSheetHidden(&self, hidden: bool) {
        // java if (hidden != this.sheetInfo.isHidden)
        if hidden != self.sheetInfo.isHidden.get() {
            // java this.parent.transaction().reset();
            self.book().transaction().reset();
        }
        // java this.sheetInfo.isHidden = hidden;
        self.sheetInfo.isHidden.set(hidden);
        // java this.parent.setDirty(true);
        self.book().setDirty(true);
        // java this.changeListeners.sheetSettingsDidFinishChanging();
        // rust: no-stmt — 监听器不改变 hidden 标志。登记 08 §1.7.1。
    }

    // java public void initPaneInfo(boolean frozenPane, boolean frozenPaneNoSplit, int x, int y, int topRow, int leftCol, int activePane) {
// java public void initPaneInfo(boolean frozenPane, boolean frozenPaneNoSplit, int x, int y, int topRow, int leftCol, int activePane) {
// java this.sheetInfo.window2.setFrozen(frozenPane);
// java this.sheetInfo.window2.setFrozenNoSplit(frozenPaneNoSplit);
// java this.sheetInfo.hasPane = true;
// java KmoPaneInfo paneInfo = new KmoPaneInfo();
// java paneInfo.setX(x);
// java paneInfo.setY(y);
// java paneInfo.setTopRow(topRow);
// java paneInfo.setLeftCol(leftCol);
// java paneInfo.setActivePane(activePane);
// java if (frozenPane) {
// java if (x == 0 && activePane == 2 && leftCol != 0) {
// java paneInfo.setLeftCol(0);
// java }
// java
// java if (y == 0 && activePane == 3 && topRow != 0) {
// java paneInfo.setTopRow(0);
// java }
// java
// java if (x != 0 && this.sheetInfo.window2.getLeftCol() + x > leftCol) {
// java paneInfo.setLeftCol(this.sheetInfo.window2.getLeftCol() + x);
// java }
// java
// java if (y != 0 && this.sheetInfo.window2.getTopRow() + y > topRow) {
// java paneInfo.setTopRow(this.sheetInfo.window2.getTopRow() + y);
// java }
// java }
// java
// java this.sheetInfo.setPaneInfo(paneInfo, true);
// java }
pub fn initPaneInfo(&self,frozen:bool,noSplit:bool,x:i32,y:i32,top:i32,left:i32,active:i32){let bits=self.sheetInfo.window2.grbit.get();self.sheetInfo.window2.grbit.set((bits&!264)|if frozen{8}else{0}|if noSplit{256}else{0});self.sheetInfo.hasPane.set(true);let mut pane=KmoPaneInfo::new();pane.setX(x);pane.setY(y);pane.setTopRow(top);pane.setLeftCol(left);pane.setActivePane(active);if frozen{if x==0&&active==2&&left!=0{pane.setLeftCol(0);}if y==0&&active==3&&top!=0{pane.setTopRow(0);}if x!=0&&self.leftTopCol()+x>left{pane.setLeftCol(self.leftTopCol()+x);}if y!=0&&self.leftTopRow()+y>top{pane.setTopRow(self.leftTopRow()+y);}}*self.sheetInfo.paneInfo.borrow_mut()=pane;}



    // java public Window2 getWindow2() {
    pub fn getWindow2(&self) -> &Window2 {
        // java return this.sheetInfo.window2;
        &self.sheetInfo.window2
    }

    // java public void initWindow2(Window2 win2) {
    pub fn initWindow2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.window2 = win2
        */
    }

    // java public void createFrozenPane(int topRow, int leftCol, int row, int col) {
    pub fn createFrozenPane(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (row == topRow && col == leftCol)
        // java this.sheetInfo.hasPane = false
        // java this.sheetInfo.window2.setFrozen(false)
        // java this.sheetInfo.window2.setFrozenNoSplit(false)
        // java this.parent.setDirty(true)
        // java this.sheetInfo.window2.setFrozen(true)
        // java if (!this.sheetInfo.hasPane)
        // java this.sheetInfo.window2.setFrozenNoSplit(true)
        // java this.sheetInfo.setPaneInfo(new KmoPaneInfo(), false)
        // java this.sheetInfo.hasPane = true
        // java this.sheetInfo.window2.setFrozenNoSplit(false)
        // java this.sheetInfo.window2.setTopRow(topRow)
        // java this.sheetInfo.window2.setLeftCol(leftCol)
        // java this.sheetInfo.paneInfo().setY(row - topRow)
        // java this.sheetInfo.paneInfo().setX(col - leftCol)
        // java this.sheetInfo.paneInfo().setTopRow(row)
        // java this.sheetInfo.paneInfo().setLeftCol(col)
        // java if (row == topRow)
        // java this.sheetInfo.paneInfo().setTopRow(0)
        // java this.sheetInfo.paneInfo().setActivePane(1)
        // java else if (col == leftCol)
        // java this.sheetInfo.paneInfo().setLeftCol(0)
        // java this.sheetInfo.paneInfo().setActivePane(2)
        // java this.sheetInfo.paneInfo().setActivePane(0)
        // java this.sheetInfo.selection().setPane((byte)this.sheetInfo.paneInfo().activePane())
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        */
    }

    // java public void removeFrozenPane() {
    pub fn removeFrozenPane(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.removeFrozenPane(true)
        */
    }

    // java public void removeFrozenPane(boolean setDirty) {
    pub fn removeFrozenPane__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.hasPane = false
        // java this.sheetInfo.window2.setFrozen(false)
        // java this.sheetInfo.window2.setFrozenNoSplit(false)
        // java this.sheetInfo.selection().setPane((byte)3)
        // java if (setDirty)
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        */
    }

    // java public void scrollPane(short topRow, short leftmostColumn) {
    pub fn scrollPane(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.paneInfo().setTopRow(this.sheetInfo.paneInfo().y() == 0 ? 0 : topRow)
        // java this.sheetInfo.paneInfo().setLeftCol(this.sheetInfo.paneInfo().x() == 0 ? 0 : leftmostColumn)
        // java this.changeListeners.sheetSettingsDidFinishChanging()
        */
    }

    // java public boolean hasFrozen() {
// java public boolean hasFrozen() {
// java return this.sheetInfo.window2.isFrozen() && this.sheetInfo.hasPane;
// java }
    pub fn hasFrozen(&self)->bool{self.sheetInfo.window2.isFrozen()&&self.sheetInfo.hasPane.get()}


    // java public boolean hasFrozenNoSplit() {
    pub fn hasFrozenNoSplit(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.window2.isFrozenNoSplit()
        */
    }

    // java public boolean hasPane() {
    pub fn hasPane(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.hasPane
        */
    }

    // java public int getPaneColSplit() {
// java public int getPaneColSplit() {
// java return this.sheetInfo.paneInfo().x();
// java }
    pub fn getPaneColSplit(&self)->i32{self.sheetInfo.paneInfo.borrow().x()}


    // java public void setPaneColSplit(int colSplit) {
    pub fn setPaneColSplit(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.paneInfo().setX(colSplit)
        */
    }

    // java public int getPaneRowSplit() {
// java public int getPaneRowSplit() {
// java return this.sheetInfo.paneInfo().y();
// java }
    pub fn getPaneRowSplit(&self)->i32{self.sheetInfo.paneInfo.borrow().y()}


    // java public void setPaneRowSplit(int rowSplit) {
    pub fn setPaneRowSplit(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.paneInfo().setY(rowSplit)
        */
    }

    // java public int getPaneTopRow() {
// java public int getPaneTopRow() {
// java return this.sheetInfo.paneInfo().topRow();
// java }
    pub fn getPaneTopRow(&self)->i32{self.sheetInfo.paneInfo.borrow().topRow()}


    // java public void setPaneTopRow(int row) {
    pub fn setPaneTopRow(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.paneInfo().setTopRow(row)
        // java this.changeListeners.sheetSettingsDidFinishChanging()
        */
    }

    // java public int getPaneLeftCol() {
// java public int getPaneLeftCol() {
// java return this.sheetInfo.paneInfo().leftCol();
// java }
    pub fn getPaneLeftCol(&self)->i32{self.sheetInfo.paneInfo.borrow().leftCol()}


    // java public void setPaneLeftCol(int col) {
    pub fn setPaneLeftCol(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.paneInfo().setLeftCol(col)
        // java this.changeListeners.sheetSettingsDidFinishChanging()
        */
    }

    // java public int getActivePane() {
// java public int getActivePane() {
// java return this.sheetInfo.paneInfo().activePane();
// java }
    pub fn getActivePane(&self)->i32{self.sheetInfo.paneInfo.borrow().activePane()}


    // java public void setSheetTabColor(int color) {
    pub fn setSheetTabColor(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.setSheetTabColor(color, false)
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetSettingsDidFinishChanging()
        */
    }

    // java public int getSheetTabColor() {
    pub fn getSheetTabColor(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.getSheetTabColor()
        */
    }

    // java public boolean isDrawHeader() {
// java public boolean isDrawHeader() {
// java return this.sheetType() == 2 ? false : this.sheetInfo.window2.isDisplayRwCol();
// java }
    pub fn isDrawHeader(&self)->bool{self.sheetType()!=2&&self.sheetInfo.window2.grbit.get()&4!=0}


    // java public void setDefaultHdr(boolean d) {
// java public void setDefaultHdr(boolean d) {
// java this.sheetInfo.window2.setDefaultHdr(d);
// java this.changeListeners.sheetSettingsDidFinishChanging();
// java }
    pub fn setDefaultHdr(&self,v:bool)->(){self.sheetInfo.window2.grbit.set((self.sheetInfo.window2.grbit.get()&!32)|if v{32}else{0});self.changeListeners.sheetSettingsDidFinishChanging();}


    // java public boolean isDefaultHdr() {
    pub fn isDefaultHdr(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.window2.isDefaultHdr()
        */
    }

    // java public void setHeaderColor(int col) {
// java public void setHeaderColor(int col) {
// java this.sheetInfo.window2.setHeaderColor(col);
// java this.parent.setDirty(true);
// java this.changeListeners.sheetSettingsDidFinishChanging();
// java }
    pub fn setHeaderColor(&self,col:i32)->(){self.sheetInfo.window2.icvHdr.set(col);self.book().setDirty(true);self.changeListeners.sheetSettingsDidFinishChanging();}


    // java public int getHeaderColor() {
// java public int getHeaderColor() {
// java return this.sheetInfo.window2.getHeaderColor();
// java }
    pub fn getHeaderColor(&self)->i32{self.sheetInfo.window2.icvHdr.get()}


    // java public byte sheetType() {
    pub fn sheetType(&self) -> i8 {
        // java return this.sheetInfo.sheetType;
        self.sheetInfo.sheetType.get()
    }

    // java public void setSheetType(byte type) {
    pub fn setSheetType(&self, type_: i8) {
        // java this.sheetInfo.sheetType = type;
        self.sheetInfo.sheetType.set(type_);
    }

    // java public void setSelected(boolean d) {
    pub fn setSelected(&self, d: bool) {
        // java this.sheetInfo.window2.setSelected(d)
        self.sheetInfo.window2.setSelected(d);
        self.changeListeners.sheetSettingsDidFinishChanging();
        // java this.changeListeners.sheetSettingsDidFinishChanging()
        // rust: no-stmt — 监听器不改变选中位。
    }

    // java public void setIsDrawHeader(boolean value) {
// java public void setIsDrawHeader(boolean value) {
// java this.sheetInfo.window2.setDisplayRwCol(value);
// java this.parent.setDirty(true);
// java this.changeListeners.sheetSettingsDidFinishChanging();
// java }
    pub fn setIsDrawHeader(&self,v:bool)->(){self.sheetInfo.window2.grbit.set((self.sheetInfo.window2.grbit.get()&!4)|if v{4}else{0});self.book().setDirty(true);self.changeListeners.sheetSettingsDidFinishChanging();}


    // java public boolean isDrawGridLines() {
// java public boolean isDrawGridLines() {
// java return this.sheetType() == 2 ? false : this.sheetInfo.window2.isDisplayGrid();
// java }
    pub fn isDrawGridLines(&self)->bool{self.sheetType()!=2&&self.sheetInfo.window2.grbit.get()&2!=0}


    // java public void setIsDrawGridLines(boolean value) {
// java public void setIsDrawGridLines(boolean value) {
// java this.sheetInfo.window2.setDisplayGrid(value);
// java this.parent.setDirty(true);
// java this.changeListeners.sheetSettingsDidFinishChanging();
// java }
    pub fn setIsDrawGridLines(&self,v:bool)->(){self.sheetInfo.window2.grbit.set((self.sheetInfo.window2.grbit.get()&!2)|if v{2}else{0});self.book().setDirty(true);self.changeListeners.sheetSettingsDidFinishChanging();}


    // java public void setIsDisplayGuts(boolean value) {
// java public void setIsDisplayGuts(boolean value) {
// java this.sheetInfo.window2.setIsDisplayGuts(value);
// java this.changeListeners.sheetSettingsDidFinishChanging();
// java }
    pub fn setIsDisplayGuts(&self,v:bool)->(){self.sheetInfo.window2.grbit.set((self.sheetInfo.window2.grbit.get()&!128)|if v{128}else{0});self.changeListeners.sheetSettingsDidFinishChanging();}


    // java public boolean isDisplayGuts() {
    pub fn isDisplayGuts(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.window2.isDisplayGuts()
        */
    }

    // java public void setIsShowZeros(boolean value) {
// java public void setIsShowZeros(boolean value) {
// java this.sheetInfo.window2.setIsShowZeros(value);
// java this.changeListeners.sheetSettingsDidFinishChanging();
// java }
pub fn setIsShowZeros(&self,v:bool){self.sheetInfo.window2.setIsShowZeros(v);self.changeListeners.sheetSettingsDidFinishChanging();}


    // java public boolean isShowZeros() {
    pub fn isShowZeros(&self)->bool {
self.sheetInfo.window2.isShowZeros()
    }

    // java public boolean isEmptySheet() {
    pub fn isEmptySheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.usedRange.tl.row == 0 && this.usedRange.tl.col == 0 && this.usedRange.br.row == 0 && this.usedRange.br.col == 0
        */
    }

    // java public boolean isEmptySheetEx() {
    pub fn isEmptySheetEx(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.usedRange.tl.row == 0 && this.usedRange.tl.col == 0 && this.usedRange.br.row <= 1 && this.usedRange.br.col <= 1 && !this.isCellExist(0, 0))
        // java return this.sheetDrawingAgg.getShapesCount() < 1
        // java return false
        */
    }

    // java public boolean isEmptyInImportTable() {
    pub fn isEmptyInImportTable(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java CellIterator it = this.getCellIterator(this.getUsedRange())
        // java while(it.hasNext())
        // java it.next()
        // java if (!this.isCellBlank(it.row(), it.col()))
        // java return false
        // java return true
        */
    }

    // java public boolean isEmptyPrintSheet() {
    pub fn isEmptyPrintSheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.usedRange.tl.row == 0 && this.usedRange.tl.col == 0 && this.usedRange.br.row <= 1 && this.usedRange.br.col <= 1 && this.isBlankPrintCell(0, 0))
        // java return this.sheetDrawingAgg.getShapesCount() < 1
        // java return false
        */
    }

    // java public KmoRect getUsedRange() {
    pub fn getUsedRange(&self) -> KmoRect {
        // java KmoRect kmoRect = new KmoRect();
        let mut kmoRect = KmoRect::KmoRect();
        // java this.getUsedRange(kmoRect);
        self.getUsedRange__2(&mut kmoRect);
        // java return kmoRect;
        kmoRect
    }

    // java public void getUsedRange(KmoRect kmoRect) {
    pub fn getUsedRange__2(&self, kmo_rect: &mut KmoRect) {
        // java kmoRect.set(this.usedRange.tl.row, this.usedRange.tl.col, this.usedRange.br.row, this.usedRange.br.col);
        {
            let used = self.usedRange.borrow();
            kmo_rect.set(used.tl.row, used.tl.col, used.br.row, used.br.col);
        }
        // java SpreadsheetVersion sv = this.parent.getSpreadsheetVersion();
        let sv = self.book().getSpreadsheetVersion();
        // java if (kmoRect.br.row >= sv.getMaxRows())
        if kmo_rect.br.row >= sv.getMaxRows() {
            // java kmoRect.br.row = sv.getMaxRows() - 1;
            kmo_rect.br.row = sv.getMaxRows() - 1;
        }
        // java if (kmoRect.br.col >= sv.getMaxColumns())
        if kmo_rect.br.col >= sv.getMaxColumns() {
            // java kmoRect.br.col = sv.getMaxColumns() - 1;
            kmo_rect.br.col = sv.getMaxColumns() - 1;
        }
    }

    // java public KmoRect fitAndGetUsedRange() {
    pub fn fitAndGetUsedRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.gridData.getMatrix().fitUsedRange(this.gridData.mergedDele())
        // java KmoRect kmoRect = new KmoRect(this.usedRange)
        // java return kmoRect
        */
    }

    // java public KmoRect getFitAndGetUsedRange() {
    pub fn getFitAndGetUsedRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect usedRange = new KmoRect()
        // java this.gridData.getMatrix().fitUsedRange(this.gridData.mergedDele(), usedRange)
        // java return usedRange
        */
    }

// java public void setUsedRange(int firstRow, int firstColumn, int endRow, int endColumn) {
// java this.usedRange.tl.row = Math.max(this.usedRange.tl.row, Math.max(0, Math.min(this.getMaxRows(), firstRow)));
// java this.usedRange.tl.col = Math.max(this.usedRange.tl.col, Math.max(0, Math.min(this.getMaxColumns(), firstColumn)));
// java this.usedRange.br.row = Math.max(this.usedRange.br.row, Math.max(0, Math.min(this.getMaxRows(), endRow)));
// java this.usedRange.br.col = Math.max(this.usedRange.br.col, Math.max(0, Math.min(this.getMaxColumns(), endColumn)));
// java if (this.usedRange.br.row < this.usedRange.tl.row) {
// java this.usedRange.br.row = this.usedRange.tl.row;
// java }
// java
// java if (this.usedRange.br.col < this.usedRange.tl.col) {
// java this.usedRange.br.col = this.usedRange.tl.col;
// java }
// java
// java }
pub fn setUsedRange(&self,firstRow:i32,firstColumn:i32,endRow:i32,endColumn:i32){let mut r=self.usedRange.borrow_mut();r.tl.row=r.tl.row.max(firstRow.min(self.getMaxRows()).max(0));r.tl.col=r.tl.col.max(firstColumn.min(self.getMaxColumns()).max(0));r.br.row=r.br.row.max(endRow.min(self.getMaxRows()).max(0));r.br.col=r.br.col.max(endColumn.min(self.getMaxColumns()).max(0));if r.br.row<r.tl.row{r.br.row=r.tl.row;}if r.br.col<r.tl.col{r.br.col=r.tl.col;}}

    // java public KmoGrid getGrid() {
    pub fn getGrid(&self) -> &KmoGrid {
        // java return this.gridData;
        &self.grid
    }

    // java public ExpFmlManager getExpFmlManager() {
    pub fn getExpFmlManager(&self) -> &RefCell<ExpFmlManager> {
        // java return this.expFmlManager;
        &self.expFmlManager
    }

    // java public KmoNoteManager noteMgr() {
    pub fn noteMgr(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetDrawingAgg.getNoteMgr()
        */
    }

    // java public KmoSparklineManager sparklineMgr() {
    pub fn sparklineMgr(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sparklineMgr
        */
    }

    // java public KmoCellValueStore getCellValueStore() {
    pub fn getCellValueStore(&self) -> Rc<RefCell<KmoCellValueStore>> {
        // java return this.parent.getCellValueStore();
        self.book().getCellValueStore()
    }

    // java public KmoColInfo createDefaultColInfo(short col) {
    pub fn createDefaultColInfo(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = KmoColInfo.newAncestorAtom(col, col, this.emptyColWidth())
        // java this.colInfos.setColInfo(colInfo)
        // java return colInfo
        */
    }

    // java public int emptyColWidth() {
    pub fn emptyColWidth(&self) -> i32 {
        // Java: int stdwidth = this.sheetInfo.getStdColWidth();
        let stdwidth = self.sheetInfo.stdColWidth.get();
        // Java: return stdwidth != -1 ? stdwidth : this.sheetInfo.getDefColWidth() * 256;
        if stdwidth != -1 { stdwidth } else { self.sheetInfo.defColWidth.get() * 256 }
    }

    // java public KmoCellPic getCellPic(int row, int col) {
    pub fn getCellPic(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData cellData = SharedObjects.kmoCellDataPool.borrowObject()
        // java KmoFormulaData fmlbuf = SharedObjects.kmoFormulaDataPool.borrowObject()
        // java String content = null
        // java this.getCellData(row, col, cellData)
        // java if (cellData.formulaType == 0)
        // java content = this.getFormattedCellValue(row, col)
        // java if (!content.startsWith("=DISPIMG"))
        // java Object var17 = null
        // java return (KmoCellPic)var17
        // java KmoCellValueStore cvs = this.getCellValueStore()
        // java cvs.formulaValue(cellData.id, fmlbuf)
        // java Ptg[] ptgs = Ptg.readTokens(fmlbuf.getTokensPlainSize(), this.getCellValueStore().formulaTokens(fmlbuf.getTokensId()), this.getBook().getSpreadsheetVersion())
        // java KmoCellPic cellPic = this.getCellPic(ptgs, cellData.formulaType)
        // java if (cellPic == null)
        // java content = content == null ? this.getFormattedCellValue(row, col) : content
        // java Ptg[] picPtgs = null
        // java picPtgs = FormulaParser.parse(content, new EvaluationWorkbook(this.parent), 0, this.getBook().sheetIndex(this), this.parent.getSpreadsheetVersion(), ParserOption.createByLanguage(KmoBooks.getLanguageType()))
        // java catch (RuntimeException var14)
        // java cellPic = this.getCellPic(picPtgs, cellData.formulaType)
        // java KmoCellPic var18 = cellPic
        // java return var18
        // java SharedObjects.kmoFormulaDataPool.returnObject(fmlbuf)
        // java SharedObjects.kmoCellDataPool.returnObject(cellData)
        */
    }

    // java public int getColMaxWidthCell(short col) {
    pub fn getColMaxWidthCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.colInfos.getColInfo(col)
        // java return colInfo == null ? -1 : colInfo.getMaxWidthCell()
        */
    }

    // java public void setColMaxWidthCell(short col, int cell) {
    pub fn setColMaxWidthCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.colInfos.fitAndGetColInfo(col)
        // java if (colInfo == null)
        // java colInfo = this.createDefaultColInfo(col)
        // java this.colInfos.setColInfo(colInfo)
        // java colInfo.setMaxWidthCell(cell)
        */
    }

    // java public int getColMinWidthCell(short col) {
    pub fn getColMinWidthCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.colInfos.getColInfo(col)
        // java return colInfo == null ? -1 : colInfo.getMinWidthCell()
        */
    }

    // java public void setColMinWidthCell(short col, int cell) {
    pub fn setColMinWidthCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.colInfos.fitAndGetColInfo(col)
        // java if (colInfo == null)
        // java colInfo = this.createDefaultColInfo(col)
        // java this.colInfos.setColInfo(colInfo)
        // java colInfo.setMinWidthCell(cell)
        */
    }

    // java public int getColWidth(int col) {
    pub fn getColWidth(&self, col: i32) -> i32 {
        // Java: KmoColInfo colInfo = this.colInfos.getColInfo(col);
        // Java: return colInfo == null ? this.emptyColWidth() : colInfo.getWidth();
        self.colInfos.borrow().getColInfo(col).map(|info| info.getWidth()).unwrap_or_else(|| self.emptyColWidth())
    }

    // java public void setColWidth(short col, int width, boolean isUnsynced) {
    pub fn setColWidth(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.changeColWidth(col, width, false)
        // java if (colInfo != null)
        // java colInfo.setUnsynced(isUnsynced)
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        */
    }

    // java public void setColWidth(short col, int width, boolean isUnsynced, boolean keepHidden) {
    pub fn setColWidth__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.changeColWidth(col, width, keepHidden)
        // java if (colInfo != null)
        // java colInfo.setUnsynced(isUnsynced)
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        */
    }

    // java public void setColWidth(short col, int width) {
    pub fn setColWidth__3(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null != this.changeColWidth(col, width, false))
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        */
    }

    // java private KmoColInfo changeColWidth(short col, int width, boolean keepHidden) {
    pub fn changeColWidth(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (width > 65440)
        // java width = 65440
        // java KmoColInfo colInfo = this.colInfos.fitAndGetColInfo(col)
        // java if (colInfo == null)
        // java colInfo = this.createDefaultColInfo(col)
        // java colInfo.setWidth(width)
        // java if (width != 0 && colInfo.isHidden() && !keepHidden)
        // java colInfo.setHidden(false)
        // java KmoRect rect = new KmoRect(0, col, this.parent.getSpreadsheetVersion().getLastRowIndex(), col)
        // java this.parent.areaWatcher().postContentChangedEvent(this, rect)
        // java return colInfo
        */
    }

    // java public boolean isColHidden(int col) {
// java public boolean isColHidden(int col) {
// java KmoColInfo colInfo = this.colInfos.getColInfo(col);
// java return colInfo == null ? false : colInfo.isHidden();
// java }
    pub fn isColHidden(&self,col:i32)->bool{self.colInfos.borrow().getColInfo(col).is_some_and(|c|c.isHidden())}

    // java public boolean isColAllHidden(int firstCol, int lastCol) {
    pub fn isColAllHidden(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (firstCol > lastCol)
        // java throw new IllegalArgumentException()
        // java IndexMapIterator<KmoColInfo> iter = this.colInfos.iterator(firstCol, lastCol)
        // java int lastChecked
        // java KmoColInfo info
        // java for(lastChecked = firstCol - 1; iter.hasNext(); lastChecked = info.getColIndex())
        // java info = iter.next()
        // java if (info.getColIndex() != lastChecked + 1)
        // java return false
        // java if (!info.isHidden())
        // java return false
        // java if (lastChecked != lastCol)
        // java return false
        // java return true
        */
    }

    // java public boolean isColAllShown(int firstCol, int lastCol) {
    pub fn isColAllShown(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (firstCol > lastCol)
        // java throw new IllegalArgumentException()
        // java IndexMapIterator<KmoColInfo> iter = this.colInfos.iterator(firstCol, lastCol)
        // java while(iter.hasNext())
        // java KmoColInfo info = iter.next()
        // java if (info.isHidden())
        // java return false
        // java return true
        */
    }

    // java public void setColHidden(short col, boolean hidden) {
    pub fn setColHidden(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.setColHiddenInner(col, hidden)
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java KmoRect rect = new KmoRect(0, col, this.parent.getSpreadsheetVersion().getLastRowIndex(), col)
        // java this.parent.areaWatcher().postContentChangedEvent(this, rect)
        // java this.calcAfterHidden(rect)
        */
    }

    // java public void setColHiddenInner(short col, boolean hidden) {
    pub fn setColHiddenInner(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.colInfos.fitAndGetColInfo(col)
        // java if (colInfo == null)
        // java colInfo = this.createDefaultColInfo(col)
        // java colInfo.setHidden(hidden)
        // java if (!hidden && colInfo.getWidth() == 0)
        // java colInfo.setWidth(this.sheetInfo.getDefColWidth())
        */
    }

    // java public boolean isColNull(int col) {
    pub fn isColNull(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.colInfos.getColInfo(col) == null
        */
    }

    // java public boolean isColFormatted(int col) {
    pub fn isColFormatted(&self,col:i32)->bool {
self.colInfos.borrow().getColInfo(col).is_some_and(|c|c.getXFIndex()!=15)
    }

    // java public int getColXFIndex(int col) {
    pub fn getColXFIndex(&self,col:i32)->i32 {
self.colInfos.borrow().getColInfo(col).map_or(15,|c|c.getXFIndex() as i32)
    }

    // java public KmoColInfos getColInfos() {
    pub fn getColInfos(&self) -> &RefCell<KmoColInfos> {
        // Java: return this.colInfos;
        &self.colInfos
    }

    // java public void setColXFIndex(short col, short ixfe) {
    pub fn setColXFIndex(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.colInfos.fitAndGetColInfo(col)
        // java if (colInfo == null)
        // java colInfo = this.createDefaultColInfo(col)
        // java colInfo.setXFIndex(ixfe)
        */
    }

    // java public void initFillRowInfo(int row, short height, short raw_ixf, short option_flags) throws ForceQuitException {
    pub fn initFillRowInfo(&self, row: i32, height: i16, raw_ixf: i16, option_flags: i16) {
        // java if (null != this.parent.getDlProvider() && this.parent.getDlProvider().isOpenForceQuit())
        if self.book().getDlProvider().is_some() && self.book().open_force_quit() {
            // java throw new ForceQuitException();
            panic!("ForceQuitException");
        // java else if (height != this.sheetInfo.getDefRowHeight() || (option_flags & 32) != 0 != this.sheetInfo.getDefRowDyZero() || (option_flags & 64) != 0 != this.sheetInfo.getDefRowUnsyned() || option_flags != 256)
        } else if (height as i32) != self.sheetInfo.getDefRowHeight()
            || (((option_flags & 32) != 0) != self.sheetInfo.getDefRowDyZero())
            || (((option_flags & 64) != 0) != self.sheetInfo.getDefRowUnsyned())
            || option_flags != 256
        {
            // java this.rowInfos.setRowInfo(new KmoRowInfo(row, height, raw_ixf, option_flags));
            self.rowInfos.setRowInfo(Some(Rc::new(KmoRowInfo::KmoRowInfo(row,height,raw_ixf,option_flags))));
        }
    }

    // java public void initFillColInfo(short col, int width, short ixf, short option_flags) {
    pub fn initFillColInfo(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.initFillColInfo(col, col, width, ixf, option_flags)
        */
    }

    // java public void initFillColInfo(int min, int max, int width, short ixf, short option_flags) {
    pub fn initFillColInfo__2(&self, min: i32, max: i32, width: i32, ixf: i16, option_flags: i16) {
        // Java: int maxColumns = this.getMaxColumns();
        let max_columns = self.getMaxColumns();
        // Java: if (min <= max && min < maxColumns) { ... }
        if min <= max && min < max_columns {
            let max = max.min(max_columns - 1);
            // Java: if (width != this.sheetInfo.getDefColWidth() || ixf != 15 || option_flags != 0)
            if width != self.getDefColWidth() || ixf != 15 || option_flags != 0 {
                self.colInfos.borrow_mut().initFillColInfo(min, max, width, ixf, option_flags);
            }
        }
    }


    // java public int getRowMaxHeightCell(int row) {
    pub fn getRowMaxHeightCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row)
        // java return rowInfo == null ? -1 : rowInfo.getMaxHeightCell()
        */
    }

    // java public void setRowMaxHeightCell(int row, short cell) {
    pub fn setRowMaxHeightCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row)
        // java if (rowInfo == null)
        // java rowInfo = this.createDefaultRowInfo(row)
        // java this.rowInfos.setRowInfo(rowInfo)
        // java rowInfo.setMaxHeightCell(cell)
        */
    }

    // java public int getRowMinHeightCell(int row) {
    pub fn getRowMinHeightCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row)
        // java return rowInfo == null ? -1 : rowInfo.getMinHeightCell()
        */
    }

    // java public void setRowMinHeightCell(int row, short cell) {
    pub fn setRowMinHeightCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row)
        // java if (rowInfo == null)
        // java rowInfo = this.createDefaultRowInfo(row)
        // java this.rowInfos.setRowInfo(rowInfo)
        // java rowInfo.setMinHeightCell(cell)
        */
    }

    // java public int getRowHeight(int row) {
    pub fn getRowHeight(&self, row: i32) -> i32 {
        // java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row);
        let row_info=self.rowInfos.getRowInfo(row);
        // java if (rowInfo == null)
        if row_info.is_none() {
            // java return this.sheetInfo.getDefRowHeight();
            return self.sheetInfo.getDefRowHeight();
        }
        // java short rowHeight = rowInfo.getHeight();
        let row_height = row_info.unwrap().getHeight();
        // java return rowHeight >= 2 && rowHeight <= 8192 ? rowHeight : this.sheetInfo.getDefRowHeight();
        if row_height >= 2 && row_height <= 8192 {
            row_height as i32
        } else {
            self.sheetInfo.getDefRowHeight()
        }
    }

    // java public boolean getRowUnsynced(int row) {
    pub fn getRowUnsynced(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row)
        // java return rowInfo == null ? this.sheetInfo.getDefRowUnsyned() : rowInfo.isUnsynced()
        */
    }

    // java public boolean getColUnsynced(int col) {
    pub fn getColUnsynced(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.colInfos.getColInfo(col)
        // java return colInfo == null ? false : colInfo.isUnsynced()
        */
    }

    // java public KmoRowInfo createDefaultRowInfo(int row) {
    pub fn createDefaultRowInfo(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int defRowHeight = this.sheetInfo.getDefRowHeight()
        // java boolean defaultRowDyZero = this.sheetInfo.getDefRowDyZero()
        // java boolean defaultRowUnsyned = this.sheetInfo.getDefRowUnsyned()
        // java KmoRowInfo rowInfo = KmoRowInfo.newAncestorAtom(row, (short)defRowHeight, (short)15, (short)0, false, defaultRowDyZero, defaultRowUnsyned, false, false, false)
        // java rowInfo.setHiddenRowsManager(this)
        // java return rowInfo
        */
    }

    // java public void setRowHeight(int row, short rowHeight, boolean isUnsynced) {
    pub fn setRowHeight(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRowInfo rowInfo = this.changeRowHeight(row, rowHeight, false)
        // java if (rowInfo != null)
        // java rowInfo.setUnsynced(isUnsynced)
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java KmoRect rect = new KmoRect(row, 0, row, this.parent.getSpreadsheetVersion().getLastColumnIndex())
        // java this.parent.areaWatcher().postContentChangedEvent(this, rect)
        */
    }

    // java public void setRowHeight(int row, short rowHeight, boolean isUnsynced, boolean keepHidden) {
    pub fn setRowHeight__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRowInfo rowInfo = this.changeRowHeight(row, rowHeight, keepHidden)
        // java if (rowInfo != null)
        // java rowInfo.setUnsynced(isUnsynced)
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java KmoRect rect = new KmoRect(row, 0, row, this.parent.getSpreadsheetVersion().getLastColumnIndex())
        // java this.parent.areaWatcher().postContentChangedEvent(this, rect)
        */
    }

    // java public void setRowHeight(int row, short rowHeight) {
    pub fn setRowHeight__3(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null != this.changeRowHeight(row, rowHeight, false))
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java KmoRect rect = new KmoRect(row, 0, row, this.parent.getSpreadsheetVersion().getLastColumnIndex())
        // java this.parent.areaWatcher().postContentChangedEvent(this, rect)
        */
    }

    // java private KmoRowInfo changeRowHeight(int row, short rowHeight, boolean keepHidden) {
    pub fn changeRowHeight(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (rowHeight > 8192)
        // java rowHeight = 8192
        // java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row)
        // java if (rowInfo == null)
        // java if (rowHeight == this.sheetInfo.getDefRowHeight())
        // java return null
        // java rowInfo = this.createDefaultRowInfo(row)
        // java this.rowInfos.setRowInfo(rowInfo)
        // java rowInfo.setHeight(rowHeight)
        // java if (rowHeight != 0 && rowInfo.isHidden() && !keepHidden)
        // java rowInfo.setHidden(false)
        // java return rowInfo
        */
    }

    // java public boolean isAreaHidden(int firstRow, int firstCol, int lastRow, int lastCol) {
    pub fn isAreaHidden(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return !this.isRowAllShown(firstRow, lastRow) || !this.isColAllShown(firstCol, lastCol)
        */
    }

    // java public boolean isRowHidden(int row) {
// java public boolean isRowHidden(int row) {
// java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row);
// java return rowInfo == null ? this.sheetInfo.getDefRowDyZero() : rowInfo.isHidden();
// java }
    pub fn isRowHidden(&self,row:i32)->bool{self.rowInfos.getRowInfo(row).map_or(self.sheetInfo.getDefRowDyZero(),|r|r.isHidden())}

    // java public boolean isRowAllHidden(int firstRow, int lastRow) {
    pub fn isRowAllHidden(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (firstRow > lastRow)
        // java throw new IllegalArgumentException()
        // java boolean defaultHidden = this.sheetInfo.getDefRowDyZero()
        // java IndexMapIterator<KmoRowInfo> iter = this.rowInfos.iterator(firstRow, lastRow)
        // java int lastChecked
        // java KmoRowInfo info
        // java for(lastChecked = firstRow - 1; iter.hasNext(); lastChecked = info.getRowIndex())
        // java info = iter.next()
        // java if (!defaultHidden && info.getRowIndex() != lastChecked + 1)
        // java return false
        // java if (!info.isHidden())
        // java return false
        // java if (lastChecked != lastRow)
        // java return defaultHidden
        // java return true
        */
    }

    // java public boolean isRowAllShown(int firstRow, int lastRow) {
    pub fn isRowAllShown(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (firstRow > lastRow)
        // java throw new IllegalArgumentException()
        // java boolean defaultHidden = this.sheetInfo.getDefRowDyZero()
        // java IndexMapIterator<KmoRowInfo> iter = this.rowInfos.iterator(firstRow, lastRow)
        // java int lastChecked
        // java KmoRowInfo info
        // java for(lastChecked = firstRow - 1; iter.hasNext(); lastChecked = info.getRowIndex())
        // java info = iter.next()
        // java if (defaultHidden && info.getRowIndex() != lastChecked + 1)
        // java return false
        // java if (info.isHidden())
        // java return false
        // java if (lastChecked != lastRow)
        // java return !defaultHidden
        // java return true
        */
    }

    // java public void setRowHiddenInner(int row, boolean hidden) {
    pub fn setRowHiddenInner(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.setRowHiddenInner(row, hidden, false)
        */
    }

    // java public void setRowHiddenInner(int row, boolean hidden, boolean filter) {
    pub fn setRowHiddenInner__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row)
        // java if (rowInfo == null)
        // java if (hidden == this.sheetInfo.getDefRowDyZero())
        // java if (hidden)
        // java this.optHiddenRowColManager.setRowOperable(row, true)
        // java return
        // java rowInfo = this.createDefaultRowInfo(row)
        // java this.rowInfos.setRowInfo(rowInfo)
        // java if (filter)
        // java rowInfo.setHiddenForFilter(hidden)
        // java rowInfo.setHidden(hidden)
        */
    }

    // java public void setRowHidden(int row, boolean hidden) {
    pub fn setRowHidden(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.setRowHiddenInner(row, hidden)
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java KmoRect rect = new KmoRect(row, 0, row, this.parent.getSpreadsheetVersion().getLastColumnIndex())
        // java this.parent.areaWatcher().postContentChangedEvent(this, rect)
        // java this.calcAfterHidden(rect)
        */
    }

    // java public boolean isRowNull(int row) {
    pub fn isRowNull(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.rowInfos.getRowInfo(row) == null
        */
    }

    // java public boolean isRowFormatted(int row) {
    pub fn isRowFormatted(&self,row:i32)->bool {
// java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row); return rowInfo == null ? false : rowInfo.isFormated();
self.rowInfos.getRowInfo(row).is_some_and(|r|r.isFormated())
    }

    // java public KmoXF getRowXF(int row) {
    pub fn getRowXF(&self,row:i32)->Option<Rc<crate::cn::wps::moss::app::xf::KmoXF::KmoXF>> {
        // java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row);
        let rowInfo=self.rowInfos.getRowInfo(row);
        // java return rowInfo == null ? this.parent.getXFManager().getXF(15) : this.parent.getXFManager().getXF(rowInfo.getXFIndex());
        self.book().getXFManager().borrow().getXF(rowInfo.map_or(15,|r|r.getXFIndex()as i32))
    }

    // java public KmoXF getColXF(int col) {
    pub fn getColXF(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.colInfos.getColInfo(col)
        // java return colInfo == null ? this.parent.getXFManager().getXF(15) : this.parent.getXFManager().getXF(colInfo.getXFIndex())
        */
    }

    // java public void setRowXF(int row, short ixfe) {
    pub fn setRowXF(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row)
        // java if (rowInfo == null)
        // java if (ixfe == 15)
        // java return
        // java rowInfo = this.createDefaultRowInfo(row)
        // java this.rowInfos.setRowInfo(rowInfo)
        // java rowInfo.setIsFormatted(true)
        // java rowInfo.setXFIndex(ixfe)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        */
    }

    // java public int getRowXFIndex(int row) {
    pub fn getRowXFIndex(&self,row:i32)->i32 {
// java KmoRowInfo rowInfo = this.rowInfos.getRowInfo(row); return rowInfo == null ? 15 : rowInfo.getXFIndex();
self.rowInfos.getRowInfo(row).map_or(15,|r|r.getXFIndex()as i32)
    }

    // java public KmoRowInfos getRowInfos() {
    pub fn getRowInfos(&self)->Rc<KmoRowInfos>{self.rowInfos.clone()}

    // java public KmoHiddenRowsManager getHiddenRowsManager() {
    pub fn getHiddenRowsManager(&self)->Rc<crate::cn::wps::moss::core::sheet::KmoHiddenRowsManager::KmoHiddenRowsManager>{
        // java return this.hiddenRowsManager;
        self.hiddenRowsManager.borrow().as_ref().expect("NullPointerException").clone()
    }

    // java public OptHiddenRowColManager getOptHiddenRowColManager() {
    pub fn getOptHiddenRowColManager(&self)->Rc<super::rowcol::OptHiddenRowColManager::OptHiddenRowColManager>{
        // java return this.optHiddenRowColManager;
        self.optHiddenRowColManager.borrow().as_ref().expect("NullPointerException").clone()
    }

    // java public boolean isCellExist(int row, int col) {
    pub fn isCellExist(&self,row:i32,col:i32)->bool {
self.grid.cellDele().exists(row,col)
    }

    // java public boolean isBlankPrintCell(int row, int col) {
    pub fn isBlankPrintCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (!this.isColHidden(col) && !this.isRowHidden(row))
        // java if (this.getMergedRange(row, col) != null)
        // java return false
        // java else if (!this.isCellExist(row, col))
        // java return true
        // java KmoCellData data = this.getCellData(row, col)
        // java if (data.ixf != 15)
        // java KmoXF xf = this.parent.getXFManager().getXF(data.ixf)
        // java if (xf != null && (xf.hasBorder() || xf.hasFill()))
        // java return false
        // java if (data.id != 0)
        // java return false
        // java return true
        // java return true
        */
    }

    // java public boolean isBlankCellValue(int row, int col) {
// java public boolean isBlankCellValue(int row, int col) {
// java return this.gridData.getMatrix().isBlankCell(row, col);
// java }
pub fn isBlankCellValue(&self,row:i32,col:i32)->bool{self.grid.getMatrix().borrow().isBlankCell(row,col)}

    // java public boolean hasRangeNotBlankCount(KmoRect range, int count) {
    pub fn hasRangeNotBlankCount(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java CellIterator it = this.getCellIterator(range)
        // java int counter = 0
        // java while(it.hasNext())
        // java it.next()
        // java if (!KmoCellValueStore.isBlankId(it.cellData().id))
        // java ++counter
        // java if (counter > count)
        // java return true
        // java return false
        */
    }

    // java public boolean isSingleCell(int firstRow, int lastRow, int firstCol, int lastCol) {
    pub fn isSingleCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (firstRow == lastRow && firstCol == lastCol)
        // java return true
        // java KmoRect rect = this.getMergedRange(firstRow, firstCol)
        // java if (rect == null)
        // java return false
        // java return rect.tl.row == firstRow && rect.tl.col == firstCol && rect.br.row == lastRow && rect.br.col == lastCol
        */
    }

    // java public boolean isSingleRow(KmoRect range) {
    pub fn isSingleRow(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int height = range.height()
        // java if (height == 1)
        // java return true
        // java ArrayList<KmoRect> mergedRanges = new ArrayList()
        // java this.getGrid().mergedDele().getContainsMergedRanges(range, mergedRanges)
        // java int mergedRangesSize = mergedRanges.size()
        // java if (mergedRangesSize == 0)
        // java return false
        // java boolean isSingleRow = true
        // java for(int i = 0; i < mergedRangesSize; ++i)
        // java if (((KmoRect)mergedRanges.get(i)).height() != height)
        // java isSingleRow = false
        // java break
        // java if (isSingleRow)
        // java for(int col = range.tl.col; col <= range.br.col; ++col)
        // java boolean isCellInMergedRange = false
        // java for(int i = 0; i < mergedRangesSize; ++i)
        // java if (((KmoRect)mergedRanges.get(i)).isContains(range.tl.row, col))
        // java isCellInMergedRange = true
        // java break
        // java if (!isCellInMergedRange)
        // java return false
        // java return isSingleRow
        */
    }

    // java public boolean isSingleCol(KmoRect range) {
    pub fn isSingleCol(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int width = range.width()
        // java if (width == 1)
        // java return true
        // java ArrayList<KmoRect> mergedRanges = new ArrayList()
        // java this.getGrid().mergedDele().getContainsMergedRanges(range, mergedRanges)
        // java int mergedRangesSize = mergedRanges.size()
        // java if (mergedRangesSize == 0)
        // java return false
        // java boolean isSingleCol = true
        // java for(int i = 0; i < mergedRangesSize; ++i)
        // java if (((KmoRect)mergedRanges.get(i)).width() != width)
        // java isSingleCol = false
        // java break
        // java if (isSingleCol)
        // java for(int row = range.tl.row; row <= range.br.row; ++row)
        // java boolean isCellInMergedRange = false
        // java for(int i = 0; i < mergedRangesSize; ++i)
        // java if (((KmoRect)mergedRanges.get(i)).isContains(row, range.tl.col))
        // java isCellInMergedRange = true
        // java break
        // java if (!isCellInMergedRange)
        // java return false
        // java return isSingleCol
        */
    }

    // java public KmoCellData getCellData(int row, int col) {
    pub fn getCellData(&self, row: i32, col: i32) -> KmoCellData {
        // java KmoCellDelegate dele = this.gridData.cellDele();
        // java KmoCellData data = new KmoCellData();
        let mut data = KmoCellData::default();
        // java dele.read(row, col, data);
        self.grid.cellDele().read(row, col, &mut data);
        // java return data;
        data
    }

    // java public int getCellId(int row, int col) {
    pub fn getCellId(&self,row:i32,col:i32)->i32 {
        // java KmoMatrix matrix=this.gridData.getMatrix();
        let matrix=self.grid.getMatrix();
        // java return matrix.get(row,col,this.cellIdBuff)?KmoCellData.getId(this.cellIdBuff):0;
        let mut cellIdBuff=self.cellIdBuff.borrow_mut();let found=matrix.borrow().get(row,col,&mut *cellIdBuff);if found{KmoCellData::getId(&*cellIdBuff)}else{0}
    }

    // java public int getColWidthTwips(int col) {
    pub fn getColWidthTwips(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return (int)cn.wps.moffice.drawing.app.util.UnitsConverter.emusToTwips(cn.wps.moffice.drawing.app.util.UnitsConverter.columnUnitsToEmus((float)this.getColWidth(col), this.getBook().getXFManager().normalCharWidthTwip))
        */
    }

    // java public KmoRect getLastSelection() {
    pub fn getLastSelection(&self) -> KmoRect {
        // java return this.lastSelection
        self.lastSelection.borrow().clone()
    }

    // java public float getAdjustedCellHeightTwips(int row, int col, KmoXF cellXF) {
    pub fn getAdjustedCellHeightTwips(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int rowHeight = this.getRowHeight(row)
        // java KmoBook book = this.getBook()
        // java ILayoutInfo layoutInfo = book.getLayoutInfo()
        // java if (layoutInfo == null)
        // java return (float)rowHeight
        // java cellXF = null != cellXF ? cellXF : this.getCellXF(row, col)
        // java if (null == cellXF)
        // java return (float)rowHeight
        // java KmoFont kmoFont = cellXF.getFont()
        // java if (kmoFont == null)
        // java return (float)rowHeight
        // java FontInfo fontInfo = new FontInfo(kmoFont.getColorPaletteIndex(), kmoFont.getBoldWeight(), kmoFont.isItalic(), kmoFont.getUnderline(), kmoFont.getFontHeight() / 20, kmoFont.getFontName(), kmoFont.getSuperSubScript(), kmoFont.isStrikeout())
        // java return layoutInfo.getAdjustedCellHeight(row, col, fontInfo, true)
        */
    }

    // java public KmoSortState getKmoSortState() {
    pub fn getKmoSortState(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.kmoSortState
        */
    }

    // java public void setKmoSortState(KmoSortState kmoSortState) {
    pub fn setKmoSortState(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.kmoSortState = kmoSortState
        */
    }

    // java public boolean isDashboard() {
    pub fn isDashboard(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetType() == 16
        */
    }

    // java public int getSheetId() {
    pub fn getSheetId(&self) -> i32 {
        // java return this.sheetInfo.sheetId;
        self.sheetInfo.sheetId
    }

    // java public CellIterator getCellIterator(int firstRow, int lastRow, int firstCol, int lastCol) {
    pub fn getCellIterator(&self, firstRow: i32, lastRow: i32, firstCol: i32, lastCol: i32) -> CellIterator {
        // java KmoRect range = new KmoRect(firstRow, firstCol, lastRow, lastCol);
        let range = KmoRect::KmoRect__bounds(firstRow, firstCol, lastRow, lastCol);
        // java return new CellIterator(this.gridData.getMatrix().getCellDataIterator(range, true, true, true));
        self.getCellIterator__2(&range)
    }

    // java public CellIterator getCellIterator(KmoRect range) {
    pub fn getCellIterator__2(&self, range: &KmoRect) -> CellIterator {
        // java return new CellIterator(this.gridData.getMatrix().getCellDataIterator(range, true, true, true));
        self.getCellIterator__3(range, true, true, true)
    }

    // java public CellIterator getCellIterator(KmoRect range, boolean rowFirst, boolean leftToRight, boolean topToBottom) {
    pub fn getCellIterator__3(&self, range: &KmoRect, rowFirst: bool, leftToRight: bool, topToBottom: bool) -> CellIterator {
        // java return new CellIterator(this.gridData.getMatrix().getCellDataIterator(range, rowFirst, leftToRight, topToBottom));
        let matrix = self.grid.getMatrix();
        CellIterator::CellIterator(KmoMatrix::getCellDataIterator(&matrix, range, rowFirst, leftToRight, topToBottom))
    }

    // java public void getCellData(int row, int col, KmoCellData celldata) {
    pub fn getCellData__2(&self, row: i32, col: i32, celldata: &mut KmoCellData) {
        // java KmoCellDelegate dele = this.gridData.cellDele();
        // java dele.read(row, col, celldata);
        self.grid.cellDele().read(row, col, celldata);
    }

    // java public void setCellData(int row, int col, KmoCellData cell) {
    pub fn setCellData(&self, row: i32, col: i32, cell: &KmoCellData) {
        // java KmoCellDelegate dele = this.gridData.cellDele();
        // java dele.write(row, col, cell);
        self.grid.cellDele().write(row, col, cell);
    }

    // java public void setCellDataCalcResult(int row, int col, KmoCellData cell) {
    pub fn setCellDataCalcResult(&self, row: i32, col: i32, cell: &KmoCellData) {
        // java KmoCellDelegate dele = this.gridData.cellDele();
        let dele = self.grid.cellDele();
        // java dele.writeCalcResult(row, col, cell);
        dele.writeCalcResult(row, col, cell);
    }

    // java public int getCellIXF(int row, int col) {
    pub fn getCellIXF(&self,row:i32,col:i32)->i32 {
// java matrix.get(row,col,xfBuff);if(hasData&&!isEmptyByte)return KmoCellData.getIXF(xfBuff);else row,then column,then 15.
let mut buff=KmoMatrixToolkit::newBuff();let hasData=self.grid.getMatrix().borrow().get(row,col,&mut buff);if hasData&&!KmoMatrix::isEmptyByte(&buff){KmoCellData::getIXF(&buff)}else if self.isRowFormatted(row){self.getRowXFIndex(row)}else if self.isColFormatted(col){self.getColXFIndex(col)}else{15}
    }

    // java public KmoXF getCellXF(int row, int col, byte[] xfBuff) {
    pub fn getCellXF(&self,row:i32,col:i32,xfBuff:&[u8])->Option<Rc<super::xf::KmoXF::KmoXF>> {
self.book().getXFManager().borrow().getXF(self.getCellIXF__2(row,col,xfBuff))
    }

    // java public int getCellIXF(int row, int col, byte[] xfBuff) {
    pub fn getCellIXF__2(&self,row:i32,col:i32,xfBuff:&[u8])->i32 {
if !KmoMatrix::isEmptyByte(xfBuff){KmoCellData::getIXF(xfBuff)}else if self.isRowFormatted(row){self.getRowXFIndex(row)}else{self.getColXFIndex(col)}
    }

    // java public int getCellIXF(int row, int col, KmoCellData data) {
    pub fn getCellIXF__3(&self,row:i32,col:i32,data:&KmoCellData)->i32 {
if !data.isNull(){data.ixf}else if self.isRowFormatted(row){self.getRowXFIndex(row)}else if self.isColFormatted(col){self.getColXFIndex(col)}else{15}
    }

    // java public KmoXF getCellXF(int row, int col) {
    pub fn getCellXF__2(&self,row:i32,col:i32)->Option<Rc<super::xf::KmoXF::KmoXF>> {
let book=self.book();let manager=book.getXFManager().borrow();manager.getXF(self.getCellIXF(row,col)).or_else(||manager.getXF(15))
    }

    // java public int getCellXFIndex(int row, int col) {
    pub fn getCellXFIndex(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData data = this.getCellData(row, col)
        // java return data.ixf
        */
    }

    // java public String getCellStringValue(int row, int col) {
// java public String getCellStringValue(int row, int col) {
// java KmoCellValueStore valueStore = this.parent.getCellValueStore();
// java KmoCellData cellData = SharedObjects.kmoCellDataPool.borrowObject();
// java this.getCellData(row, col, cellData);
// java int valueId = cellData.id;
// java SharedObjects.kmoCellDataPool.returnObject(cellData);
// java if (KmoCellValueStore.identifyTypeByID(valueId) == 7) {
// java KmoFormulaData fmlData = SharedObjects.kmoFormulaDataPool.borrowObject();
// java valueStore.formulaValue(valueId, fmlData);
// java valueId = fmlData.getCacheValueId();
// java fmlData = SharedObjects.kmoFormulaDataPool.returnObject(fmlData);
// java }
// java
// java return valueStore.stringValue(valueId);
// java }
pub fn getCellStringValue(&self,row:i32,col:i32)->String {
let book=self.getBook();let store=book.getCellValueStore();let mut valueId=self.getCellData(row,col).id;if crate::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore::identifyTypeByID(valueId)==7{let mut f=KmoFormulaData::KmoFormulaData();store.borrow().formulaValue(valueId,&mut f);valueId=f.getCacheValueId();}let value=store.borrow_mut().stringValue(valueId);value
}


    // java public boolean getCellBooleanValue(int row, int col) {
// java public boolean getCellBooleanValue(int row, int col) {
// java KmoCellValueStore valueStore = this.parent.getCellValueStore();
// java KmoCellData cellData = SharedObjects.kmoCellDataPool.borrowObject();
// java this.getCellData(row, col, cellData);
// java int valueId = cellData.id;
// java SharedObjects.kmoCellDataPool.returnObject(cellData);
// java if (KmoCellValueStore.identifyTypeByID(valueId) == 7) {
// java KmoFormulaData fmlData = SharedObjects.kmoFormulaDataPool.borrowObject();
// java valueStore.formulaValue(valueId, fmlData);
// java valueId = fmlData.getCacheValueId();
// java fmlData = SharedObjects.kmoFormulaDataPool.returnObject(fmlData);
// java }
// java
// java return valueStore.booleanValue(valueId);
// java }
pub fn getCellBooleanValue(&self,row:i32,col:i32)->bool {
let book=self.getBook();let store=book.getCellValueStore();let mut valueId=self.getCellData(row,col).id;if crate::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore::identifyTypeByID(valueId)==7{let mut f=KmoFormulaData::KmoFormulaData();store.borrow().formulaValue(valueId,&mut f);valueId=f.getCacheValueId();}let value=store.borrow_mut().booleanValue(valueId);value
}


    // java public byte getCellErrorCode(int row, int col) {
// java public byte getCellErrorCode(int row, int col) {
// java KmoCellValueStore valueStore = this.parent.getCellValueStore();
// java KmoCellData cellData = SharedObjects.kmoCellDataPool.borrowObject();
// java this.getCellData(row, col, cellData);
// java int valueId = cellData.id;
// java SharedObjects.kmoCellDataPool.returnObject(cellData);
// java if (KmoCellValueStore.identifyTypeByID(valueId) == 7) {
// java KmoFormulaData fmlData = SharedObjects.kmoFormulaDataPool.borrowObject();
// java valueStore.formulaValue(valueId, fmlData);
// java valueId = fmlData.getCacheValueId();
// java fmlData = SharedObjects.kmoFormulaDataPool.returnObject(fmlData);
// java }
// java
// java return valueStore.errorValue(valueId);
// java }
pub fn getCellErrorCode(&self,row:i32,col:i32)->i8 {
let book=self.getBook();let store=book.getCellValueStore();let mut valueId=self.getCellData(row,col).id;if crate::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore::identifyTypeByID(valueId)==7{let mut f=KmoFormulaData::KmoFormulaData();store.borrow().formulaValue(valueId,&mut f);valueId=f.getCacheValueId();}let value=store.borrow_mut().errorValue(valueId);value
}


    // java public double getCellNumericValue(int row, int col) {
// java public double getCellNumericValue(int row, int col) {
// java KmoCellValueStore valueStore = this.parent.getCellValueStore();
// java KmoCellData cellData = SharedObjects.kmoCellDataPool.borrowObject();
// java this.getCellData(row, col, cellData);
// java int valueId = cellData.id;
// java SharedObjects.kmoCellDataPool.returnObject(cellData);
// java if (KmoCellValueStore.identifyTypeByID(valueId) == 7) {
// java KmoFormulaData fmlData = SharedObjects.kmoFormulaDataPool.borrowObject();
// java valueStore.formulaValue(valueId, fmlData);
// java valueId = fmlData.getCacheValueId();
// java fmlData = SharedObjects.kmoFormulaDataPool.returnObject(fmlData);
// java }
// java
// java return valueStore.doubleValue(valueId);
// java }
pub fn getCellNumericValue(&self,row:i32,col:i32)->f64 {
let book=self.getBook();let store=book.getCellValueStore();let mut valueId=self.getCellData(row,col).id;if crate::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore::identifyTypeByID(valueId)==7{let mut f=KmoFormulaData::KmoFormulaData();store.borrow().formulaValue(valueId,&mut f);valueId=f.getCacheValueId();}let value=store.borrow_mut().doubleValue(valueId);value
}


    // java public int getCellValueType(int row, int col, CellTypeInfo cellTypeInfo) {
    pub fn getCellValueType(
        &self,
        row: i32,
        col: i32,
        mut cell_type_info: Option<&mut CellTypeInfo>,
    ) -> i32 {
        // java int id = 0;
        let mut id: i32 = 0;
        // java int type = 0;
        let mut type_: i32 = 0;
        // java byte[] cellTypeBuff = cellTypeInfo != null ? cellTypeInfo.cellTypeBuff : (byte[])KmoSheetSharedObjects.cellDataBuff.get();
        let mut local = KmoMatrixToolkit::newBuff();
        let matrix = self.grid.getMatrix();
        let got = match cell_type_info.as_mut() {
            Some(info) => matrix.borrow().get(row, col, &mut info.cellTypeBuff),
            None => matrix.borrow().get(row, col, &mut local),
        };
        // java if (this.gridData.getMatrix().get(row, col, cellTypeBuff))
        if got {
            // java id = KmoCellData.getId(cellTypeBuff);
            let buff: &[u8] = match cell_type_info.as_ref() {
                Some(info) => &info.cellTypeBuff,
                None => &local,
            };
            id = KmoCellData::getId(buff);
        }
        // java type = KmoCellValueStore.identifyTypeByID(id);
        type_ = KmoCellValueStore::identifyTypeByID(id);
        // java if (type == 7)
        if type_ == 7 {
            // java id = this.getCellValueStore().formulaCachaId(id);
            id = self.getCellValueStore().borrow().formulaCachaId(id);
            // java type = KmoCellValueStore.identifyTypeByID(id);
            type_ = KmoCellValueStore::identifyTypeByID(id);
        }
        // java if (cellTypeInfo != null)
        if let Some(info) = cell_type_info {
            // java cellTypeInfo.cellType = type;
            info.cellType = type_;
            // java cellTypeInfo.hasRun = type == 2 ? this.getCellValueStore().isRunsString2(id) : false;
            info.hasRun = if type_ == 2 {
                self.getCellValueStore().borrow().isRunsString2(id)
            } else {
                false
            };
        }
        // java return type;
        type_
    }

    // java public int getCellValueType(int row, int col) {
    pub fn getCellValueType__2(&self, row: i32, col: i32) -> i32 {
        // java return this.getCellValueType(row, col, (CellTypeInfo)null);
        self.getCellValueType(row, col, None)
    }

    // java public int getCellValueId(int row, int col) {
    pub fn getCellValueId(&self, row: i32, col: i32) -> i32 {
        // java int id = 0;
        let mut id = 0;
        // java if (this.gridData.getMatrix().get(row, col, cellTypeBuff))
        let mut buff = KmoMatrixToolkit::newBuff();
        if self.grid.getMatrix().borrow().get(row, col, &mut buff) {
            // java id = KmoCellData.getId(cellTypeBuff);
            id = KmoCellData::getId(&buff);
        }
        // java id = this.transFmlIdToValueId(id);
        id = self.transFmlIdToValueId(id);
        // java return var5;
        id
    }

    // java public int transFmlIdToValueId(int fmlId) {
    pub fn transFmlIdToValueId(&self, mut fmlId: i32) -> i32 {
        // java if (KmoCellValueStore.isFormulaId(fmlId))
        if KmoCellValueStore::isFormulaId(fmlId) {
            // java this.getCellValueStore().formulaValue(fmlId, fmlData);
            // java fmlId = fmlData.getCacheValueId();
            fmlId = self.getCellValueStore().borrow().formulaCachaId(fmlId);
        }
        // java return fmlId;
        fmlId
    }

    // java public int getCellType(int row, int col) {
    pub fn getCellType(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData data = this.getCellData(row, col)
        // java return KmoCellValueStore.identifyTypeByID(data.id)
        */
    }

    // java public int getCellFmlType(int row, int col) {
    pub fn getCellFmlType(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData data = this.getCellData(row, col)
        // java return data.formulaType
        */
    }

    // java public int getCellDateType(int row, int col) {
    pub fn getCellDateType(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData data = this.getCellData(row, col)
        // java if (!KmoCellValueStore.isNumericId(data.id))
        // java return 0
        // java Double value = this.getCellValueStore().doubleValue(data.id)
        // java if (!DateUtil.isValidExcelDate(value))
        // java return 0
        // java KmoXF xf = this.getCellXF(row, col)
        // java NumFmtSection section = new NumFmtSection()
        // java String numFmtStr = xf.getNumFormat()
        // java if (numFmtStr == null)
        // java numFmtStr = "General"
        // java section.reset(numFmtStr)
        // java String realFmt = section.getFormatSection(value)
        // java return DateUtil.isDateTimeFormat(xf.getFormatIndex(), realFmt)
        */
    }

    // java public String getCellFormula(int row, int col) {
    pub fn getCellFormula(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java Ptg[] ptgs = this.evaluationPresent.getCellFmlTokens(this, row, col, true)
        // java return FormulaRenderer.toFormulaString(new EvaluationWorkbook(this.parent), ptgs, RendererOption.createByLanguage(KmoBooks.getLanguageType()))
        */
    }

    // java public String getBackboardString(int row, int col) {
    pub fn getBackboardString(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.getFormattedCellValue(row, col)
        */
    }

    // java public String getEditString(int row, int col) {
    pub fn getEditString(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.getEditString(row, col, true)
        */
    }

    // java public String getEditString(int row, int col, boolean useModerneDigit) {
    pub fn getEditString__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (!this.isCellExist(row, col))
        // java return ""
        // java KmoCellData cd = SharedObjects.kmoCellDataPool.borrowObject()
        // java this.getCellData(row, col, cd)
        // java KmoXF xf = this.getCellXF(row, col)
        // java int valueId = cd.id
        // java if (KmoCellValueStore.identifyTypeByID(valueId) == 7)
        // java String var29 = "=" + this.getCellFormula(row, col)
        // java return var29
        // java catch (RuntimeException var22)
        // java KmoFormulaData fmlbuf = new KmoFormulaData()
        // java this.getCellValueStore().formulaValue(valueId, fmlbuf)
        // java valueId = fmlbuf.getCacheValueId()
        // java switch (KmoCellValueStore.identifyTypeByID(valueId))
        // java case 0:
        // java String var28 = ""
        // java return var28
        // java case 1:
        // java double value = this.getCellValueStore().doubleValue(valueId)
        // java if (!Double.isInfinite(value) && !Double.isNaN(value))
        // java if (xf == null)
        // java String var33 = String.valueOf(value)
        // java return var33
        // java String numRet = null
        // java short numfmt = xf.getFormatIndex()
        // java String numfmtstr = xf.getNumFormat()
        // java if (numfmtstr == null)
        // java numfmtstr = "General"
        // java NumFmtSection fmtSection = new NumFmtSection()
        // java fmtSection.reset(numfmtstr)
        // java numfmtstr = fmtSection.getFormatSection(value)
        // java NumfmtCategory numfmtCat = NumfmtCategory.getNumfmtCat(numfmt, numfmtstr)
        // java boolean isSimpleNum = true
        // java Double.parseDouble(numfmtstr)
        // java catch (NumberFormatException var21)
        // java isSimpleNum = false
        // java if (numfmtCat == NumfmtCategory.FRACTION || numfmtCat == NumfmtCategory.DATE || isSimpleNum)
        // java numRet = this.getCustomResult(numfmtstr, value)
        // java LangRes lang = LangRes.createByLangType(KmoBooks.getLanguageType())
        // java if (DateUtil.isValidExcelDate(value) && (numfmtCat == NumfmtCategory.DATE || numfmtCat == NumfmtCategory.TIME || numfmtCat == NumfmtCategory.DATE_TIME))
        // java TimeCalc.TimeStruct tm = DateUtil.getGregDate(value, this.getBook().isUsing1904())
        // java boolean dateValHasIntegerPart = Math.floor(value) != (double)0.0F
        // java boolean hasDecimal = Math.floor(value) != value
        // java if (numfmtCat != NumfmtCategory.DATE_TIME && (!dateValHasIntegerPart || !hasDecimal) && (!dateValHasIntegerPart || numfmtCat != NumfmtCategory.TIME) && (numfmtCat != NumfmtCategory.DATE || !hasDecimal))
        // java if (numfmtCat != NumfmtCategory.DATE && !dateValHasIntegerPart)
        // java numfmtstr = String.format("%d:%02d:%02d", tm.mHour, tm.mMinute, tm.mSecond)
        // java if (numRet != null)
        // java String var49 = numRet
        // java return var49
        // java char sep = lang.DATE.PREFER_SEPERATOR_CHAR
        // java switch (lang.DATE.SHORT_DATEFMT_TYPE)
        // java case DMY:
        // java numfmtstr = String.format("%d%c%d%c%d", tm.mMonthDay, sep, tm.mMonth, sep, tm.mYear)
        // java break
        // java case MDY:
        // java numfmtstr = String.format("%d%c%d%c%d", tm.mMonth, sep, tm.mMonthDay, sep, tm.mYear)
        // java break
        // java case YMD:
        // java numfmtstr = String.format("%d%c%d%c%d", tm.mYear, sep, tm.mMonth, sep, tm.mMonthDay)
        // java break
        // java default:
        // java throw new IllegalStateException("Unknown value")
        // java char sep = lang.DATE.PREFER_SEPERATOR_CHAR
        // java switch (lang.DATE.SHORT_DATEFMT_TYPE)
        // java case DMY:
        // java numfmtstr = String.format("%d%c%d%c%d %d:%02d:%02d", tm.mMonthDay, sep, tm.mMonth, sep, tm.mYear, tm.mHour, tm.mMinute, tm.mSecond)
        // java break
        // java case MDY:
        // java numfmtstr = String.format("%d%c%d%c%d %d:%02d:%02d", tm.mMonth, sep, tm.mMonthDay, sep, tm.mYear, tm.mHour, tm.mMinute, tm.mSecond)
        // java break
        // java case YMD:
        // java numfmtstr = String.format("%d%c%d%c%d %d:%02d:%02d", tm.mYear, sep, tm.mMonth, sep, tm.mMonthDay, tm.mHour, tm.mMinute, tm.mSecond)
        // java break
        // java default:
        // java throw new IllegalStateException("Unknown value")
        // java if (numRet != null)
        // java String var40 = numRet
        // java return var40
        // java if (numfmtCat == NumfmtCategory.PERCENTAGE)
        // java BigDecimal d = BigDecimal.valueOf(value).multiply(BigDecimal.valueOf((double)100.0F))
        // java double pv = d.doubleValue()
        // java if (pv == Math.floor(pv))
        // java numfmtstr = String.format("%.0f%%", pv).replace('.', lang.SEPERATORS.DECIMAL_SEPARATOR).replace(',', lang.SEPERATORS.DECIMAL_SEPARATOR)
        // java numfmtstr = Double.toString(pv).replace('.', lang.SEPERATORS.DECIMAL_SEPARATOR).replace(',', lang.SEPERATORS.DECIMAL_SEPARATOR) + "%"
        // java else if (value == (double)0.0F || -1.0E15 < value && value < -1.0E-15 || 1.0E-15 < value && value < 1.0E15)
        // java if (value == Math.floor(value))
        // java numfmtstr = String.format("%.0f", value).replace('.', lang.SEPERATORS.DECIMAL_SEPARATOR).replace(',', lang.SEPERATORS.DECIMAL_SEPARATOR)
        // java String decimal = String.format("%.30f", value).replace('.', lang.SEPERATORS.DECIMAL_SEPARATOR).replace(',', lang.SEPERATORS.DECIMAL_SEPARATOR)
        // java int trimTrailingZeros = 0
        // java for(; decimal.charAt(decimal.length() - trimTrailingZeros - 1) == '0' || decimal.charAt(decimal.length() - trimTrailingZeros - 1) == 1632; ++trimTrailingZeros)
        // java decimal = decimal.substring(0, decimal.length() - trimTrailingZeros)
        // java char c = decimal.charAt(decimal.length() - 1)
        // java if (c == lang.SEPERATORS.DECIMAL_SEPARATOR || c == 1643)
        // java decimal = decimal.substring(0, decimal.length() - 1)
        // java numfmtstr = decimal
        // java numfmtstr = Double.toString(value)
        // java if (useModerneDigit)
        // java String var43 = DigitsRes.translateDigits(numfmtstr, "٠١٢٣٤٥٦٧٨٩")
        // java return var43
        // java String var42 = numfmtstr
        // java return var42
        // java String numfmtCat = ""
        // java return numfmtCat
        // java case 2:
        // java String s = this.getCellValueStore().stringValue(valueId)
        // java if (xf != null && xf.get123Prefix())
        // java String var31 = "'" + s
        // java return var31
        // java String var30 = s
        // java return var30
        // java case 3:
        // java case 4:
        // java default:
        // java throw new IllegalStateException("illegal cell type")
        // java case 5:
        // java String var25 = this.getCellValueStore().booleanValue(valueId) ? "TRUE" : "FALSE"
        // java return var25
        // java case 6:
        // java String errorText = ErrorEval.getText(this.getCellValueStore().errorValue(valueId))
        // java return errorText
        // java case 7:
        // java throw new RuntimeException("公式的缓存值类型，不可能是公式")
        // java cd = SharedObjects.kmoCellDataPool.returnObject(cd)
        */
    }

    // java public void setCellRawValue(int row, int col, StringBuilder stringBuilder) {
    pub fn setCellRawValue(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoMergeDelegate mergedDelegate = this.getGrid().mergedDele()
        // java if (KmoMatrix.isColValid(col, this.getMaxColumns()) && KmoMatrix.isRowValid(row, this.getMaxRows()) && mergedDelegate.isMergedRangeLT(row, col))
        // java KmoRect dst = new KmoRect(row, col, row, col)
        // java this.sheetOp().clearContents(dst)
        // java this.setCellRawValue(row, col, stringBuilder.toString())
        */
    }

    // java public void setCellRawValue(int row, int col, String raw) {
    pub fn setCellRawValue__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.isSlashHeader(row, col))
        // java this.setValueForSlashHeader(row, col, raw)
        // java boolean hasArrayFml = this.getExpFmlManager().isPartOfArrFml(row, col)
        // java this.setCellRawValue(row, col, raw, hasArrayFml, new KmoRect(row, col, row, col), true, false)
        */
    }

    // java public void setCellRawValue(int row, int col, String raw, boolean isAnalyzeHyper) {
    pub fn setCellRawValue__3(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean hasArrayFml = this.getExpFmlManager().isPartOfArrFml(row, col)
        // java this.setCellRawValue(row, col, raw, hasArrayFml, new KmoRect(row, col, row, col), isAnalyzeHyper, false)
        */
    }

    // java public void setCellRawValue(int row, int col, String raw, boolean isAnalyzeHyper, boolean isCoverOldNumfmt) {
    pub fn setCellRawValue__4(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean hasArrayFml = this.getExpFmlManager().isPartOfArrFml(row, col)
        // java this.setCellRawValue(row, col, raw, hasArrayFml, new KmoRect(row, col, row, col), isAnalyzeHyper, isCoverOldNumfmt)
        */
    }

    // java public void setCellRawValue(final int activeRow, final int activeCol, String raw, boolean arrayFml, KmoRect selection, boolean isAnalyzeHyper, boolean isCoverOldNumfmt) {
    pub fn setCellRawValue__5(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java ExpFmlManager expfmlMgr = this.getExpFmlManager()
        // java if (arrayFml && selection.isSingleCell() && expfmlMgr.isPartOfArrFml(activeRow, activeCol))
        // java ExpFmlManager.ExpFml expfml = expfmlMgr.query(activeRow, activeCol, false)
        // java selection = new KmoRect(expfml.firstRow, expfml.firstCol, expfml.lastRow, expfml.lastCol)
        // java if (!expfmlMgr.isContainsAllIntersectArrFml(selection))
        // java throw new ArrayFormulaModifyFailedException()
        // java final CalculationChain cc = this.parent.getCalculationChain()
        // java final int sheetIndex = this.getSheetIndex()
        // java List<ExpFmlManager.ExpFml> expfmls = new ArrayList()
        // java expfmlMgr.query(selection, true, expfmls)
        // java for(ExpFmlManager.ExpFml shrfml : expfmls)
        // java expfmlMgr.unlinkSharedFormula(shrfml)
        // java if (cc != null)
        // java cc.unregisterFormula(sheetIndex, selection.tl.row, selection.br.row, selection.tl.col, selection.br.col)
        // java expfmls.clear()
        // java expfmlMgr.query(selection, false, expfmls)
        // java for(ExpFmlManager.ExpFml arrfml : expfmls)
        // java expfmlMgr.unregister(arrfml, false)
        // java RecogniseResult rs = null
        // java if (raw == null || raw.length() == 0)
        // java rs = new RecogniseResult("")
        // java Ptg[] ptgs = null
        // java if (rs == null && raw.length() > 1 && LangHelper.isOprEqual(raw.charAt(0)))
        // java ptgs = FormulaParser.parse(raw.substring(1), new EvaluationWorkbook(this.parent), 0, sheetIndex, this.parent.getSpreadsheetVersion(), ParserOption.createByLanguage(KmoBooks.getLanguageType()))
        // java catch (RuntimeException var27)
        // java KmoXF oldXF = this.getCellXF(activeRow, activeCol)
        // java short oldNumfmt = oldXF.getFormatIndex()
        // java String oldNumfmtStr = oldXF.getNumFormat()
        // java if (oldNumfmtStr == null)
        // java oldNumfmtStr = "General"
        // java NumfmtCategory oldNumfmtCat = NumfmtCategory.getNumfmtCat(oldNumfmt, oldNumfmtStr)
        // java constRecognisor.setForceText(false)
        // java constRecognisor.setUsingDate1904(this.getBook().isUsing1904())
        // java constRecognisor.setPreferedNumfmtCategory(oldNumfmtCat)
        // java constRecognisor.setKeepTailZero(true)
        // java if (rs == null && ptgs == null)
        // java rs = constRecognisor.recognise(raw)
        // java if (rs == null && ptgs == null && raw.length() > 1 && (LangHelper.isOprPlus(raw.charAt(0)) || LangHelper.isOprMinus(raw.charAt(0))))
        // java ptgs = FormulaParser.parse(raw, new EvaluationWorkbook(this.parent), 0, sheetIndex, this.parent.getSpreadsheetVersion(), ParserOption.createByLanguage(KmoBooks.getLanguageType()))
        // java catch (RuntimeException var26)
        // java if (rs == null && ptgs == null)
        // java constRecognisor.setForceText(true)
        // java rs = constRecognisor.recognise(raw)
        // java if (ptgs != null)
        // java rs = FormulaHelper.getFormulaSuggestedNumfmt(ptgs, KmoBooks.getLanguageType())
        // java if (arrayFml)
        // java expfmlMgr.registerArrayFml(selection.tl.row, selection.br.row, selection.tl.col, selection.br.col, Formula.create(ptgs, this.getVersion()))
        // java Ptg[] cellPtgs = new Ptg[]{ExpPtg.createExpPtg(this.getVersion(), selection.tl.row, selection.tl.col)}
        // java Formula cellFml = Formula.create(cellPtgs, this.getVersion())
        // java for(int r = selection.tl.row; r <= selection.br.row; ++r)
        // java for(int c = selection.tl.col; c <= selection.br.col; ++c)
        // java this._setCellCplFml(r, c, raw, 2, cellFml, rs)
        // java else if (selection.height() * selection.width() > 10 && FormulaHelper.fmlCanBeSharedfml(ptgs))
        // java ptgs = FormulaHelper.replaceWithNPtgs(ptgs, activeRow, activeCol, this.parent.getSpreadsheetVersion())
        // java expfmlMgr.registerSharedFml(selection.tl.row, selection.br.row, selection.tl.col, selection.br.col, Formula.create(ptgs, this.getVersion()))
        // java Ptg[] cellPtgs = new Ptg[]{ExpPtg.createExpPtg(this.getVersion(), selection.tl.row, selection.tl.col)}
        // java Formula cellFml = Formula.create(cellPtgs, this.getVersion())
        // java for(int r = selection.tl.row; r <= selection.br.row; ++r)
        // java for(int c = selection.tl.col; c <= selection.br.col; ++c)
        // java this._setCellCplFml(r, c, raw, 3, cellFml, rs)
        // java else if (!selection.isSingleCell())
        // java ptgs = FormulaHelper.replaceWithNPtgs(ptgs, activeRow, activeCol, this.parent.getSpreadsheetVersion())
        // java for(int r = selection.tl.row; r <= selection.br.row; ++r)
        // java for(int c = selection.tl.col; c <= selection.br.col; ++c)
        // java Formula cellFml = Formula.create(FormulaHelper.replaceNPtgs(ptgs, r, c, this.getVersion()), this.getVersion())
        // java this._setCellCplFml(r, c, raw, 1, cellFml, rs)
        // java Formula cellFml = Formula.create(ptgs, this.getVersion())
        // java this._setCellCplFml(activeRow, activeCol, raw, 1, cellFml, rs)
        // java for(int r = selection.tl.row; r <= selection.br.row; ++r)
        // java for(int c = selection.tl.col; c <= selection.br.col; ++c)
        // java this._setCellRgnResult(r, c, raw, rs, isCoverOldNumfmt)
        // java if (this.tableManager != null)
        // java this.tableManager.setColName(this.getFormattedCellValue(activeRow, activeCol), activeRow, activeCol)
        // java if (!rs.hasSqleadingFlag() && rs.getValue() instanceof String && isAnalyzeHyper)
        // java this.analyzeHyperlinkWithRawCellValue(raw, activeRow, activeCol)
        // java else if (ptgs != null && ptgs.length > 0 && ptgs[ptgs.length - 1] instanceof AbstractFunctionPtg)
        // java AbstractFunctionPtg fptg = (AbstractFunctionPtg)ptgs[ptgs.length - 1]
        // java if ("HYPERLINK".equalsIgnoreCase(fptg.getName()) && fptg.getFunctionIndex() == 359)
        // java KmoRect hyperRange = new KmoRect(activeRow, activeCol, activeRow, activeCol)
        // java EtNumFmtOp.INSTANCE.setRangeBuiltinStyle(this, hyperRange, 8)
        // java final KmoRect tSelection = selection
        // java final int fSheetIndex = sheetIndex
        // java final int fTlRow = selection.tl.row
        // java final int fBrRow = selection.br.row
        // java final int fTlCol = selection.tl.col
        // java final int fBrCol = selection.br.col
        // java final int fActiveRow = activeRow
        // java final int fActiveCol = activeCol
        // java this.parent.areaWatcher().startMergeEvents()
        // java if (cc != null)
        // java boolean isSynCalc = this.parent.isSynCalc()
        // java cc.interrupt()
        // java cc.registerFormula(sheetIndex, selection.tl.row, selection.br.row, selection.tl.col, selection.br.col)
        // java if (!isSynCalc)
        // java cc.calculate(new CalcChain.ICalculationProgressCallback()
        // java if (false) public void onStart()
        // java cc.markRecalculation(fSheetIndex, fTlRow, fBrRow, fTlCol, fBrCol)
        // java if (false) public boolean onProgress(int sheet, int row, int col)
        // java return sheet == fSheetIndex && row == fActiveRow && col == fActiveCol
        // java if (false) public void onEnd()
        // java KmoSheet.this.parent.setDirty(true)
        // java KmoSheet.this.parent.getActiveSheet().changeListenerGroup().sheetContentsDidFinishChanging()
        // java KmoSheet.this.parent.areaWatcher().postContentChangedEvent(KmoSheet.this, new KmoRect(tSelection))
        // java cc.markRecalculation(sheetIndex, selection.tl.row, selection.br.row, selection.tl.col, selection.br.col)
        // java cc.synCalculate()
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java this.parent.areaWatcher().postContentChangedEvent(this, new KmoRect(selection))
        // java this.parent.areaWatcher().finishMergeEvents()
        */
    }

    // java private void analyzeHyperlinkWithRawCellValue(String rawValue, int row, int col) {
    pub fn analyzeHyperlinkWithRawCellValue(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (!KmoHyperLinkManager.isContainIllegalBlank(rawValue))
        // java if (this.hyperlinkMgr.isHyperlinkString(rawValue))
        // java KmoRect hyperRange = new KmoRect(row, col, row, col)
        // java this.hyperlinkMgr.clearRangeHyperlinks(hyperRange, true)
        // java KmoHyperlink hyperlink = new KmoHyperlink()
        // java hyperlink.setHyperlinkType(1)
        // java if (this.hyperlinkMgr.isFileLinkString(rawValue))
        // java hyperlink.setHyperlinkType(0)
        // java hyperlink.setRange(hyperRange)
        // java hyperlink.setAddress(rawValue)
        // java hyperlink.setLabel(rawValue)
        // java this.hyperlinkMgr.addHLinkWithTransAddress(hyperlink)
        // java EtNumFmtOp.INSTANCE.setRangeBuiltinStyle(this, hyperRange, 8)
        */
    }

    // java private void _setCellCplFml(int row, int col, String raw, int formulaType, Formula cellFml, RecogniseResult rs) {
    pub fn _setCellCplFml(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellValueStore cvs = this.getCellValueStore()
        // java KmoCellData cellData = SharedObjects.kmoCellDataPool.borrowObject()
        // java KmoFormulaData formulaData = SharedObjects.kmoFormulaDataPool.borrowObject()
        // java Ptg[] ptgs = cellFml.getTokens()
        // java this.getCellData(row, col, cellData)
        // java int valueid = cellData.id
        // java if (KmoCellValueStore.identifyTypeByID(valueid) == 7)
        // java KmoFormulaData fml = SharedObjects.kmoFormulaDataPool.borrowObject()
        // java this.parent.getCellValueStore().formulaValue(valueid, fml)
        // java valueid = fml.getCacheValueId()
        // java SharedObjects.kmoFormulaDataPool.returnObject(fml)
        // java if (FormulaHelper.hasUnimplementedFunctions(ptgs, new EvaluationWorkbook(this.getBook())))
        // java valueid = this.getCellValueStore().newErrorValue(29)
        // java int tokensId = cvs.newFormulaTokens(cellFml.getByteEncoding())
        // java formulaData.setTokens(tokensId, cellFml.getEncodedTokenSize())
        // java formulaData.setCacheValueId(valueid)
        // java cellData.formulaType = formulaType
        // java cellData.id = cvs.newFormulaValue(formulaData)
        // java if (0 == cellData.ixf || 4095 == cellData.ixf)
        // java if (this.isRowFormatted(row))
        // java cellData.ixf = this.getRowXFIndex(row)
        // java else if (this.isColFormatted(col))
        // java cellData.ixf = this.getColXFIndex(col)
        // java cellData.ixf = 15
        // java KmoXF oldXF = this.getCellXF(row, col)
        // java short oldNumfmt = oldXF.getFormatIndex()
        // java String oldNumfmtStr = oldXF.getNumFormat()
        // java if (oldNumfmtStr == null)
        // java oldNumfmtStr = "General"
        // java NumfmtCategory oldNumfmtCat = NumfmtCategory.getNumfmtCat(oldNumfmt, oldNumfmtStr)
        // java this.handleLocalNumber(ptgs, rs)
        // java int newIXF = this._newXF(cellData.ixf, oldXF, oldNumfmt, oldNumfmtCat, raw, rs, false)
        // java if (newIXF != 4095)
        // java cellData.ixf = newIXF
        // java cellData.isPicInCell = this.getCellPic(ptgs, formulaType) != null
        // java this.setCellData(row, col, cellData)
        // java cellData = SharedObjects.kmoCellDataPool.returnObject(cellData)
        // java formulaData = SharedObjects.kmoFormulaDataPool.returnObject(formulaData)
        */
    }

    // java private void handleLocalNumber(Ptg[] ptgs, RecogniseResult rs) {
    pub fn handleLocalNumber(&self, ptgs: &[Ptg], rs: &mut crate::cn::wps::et::ss::constant::RecogniseResult::RecogniseResult) {
        use crate::cn::wps::et::ss::multlang::res::DigitsResResult::DigitsResResult;
        use crate::cn::wps::et::ss::formula::ptg::Ptg::col_index;
        // java for(int i = 0; i < ptgs.length; ++i) {
        let mut i = 0;
        while i < ptgs.len() {
            // java Ptg ptg = ptgs[i];
            let ptg = &ptgs[i];
            // java if (ptg instanceof AreaPtgBase) {
            let area = match &ptg.kind {
                PtgKind::Area { first_row, last_row, first_col_raw, last_col_raw, .. } |
                PtgKind::Area3D { first_row, last_row, first_col_raw, last_col_raw, .. } |
                PtgKind::Area3DExt { first_row, last_row, first_col_raw, last_col_raw, .. } => Some((*first_row, *last_row, col_index(*first_col_raw), col_index(*last_col_raw))),
                _ => None,
            };
            if let Some((firstRow, lastRow, firstColumn, lastColumn)) = area {
                // java AreaPtgBase areaPtgBase = (AreaPtgBase)ptg; int firstRow = areaPtgBase.getFirstRow(); int lastRow = areaPtgBase.getLastRow(); int firstColumn = areaPtgBase.getFirstColumn(); int lastColumn = areaPtgBase.getLastColumn();
                // java CellIterator it = this.getCellIterator(firstRow, lastRow, firstColumn, lastColumn);
                let mut it = self.getCellIterator(firstRow, lastRow, firstColumn, lastColumn);
                // java while(it.hasNext()) {
                while it.hasNext() {
                    // java it.next(); int row = it.row(); int col = it.col();
                    it.next(); let row = it.row(); let col = it.col();
                    // java KmoXF oldXF = this.getCellXF(row, col); String numfmt = oldXF.getNumFormat();
                    let oldXF = self.getCellXF__2(row, col).expect("NullPointerException"); let numfmt = oldXF.getNumFormat();
                    // java if (DigitsResResult.isLocalNumberFmt(numfmt)) { rs.setSuggestedNumfmt(numfmt); break; }
                    if DigitsResResult::isLocalNumberFmt(numfmt.as_deref()) { rs.setSuggestedNumfmt(numfmt); break; }
                }
            } else {
                // java } else if (ptg instanceof RefPtgBase) {
                let reference = match &ptg.kind {
                    PtgKind::Ref { row, col_raw, .. } | PtgKind::Ref3D { row, col_raw, .. } | PtgKind::Ref3DExt { row, col_raw, .. } => Some((*row, col_index(*col_raw))),
                    _ => None,
                };
                if let Some((row, col)) = reference {
                    // java RefPtgBase refPtgBase = (RefPtgBase)ptg; int row = refPtgBase.getRow(); int col = refPtgBase.getColumn();
                    // java KmoXF oldXF = this.getCellXF(row, col); String numfmt = oldXF.getNumFormat();
                    let oldXF = self.getCellXF__2(row, col).expect("NullPointerException"); let numfmt = oldXF.getNumFormat();
                    // java if (DigitsResResult.isLocalNumberFmt(numfmt)) { rs.setSuggestedNumfmt(numfmt); break; }
                    if DigitsResResult::isLocalNumberFmt(numfmt.as_deref()) { rs.setSuggestedNumfmt(numfmt); break; }
                }
            }
            i += 1;
        }
    }

    // java private void _setCellRgnResult(int row, int col, String raw, RecogniseResult rs, boolean isCoverOldNumfmt) {
    pub fn _setCellRgnResult(&self, row: i32, col: i32, raw: Option<&str>, rs: &crate::cn::wps::et::ss::constant::RecogniseResult::RecogniseResult, isCoverOldNumfmt: bool) {
        use crate::cn::wps::et::ss::{constant::NumfmtCategory::NumfmtCategory, statemachine::RecogniseResultRl::Value};
        // java KmoCellData cellData = SharedObjects.kmoCellDataPool.borrowObject()
        let mut cellData = KmoCellData::default();
        // java this.getCellData(row, col, cellData)
        self.getCellData__2(row, col, &mut cellData);
        // java cellData.isPicInCell = false
        cellData.isPicInCell = false;
        // java cellData.formulaType = 0
        cellData.formulaType = 0;
        // java if (0 == cellData.ixf || 4095 == cellData.ixf)
        if cellData.ixf == 0 || cellData.ixf == 4095 {
        // java if (this.isRowFormatted(row))
            if self.isRowFormatted(row) {
        // java cellData.ixf = this.getRowXFIndex(row)
                cellData.ixf = self.getRowXFIndex(row);
            }
        // java else if (this.isColFormatted(col))
            else if self.isColFormatted(col) {
        // java cellData.ixf = this.getColXFIndex(col)
                cellData.ixf = self.getColXFIndex(col);
            } else {
        // java cellData.ixf = 15
                cellData.ixf = 15;
            }
        }
        // java KmoXF oldXF = this.getCellXF(row, col)
        let oldXF = self.getCellXF__2(row, col).expect("NullPointerException KmoXF");
        // java short oldNumfmt = oldXF.getFormatIndex()
        let oldNumfmt = oldXF.getFormatIndex();
        // java String oldNumfmtStr = oldXF.getNumFormat()
        // java if (oldNumfmtStr == null)
        // java oldNumfmtStr = "General"
        let oldNumfmtStr = oldXF.getNumFormat().unwrap_or_else(|| "General".into());
        // java NumfmtCategory oldNumfmtCat = NumfmtCategory.getNumfmtCat(oldNumfmt, oldNumfmtStr)
        let oldNumfmtCat = NumfmtCategory::getNumfmtCat(oldNumfmt, &oldNumfmtStr);
        // java if (NumfmtCategory.GENERAL == oldNumfmtCat || NumfmtCategory.GENERAL != oldNumfmtCat && !this.isBlankCellValue(row, col))
        if oldNumfmtCat == NumfmtCategory::GENERAL || oldNumfmtCat != NumfmtCategory::GENERAL && !self.isBlankCellValue(row, col) {
        // java int newIXF = this._newXF(cellData.ixf, oldXF, oldNumfmt, oldNumfmtCat, raw, rs, isCoverOldNumfmt)
            let newIXF = self._newXF(cellData.ixf, &oldXF, oldNumfmt, oldNumfmtCat, raw, rs, isCoverOldNumfmt);
        // java if (newIXF != 4095)
        // java cellData.ixf = newIXF
            if newIXF != 4095 { cellData.ixf = newIXF; }
        }
        // java Object value = rs.getValue()
        let value = rs.getValue();
        // java if (value instanceof String)
        if let Some(Value::Text(mut v)) = value {
        // java String v = (String)value
        // java if (rs.hasSqleadingFlag())
        // java v = v.substring(1)
            if rs.hasSqleadingFlag() { v = v[1..].to_vec(); }
        // java cellData.id = v.length() == 0 ? 0 : this.getCellValueStore().newStringValue(v)
            cellData.id = if v.is_empty() { 0 } else {
                let v = String::from_utf16(&v).expect("unpaired UTF-16 outside current string-store closure");
                self.getCellValueStore().borrow_mut().newStringValue(Some(&v))
            };
        }
        // java else if (value instanceof Boolean)
        else if let Some(Value::Bool(value)) = value {
        // java cellData.id = this.getCellValueStore().newBooleanValue((Boolean)value)
            cellData.id = self.getCellValueStore().borrow().newBooleanValue(value);
        } else {
        // java if (!(value instanceof Double))
        // java throw new IllegalStateException("wrong type of const")
            let Some(Value::Number(value)) = value else { panic!("IllegalStateException wrong type of const"); };
        // java if (rs.hasErrorFlag())
            if rs.hasErrorFlag() {
        // java cellData.id = this.getCellValueStore().newErrorValue(((Double)value).byteValue())
                cellData.id = self.getCellValueStore().borrow().newErrorValue((value as i32 as i8) as i32);
            } else {
        // java double d = (Double)value
                let mut d = value;
        // java if (isCoverOldNumfmt && oldNumfmtCat == NumfmtCategory.PERCENTAGE)
                if isCoverOldNumfmt && oldNumfmtCat == NumfmtCategory::PERCENTAGE {
        // java d = Double.parseDouble(raw)
        // java catch (NumberFormatException var21)
                    if let Some(parsed) = crate::java_runtime::parseDouble(raw.expect("NullPointerException raw")) { d = parsed; }
                }
        // java if (!this.isFullPrecision())
                if !self.isFullPrecision() {
        // java NumFmtResult formatResult = (NumFmtResult)this.shared.numFormatResult.get()
        // java formatResult.clear()
                    self.shared.numFormatResult().borrow_mut().clear();
        // java KmoXF cellXF = this.parent.getXFManager().getXF(cellData.ixf)
                    let cellXF = self.book().getXFManager().borrow().getXF(cellData.ixf).expect("NullPointerException KmoXF");
        // java NumfmtCategory newNumfmtCat = NumfmtCategory.getNumfmtCat(cellXF.getFormatIndex(), cellXF.getNumFormat())
                    let code = cellXF.getNumFormat().expect("NullPointerException numfmt");
                    let newNumfmtCat = NumfmtCategory::getNumfmtCat(cellXF.getFormatIndex(), &code);
        // java if (newNumfmtCat == NumfmtCategory.NUMBER || newNumfmtCat == NumfmtCategory.ACCOUNTING || newNumfmtCat == NumfmtCategory.CURRENCY || newNumfmtCat == NumfmtCategory.PERCENTAGE || newNumfmtCat == NumfmtCategory.SCIENTIC)
                    if matches!(newNumfmtCat, NumfmtCategory::NUMBER | NumfmtCategory::ACCOUNTING | NumfmtCategory::CURRENCY | NumfmtCategory::PERCENTAGE | NumfmtCategory::SCIENTIC) {
        // java INumFormatter numFormatter = KmoApp.sharedApp().numFormatter()
                        super::KmoApp::KmoApp::sharedApp().numFormatter(|numFormatter| {
        // java if (numFormatter != null)
                            if let Some(numFormatter) = numFormatter {
        // java d = numFormatter.formatValue(d, this.parent.isUsing1904(), cellXF.getNumFormat())
                                d = numFormatter.formatValue(d, self.book().isUsing1904(), &code);
                            }
                        });
                    }
                }
        // java cellData.id = this.getCellValueStore().newDoubleValue(d)
                cellData.id = self.getCellValueStore().borrow_mut().newDoubleValue(d);
            }
        }
        // java this.setCellData(row, col, cellData)
        self.setCellData(row, col, &cellData);
        // java cellData = SharedObjects.kmoCellDataPool.returnObject(cellData)
        // rust: stack buffer has the original borrow/return scope; no references escape.
    }

    // java private int _newXF(int oldIXF, KmoXF oldXF, short oldNumfmt, NumfmtCategory oldNumfmtCat, String raw, RecogniseResult rs, boolean isCoverOldNumfmt) {
    pub fn _newXF(&self, oldIXF: i32, oldXF: &super::xf::KmoXF::KmoXF, oldNumfmt: i16, oldNumfmtCat: crate::cn::wps::et::ss::constant::NumfmtCategory::NumfmtCategory, raw: Option<&str>, rs: &crate::cn::wps::et::ss::constant::RecogniseResult::RecogniseResult, isCoverOldNumfmt: bool) -> i32 {
        // java int newIXF = 4095
        let mut newIXF = 4095;
        // java KmoXF newXF = null
        let mut newXF = None;
        let book = self.book();
        let manager = book.getXFManager().borrow();
        // java if (!oldXF.getWrapText() && raw != null && (raw.indexOf(13) >= 0 || raw.indexOf(10) >= 0))
        if !oldXF.getWrapText() && raw.is_some_and(|v| v.contains(['\r', '\n'])) {
        // java newIXF = this.getBook().getXFManager().cloneCellXF(oldIXF)
            newIXF = manager.cloneCellXF(oldIXF);
        // java newXF = this.getBook().getXFManager().getXF(newIXF)
            newXF = manager.getXF(newIXF);
        // java newXF.setWrapText(true)
            newXF.as_ref().expect("NullPointerException KmoXF").setWrapText(true);
        }
        // java if (rs.hasSqleadingFlag() != oldXF.get123Prefix())
        if rs.hasSqleadingFlag() != oldXF.get123Prefix() {
        // java if (newXF == null)
            if newXF.is_none() {
        // java newIXF = this.getBook().getXFManager().cloneCellXF(oldIXF)
                newIXF = manager.cloneCellXF(oldIXF);
        // java newXF = this.getBook().getXFManager().getXF(newIXF)
                newXF = manager.getXF(newIXF);
            }
        // java newXF.set123Prefix(rs.hasSqleadingFlag())
            newXF.as_ref().unwrap().set123Prefix(rs.hasSqleadingFlag());
        }
        // java boolean needReplaceNumfmt = _needReplaceNumfmt(oldNumfmt, oldNumfmtCat, rs, isCoverOldNumfmt)
        let needReplaceNumfmt = Self::_needReplaceNumfmt(oldNumfmt, oldNumfmtCat, rs, isCoverOldNumfmt);
        // java if (needReplaceNumfmt)
        if needReplaceNumfmt {
        // java if (newXF == null)
            if newXF.is_none() {
        // java newIXF = this.getBook().getXFManager().cloneCellXF(oldIXF)
                newIXF = manager.cloneCellXF(oldIXF);
        // java newXF = this.getBook().getXFManager().getXF(newIXF)
                newXF = manager.getXF(newIXF);
            }
        // java short newNumfmt = (short)rs.getBuiltInNumfmtIdx()
            let mut newNumfmt = rs.getBuiltInNumfmtIdx() as i16;
        // java if (newNumfmt < 0)
            if newNumfmt < 0 {
        // java newNumfmt = (short)this.getBook().getXFManager().checkFormat(rs.getSuggestedNumfmt())
                newNumfmt = manager.checkFormat(rs.getSuggestedNumfmt().as_deref()) as i16;
            }
        // java newXF.setFormatIndex(newNumfmt)
            newXF.as_ref().unwrap().setFormatIndex(newNumfmt);
        // java newXF.setNumFormat((String)null)
            newXF.as_ref().unwrap().setNumFormat(None);
        // java newXF.setValidFormat(true)
            newXF.as_ref().unwrap().setValidFormat(true);
        }
        // java return newIXF
        newIXF
    }

    // java private static boolean _needReplaceNumfmt(short oldNumfmt, NumfmtCategory oldNumfmtCat, RecogniseResult rs, boolean isCoverOldNumfmt) {
    pub fn _needReplaceNumfmt(oldNumfmt: i16, oldNumfmtCat: crate::cn::wps::et::ss::constant::NumfmtCategory::NumfmtCategory, rs: &crate::cn::wps::et::ss::constant::RecogniseResult::RecogniseResult, isCoverOldNumfmt: bool) -> bool {
        use crate::cn::wps::et::ss::constant::NumfmtCategory::NumfmtCategory::*;
        // java if (oldNumfmt != rs.getBuiltInNumfmtIdx() && (rs.getBuiltInNumfmtIdx() >= 0 || rs.getSuggestedNumfmt() != null))
        if oldNumfmt as i32 != rs.getBuiltInNumfmtIdx() && (rs.getBuiltInNumfmtIdx() >= 0 || rs.getSuggestedNumfmt().is_some()) {
        // java if (rs.getBuiltInNumfmtIdx() != 0 && isCoverOldNumfmt)
        // java return true
            if rs.getBuiltInNumfmtIdx() != 0 && isCoverOldNumfmt { return true; }
        // java else if (oldNumfmt == 0)
        // java return true
            else if oldNumfmt == 0 { return true; }
        // java switch (oldNumfmtCat)
            match oldNumfmtCat {
        // java case NONE:
        // java return false
                NONE => false,
        // java case GENERAL:
        // java return true
                GENERAL => true,
        // java case NUMBER:
        // java return false
                NUMBER => false,
        // java case CURRENCY:
                CURRENCY => {
        // java if (!rs.hasTimeFlag() && !rs.hasDateFlag() && !rs.hasPercentageFlag() && !rs.hasFractionFlag() && !rs.hasSenticFlag())
        // java return false
                    if !rs.hasTimeFlag() && !rs.hasDateFlag() && !rs.hasPercentageFlag() && !rs.hasFractionFlag() && !rs.hasSenticFlag() { return false; }
        // java return true
                    true
                },
        // java case TIME:
                TIME => {
        // java if (!rs.hasCurrencyFlag() && !rs.hasPercentageFlag() && !rs.hasFractionFlag() && !rs.hasSenticFlag())
                    if !rs.hasCurrencyFlag() && !rs.hasPercentageFlag() && !rs.hasFractionFlag() && !rs.hasSenticFlag() {
        // java if (rs.hasDateFlag())
        // java return true
                        if rs.hasDateFlag() { return true; }
        // java return false
                        return false;
                    }
        // java return true
                    true
                },
        // java case DATE:
                DATE => {
        // java if (!rs.hasCurrencyFlag() && !rs.hasPercentageFlag() && !rs.hasFractionFlag() && !rs.hasSenticFlag())
                    if !rs.hasCurrencyFlag() && !rs.hasPercentageFlag() && !rs.hasFractionFlag() && !rs.hasSenticFlag() {
        // java if (rs.hasTimeFlag() && !rs.hasDateFlag())
        // java return true
                        if rs.hasTimeFlag() && !rs.hasDateFlag() { return true; }
        // java return false
                        return false;
                    }
        // java return true
                    true
                },
        // java case DATE_TIME:
                DATE_TIME => {
        // java if (!rs.hasCurrencyFlag() && !rs.hasPercentageFlag() && !rs.hasFractionFlag() && !rs.hasSenticFlag())
                    if !rs.hasCurrencyFlag() && !rs.hasPercentageFlag() && !rs.hasFractionFlag() && !rs.hasSenticFlag() {
        // java if (rs.hasTimeFlag() && !rs.hasDateFlag())
        // java return true
                        if rs.hasTimeFlag() && !rs.hasDateFlag() { return true; }
        // java return false
                        return false;
                    }
        // java return true
                    true
                },
        // java case PERCENTAGE:
                PERCENTAGE => {
        // java if (!rs.hasCurrencyFlag() && !rs.hasTimeFlag() && !rs.hasDateFlag() && !rs.hasSenticFlag())
        // java return false
                    if !rs.hasCurrencyFlag() && !rs.hasTimeFlag() && !rs.hasDateFlag() && !rs.hasSenticFlag() { return false; }
        // java return true
                    true
                },
        // java case FRACTION:
                FRACTION => {
        // java if (!rs.hasCurrencyFlag() && !rs.hasTimeFlag() && !rs.hasDateFlag() && !rs.hasPercentageFlag() && !rs.hasSenticFlag())
        // java return false
                    if !rs.hasCurrencyFlag() && !rs.hasTimeFlag() && !rs.hasDateFlag() && !rs.hasPercentageFlag() && !rs.hasSenticFlag() { return false; }
        // java return true
                    true
                },
        // java case SCIENTIC:
                SCIENTIC => {
        // java if (!rs.hasCurrencyFlag() && !rs.hasTimeFlag() && !rs.hasDateFlag() && !rs.hasPercentageFlag() && !rs.hasFractionFlag())
        // java return false
                    if !rs.hasCurrencyFlag() && !rs.hasTimeFlag() && !rs.hasDateFlag() && !rs.hasPercentageFlag() && !rs.hasFractionFlag() { return false; }
        // java return true
                    true
                },
        // java default:
        // java return false
                _ => false,
            }
        } else {
        // java return false
            false
        }
    }

    // java public String getFormattedCellValue(int row, int col) {
    pub fn getFormattedCellValue(&self,row:i32,col:i32)->String {
// java if(!this.isCellExist(row,col))return "";else{NumFmtResult result=this.getNumFormatResult(row,col,Integer.MAX_VALUE);return result.numfmtStr();}
if !self.isCellExist(row,col){String::new()}else{self.getNumFormatResult(row,col,i32::MAX).borrow().numfmtStr().into()}
    }

    // java public NumFmtResult getNumFormatResult(int row, int col, int maxWidth) {
    pub fn getNumFormatResult(&self,row:i32,col:i32,mut maxWidth:i32)->Rc<RefCell<kfc::cn::wps::kfc::numfmt::format::NumFmtResult::NumFmtResult>> {
let result=self.shared.numFormatResult();
// java NumFmtResult formatResult=this.shared.numFormatResult.get();formatResult.clear();if(row>=0&&col>=0){...}
{let mut formatResult=result.borrow_mut();formatResult.clear();if row>=0&&col>=0{
    // java KmoCellData cellData=SharedObjects.kmoCellDataPool.borrowObject();getCellData(row,col,cellData);
    // Rust stack buffers have the same pool borrow/return scope and do not escape.
    let cellData=self.getCellData(row,col);let mut valueid=cellData.id;let book=self.book();let store=book.getCellValueStore();
    // java if(identifyTypeByID(valueid)==7){formulaValue(valueid,fml);valueid=fml.getCacheValueId();return fml to pool;}
    if KmoCellValueStore::identifyTypeByID(valueid)==7{let mut fml=KmoFormulaData::KmoFormulaData();store.borrow().formulaValue(valueid,&mut fml);valueid=fml.getCacheValueId();}
    match KmoCellValueStore::identifyTypeByID(valueid){
        1=>{
            let value=store.borrow_mut().doubleValue(valueid);
            // java if(value==0&&maxWidth!=Integer.MAX_VALUE&&!isShowZeros())formatResult.set("",null);
            if value==0.0&&maxWidth!=i32::MAX&&!self.isShowZeros(){formatResult.set("",None);}else{
                let manager=book.getXFManager().borrow();let xf=manager.getXF(cellData.ixf);
                // java condition format precedes effective XF;fallback General.
                let code=self.cformatter.getNumFmtCode(&manager,row,col).or_else(||xf.as_ref().and_then(|x|x.getNumFormat())).unwrap_or_else(||"General".into());
                // java if(maxWidth<=0)maxWidth=xf!=null?calcCellCharWidth(row,col,xf):Integer.MAX_VALUE;
                if maxWidth<=0{maxWidth=xf.as_ref().map_or(i32::MAX,|x|self.calcCellCharWidth(row,col,x));}
                // java INumFormatter numFormatter=KmoApp.sharedApp().numFormatter();if(numFormatter!=null)numFormatter.format(value,formatCode,maxWidth,parent.isUsing1904(),formatResult);
                super::KmoApp::KmoApp::sharedApp().numFormatter(|formatter|{if let Some(formatter)=formatter{formatter.format_num(value,&code,maxWidth,book.isUsing1904(),&mut formatResult);}});
            }
        },
        2=>{
            let manager=book.getXFManager().borrow();let code=self.cformatter.getNumFmtCode(&manager,row,col).or_else(||manager.getXF(cellData.ixf).and_then(|x|x.getNumFormat())).unwrap_or_else(||"General".into());let src=store.borrow_mut().stringValue(valueid);
            // java if(!fmtCode.equals("@")&&!fmtCode.equalsIgnoreCase("General")&&src!=null)formatter.format(...);else formatResult.set(src!=null?src:"",null);
            if code!="@"&&!code.eq_ignore_ascii_case("General"){super::KmoApp::KmoApp::sharedApp().numFormatter(|formatter|{if let Some(formatter)=formatter{formatter.format_str(&src,&code,maxWidth,&mut formatResult);}});}else{formatResult.set(&src,None);}
        },
        // java case 5 TRUE/FALSE;case 6 ErrorConstants.getText;other types leave cleared result.
        5=>formatResult.set(if store.borrow().booleanValue(valueid){"TRUE"}else{"FALSE"},None),
        6=>formatResult.set(&crate::org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants::getText(store.borrow().errorValue(valueid) as i32),None),_=>{}
    }
}}
result
    }

    // java private int calcCellCharWidth(int row, int col, KmoXF cellxf) {
    pub fn calcCellCharWidth(&self,_row:i32,col:i32,cellxf:&super::xf::KmoXF::KmoXF)->i32 {
// java cw=xfManager.normalCharWidth;w=getColWidth(col);merged=getMergedRange(row,col);sum widths from col+1 through merged.br.col.
let book=self.book();let manager=book.getXFManager().borrow();let cw=manager.normalCharWidth.get();let mut w=self.getColWidth(col);if let Some(merged)=self.getMergedRange(_row,col){for i in col+1..=merged.br.col{w=w.wrapping_add(self.getColWidth(i));}}
// java int colWidth=(int)((float)((w+(int)(128.0F/cw))/256)*cw);
let colWidth=((w.wrapping_add((128.0f32/cw) as i32)/256) as f32*cw) as i32;
// java normalFontSize=getNormalStyle().getFont().getFontHeight();cellFontSize=cellxf.getFont()!=null?height:normalFontSize;cellFontWidth=cellFontSize/normalFontSize*cw;return(int)(colWidth/cellFontWidth);
let xf=manager.getNormalStyle().expect("null Normal style");let normal=xf.getFont().expect("null Normal font").getFontHeight() as f32;let size=cellxf.getFont().map_or(normal,|f|f.getFontHeight() as f32);let charWidth=size/normal*cw; (colWidth as f32/charWidth) as i32
    }

    // java public void setCellIXF(int row, int col, int ixf) {
    pub fn setCellIXF(&self, row: i32, col: i32, ixf: i32) {
        // java KmoCellData cellData = this.getCellData(row, col)
        let mut cellData = self.getCellData(row, col);
        // java cellData.ixf = ixf
        cellData.ixf = ixf;
        // java if (!this.isFullPrecision())
        // java this.updateCellValue(row, col, cellData)
        if !self.isFullPrecision() { self.updateCellValue(row, col, &mut cellData); }
        // java this.setCellData(row, col, cellData)
        self.setCellData(row, col, &cellData);
    }

    // java private void updateCellValue(int row, int col, KmoCellData cellData) {
    pub fn updateCellValue(&self, row: i32, col: i32, cellData: &mut KmoCellData) {
        // java if (!this.isFullPrecision())
        if !self.isFullPrecision() {
        // java KmoXF cellXF = this.parent.getXFManager().getXF(cellData.ixf)
            let cellXF = self.book().getXFManager().borrow().getXF(cellData.ixf);
        // java if (KmoCellValueStore.identifyTypeByID(cellData.id) == 1)
            if KmoCellValueStore::identifyTypeByID(cellData.id) == 1 {
        // java double d = this.getCellNumericValue(row, col)
                let mut d = self.getCellNumericValue(row, col);
        // java INumFormatter numFormatter = KmoApp.sharedApp().numFormatter()
                super::KmoApp::KmoApp::sharedApp().numFormatter(|numFormatter| {
        // java if (numFormatter != null)
                    if let Some(numFormatter) = numFormatter {
        // java d = numFormatter.formatValue(d, this.parent.isUsing1904(), cellXF.getNumFormat())
                        d = numFormatter.formatValue(d, self.book().isUsing1904(), &cellXF.as_ref().expect("NullPointerException KmoXF").getNumFormat().expect("NullPointerException numfmt"));
                    }
                });
        // java cellData.id = this.getCellValueStore().newDoubleValue(d)
                cellData.id = self.getCellValueStore().borrow_mut().newDoubleValue(d);
            }
        }
    }

    // java public void setCellBlank(int row, int col) {
    pub fn setCellBlank(&self, row: i32, col: i32) {
        // java KmoCellData cellData = this.getCellData(row, col)
        let mut cellData = self.getCellData(row, col);
        // java cellData.formulaType = 0
        cellData.formulaType = 0;
        // java cellData.id = 0
        cellData.id = 0;
        // java if (cellData.ixf == 0)
        // java cellData.ixf = 15
        if cellData.ixf == 0 { cellData.ixf = 15; }
        // java this.setCellData(row, col, cellData)
        self.setCellData(row, col, &cellData);
        // java this.parent.setDirty(true)
        self.book().setDirty(true);
        // java this.changeListeners.sheetContentsDidFinishChanging()
        self.changeListeners.sheetContentsDidFinishChanging();
        // java this.parent.areaWatcher().postContentChangedEvent(this, row, col)
        self.book().areaWatcher().borrow_mut().postContentChangedEvent(self, row, col);
    }

    // java public void setCellBoolValue(int row, int col, boolean value) {
    pub fn setCellBoolValue(&self, row: i32, col: i32, value: bool) {
        // java KmoCellData cellData = this.getCellData(row, col)
        let mut cellData = self.getCellData(row, col);
        // java cellData.formulaType = 0
        cellData.formulaType = 0;
        // java cellData.id = this.parent.getCellValueStore().newBooleanValue(value)
        cellData.id = self.book().getCellValueStore().borrow_mut().newBooleanValue(value);
        // java if (cellData.ixf == 0)
        // java cellData.ixf = 15
        if cellData.ixf == 0 { cellData.ixf = 15; }
        // java this.setCellData(row, col, cellData)
        self.setCellData(row, col, &cellData);
        // java this.parent.setDirty(true)
        self.book().setDirty(true);
        // java this.changeListeners.sheetContentsDidFinishChanging()
        self.changeListeners.sheetContentsDidFinishChanging();
        // java this.parent.areaWatcher().postContentChangedEvent(this, row, col)
        self.book().areaWatcher().borrow_mut().postContentChangedEvent(self, row, col);
    }

    // java public void setCellNumericValue(int row, int col, double value) {
    pub fn setCellNumericValue(&self, row: i32, col: i32, value: f64) {
        // java KmoCellData cellData = this.getCellData(row, col)
        let mut cellData = self.getCellData(row, col);
        // java cellData.formulaType = 0
        cellData.formulaType = 0;
        // java cellData.id = this.parent.getCellValueStore().newDoubleValue(value)
        cellData.id = self.book().getCellValueStore().borrow_mut().newDoubleValue(value);
        // java if (cellData.ixf == 0)
        // java cellData.ixf = 15
        if cellData.ixf == 0 { cellData.ixf = 15; }
        // java this.setCellData(row, col, cellData)
        self.setCellData(row, col, &cellData);
        // java this.parent.setDirty(true)
        self.book().setDirty(true);
        // java this.changeListeners.sheetContentsDidFinishChanging()
        self.changeListeners.sheetContentsDidFinishChanging();
        // java this.parent.areaWatcher().postContentChangedEvent(this, row, col)
        self.book().areaWatcher().borrow_mut().postContentChangedEvent(self, row, col);
    }

    // java public void setCellErrorValue(int row, int col, byte errType) {
    pub fn setCellErrorValue(&self, row: i32, col: i32, errType: i8) {
        // java KmoCellData cellData = this.getCellData(row, col)
        let mut cellData = self.getCellData(row, col);
        // java cellData.formulaType = 0
        cellData.formulaType = 0;
        // java cellData.id = this.parent.getCellValueStore().newErrorValue(errType)
        cellData.id = self.book().getCellValueStore().borrow_mut().newErrorValue(errType as i32);
        // java if (cellData.ixf == 0)
        // java cellData.ixf = 15
        if cellData.ixf == 0 { cellData.ixf = 15; }
        // java this.setCellData(row, col, cellData)
        self.setCellData(row, col, &cellData);
        // java this.parent.setDirty(true)
        self.book().setDirty(true);
        // java this.changeListeners.sheetContentsDidFinishChanging()
        self.changeListeners.sheetContentsDidFinishChanging();
        // java this.parent.areaWatcher().postContentChangedEvent(this, row, col)
        self.book().areaWatcher().borrow_mut().postContentChangedEvent(self, row, col);
    }

    // java public void setCellStringValue(int row, int col, String value) {
    pub fn setCellStringValue(&self, row: i32, col: i32, value: Option<&str>) {
        // java KmoCellData cellData = this.getCellData(row, col)
        let mut cellData = self.getCellData(row, col);
        // java cellData.formulaType = 0
        cellData.formulaType = 0;
        // java cellData.id = this.parent.getCellValueStore().newStringValue(value)
        cellData.id = self.book().getCellValueStore().borrow_mut().newStringValue(value);
        // java if (cellData.ixf == 0)
        // java cellData.ixf = 15
        if cellData.ixf == 0 { cellData.ixf = 15; }
        // java this.setCellData(row, col, cellData)
        self.setCellData(row, col, &cellData);
        // java this.parent.setDirty(true)
        self.book().setDirty(true);
        // java this.changeListeners.sheetContentsDidFinishChanging()
        self.changeListeners.sheetContentsDidFinishChanging();
        // java this.parent.areaWatcher().postContentChangedEvent(this, row, col)
        self.book().areaWatcher().borrow_mut().postContentChangedEvent(self, row, col);
    }

    // java public void setCellUnicodeStringValue(int row, int col, UnicodeString value) {
    pub fn setCellUnicodeStringValue(&self, row: i32, col: i32, value: Option<Rc<crate::cn::wps::moss::filefmt::biff8::record::common::UnicodeString::UnicodeString>>) {
        // java KmoCellData cellData = this.getCellData(row, col)
        let mut cellData = self.getCellData(row, col);
        // java cellData.formulaType = 0
        cellData.formulaType = 0;
        // java cellData.id = this.parent.getCellValueStore().newRunStringValue(value)
        cellData.id = self.book().getCellValueStore().borrow_mut().newRunStringValue(value);
        // java if (cellData.ixf == 0)
        // java cellData.ixf = 15
        if cellData.ixf == 0 { cellData.ixf = 15; }
        // java this.setCellData(row, col, cellData)
        self.setCellData(row, col, &cellData);
        // java this.parent.setDirty(true)
        self.book().setDirty(true);
        // java this.changeListeners.sheetContentsDidFinishChanging()
        self.changeListeners.sheetContentsDidFinishChanging();
        // java this.parent.areaWatcher().postContentChangedEvent(this, row, col)
        self.book().areaWatcher().borrow_mut().postContentChangedEvent(self, row, col);
    }

    // java public void initCellFormula(int row, int col, String fml, int valueId, int ixf) {
    pub fn initCellFormula(&self, row: i32, col: i32, fml: &str, valueId: i32, ixf: i32) -> Result<(), FormulaParser::FormulaParseException> {
        // java int sheetindex = this.getSheetIndex()
        let sheetindex = self.getSheetIndex();
        // java Ptg[] ptgs = FormulaParser.parse(fml, new EvaluationWorkbook(this.parent), 6, sheetindex, SpreadsheetVersion.EXCEL2007, ParserOption.createOOXMLParserOption())
        let ptgs = FormulaParser::parse(fml, &EvaluationWorkbook::EvaluationWorkbook(&self.book()), 6,
            sheetindex, SpreadsheetVersion::EXCEL2007, &ParserOption::createOOXMLParserOption())?;
        // java byte[] array = new byte[Ptg.getEncodedSize(ptgs)]
        // java Ptg.serializePtgs(ptgs, array, 0)
        // rust: serializer allocates precisely getEncodedSize bytes and writes at offset zero.
        let array = crate::cn::wps::et::ss::formula::ptg::Ptg::serializePtgs(&ptgs);
        // java KmoFormulaData formulaData = SharedObjects.kmoFormulaDataPool.borrowObject()
        let mut formulaData = KmoFormulaData::KmoFormulaData();
        // java KmoCellValueStore cvs = this.getCellValueStore()
        let cvs = self.getCellValueStore();
        // java formulaData.setTokens(cvs.newFormulaTokens(array), Ptg.getEncodedSizeWithoutArrayData(ptgs))
        formulaData.setTokens(cvs.borrow_mut().newFormulaTokens(Some(&array)),
            crate::cn::wps::et::ss::formula::ptg::Ptg::getEncodedSizeWithoutArrayData(&ptgs));
        // java if (valueId != -1)
        // java formulaData.setCacheValueId(valueId)
        if valueId != -1 { formulaData.setCacheValueId(valueId); }
        // java KmoCellData cellData = this.getCellData(row, col)
        let mut cellData = self.getCellData(row, col);
        // java cellData.formulaType = 1
        cellData.formulaType = 1;
        // java cellData.ixf = ixf
        cellData.ixf = ixf;
        // java cellData.id = cvs.newFormulaValue(formulaData)
        cellData.id = cvs.borrow_mut().newFormulaValue(&formulaData);
        // java formulaData = SharedObjects.kmoFormulaDataPool.returnObject(formulaData)
        // rust: local pool buffer drops after its serialized copy enters the store.
        // java this.setCellData(row, col, cellData)
        self.setCellData(row, col, &cellData);
        Ok(())
    }

    // java public void setCellFormulaValue(int row, int col, KmoFormulaData fml) {
    pub fn setCellFormulaValue(&self, row: i32, col: i32, fml: &KmoFormulaData) {
        // java this.setCellFormulaValue(row, col, fml, 1)
        self.setCellFormulaValue__2(row, col, fml, 1);
        // java this.changeListeners.sheetContentsDidFinishChanging()
        self.changeListeners.sheetContentsDidFinishChanging();
    }

    // java public void setCellFormulaValue(int row, int col, KmoFormulaData fml, int formula_type) {
    pub fn setCellFormulaValue__2(&self, row: i32, col: i32, fml: &KmoFormulaData, formula_type: i32) {
        // java KmoCellData cellData = this.getCellData(row, col)
        let mut cellData = self.getCellData(row, col);
        // java cellData.formulaType = formula_type
        cellData.formulaType = formula_type;
        // java cellData.id = this.parent.getCellValueStore().newFormulaValue(fml)
        cellData.id = self.book().getCellValueStore().borrow_mut().newFormulaValue(fml);
        // java this.setCellData(row, col, cellData)
        self.setCellData(row, col, &cellData);
        // java this.changeListeners.sheetContentsDidFinishChanging()
        self.changeListeners.sheetContentsDidFinishChanging();
    }

    // java public void setSheetRightToLeft(boolean rightToLeft) {
// java public void setSheetRightToLeft(boolean rightToLeft) {
// java this.sheetInfo.window2.setIsRTL(rightToLeft);
// java }
    pub fn setSheetRightToLeft(&self,v:bool){let info=&self.sheetInfo;info.window2.grbit.set((info.window2.grbit.get()&!64)|if v{64}else{0});}


    // java public boolean isSheetRightToLeft() {
// java public boolean isSheetRightToLeft() {
// java return this.sheetInfo.window2.getIsRTL();
// java }
    pub fn isSheetRightToLeft(&self)->bool{self.sheetInfo.window2.grbit.get()&64!=0}


    // java public KmoBook getBook() {
    pub fn getBook(&self) -> Rc<KmoBook> {
        // java return this.parent;
        self.book()
    }

    // java public void merge(KmoRect rect) {
    pub fn merge(&self, rect: &KmoRect) {
        // Java: KmoMergeDelegate dele = this.gridData.mergedDele(); dele.merge(rect);
        self.grid.mergedDele().merge(rect);
    }

    pub fn allMergedRangesForSnapshot(&self) -> Vec<KmoRect> {
        let mut out = Vec::new();
        let max_row = self.getMaxRows() - 1;
        let max_col = self.getMaxColumns() - 1;
        self.grid.mergedDele().getIntersectMergedRanges(&KmoRect::KmoRect__bounds(0, 0, max_row, max_col), &mut out);
        out
    }

    // java public void unmerge(KmoRect rect) {
    pub fn unmerge(&self, rect: &KmoRect) {
        // java KmoMergeDelegate dele = this.gridData.mergedDele();
        let dele = self.grid.mergedDele();
        // java dele.unmerge(rect);
        dele.unmerge(rect);
    }

    // java public KmoRect expandMergeBounds(KmoRect rect) {
    pub fn expandMergeBounds(&self, rect: &KmoRect) -> KmoRect {
        // java KmoMergeDelegate dele = this.gridData.mergedDele();
        let dele = self.grid.mergedDele();
        // java return dele.expandBounds(rect);
        dele.expandBounds(rect)
    }

    // java public List<KmoRect> getMergedCells(KmoRect range) {
    pub fn getMergedCells__range(&self, range: &KmoRect) -> Vec<KmoRect> {
        // java KmoMergeDelegate dele = this.gridData.mergedDele();
        let dele = self.grid.mergedDele();
        // java List<KmoRect> mergedRanges = new ArrayList();
        let mut mergedRanges = Vec::new();
        // java dele.getIntersectMergedRanges(range, mergedRanges);
        dele.getIntersectMergedRanges(range, &mut mergedRanges);
        // java return mergedRanges;
        mergedRanges
    }

    // java public boolean isContainsMergedCells(KmoRect range) {
    pub fn isContainsMergedCells(&self, range: &KmoRect) -> bool {
        // java return this.getMergedCells(range).size() != 0;
        !self.getMergedCells__range(range).is_empty()
    }

    // java public boolean isContainsHorMergedCells(KmoRect range) {
    pub fn isContainsHorMergedCells(&self, range: &KmoRect) -> bool {
        // java for(KmoRect rect : this.getMergedCells(range))
        for rect in self.getMergedCells__range(range) {
            // java if (rect.br.col != rect.tl.col)
            if rect.br.col != rect.tl.col {
                // java return true;
                return true;
            }
        }
        // java return false;
        false
    }

    // java public boolean isContainsVecMergedCells(KmoRect range) {
    pub fn isContainsVecMergedCells(&self, range: &KmoRect) -> bool {
        // java for(KmoRect rect : this.getMergedCells(range))
        for rect in self.getMergedCells__range(range) {
            // java if (rect.br.row != rect.tl.row)
            if rect.br.row != rect.tl.row {
                // java return true;
                return true;
            }
        }
        // java return false;
        false
    }

    // java public KmoRect getMergedRange(int row, int col) {
    pub fn getMergedRange(&self, row: i32, col: i32) -> Option<KmoRect> {
        // java KmoMergeDelegate dele = this.gridData.mergedDele();
        let dele = self.grid.mergedDele();
        // java return dele.getMergedCells(row, col);
        dele.getMergedCells(row, col)
    }

    // java public List<KmoRect> getMergedRanges(KmoRect rect, List<KmoRect> out) {
    pub fn getMergedRanges(&self, rect: &KmoRect, out: Option<&mut Vec<KmoRect>>) -> Vec<KmoRect> {
        // java if (out == null)
        if let Some(out) = out {
            // java KmoMergeDelegate dele = this.gridData.mergedDele();
            // java dele.getIntersectMergedRanges(rect, out);
            self.grid.mergedDele().getIntersectMergedRanges(rect, out);
            // java return out;
            out.clone()
        } else {
            // java out = new ArrayList();
            let mut owned = Vec::new();
            // java dele.getIntersectMergedRanges(rect, out);
            self.grid.mergedDele().getIntersectMergedRanges(rect, &mut owned);
            // java return out;
            owned
        }
    }

    // java public void delayedFormulaInit() {
    pub fn delayedFormulaInit(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.rawFormulas != null && this.rawFormulas.size() != 0)
        // java KmoCellValueStore valueStore = this.getCellValueStore()
        // java KmoFormulaData formulaData = new KmoFormulaData()
        // java KmoCellData cellData = new KmoCellData()
        // java Map<Integer, Integer> expTkMap = new HashMap()
        // java int i = 0
        // java for(int s = this.rawFormulas.size(); i < s; ++i)
        // java FormulaRecord fmlRec = (FormulaRecord)this.rawFormulas.get(i)
        // java Formula fml = fmlRec.getFormula()
        // java if (fml != null)
        // java byte[] encodedToken = fml.getByteEncoding()
        // java if (encodedToken != null && encodedToken.length != 0 && encodedToken[0] != 2)
        // java this.getCellData(fmlRec.getRow(), fmlRec.getColumn() & '\uffff', cellData)
        // java formulaData.setCacheValueId(cellData.id)
        // java if (encodedToken[0] == 1 && encodedToken.length == ExpPtg.getSize(this.getVersion()))
        // java Integer key = LittleEndian.getInt(encodedToken, 1)
        // java Integer id = (Integer)expTkMap.get(key)
        // java if (id == null)
        // java id = valueStore.newFormulaTokens(encodedToken)
        // java expTkMap.put(key, id)
        // java formulaData.setTokens(id, fml.getEncodedTokenSize())
        // java int id = valueStore.newFormulaTokens(encodedToken)
        // java formulaData.setTokens(id, fml.getEncodedTokenSize())
        // java cellData.id = valueStore.newFormulaValue(formulaData)
        // java cellData.formulaType = 1
        // java if (encodedToken[0] == 1 && encodedToken.length == ExpPtg.getSize(this.getVersion()))
        // java if (fmlRec.isSharedFormula())
        // java cellData.formulaType = 3
        // java this.addShareFml(fmlRec)
        // java cellData.formulaType = 2
        // java Ptg[] ptgs = Ptg.readTokens(formulaData.getTokensPlainSize(), valueStore.formulaTokens(formulaData.getTokensId()), this.getVersion())
        // java cellData.isPicInCell = this.getCellPic(ptgs, cellData.formulaType) != null
        // java this.setCellData(fmlRec.getRow(), fmlRec.getColumn() & '\uffff', cellData)
        */
    }

    // java public void notifyCellChange(int row, int col, boolean isInCalc) {
    pub fn notifyCellChange(&self, row: i32, col: i32, _is_in_calc: bool) {
        // java this.parent.setLastModify(this.getSheetIndex(), row, col);
        let index = self.getSheetIndex();
        self.book().setLastModify(index, row, col);
    }

    // java private static Ptg[] findRefPtgs(KmoSheet sheet, int row, int col) {
    pub fn findRefPtgs(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellValueStore valueStore = sheet.getBook().getCellValueStore()
        // java KmoCellData cellData = SharedObjects.kmoCellDataPool.borrowObject()
        // java sheet.getCellData(row, col, cellData)
        // java int valueId = cellData.id
        // java SharedObjects.kmoCellDataPool.returnObject(cellData)
        // java if (KmoCellValueStore.identifyTypeByID(valueId) == 7)
        // java KmoFormulaData fmlData = SharedObjects.kmoFormulaDataPool.borrowObject()
        // java valueStore.formulaValue(valueId, fmlData)
        // java valueId = fmlData.getCacheValueId()
        // java fmlData = SharedObjects.kmoFormulaDataPool.returnObject(fmlData)
        // java if (KmoCellValueStore.identifyTypeByID(valueId) == 2)
        // java String value = valueStore.stringValue(valueId)
        // java if (value != null && value.startsWith("=DISPIMG"))
        // java return FormulaParser.parse(value.substring(1), new EvaluationWorkbook(sheet.getBook()), 0, sheet.getSheetIndex(), sheet.getBook().getSpreadsheetVersion(), ParserOption.createByLanguage(KmoBooks.getLanguageType()))
        // java return null
        */
    }

    // java private static Ptg[] findRefPtgs(KmoSheet sheet, int row, int col)
    fn findRefPtgs__at(sheet: &KmoSheet, row: i32, col: i32) -> Option<Vec<Ptg>> {
        // java KmoCellValueStore valueStore = sheet.getBook().getCellValueStore();
        let value_store = sheet.getBook().getCellValueStore();
        // java sheet.getCellData(row, col, cellData);
        let cell = sheet.getCellData(row, col);
        // java int valueId = cellData.id;
        let mut value_id = cell.id;
        // java if (KmoCellValueStore.identifyTypeByID(valueId) == 7)
        if KmoCellValueStore::identifyTypeByID(value_id) == 7 {
            // java valueStore.formulaValue(valueId, fmlData);
            let mut formula = KmoFormulaData::KmoFormulaData();
            value_store.borrow().formulaValue(value_id, &mut formula);
            // java valueId = fmlData.getCacheValueId();
            value_id = formula.getCacheValueId();
        }
        // java if (KmoCellValueStore.identifyTypeByID(valueId) == 2)
        if KmoCellValueStore::identifyTypeByID(value_id) == 2 {
            // java String value = valueStore.stringValue(valueId);
            let value = value_store.borrow_mut().stringValue(value_id);
            // java if (value != null && value.startsWith("=DISPIMG"))
            if value.starts_with("=DISPIMG") {
                // java return FormulaParser.parse(value.substring(1), ...);
                let formula: String = value.chars().skip(1).collect();
                let book = sheet.getBook();
                let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
                // java ParserOption.createByLanguage(KmoBooks.getLanguageType())
                // 分隔符用 OOXML。DISPIMG 走 AnalysisToolPak.checkUDFName，解析成功返回 NamePtg。
                let opt = ParserOption::createOOXMLParserOption();
                match FormulaParser::parse(&formula, &workbook, 0, sheet.getSheetIndex(), sheet.getVersion(), &opt) {
                    Ok(tokens) => return Some(tokens),
                    Err(err) => panic!("FormulaParseException: {}", err.message),
                }
            }
        }
        // java return null;
        None
    }

    fn java_string_hash(text: &str) -> i32 {
        let mut hash = 0i32;
        for unit in text.encode_utf16() {
            hash = hash.wrapping_mul(31).wrapping_add(i32::from(unit));
        }
        hash
    }

    // java public KmoCellPic getCellPic(Ptg[] ptgs, int formulaType) {
    pub fn getCellPic__2(&self, ptgs: &[Ptg], formula_type: i32) -> Option<KmoCellPic> {
        // java if (formulaType != 0 && ptgs != null && ptgs.length != 0)
        if formula_type == 0 || ptgs.is_empty() {
            // java return null;
            return None;
        }
        let mut owned = ptgs.to_vec();
        // java if (formulaType != 1 && ptgs[0] instanceof ExpPtg)
        if formula_type != 1 {
            if let PtgKind::Exp { row, col, .. } = owned[0].kind {
                // java ExpFmlManager.ExpFml evalfml = this.getExpFmlManager().query(...);
                let evalfml = self
                    .getExpFmlManager()
                    .borrow()
                    .query(row, i32::from(col), formula_type == 3);
                // java if (evalfml == null) return null;
                let Some(evalfml) = evalfml else {
                    return None;
                };
                // java ptgs = evalfml.getPtgs();
                owned = evalfml.getPtgs();
                if owned.is_empty() {
                    return None;
                }
            }
        }
        let mut tokens = owned;
        // java if (ptgs[0] instanceof RefPtg) — RefNPtg 不在这个分支
        if let PtgKind::Ref {
            row,
            col_raw,
            relative: false,
            ..
        } = tokens[0].kind
        {
            // java ptgs = findRefPtgs(this, refPtg.getRow(), refPtg.getColumn());
            let Some(resolved) = Self::findRefPtgs__at(self, row, i32::from(col_raw & 0x3fff)) else {
                return None;
            };
            tokens = resolved;
        } else if let PtgKind::Ref3D {
            extern_index,
            row,
            col_raw,
            ..
        } = tokens[0].kind
        {
            // java EvaluationExtSheet extSheet = new EvaluationExtSheet(this.getBook(), refPtg.getExternSheetIndex());
            let book = self.getBook();
            let ext_sheet = EvaluationExtSheet::EvaluationExtSheet(&book, extern_index);
            // java if (extSheet.getBookType() == 4) return null;
            if ext_sheet.getBookType() == 4 {
                return None;
            }
            // java int sheetIndex = extSheet.getFirstSheetIndex();
            let sheet_index = ext_sheet.getFirstSheetIndex();
            // java if (sheetIndex >= 0 && sheetIndex < this.getBook().sheetCount())
            if sheet_index >= 0 && sheet_index < book.sheetCount() {
                let col = i32::from(col_raw & 0x3fff);
                let resolved = if sheet_index == self.getSheetIndex() {
                    Self::findRefPtgs__at(self, row, col)
                } else {
                    let other = book.sheetAt(sheet_index);
                    let other = other.borrow();
                    Self::findRefPtgs__at(&other, row, col)
                };
                // java ptgs = findRefPtgs(...); 找不到引用格时 ptgs 为 null
                let Some(resolved) = resolved else {
                    return None;
                };
                tokens = resolved;
            }
        }
        // java if (ptgs != null && ptgs.length >= 3 && ptgs[0] instanceof NamePtg)
        if tokens.len() >= 3 {
            if let PtgKind::Name { index } = tokens[0].kind {
                // java nameRecord = getLinkTable().getNameRecord(ptg.getIndex()); 异常则保持 null
                let name_record = self.getBook().getLinkTable().borrow().tryNameRecord(index).unwrap_or(None);
                let Some(name_record) = name_record else {
                    return None;
                };
                // java "_xlfn.DISPIMG".equals(formuleName)
                if name_record.borrow().getNameText() == "_xlfn.DISPIMG" {
                    if let PtgKind::Str(pi_name) = &tokens[1].kind {
                        // java getDispingKmoPicture(piName.hashCode())
                        match self.getBook().getDispingKmoPicture(Self::java_string_hash(pi_name)) {
                            Some(mut pic) => {
                                pic.mType = match &tokens[2].kind {
                                    PtgKind::Int(value) => i32::from(*value),
                                    _ => -1,
                                };
                                return Some(pic);
                            }
                            // java kmoCellPic = KmoCellPic.emptyKmoCellPic;
                            None => return Some(KmoCellPic::emptyKmoCellPic()),
                        }
                    }
                    return Some(KmoCellPic::emptyKmoCellPic());
                }
            }
        }
        None
    }

    // java private void addShareFml(FormulaRecord fmlRec) {
    pub fn addShareFml(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (fmlRec.isSharedFormula())
        // java Ptg[] ptgs = fmlRec.getParsedExpression()
        // java if (ptgs.length != 0)
        // java if (ptgs[0].getSid() == 1)
        // java int tmpRow = fmlRec.getRow()
        // java int tmpCol = fmlRec.getColumn()
        // java if (!this.isMasterCell(tmpRow, tmpCol))
        // java ExpPtg expPtg = (ExpPtg)ptgs[0]
        // java int masterRow = expPtg.getRow()
        // java int masterCol = expPtg.getColumn()
        // java ExpFmlManager.ExpFml fml = (ExpFmlManager.ExpFml)this.sharedFormulaMap.get(masterRow << 14 | masterCol & 16383)
        // java if (fml != null)
        // java fml.addShareCell(tmpRow, tmpCol)
        // java List<ExpFmlManager.ExpFml> expFmlLst = this.getExpFmlManager().getMatchedSharedFmls(masterRow, masterCol)
        // java for(int i = 0; i < expFmlLst.size(); ++i)
        // java ExpFmlManager.ExpFml expFml = (ExpFmlManager.ExpFml)expFmlLst.get(i)
        // java if (tmpRow >= expFml.firstRow && tmpRow <= expFml.lastRow && tmpCol >= expFml.firstCol && tmpCol <= expFml.lastCol)
        // java expFml.addShareCell(tmpRow, tmpCol)
        // java return
        */
    }

    // java private boolean isMasterCell(int tmpRow, int tmpCol) {
    pub fn isMasterCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java List<ExpFmlManager.ExpFml> expFmlLst = this.getExpFmlManager().getMatchedSharedFmls(tmpRow, tmpCol)
        // java for(int i = 0; i < expFmlLst.size(); ++i)
        // java ExpFmlManager.ExpFml expFml = (ExpFmlManager.ExpFml)expFmlLst.get(i)
        // java if (expFml.firstRow == tmpRow && expFml.firstCol == tmpCol)
        // java return true
        // java return false
        */
    }

    // java public KmoHyperLinkManager hyperlinkMgr() {
    pub fn hyperlinkMgr(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.hyperlinkMgr
        */
    }

    // java public List<DVRecord> getDataValidations() {
    pub fn getDataValidations(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java List<KmoDataValidation> dvs = this.dvMgr.getAllDVs()
        // java ArrayList<DVRecord> rets = new ArrayList()
        // java int i = 0
        // java for(int size = dvs.size(); i < size; ++i)
        // java rets.add(KmoDataValidation.transKmoDVToRecord((KmoDataValidation)dvs.get(i), this.getVersion()))
        // java return rets
        */
    }

    // java public int getPivotTableCount() {
    pub fn getPivotTableCount(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java List<KmoPivotTableDefine> list = this.getBook().getPivotTable().getTables()
        // java int count = 0
        // java for(KmoPivotTableDefine tableDefine : list)
        // java if (tableDefine.getSheet().name().equals(this.name()))
        // java ++count
        // java return count
        */
    }

    // java public short getDataValidationFlags() {
    pub fn getDataValidationFlags(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.dviFlags
        */
    }

    // java public void setDataValidationFlags(short f) {
    pub fn setDataValidationFlags(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.dviFlags = f
        */
    }

    // java public void addDataValidation(DVRecord dv) {
    pub fn addDataValidation(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java List<KmoDataValidation> dvs = KmoDataValidation.createFromDVRecord(dv)
        // java int i = 0
        // java for(int size = dvs.size(); i < size; ++i)
        // java this.dvMgr.addDataValidation((KmoDataValidation)dvs.get(i))
        */
    }

    // java public KmoDVManager dvMgr() {
    pub fn dvMgr(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.dvMgr
        */
    }

    // java public IgnoredErrorManager getIgnoredErrorManager() {
    pub fn getIgnoredErrorManager(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.ignoredErrorManager
        */
    }

    // java public void setScale(short nscl, short dscl) {
// java public void setScale(short nscl, short dscl) {
// java this.sheetInfo.nscl = nscl;
// java this.sheetInfo.dscl = dscl;
// java }
pub fn setScale(&self,n:i16,d:i16){self.sheetInfo.nscl.set(n);self.sheetInfo.dscl.set(d);}

    // java public short getNScale() {
// java public short getNScale() {
// java return this.sheetInfo.nscl;
// java }
pub fn getNScale(&self)->i16{self.sheetInfo.nscl.get()}

    // java public short getDScale() {
// java public short getDScale() {
// java return this.sheetInfo.dscl;
// java }
pub fn getDScale(&self)->i16{self.sheetInfo.dscl.get()}

    // java public boolean isSelected() {
    pub fn isSelected(&self) -> bool {
        // java return this.sheetInfo.window2.isSelected();
        self.sheetInfo.window2.isSelected()
    }

    // java public boolean isAllSelected() {
    pub fn isAllSelected(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect selection = this.getSelectionRect()
        // java return selection.width() >= this.getMaxColumns() && selection.height() >= this.getMaxRows()
        */
    }

    // java public KmoRect getUsedRangeAndSelectionRect() {
    pub fn getUsedRangeAndSelectionRect(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect kmoRect = new KmoRect(this.getSelectionRect())
        // java if (kmoRect.isFullCol(this.getMaxRows()))
        // java kmoRect.br.row = Math.min(this.getUsedRange().br.row, kmoRect.br.row)
        // java if (kmoRect.isFullRow(this.getMaxColumns()))
        // java kmoRect.br.col = Math.min(this.getUsedRange().br.col, kmoRect.br.col)
        // java return kmoRect
        */
    }

    // java public boolean isLargeSelectedRect() {
    pub fn isLargeSelectedRect(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect kmoRect = this.getUsedRangeAndSelectionRect()
        // java return this.isLargeSelectedRect(kmoRect)
        */
    }

    // java public boolean isLargeSelectedRect(KmoRect kmoRect) {
    pub fn isLargeSelectedRect__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int maxSize = 131072
        // java if (this.sheetOp().hasAutoFilter())
        // java maxSize = 1048576
        // java return (long)kmoRect.width() * (long)kmoRect.height() >= (long)maxSize
        */
    }

    // java public KmoRect getAllUsedRange() {
    pub fn getAllUsedRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect textUsedRange = this.getUsedRange()
        // java KmoRect range = new KmoRect(0, 0, textUsedRange.br.row, textUsedRange.br.col)
        // java KmoRect drawingAggUsedRange = this.getSheetDrawingAgg().getUsedRange()
        // java if (drawingAggUsedRange != null)
        // java range.br.row = Math.max(range.br.row, drawingAggUsedRange.br.row)
        // java if (drawingAggUsedRange != null)
        // java range.br.col = Math.max(range.br.col, drawingAggUsedRange.br.col)
        // java range.br.row = Math.max(range.br.row, this.getRowInfos().getLastIndex())
        // java range.br.col = Math.max(range.br.col, this.getColInfos().getLastIndex())
        // java range.br.row = Math.min(range.br.row, this.getMaxRows() - 1)
        // java range.br.col = Math.min(range.br.col, this.getMaxColumns() - 1)
        // java return range
        */
    }

    // java public int minRowInfoIndex() {
// java public int minRowInfoIndex() {
// java int ret = this.rowInfos.getFirstRow();
// java return ret < 0 ? 0 : ret;
// java }
    pub fn minRowInfoIndex(&self)->i32{self.rowInfos.getFirstRow().max(0)}

    // java public int maxRowInfoIndex() {
// java public int maxRowInfoIndex() {
// java int ret = this.rowInfos.getLastRow();
// java return ret < 0 ? 0 : ret;
// java }
    pub fn maxRowInfoIndex(&self)->i32{self.rowInfos.getLastRow().max(0)}

    // java public int maxColumnInfoIndex() {
// java public int maxColumnInfoIndex() {
// java int ret = this.colInfos.getLastCol();
// java return ret < 0 ? 0 : ret;
// java }
    pub fn maxColumnInfoIndex(&self)->i32{self.colInfos.borrow().getLastCol().max(0)}

    // java public int leftTopRow() {
// java public int leftTopRow() {
// java return this.sheetInfo.window2.getTopRow();
// java }
    pub fn leftTopRow(&self)->i32{self.sheetInfo.window2.topRow.get()}


    // java public void setLeftTopRow(int row) {
// java public void setLeftTopRow(int row) {
// java this.sheetInfo.window2.setTopRow(row);
// java this.changeListeners.sheetSettingsDidFinishChanging();
// java }
    pub fn setLeftTopRow(&self,row:i32)->(){self.sheetInfo.window2.topRow.set(row);self.changeListeners.sheetSettingsDidFinishChanging();}


    // java public int leftTopCol() {
// java public int leftTopCol() {
// java return this.sheetInfo.window2.getLeftCol();
// java }
    pub fn leftTopCol(&self)->i32{self.sheetInfo.window2.leftCol.get()}


    // java public void setLeftTopCol(int col) {
// java public void setLeftTopCol(int col) {
// java this.sheetInfo.window2.setLeftCol(col);
// java }
    pub fn setLeftTopCol(&self,col:i32)->(){self.sheetInfo.window2.leftCol.set(col);}


    // java public void initGutsRecords(GutsRecord gr) {
    pub fn initGutsRecords(&self, gr: GutsRecord) {
        // java this.sheetInfo.setGuts(gr, true);
        *self.sheetInfo.guts.borrow_mut() = gr;
    }

    // java public GutsRecord getGutsRec() {
    pub fn getGutsRec(&self) -> GutsRecord {
        // java return this.sheetInfo.getGuts();
        self.sheetInfo.guts.borrow().clone()
    }

    // java public void initWSBool(WSBoolRecord ws) {
    pub fn initWSBool(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.setWSBool(ws, true)
        */
    }

    // java public WSBoolRecord getWSBool() {
    pub fn getWSBool(&self) -> &RefCell<WSBoolRecord> {
        // java return this.sheetInfo.getWsbool();
        &self.sheetInfo.wsbool
    }

    // java public void setCalcMode(short calcMode) {
    pub fn setCalcMode(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.calcMode = calcMode
        */
    }

    // java public short getCalcMode() {
    pub fn getCalcMode(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.calcMode
        */
    }

    // java public void setFullPrecision(boolean precision) {
    pub fn setFullPrecision(&self, precision: bool) {
        // java this.sheetInfo.fullPrecision = precision
        self.sheetInfo.fullPrecision.set(precision);
    }

    // java public boolean isFullPrecision() {
    pub fn isFullPrecision(&self) -> bool {
        // java return this.sheetInfo.fullPrecision && this.getBook().getCalcPr().isFullPrecision()
        self.sheetInfo.fullPrecision.get() && self.book().getCalcPr().borrow().isFullPrecision()
    }

    // java public void setCalcCount(short calcCount) {
    pub fn setCalcCount(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.calcCount = calcCount
        */
    }

    // java public short getCalcCount() {
    pub fn getCalcCount(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.calcCount
        */
    }

    // java public void setRefMode(short refMode) {
    pub fn setRefMode(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.refMode = refMode
        */
    }

    // java public short getRefMode() {
    pub fn getRefMode(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.refMode
        */
    }

    // java public void setIteration(boolean iteration) {
    pub fn setIteration(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.iteration = iteration
        */
    }

    // java public boolean getIteration() {
    pub fn getIteration(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.iteration
        */
    }

    // java public void setIterationDelta(double delta) {
    pub fn setIterationDelta(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.delta = delta
        */
    }

    // java public double getIterationDelta() {
    pub fn getIterationDelta(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.delta
        */
    }

    // java public void setSaveRecalc(boolean saveRecalc) {
    pub fn setSaveRecalc(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.saveRecalc = saveRecalc
        */
    }

    // java public boolean getSaveRecalc() {
    pub fn getSaveRecalc(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.saveRecalc
        */
    }

    // java public void setGridset(boolean gridset) {
    pub fn setGridset(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheetInfo.gridSet = gridset
        */
    }

    // java public boolean getGridset() {
    pub fn getGridset(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.gridSet
        */
    }

    // java public SheetDrawingAgg getSheetDrawingAgg() {
    pub fn getSheetDrawingAgg(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetDrawingAgg
        */
    }

    // java public KmoHyperlink getHyperlink(int row, int col) {
    pub fn getHyperlink(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoFmlHyperLinkManager fmlHyperLinkManager = this.parent.getFmlHyperLinkManager()
        // java KmoHyperlink hyperlink = null
        // java if (fmlHyperLinkManager != null)
        // java hyperlink = fmlHyperLinkManager.getLink(row, col, this.getSheetIndex())
        // java return hyperlink != null ? hyperlink : this.hyperlinkMgr().getCellHyperlink(row, col)
        */
    }

    // java public boolean isCellHasHyperlink(int row, int col) {
    pub fn isCellHasHyperlink(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect r = this.getMergedRange(row, col)
        // java if (r != null)
        // java row = r.tl.row
        // java col = r.tl.col
        // java return this.parent.getFmlHyperLinkManager().getLink(row, col, this.getSheetIndex()) != null ? true : this.hyperlinkMgr.isCellHasHyperlink(row, col)
        */
    }

    // java public boolean isContainsHyperlink(KmoRect range) {
    pub fn isContainsHyperlink(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.hyperlinkMgr.isRangeHasHyperlink(range)
        */
    }

    // java public boolean isCellHasComments(int row, int col) {
    pub fn isCellHasComments(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData cell = SharedObjects.kmoCellDataPool.borrowObject()
        // java boolean var4
        // java this.getCellData(row, col, cell)
        // java var4 = cell.hasComment
        // java cell = SharedObjects.kmoCellDataPool.returnObject(cell)
        // java return var4
        */
    }

    // java public boolean isContainsComment(KmoRect range) {
    pub fn isContainsComment(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.noteMgr().isContainsComments(range)
        */
    }

    // java public String getCommentString(int row, int col) {
    pub fn getCommentString(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoComment comment = this.sheetDrawingAgg.getNoteMgr().findNote(row, col)
        // java return comment == null ? "" : comment.getShapeText().getString()
        */
    }

    // java public Bitmap getBgPicBitmap() {
    pub fn getBgPicBitmap(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.backgroundRecord == null && this.backgroundByte == null)
        // java return null
        // java else if (this.backgroundBitmap != null && !this.backgroundBitmap.isRecycled())
        // java return this.backgroundBitmap
        // java if (this.backgroundRecord != null)
        // java this.backgroundBitmap = ETBitmapFactory.bitmapRecord2Bitmap(this.backgroundRecord, true)
        // java else if (this.backgroundByte != null)
        // java this.backgroundBitmap = ETBitmapFactory.picStream2Bitmap(this.backgroundByte, this.backgroundType, true)
        // java return this.backgroundBitmap
        */
    }

    // java public void setBackgroundImage(BitmapRecord bi) {
    pub fn setBackgroundImage(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.backgroundRecord = bi
        */
    }

    // java public BitmapRecord getBackgroundImage() {
    pub fn getBackgroundImage(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.backgroundRecord != null)
        // java return this.backgroundRecord
        // java else if (this.backgroundByte != null)
        // java return this.backgroundBitmap != null && !this.backgroundBitmap.isRecycled() ? ETBitmapFactory.bitmap2BitmapRecord(this.backgroundBitmap) : ETBitmapFactory.picStream2BitmapRecord(this.backgroundByte, this.backgroundType)
        // java return null
        */
    }

    // java public void setBackgroundImage2(byte[] stream, int picType) {
    pub fn setBackgroundImage2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.backgroundByte = stream
        // java this.backgroundType = picType
        */
    }

    // java public byte[] getBackgroundImage2() {
    pub fn getBackgroundImage2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.backgroundByte != null)
        // java return this.backgroundByte
        // java else if (this.backgroundRecord != null)
        // java this.backgroundType = 6
        // java return this.backgroundBitmap != null && !this.backgroundBitmap.isRecycled() ? ETBitmapFactory.bitmap2PicStream(this.backgroundBitmap) : ETBitmapFactory.bitmapRecord2PicStream(this.backgroundRecord)
        // java return null
        */
    }

    // java public int getBackgroundImageType() {
    pub fn getBackgroundImageType(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.backgroundType
        */
    }

    // java public KmoPrintSetting getPrintSettings() {
    pub fn getPrintSettings(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.printSettings
        */
    }

    // java public RangeQuickCalcResult getRangeQuickCalc(KmoRect range) {
    pub fn getRangeQuickCalc(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect rect = this.usedRange.getIntersectRect(range)
        // java if (rect == null)
        // java return new RangeQuickCalcResult((double)0.0F, (double)0.0F, (double)0.0F, (double)0.0F, 0, 0)
        // java double sum = (double)0.0F
        // java int notNullCount = 0
        // java int averageCount = 0
        // java double maxValue = -Double.MAX_VALUE
        // java double minValue = Double.MAX_VALUE
        // java double average = (double)0.0F
        // java KmoMatrixCellIterator it = this.getGrid().getMatrix().getCellDataIterator(rect, true, true, true)
        // java while(it.hasNext())
        // java it.next()
        // java if (this.isCellExist(it.row(), it.col()) && !this.isRowHidden(it.row()))
        // java int cellType = this.getCellValueType(it.row(), it.col())
        // java if (cellType != 0)
        // java ++notNullCount
        // java if (cellType == 1)
        // java double cellValue = this.getCellNumericValue(it.row(), it.col())
        // java sum += cellValue
        // java maxValue = Math.max(maxValue, cellValue)
        // java minValue = Math.min(minValue, cellValue)
        // java ++averageCount
        // java if (averageCount == 0)
        // java maxValue = (double)0.0F
        // java minValue = (double)0.0F
        // java else if (!Double.isInfinite(sum) && !Double.isNaN(sum))
        // java average = sum / (double)averageCount
        // java average = sum
        // java int precision = 9
        // java if (this.canRoundedDouble(sum))
        // java sum = Numeric.round(sum, 9)
        // java sum = (double)0.0F
        // java if (this.canRoundedDouble(average))
        // java average = Numeric.round(average, 9)
        // java average = (double)0.0F
        // java if (this.canRoundedDouble(maxValue))
        // java maxValue = Numeric.round(maxValue, 9)
        // java maxValue = (double)0.0F
        // java if (this.canRoundedDouble(minValue))
        // java minValue = Numeric.round(minValue, 9)
        // java minValue = (double)0.0F
        // java return new RangeQuickCalcResult(sum, average, maxValue, minValue, notNullCount, averageCount)
        */
    }

    // java private boolean canRoundedDouble(double d) {
    pub fn canRoundedDouble(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return !Double.isInfinite(d) && !Double.isNaN(d)
        */
    }

    // java public KmoSheetProtect getSheetProtect() {
    pub fn getSheetProtect(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetProtect
        */
    }

    // java protected void resetColInfo(int col) {
    pub fn resetColInfo(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfo colInfo = this.colInfos.fitAndGetColInfo(col)
        // java if (colInfo != null)
        // java this.colInfos.getColInfo(col).reset()
        */
    }

    // java protected void resetRowInfo(int row) {
    pub fn resetRowInfo(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRowInfo info = this.rowInfos.getRowInfo(row)
        // java if (info != null)
        // java info.setHeight((short)this.sheetInfo.getDefRowHeight())
        // java info.setXFIndex((short)15)
        // java info.setOptionFlags((short)256)
        // java info.setHidden(this.sheetInfo.getDefRowDyZero())
        // java info.resetHiddenAttr(this, this.sheetInfo.getDefRowDyZero())
        */
    }

    // java public void setRangeXF(KmoRect range, KmoXF xf, KmoXFMask xfMask) {
    pub fn setRangeXF(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.setRangeXF(range, xf, xfMask, true)
        */
    }

    // java public void setRangeXF(KmoRect range, KmoXF xf, KmoXFMask xfMask, boolean ingoreHiddenCellIfFilterOn) {
    pub fn setRangeXF__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.getGrid().getMatrix().addChangeRange(range)
        // java this.changeListeners.startMergeEvents()
        // java boolean isJumpEmptyCell = false
        // java boolean isFilterOn = this.sheetOp().isFilterOn()
        // java KmoXFManager xfMgr = this.parent.getXFManager()
        // java if (range.height() == this.getMaxRows())
        // java for(int col = range.tl.col; col <= range.br.col; ++col)
        // java if (!isFilterOn || !this.isColHidden(col))
        // java KmoXF oldXF = xfMgr.getXF(this.getColXFIndex(col))
        // java int newIXF = xfMgr.getNewIXF(oldXF, xf, xfMask)
        // java this.setColXFIndex((short)col, (short)newIXF)
        // java isJumpEmptyCell = true
        // java else if (range.width() == this.getMaxColumns())
        // java for(int row = range.tl.row; row <= range.br.row; ++row)
        // java if (!isFilterOn || !this.isRowHidden(row))
        // java KmoXF oldXF = xfMgr.getXF(this.getRowXFIndex(row))
        // java int newIXF = xfMgr.getNewIXF(oldXF, xf, xfMask)
        // java this.setRowXF(row, (short)newIXF)
        // java isJumpEmptyCell = true
        // java this.setRangeXF(range, xf, xfMask, isJumpEmptyCell, ingoreHiddenCellIfFilterOn && isFilterOn)
        // java if (range.height() == this.getMaxRows())
        // java List<Integer> formattedRows = new ArrayList()
        // java for(int row = range.tl.row; row <= range.br.row; ++row)
        // java if (this.isRowFormatted(row))
        // java formattedRows.add(row)
        // java for(int col = range.tl.col; col <= range.br.col; ++col)
        // java for(Integer row : formattedRows)
        // java if (!this.isCellExist(row, col))
        // java KmoXF oldXF = xfMgr.getXF(this.getRowXFIndex(row))
        // java int newIXF = xfMgr.getNewIXF(oldXF, xf, xfMask)
        // java this.setCellIXF(row, col, newIXF)
        // java else if (range.width() == this.getMaxColumns())
        // java List<Integer> formattedCols = new ArrayList()
        // java for(int col = range.tl.col; col <= range.br.col; ++col)
        // java if (this.isColFormatted(col))
        // java formattedCols.add(col)
        // java for(int row = range.tl.row; row <= range.br.row; ++row)
        // java for(Integer col : formattedCols)
        // java if (!this.isCellExist(row, col))
        // java KmoXF oldXF = xfMgr.getXF(this.getColXFIndex(col))
        // java int newIXF = xfMgr.getNewIXF(oldXF, xf, xfMask)
        // java this.setCellIXF(row, col, newIXF)
        // java this.parent.pasteMgr().cancelPasteState()
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java this.getBook().areaWatcher().postContentChangedEvent(this, range)
        // java this.changeListeners.finishMergeEvents()
        */
    }

    // java public void setColFormat(int col, KmoXF xf, KmoXFMask xfMask) {
    pub fn setColFormat(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoXFManager xfMgr = this.parent.getXFManager()
        // java KmoXF oldXF = xfMgr.getXF(this.getColXFIndex(col))
        // java int newIXF = xfMgr.getNewIXF(oldXF, xf, xfMask)
        // java this.setColXFIndex((short)col, (short)newIXF)
        // java CellIterator it = this.getCellIterator(new KmoRect(0, col, this.getMaxRows() - 1, col))
        // java while(it.hasNext())
        // java it.next()
        // java this.changeCellXF(it.row(), it.col(), xf, xfMask)
        // java List<Integer> formattedRows = new ArrayList()
        // java IndexMapIterator<KmoRowInfo> iterator = this.rowInfos.iterator()
        // java while(iterator.hasNext())
        // java iterator.next()
        // java if (((KmoRowInfo)iterator.data()).isFormated())
        // java formattedRows.add(iterator.index())
        // java for(Integer row : formattedRows)
        // java if (!this.isCellExist(row, col))
        // java oldXF = xfMgr.getXF(this.getRowXFIndex(row))
        // java newIXF = xfMgr.getNewIXF(oldXF, xf, xfMask)
        // java this.setCellIXF(row, col, newIXF)
        */
    }

    // java public void setRowsXF(int startRow, int endRow, KmoXF xf, KmoXFMask xfMask) {
    pub fn setRowsXF(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean isFilterOn = this.sheetOp().isFilterOn()
        // java KmoXFManager xfMgr = this.parent.getXFManager()
        // java for(int row = startRow; row <= endRow; ++row)
        // java if (!isFilterOn || !this.isRowHidden(row))
        // java KmoXF oldXF = xfMgr.getXF(this.getRowXFIndex(row))
        // java int newIXF = xfMgr.getNewIXF(oldXF, xf, xfMask)
        // java this.setRowXF(row, (short)newIXF)
        */
    }

    // java public void setColsXF(int startCol, int endCol, KmoXF xf, KmoXFMask xfMask) {
    pub fn setColsXF(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean isFilterOn = this.sheetOp().isFilterOn()
        // java KmoXFManager xfMgr = this.parent.getXFManager()
        // java for(int col = startCol; col <= endCol; ++col)
        // java if (!isFilterOn || !this.isColHidden(col))
        // java KmoXF oldXF = xfMgr.getXF(this.getColXFIndex(col))
        // java int newIXF = xfMgr.getNewIXF(oldXF, xf, xfMask)
        // java this.setColXFIndex((short)col, (short)newIXF)
        */
    }

    // java public boolean setXFontName(KmoRect range, ITypeface typeface, KmoXF xf, KmoXFMask mask, KmoRect intersectRange) {
    pub fn setXFontName(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean result = false
        // java this.changeListeners.startMergeEvents()
        // java KmoCellData data = new KmoCellData()
        // java for(int r = intersectRange.tl.row; r <= intersectRange.br.row; ++r)
        // java for(int c = intersectRange.tl.col; c <= intersectRange.br.col; ++c)
        // java this.getCellData(r, c, data)
        // java KmoXF oldXf = this.getCellXF(r, c)
        // java UnicodeString string = null
        // java String text = null
        // java if (this.getCellValueStore().isRunsString2(data.id))
        // java string = this.getCellValueStore().runsStringValue(data.id)
        // java text = string != null ? string.toString() : ""
        // java text = this.getEditString(r, c)
        // java if (null != typeface && !text.isEmpty() && data.formulaType == 0 && this.getCellValueType(r, c) != 1)
        // java int i = 0
        // java UnicodeString unicodeString = null
        // java if (string != null)
        // java unicodeString = (UnicodeString)string.clone()
        // java for(Boolean lastState = null; i < text.length(); ++i)
        // java char oneChar = text.charAt(i)
        // java if (!this.isBreakSpanChar(oneChar))
        // java boolean hasUnicodeBlock = typeface.hasUnicodeBlock(oneChar)
        // java if (lastState == null || hasUnicodeBlock != lastState)
        // java KmoFont setFont = oldXf.getFont()
        // java if (hasUnicodeBlock)
        // java KmoFont font = KmoFont.createFont(setFont)
        // java font.setFontName(xf.getFont().getFontName())
        // java setFont = this.getBook().getXFManager().findFontByProp(font)
        // java if (setFont == null)
        // java setFont = font
        // java FormatRun formatRun = new FormatRun((short)i, this.getBook().getXFManager().checkFontIndex(setFont))
        // java if (unicodeString == null)
        // java unicodeString = new UnicodeString(text)
        // java unicodeString.addFormatRun(formatRun)
        // java lastState = hasUnicodeBlock
        // java this.changeCellXF(r, c, xf, mask)
        // java if (unicodeString != null && (unicodeString.getFormatRunCount() != 1 || unicodeString.getFormatRun(0).getCharacterPos() != 0))
        // java int id = this.getCellValueStore().newRunStringValue(unicodeString)
        // java this.getCellData(r, c, data)
        // java data.id = id
        // java this.setCellData(r, c, data)
        // java result = true
        // java this.changeCellXF(r, c, xf, mask)
        // java result = true
        // java this.parent.pasteMgr().cancelPasteState()
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java this.parent.areaWatcher().postContentChangedEvent(this, range)
        // java this.changeListeners.finishMergeEvents()
        // java return result
        */
    }

    // java private boolean isBreakSpanChar(char oneChar) {
    pub fn isBreakSpanChar(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java switch (oneChar)
        // java case '\t':
        // java case '\n':
        // java case '\u000b':
        // java case '\r':
        // java case '\u001e':
        // java case ' ':
        // java case '-':
        // java return true
        // java default:
        // java return false
        */
    }

    // java private void changeCellXF(int row, int col, KmoXF xf, KmoXFMask xfMask) {
    pub fn changeCellXF(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoXF oldXF = this.getCellXF(row, col)
        // java int newIXF = this.parent.getXFManager().getNewIXF(oldXF, xf, xfMask)
        // java this.setCellIXF(row, col, newIXF)
        // java KmoCellData data = this.getCellData(row, col)
        // java KmoCellValueStore store = this.getCellValueStore()
        // java if (store.isRunsString2(data.id) && xfMask.isAppliedFont())
        // java UnicodeString ustr = store.runsStringValue(data.id)
        // java if (ustr != null && ustr.getFormatRunCount() > 0)
        // java data.id = store.newRunStringValue((UnicodeString)ustr.clone())
        // java this.changeUnicodeString(xf, xfMask, store.runsStringValue(data.id))
        // java this.setCellData(row, col, data)
        */
    }

    // java private void changeUnicodeString(KmoXF xf, KmoXFMask mask, UnicodeString ustr) {
    pub fn changeUnicodeString(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (ustr != null)
        // java int i = 0
        // java for(int count = ustr.getFormatRunCount(); i < count; ++i)
        // java FormatRun run = ustr.getFormatRun(i)
        // java this.changeFormatRun(xf, mask, run)
        */
    }

    // java private void changeFormatRun(KmoXF xf, KmoXFMask mask, FormatRun run) {
    pub fn changeFormatRun(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoXFManager xfMgr = this.getBook().getXFManager()
        // java KmoFont font = KmoFont.createFont(xfMgr.getFont(run.getFontIndex()))
        // java KmoFont xfFont = xf.getFont()
        // java if (mask.isFontName())
        // java font.setFontName(xfFont.getFontName())
        // java if (mask.isFontHeight())
        // java font.setFontHeight(xfFont.getFontHeight())
        // java if (mask.isFontCharSet())
        // java font.setCharset(xfFont.getCharset())
        // java if (mask.isFontBold())
        // java font.setBoldWeight(xfFont.getBoldWeight())
        // java if (mask.isFontItalic())
        // java font.setItalic(xfFont.isItalic())
        // java if (mask.isFontStrikeout())
        // java font.setStrikeout(xfFont.isStrikeout())
        // java if (mask.isFontUls())
        // java font.setUnderline(xfFont.getUnderline())
        // java if (mask.isFontSSS())
        // java font.setSuperSubScript(xfFont.getSuperSubScript())
        // java if (mask.isFontIcv())
        // java font.setColorPaletteIndex(xfFont.getColorPaletteIndex())
        // java int newIndex = xfMgr.addFont(font)
        // java run.setFontIndex(newIndex)
        */
    }

    // java public void setRangeXF(KmoRect range, KmoXF xf, KmoXFMask xfMask, boolean isJumpEmptyCell, boolean isFilterOn) {
    pub fn setRangeXF__3(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.changeListeners.startMergeEvents()
        // java if (isJumpEmptyCell)
        // java CellIterator it = this.getCellIterator(range)
        // java while(it.hasNext())
        // java it.next()
        // java if (!it.cellData().isNull() && (!isFilterOn || !this.isRowHidden(it.row()) && !this.isColHidden(it.col())))
        // java this.changeCellXF(it.row(), it.col(), xf, xfMask)
        // java for(int row = range.tl.row; row <= range.br.row; ++row)
        // java if (!isFilterOn || !this.isRowHidden(row))
        // java for(int col = range.tl.col; col <= range.br.col; ++col)
        // java if (!isFilterOn || !this.isColHidden(col))
        // java this.changeCellXF(row, col, xf, xfMask)
        // java this.parent.pasteMgr().cancelPasteState()
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java this.changeListeners.finishMergeEvents()
        */
    }

    // java public void getRangeXF(KmoRect range, KmoXF xf, KmoXFMask mask) {
    pub fn getRangeXF(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoXF standardFormat = this.getCellXF(range.tl.row, range.tl.col)
        // java if (this.getCellIXF(range.tl.row, range.tl.col) == 15 || this.getCellIXF(range.tl.row, range.br.col) == 15 || this.getCellIXF(range.br.row, range.tl.col) == 15 || this.getCellIXF(range.br.row, range.br.col) == 15)
        // java standardFormat = this.parent.getXFManager().getXF(15)
        // java xf.completeCopy(standardFormat)
        // java mask.markAll(true)
        // java CellIterator cellIt = this.getCellIterator(range)
        // java boolean filterOn = this.sheetOp.isFilterOn()
        // java while(cellIt.hasNext())
        // java cellIt.next()
        // java if (!filterOn || !this.isRowHidden(cellIt.row()))
        // java KmoXF compareFormat = this.getCellXF(cellIt.row(), cellIt.col())
        // java this.compareDiffOfXF(standardFormat, compareFormat, mask)
        // java this.getRangeBorderFormat(range, xf, mask)
        */
    }

    // java protected void getRangeBorderFormat(KmoRect range, KmoXF xf, KmoXFMask mask) {
    pub fn getRangeBorderFormat(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java mask.markAllBorder(true)
        // java KmoXF standardFormat = this.getCellXF(range.tl.row, range.tl.col)
        // java if (standardFormat.getBorderTop() == 0)
        // java mask.markDgTop(false)
        // java mask.markIcvTop(false)
        // java for(int col = range.tl.col + 1; col <= range.br.col; ++col)
        // java KmoXF compareFormat = this.getCellXF(range.tl.row, col)
        // java if (standardFormat.getBorderTop() != compareFormat.getBorderTop() || standardFormat.getTopBorderPaletteIdx() != compareFormat.getTopBorderPaletteIdx())
        // java mask.markDgTop(false)
        // java mask.markIcvTop(false)
        // java break
        // java if (mask.isDgTop())
        // java xf.setBorderTop(standardFormat.getBorderTop())
        // java xf.setTopBorderPaletteIdx(standardFormat.getTopBorderPaletteIdx())
        // java standardFormat = this.getCellXF(range.br.row, range.tl.col)
        // java if (standardFormat.getBorderBottom() == 0)
        // java mask.markDgBottom(false)
        // java mask.markIcvBottom(false)
        // java for(int col = range.tl.col + 1; col <= range.br.col; ++col)
        // java KmoXF compareFormat = this.getCellXF(range.br.row, col)
        // java if (standardFormat.getBorderBottom() != compareFormat.getBorderBottom() || standardFormat.getBottomBorderPaletteIdx() != compareFormat.getBottomBorderPaletteIdx())
        // java mask.markDgBottom(false)
        // java mask.markIcvBottom(false)
        // java break
        // java if (mask.isDgBottom())
        // java xf.setBorderBottom(standardFormat.getBorderBottom())
        // java xf.setBottomBorderPaletteIdx(standardFormat.getBottomBorderPaletteIdx())
        // java standardFormat = this.getCellXF(range.tl.row, range.tl.col)
        // java if (standardFormat.getBorderLeft() == 0)
        // java mask.markDgLeft(false)
        // java mask.markIcvLeft(false)
        // java for(int row = range.tl.row + 1; row <= range.br.row; ++row)
        // java KmoXF compareFormat = this.getCellXF(row, range.tl.col)
        // java if (standardFormat.getBorderLeft() != compareFormat.getBorderLeft() || standardFormat.getLeftBorderPaletteIdx() != compareFormat.getLeftBorderPaletteIdx())
        // java mask.markDgLeft(false)
        // java mask.markIcvLeft(false)
        // java break
        // java if (mask.isDgLeft())
        // java xf.setBorderLeft(standardFormat.getBorderLeft())
        // java xf.setLeftBorderPaletteIdx(standardFormat.getLeftBorderPaletteIdx())
        // java standardFormat = this.getCellXF(range.tl.row, range.br.col)
        // java if (standardFormat.getBorderRight() == 0)
        // java mask.markDgRight(false)
        // java mask.markIcvRight(false)
        // java for(int row = range.tl.row + 1; row <= range.br.row; ++row)
        // java KmoXF compareFormat = this.getCellXF(row, range.br.col)
        // java if (standardFormat.getBorderRight() != compareFormat.getBorderRight() || standardFormat.getRightBorderPaletteIdx() != compareFormat.getRightBorderPaletteIdx())
        // java mask.markDgRight(false)
        // java mask.markIcvRight(false)
        // java break
        // java if (mask.isDgRight())
        // java xf.setBorderRight(standardFormat.getBorderRight())
        // java xf.setRightBorderPaletteIdx(standardFormat.getRightBorderPaletteIdx())
        // java if (range.height() == 1)
        // java mask.markInsideHorizoncal(false)
        // java boolean isWholeRow = range.width() >= this.getMaxColumns()
        // java KmoXF firstCellFormat = this.getCellXF(range.tl.row, range.tl.col)
        // java KmoXF lastCellFormat = this.getCellXF(range.br.row, range.tl.col)
        // java if (firstCellFormat.getBorderBottom() == 0)
        // java mask.markInsideHorizoncal(false)
        // java if (lastCellFormat.getBorderTop() == 0)
        // java mask.markInsideHorizoncal(false)
        // java for(int row = range.tl.row; row <= range.br.row && mask.isInsideHorizoncal(); ++row)
        // java int col = range.tl.col
        // java while(col <= range.br.col)
        // java KmoXF compareFormat = this.getCellXF(row, col)
        // java if (row == range.br.row || firstCellFormat.getBorderBottom() == compareFormat.getBorderBottom() && firstCellFormat.getBottomBorderPaletteIdx() == compareFormat.getBottomBorderPaletteIdx())
        // java if (row == range.tl.row || lastCellFormat.getBorderTop() == compareFormat.getBorderTop() && lastCellFormat.getTopBorderPaletteIdx() == compareFormat.getTopBorderPaletteIdx())
        // java if (isWholeRow)
        // java break
        // java ++col
        // java continue
        // java mask.markInsideHorizoncal(false)
        // java break
        // java mask.markInsideHorizoncal(false)
        // java break
        // java if (range.width() == 1)
        // java mask.markInsideVertical(false)
        // java boolean isWholeCol = range.height() >= this.getMaxRows()
        // java KmoXF firstCellFormat = this.getCellXF(range.tl.row, range.tl.col)
        // java KmoXF lastCellFormat = this.getCellXF(range.tl.row, range.br.col)
        // java if (firstCellFormat.getBorderRight() == 0)
        // java mask.markInsideVertical(false)
        // java if (lastCellFormat.getBorderLeft() == 0)
        // java mask.markInsideVertical(false)
        // java for(int col = range.tl.col; col <= range.br.col && mask.isInsideVertical(); ++col)
        // java int row = range.tl.row
        // java while(row <= range.br.row)
        // java KmoXF compareFormat = this.getCellXF(row, col)
        // java if (col == range.br.col || firstCellFormat.getBorderRight() == compareFormat.getBorderRight() && firstCellFormat.getRightBorderPaletteIdx() == compareFormat.getRightBorderPaletteIdx())
        // java if (col == range.tl.col || lastCellFormat.getBorderLeft() == compareFormat.getBorderLeft() && lastCellFormat.getLeftBorderPaletteIdx() == compareFormat.getLeftBorderPaletteIdx())
        // java if (isWholeCol)
        // java break
        // java ++row
        // java continue
        // java mask.markInsideVertical(false)
        // java break
        // java mask.markInsideVertical(false)
        // java break
        // java standardFormat = this.getCellXF(range.tl.row, range.tl.col)
        // java if (standardFormat.getDiag() != 0 && standardFormat.getAdtlDiagLineStyle() != 0)
        // java if (standardFormat.getDiag() == 1)
        // java mask.markDgDiagUp(false)
        // java mask.markIcvDiagUp(false)
        // java else if (standardFormat.getDiag() == 2)
        // java mask.markDgDiagDown(false)
        // java mask.markIcvDiagDown(false)
        // java boolean hasDiag = true
        // java for(int row = range.tl.row; row <= range.br.row; ++row)
        // java for(int col = range.tl.col; col <= range.br.col; ++col)
        // java KmoXF cellFormat = this.getCellXF(row, col)
        // java if (cellFormat.getDiag() != standardFormat.getDiag() || cellFormat.getAdtlDiag() != standardFormat.getAdtlDiag())
        // java mask.markDgDiagDown(false)
        // java mask.markDgDiagUp(false)
        // java mask.markIcvDiagDown(false)
        // java mask.markIcvDiagUp(false)
        // java hasDiag = false
        // java break
        // java if (!hasDiag)
        // java break
        // java if (hasDiag)
        // java xf.setDiag(standardFormat.getDiag())
        // java xf.setAdtlDiagLineStyle(standardFormat.getAdtlDiagLineStyle())
        // java xf.setAdtlDiag(standardFormat.getAdtlDiag())
        // java mask.markDgDiagDown(false)
        // java mask.markDgDiagUp(false)
        // java mask.markIcvDiagDown(false)
        // java mask.markIcvDiagUp(false)
        */
    }

    // java private void compareDiffOfXF(KmoXF xf1, KmoXF xf2, KmoXFMask mask) {
    pub fn compareDiffOfXF(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (xf1.getAlignment() != xf2.getAlignment())
        // java mask.markAlcH(false)
        // java if (xf1.getVerticalAlignment() != xf2.getVerticalAlignment())
        // java mask.markAlcV(false)
        // java if (xf1.getIndent() != xf2.getIndent())
        // java mask.markIndent(false)
        // java if (xf1.getShrinkToFit() != xf2.getShrinkToFit())
        // java mask.markShrinkToFit(false)
        // java if (xf1.getWrapText() != xf2.getWrapText())
        // java mask.markWrap(false)
        // java if (xf1.getRotation() != xf2.getRotation())
        // java mask.markTrot(false)
        // java if (xf1.getReadingOrder() != xf2.getReadingOrder())
        // java mask.markReadingOrder(false)
        // java String xf1NumFmt = xf1.getNumFormat()
        // java String xf2NumFmt = xf2.getNumFormat()
        // java if (xf1NumFmt != null && !xf1NumFmt.equals(xf2NumFmt))
        // java mask.markNumfmt(false)
        // java else if (xf1NumFmt == null && xf2NumFmt != null)
        // java mask.markNumfmt(false)
        // java KmoFont font1 = xf1.getFont()
        // java KmoFont font2 = xf2.getFont()
        // java if (font1.getBoldWeight() != font2.getBoldWeight())
        // java mask.markFontBold(false)
        // java if (font1.getCharset() != font2.getCharset())
        // java mask.markFontCharSet(false)
        // java if (font1.getColorPaletteIndex() != font2.getColorPaletteIndex())
        // java mask.markFontIcv(false)
        // java if (font1.getFamily() != font2.getFamily())
        // java mask.markFontFamily(false)
        // java if (font1.getFontHeight() != font2.getFontHeight())
        // java mask.markFontHeight(false)
        // java if (!font1.getFontName().equals(font2.getFontName()))
        // java mask.markFontHeight(false)
        // java if (font1.getSuperSubScript() != font2.getSuperSubScript())
        // java mask.markFontSSS(false)
        // java if (font1.getUnderline() != font2.getUnderline())
        // java mask.markFontUls(false)
        // java if (font1.isItalic() != font2.isItalic())
        // java mask.markFontItalic(false)
        // java if (xf1.getAdtlFillPattern() != xf2.getAdtlFillPattern())
        // java mask.markFillStyle(false)
        // java mask.markFillBack(false)
        // java mask.markFillFore(false)
        // java if (xf1.getFillBackground() != xf2.getFillBackground())
        // java mask.markFillBack(false)
        // java if (xf1.getFillForeground() != xf2.getFillForeground())
        // java mask.markFillFore(false)
        // java if (xf1.isHidden() != xf2.isHidden())
        // java mask.markHidden(false)
        // java if (xf1.isLocked() != xf2.isLocked())
        // java mask.markLock(false)
        */
    }

    // java public int[][] getRangeCellsIXF(KmoRect srcRange) {
    pub fn getRangeCellsIXF(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int srcH = srcRange.height()
        // java int srcW = srcRange.width()
        // java int[][] srcCells = new int[srcH][srcW]
        // java for(CellIterator it = this.getCellIterator(srcRange); it.hasNext(); srcCells[it.row() - srcRange.tl.row][it.col() - srcRange.tl.col] = KmoCellData.getIXF(it.iter.data()))
        // java it.next()
        // java for(int row = srcRange.tl.row; row <= srcRange.br.row; ++row)
        // java if (this.isRowFormatted(row))
        // java int r = row - srcRange.tl.row
        // java int rowXFIndex = this.getRowXFIndex(row)
        // java for(int c = 0; c < srcW; ++c)
        // java if (srcCells[r][c] == 0)
        // java srcCells[r][c] = rowXFIndex
        // java for(int col = srcRange.tl.col; col <= srcRange.br.col; ++col)
        // java if (this.isColFormatted(col))
        // java int c = col - srcRange.tl.col
        // java int colXFIndex = this.getColXFIndex(col)
        // java for(int r = 0; r < srcH; ++r)
        // java if (srcCells[r][c] == 0)
        // java srcCells[r][c] = colXFIndex
        // java return srcCells
        */
    }

    // java public void recalcCell(final int row, final int col) {
    pub fn recalcCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java final CalculationChain cc = this.parent.getCalculationChain()
        // java if (cc != null)
        // java boolean isSynCalc = this.parent.isSynCalc()
        // java if (!isSynCalc)
        // java cc.calculate(new CalcChain.ICalculationCallback()
        // java if (false) public void onStart()
        // java cc.markRecalculation(KmoSheet.this.getSheetIndex(), row, col)
        // java if (false) public void onEnd()
        // java KmoSheet.this.parent.areaWatcher().postContentChangedEvent(KmoSheet.this, new KmoRect(row, col, row, col))
        // java KmoSheet.this.getBook().getActiveSheet().changeListenerGroup().sheetContentsDidFinishChanging()
        // java catch (NullPointerException var2)
        // java cc.markRecalculation(this.getSheetIndex(), row, col)
        // java cc.synCalculate()
        */
    }

    // java public void setCellNumfmt(int row, int col, String numfmtCode) {
    pub fn setCellNumfmt(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoXF oldXF = this.getCellXF(row, col)
        // java KmoXFMask mask = new KmoXFMask()
        // java mask.markNumfmt(true)
        // java KmoXF opXF = KmoXF.shareEmptyCellXF()
        // java opXF = KmoXF.createXF(opXF)
        // java opXF.setNumFormat(numfmtCode)
        // java this.setCellIXF(row, col, this.parent.getXFManager().getNewIXF(oldXF, opXF, mask))
        */
    }

    // java public void replaceCellStringValue(int row, int col, String value) {
    pub fn replaceCellStringValue(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java CellTypeInfo cellTypeInfo = new CellTypeInfo()
        // java this.getCellValueType(row, col, cellTypeInfo)
        // java String text = ""
        // java if (2 == cellTypeInfo.cellType)
        // java text = this.getCellStringValue(row, col)
        // java KmoCellData cellData = this.getCellData(row, col)
        // java cellData.formulaType = 0
        // java if (text.length() != 0 && text.length() == value.length() && cellTypeInfo.hasRun)
        // java UnicodeString unicodeString = this.parent.getCellValueStore().runsStringValue(cellData.id)
        // java UnicodeString runString = (UnicodeString)unicodeString.clone()
        // java runString.setString(value)
        // java cellData = this.setRunsCellData(runString, row, col)
        // java cellData.id = this.parent.getCellValueStore().newStringValue(value)
        // java if (cellData.ixf == 0)
        // java cellData.ixf = 15
        // java this.setCellData(row, col, cellData)
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java this.parent.areaWatcher().postContentChangedEvent(this, row, col)
        */
    }

    // java public void modifyFontName(String fontName, int pos, int row, int col) {
    pub fn modifyFontName(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (pos >= 0)
        // java CellTypeInfo cellTypeInfo = new CellTypeInfo()
        // java this.getCellValueType(row, col, cellTypeInfo)
        // java String text = ""
        // java List<FormatRun> runs = null
        // java if (2 == cellTypeInfo.cellType)
        // java text = this.getCellStringValue(row, col)
        // java UnicodeString src = null
        // java int firstEndPos
        // java if (cellTypeInfo.hasRun)
        // java KmoCellData data = this.getCellData(row, col)
        // java src = this.getCellValueStore().runsStringValue(data.id)
        // java runs = src.getFormatRuns()
        // java firstEndPos = ((FormatRun)runs.get(0)).getCharacterPos()
        // java firstEndPos = text.length()
        // java boolean modify = false
        // java KmoXF oldXF = this.getCellXF(row, col)
        // java if (pos < firstEndPos)
        // java KmoXF xf = KmoXF.shareEmptyCellXF()
        // java xf = KmoXF.createXF(xf)
        // java KmoXFMask mask = new KmoXFMask()
        // java short fontIndex = this.createFont(fontName, xf.getFontIndex())
        // java xf.setFontIndex(fontIndex)
        // java xf.setFont(this.parent.getXFManager().getFont(xf.getFontIndex()))
        // java mask.markFontName(true)
        // java if (pos == 0)
        // java int newIXF = this.parent.getXFManager().getNewIXF(oldXF, xf, mask)
        // java this.setCellIXF(row, col, newIXF)
        // java UnicodeString runsString = this.splitUnicodeString(text, pos, src, fontIndex, oldXF.getFontIndex())
        // java if (!cellTypeInfo.hasRun && runsString != null)
        // java this.setRunsCellData(runsString, row, col)
        // java modify = true
        // java int index = this.findFormatRun(pos, runs)
        // java short fontIndex = this.createFont(fontName, src.getFormatRun(index).getFontIndex())
        // java this.splitUnicodeString(text, pos, src, fontIndex, oldXF.getFontIndex())
        // java if (modify)
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        */
    }

    // java private KmoCellData setRunsCellData(UnicodeString runsString, int row, int col) {
    pub fn setRunsCellData(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int id = this.getCellValueStore().newRunStringValue(runsString)
        // java KmoCellData data = this.getCellData(row, col)
        // java data.id = id
        // java this.setCellData(row, col, data)
        // java return data
        */
    }

    // java private int findFormatRun(int pos, List<FormatRun> runs) {
    pub fn findFormatRun(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int index = -1
        // java int runsSize = runs != null ? runs.size() : 0
        // java for(int i = runsSize - 1; i >= 0; --i)
        // java FormatRun run = (FormatRun)runs.get(i)
        // java int startPos = run.getCharacterPos()
        // java if (pos >= startPos)
        // java index = i
        // java break
        // java return index
        */
    }

    // java private UnicodeString splitUnicodeString(String text, int pos, UnicodeString src, short fontIndex, short cellFontIndex) {
    pub fn splitUnicodeString(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (src == null)
        // java src = new UnicodeString(text)
        // java int runsSize = src.getFormatRunCount()
        // java int index = this.findFormatRun(pos, src.getFormatRuns())
        // java if (index < 0)
        // java if (text.length() == 1)
        // java return null
        // java int nextIndex = this.findFormatRun(pos + 1, src.getFormatRuns())
        // java if (pos > 0)
        // java FormatRun formatRun = new FormatRun((short)pos, fontIndex)
        // java src.addFormatRun(formatRun)
        // java if (pos + 1 < text.length() && nextIndex < 0)
        // java FormatRun formatRun = new FormatRun((short)(pos + 1), cellFontIndex)
        // java src.addFormatRun(formatRun)
        // java FormatRun indexRun = src.getFormatRun(index)
        // java FormatRun nextRun = index + 1 < runsSize ? src.getFormatRun(index + 1) : null
        // java short indexFontIndex = indexRun.getFontIndex()
        // java if (indexRun.getCharacterPos() == pos)
        // java indexRun.setFontIndex(fontIndex)
        // java FormatRun formatRun = new FormatRun((short)pos, fontIndex)
        // java src.addFormatRun(formatRun)
        // java if (nextRun != null && pos + 1 < nextRun.getCharacterPos() || nextRun == null && pos + 1 < text.length())
        // java FormatRun formatRun1 = new FormatRun((short)(pos + 1), indexFontIndex)
        // java src.addFormatRun(formatRun1)
        // java return src
        */
    }

    // java private short createFont(String fontName, int fontIndex) {
    pub fn createFont(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoXFManager xfMgr = this.getBook().getXFManager()
        // java KmoFont defaultFont = xfMgr.getFont(fontIndex)
        // java KmoFont font = KmoFont.createFont(defaultFont)
        // java font.setFontName(fontName)
        // java KmoFont find = xfMgr.findFontByProp(font)
        // java if (find == null)
        // java find = font
        // java return xfMgr.checkFontIndex(find)
        */
    }

    // java public void destroy() {
    pub fn destroy(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.clearChangeEvents()
        // java this.sheetDrawingAgg.destroy()
        // java this.shared.gridSheetData = null
        // java this.sharedFormulaMap.clear()
        */
    }

    // java public String toString() {
    pub fn toString(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheetInfo.name
        */
    }

    // java public int setCellRawOnlyNumFmt(int row, int col, String raw) {
    pub fn setCellRawOnlyNumFmt(&self,row:i32,col:i32,raw:Option<&str>)->i32 {
        let raw16=raw.map(|s|s.encode_utf16().collect::<Vec<_>>());let mut constRecognisor=SHEET_CONST_RECOGNISOR.lock().unwrap();
        // java RecogniseResult rs = constRecognisor.recognise(raw);
        let mut rs=constRecognisor.recognise(raw16.as_deref()).unwrap_or_else(|_|panic!("ConstRecognisor evaluation failure"));
        // java if (null == rs) {
        if rs.is_none(){
            // java constRecognisor.setForceText(true); rs = constRecognisor.recognise(raw);
            constRecognisor.setForceText(true);rs=constRecognisor.recognise(raw16.as_deref()).unwrap_or_else(|_|panic!("ConstRecognisor evaluation failure"));
        }
        // java int numfmt = 0;
        let mut numfmt=0;
        // java if (null != rs) {
        if let Some(rs)=rs{
            // java KmoRect kmoRect = new KmoRect(row, col, row, col); this.getExpFmlManager().unlinkSharedFormula(kmoRect);
            let kmoRect=KmoRect::KmoRect__bounds(row,col,row,col);self.getExpFmlManager().borrow().unlinkSharedFormula__range(&kmoRect);
            // java this._setCellRgnResult(row, col, raw, rs, false); numfmt = rs.getBuiltInNumfmtIdx();
            self._setCellRgnResult(row,col,raw,&rs,false);numfmt=rs.getBuiltInNumfmtIdx();
        }
        // java constRecognisor.setForceText(false); return numfmt;
        constRecognisor.setForceText(false);numfmt
    }

    // java public void initConstRecognisor() {
    pub fn initConstRecognisor(&self) {
        // java NumfmtCategory oldNumfmtCat = NumfmtCategory.getNumfmtCat((short)15, "General");
        let oldNumfmtCat=crate::cn::wps::et::ss::constant::NumfmtCategory::NumfmtCategory::getNumfmtCat(15,"General");let mut constRecognisor=SHEET_CONST_RECOGNISOR.lock().unwrap();
        // java constRecognisor.setForceText(false);
        constRecognisor.setForceText(false);
        // java constRecognisor.setUsingDate1904(this.getBook().isUsing1904());
        constRecognisor.setUsingDate1904(self.getBook().isUsing1904());
        // java constRecognisor.setPreferedNumfmtCategory(oldNumfmtCat);
        constRecognisor.setPreferedNumfmtCategory(oldNumfmtCat);
        // java constRecognisor.setKeepTailZero(true);
        constRecognisor.setKeepTailZero(true);
    }

    // java public EvaluationPresent getPresent() {
    pub fn getPresent(&self)->Rc<crate::cn::wps::moss::engine::eval::EvaluationPresent::EvaluationPresent>{
        // java return this.evaluationPresent;
        self.evaluationPresent.clone()
    }

    // java public boolean isCellBlank(int row, int col) {
    pub fn isCellBlank(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData cellData = this.getCellData(row, col)
        // java return cellData.isNull() || cellData.isBlank()
        */
    }

    // java public boolean isKmoRectBlank(KmoRect rect) {
    pub fn isKmoRectBlank(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java CellIterator cellIterator = this.getCellIterator(rect)
        // java while(cellIterator.hasNext())
        // java cellIterator.next()
        // java if (!this.isCellBlank(cellIterator.row(), cellIterator.col()))
        // java return false
        // java return true
        */
    }

    // java public KmoRect getAdjacentDataRect() {
    pub fn getAdjacentDataRect(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.getAdjacentDataRect((KmoRect)null)
        */
    }

    // java public KmoRect getAdjacentDataRect(KmoRect curRect) {
    pub fn getAdjacentDataRect__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java String path = this.getBook().getFilePath()
        // java String pathExtension = StringUtil.pathExtension(path).toUpperCase()
        // java KmoRect selection = curRect == null ? this.getSelectionRect() : curRect
        // java CellRangeAddress range = ExpandHelper.rect2range(selection)
        // java String csvType = "CSV"
        // java boolean expandSuccess
        // java if (csvType.equals(pathExtension))
        // java expandSuccess = ExpandHelper.expandRange(range, this, new KmoRect(0, 0, this.getMaxRows() - 1, this.getMaxColumns() - 1))
        // java KmoRect usedRange = this.getUsedRange()
        // java if (usedRange.intersects(selection))
        // java expandSuccess = ExpandHelper.expandRange(range, this)
        // java expandSuccess = ExpandHelper.expandRange(range, this, new KmoRect(0, 0, this.getMaxRows() - 1, this.getMaxColumns() - 1))
        // java KmoRect rect
        // java if (expandSuccess)
        // java rect = ExpandHelper.range2rect(range)
        // java rect = new KmoRect(0, 0, this.getMaxRows() - 1, this.getMaxColumns() - 1)
        // java return this.gridData.mergedDele().expandBounds(rect)
        */
    }

    // java private KmoRect computeKmoRect(KmoRect rect, boolean initial, int minRow, int minCol, int maxRow, int maxCol, Map<KmoRect, Boolean> mergedRectMap) {
    pub fn computeKmoRect(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect newRect = this.modifyRect(rect, minRow, minCol, maxRow, maxCol)
        // java boolean find = false
        // java if (newRect.tl.row < rect.tl.row)
        // java find = this.findDataInTopRow(mergedRectMap, newRect)
        // java if (newRect.tl.col < rect.tl.col)
        // java find = this.findDataInLeftCol(mergedRectMap, newRect) || find
        // java if (newRect.br.row > rect.br.row)
        // java find = this.findDataInBottomRow(mergedRectMap, newRect) || find
        // java if (newRect.br.col > rect.br.col)
        // java find = this.findDataInRightCol(mergedRectMap, newRect) || find
        // java if (find)
        // java return this.computeKmoRect(newRect, false, minRow, minCol, maxRow, maxCol, mergedRectMap)
        // java return initial ? new KmoRect(0, 0, this.getMaxRows() - 1, this.getMaxColumns() - 1) : rect
        */
    }

    // java private KmoRect modifyRect(KmoRect rect, int minRow, int minCol, int maxRow, int maxCol) {
    pub fn modifyRect(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int topRow = rect.tl.row
        // java if (topRow > minRow)
        // java --topRow
        // java int leftCol = rect.tl.col
        // java if (leftCol > minCol)
        // java --leftCol
        // java int bottomRow = rect.br.row
        // java if (bottomRow < maxRow)
        // java ++bottomRow
        // java int rightCol = rect.br.col
        // java if (rightCol < maxCol)
        // java ++rightCol
        // java return new KmoRect(topRow, leftCol, bottomRow, rightCol)
        */
    }

    // java private boolean findDataInTopRow(Map<KmoRect, Boolean> mergedRectMap, KmoRect newRect) {
    pub fn findDataInTopRow(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean find = false
        // java for(int i = newRect.tl.col; i <= newRect.br.col; ++i)
        // java KmoRect rect = this.gridData.mergedDele().getMergedCells(newRect.tl.row, i)
        // java if (rect == null)
        // java if (!this.isCellBlank(newRect.tl.row, i))
        // java find = true
        // java break
        // java else if (this.findDataInMergeRect(mergedRectMap, rect))
        // java find = true
        // java break
        // java if (!find)
        // java ++newRect.tl.row
        // java return find
        */
    }

    // java private boolean findDataInLeftCol(Map<KmoRect, Boolean> mergedRectMap, KmoRect newRect) {
    pub fn findDataInLeftCol(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean find = false
        // java for(int i = newRect.tl.row; i <= newRect.br.row; ++i)
        // java KmoRect rect = this.gridData.mergedDele().getMergedCells(i, newRect.tl.col)
        // java if (rect == null)
        // java if (!this.isCellBlank(i, newRect.tl.col))
        // java find = true
        // java break
        // java else if (this.findDataInMergeRect(mergedRectMap, rect))
        // java find = true
        // java break
        // java if (!find)
        // java ++newRect.tl.col
        // java return find
        */
    }

    // java private boolean findDataInBottomRow(Map<KmoRect, Boolean> mergedRectMap, KmoRect newRect) {
    pub fn findDataInBottomRow(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean find = false
        // java for(int i = newRect.tl.col; i <= newRect.br.col; ++i)
        // java KmoRect rect = this.gridData.mergedDele().getMergedCells(newRect.br.row, i)
        // java if (rect == null)
        // java if (!this.isCellBlank(newRect.br.row, i))
        // java find = true
        // java break
        // java else if (this.findDataInMergeRect(mergedRectMap, rect))
        // java find = true
        // java break
        // java if (!find)
        // java --newRect.br.row
        // java return find
        */
    }

    // java private boolean findDataInRightCol(Map<KmoRect, Boolean> mergedRectMap, KmoRect newRect) {
    pub fn findDataInRightCol(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean find = false
        // java for(int i = newRect.tl.row; i <= newRect.br.row; ++i)
        // java KmoRect rect = this.gridData.mergedDele().getMergedCells(i, newRect.br.col)
        // java if (rect == null)
        // java if (!this.isCellBlank(i, newRect.br.col))
        // java find = true
        // java break
        // java else if (this.findDataInMergeRect(mergedRectMap, rect))
        // java find = true
        // java break
        // java if (!find)
        // java --newRect.br.col
        // java return find
        */
    }

    // java private boolean findDataInMergeRect(Map<KmoRect, Boolean> mergedRectMap, KmoRect mergeRect) {
    pub fn findDataInMergeRect(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean ret = false
        // java if (mergedRectMap.containsKey(mergeRect))
        // java if (!(Boolean)mergedRectMap.get(mergeRect))
        // java ret = true
        // java boolean blank = this.isKmoRectBlank(mergeRect)
        // java mergedRectMap.put(mergeRect, blank)
        // java if (!blank)
        // java ret = true
        // java return ret
        */
    }

    // java public KmoRect moveUpToDataBorder(boolean isCtrlShift) {
    pub fn moveUpToDataBorder(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int activeRow = this.getSelection().activeRow()
        // java if (isCtrlShift)
        // java activeRow = this.getSelectionRect().tl.row
        // java if (activeRow == 0)
        // java return null
        // java int activeCol = this.getSelection().activeCol()
        // java HashMap<Integer, Boolean> tmpMap = new LinkedHashMap(3)
        // java int dstRow
        // java for(dstRow = activeRow - 1; dstRow > 0; --dstRow)
        // java boolean preRowBlank = this.isCellBlankBaseRow(tmpMap, dstRow + 1, activeCol)
        // java boolean curRowBlank = this.isCellBlankBaseRow(tmpMap, dstRow, activeCol)
        // java boolean nextRowBlank = this.isCellBlankBaseRow(tmpMap, dstRow - 1, activeCol)
        // java boolean downBorder = !curRowBlank && preRowBlank
        // java boolean upBorder = !curRowBlank && nextRowBlank
        // java if (downBorder || upBorder)
        // java break
        // java tmpMap.remove(dstRow + 1)
        // java return new KmoRect(dstRow, activeCol, dstRow, activeCol)
        */
    }

    // java public KmoRect moveDownToDataBorder(boolean isCtrlShift) {
    pub fn moveDownToDataBorder(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int activeRow = this.getSelection().activeRow()
        // java if (isCtrlShift)
        // java activeRow = this.getSelectionRect().br.row
        // java if (activeRow == this.getMaxRows() - 1)
        // java return null
        // java int activeCol = this.getSelection().activeCol()
        // java HashMap<Integer, Boolean> tmpMap = new LinkedHashMap(3)
        // java int dstRow
        // java for(dstRow = activeRow + 1; dstRow < this.getMaxRows() - 1; ++dstRow)
        // java boolean preRowBlank = this.isCellBlankBaseRow(tmpMap, dstRow - 1, activeCol)
        // java boolean curRowBlank = this.isCellBlankBaseRow(tmpMap, dstRow, activeCol)
        // java boolean nextRowBlank = this.isCellBlankBaseRow(tmpMap, dstRow + 1, activeCol)
        // java boolean upBorder = !curRowBlank && preRowBlank
        // java boolean downBorder = !curRowBlank && nextRowBlank
        // java if (downBorder || upBorder)
        // java break
        // java tmpMap.remove(dstRow - 1)
        // java return new KmoRect(dstRow, activeCol, dstRow, activeCol)
        */
    }

    // java public KmoRect moveLeftToDataBorder(boolean isCtrlShift) {
    pub fn moveLeftToDataBorder(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int activeCol = this.getSelection().activeCol()
        // java if (isCtrlShift)
        // java activeCol = this.getSelectionRect().tl.col
        // java if (activeCol == 0)
        // java return null
        // java int activeRow = this.getSelection().activeRow()
        // java HashMap<Integer, Boolean> tmpMap = new LinkedHashMap(3)
        // java int dstCol
        // java for(dstCol = activeCol - 1; dstCol > 0; --dstCol)
        // java boolean preColBlank = this.isCellBlankBaseCol(tmpMap, activeRow, dstCol + 1)
        // java boolean curColBlank = this.isCellBlankBaseCol(tmpMap, activeRow, dstCol)
        // java boolean nextColBlank = this.isCellBlankBaseCol(tmpMap, activeRow, dstCol - 1)
        // java boolean rightBorder = !curColBlank && preColBlank
        // java boolean leftBorder = !curColBlank && nextColBlank
        // java if (rightBorder || leftBorder)
        // java break
        // java tmpMap.remove(dstCol + 1)
        // java return new KmoRect(activeRow, dstCol, activeRow, dstCol)
        */
    }

    // java public KmoRect moveRightToDataBorder(boolean isCtrlShift) {
    pub fn moveRightToDataBorder(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int activeCol = this.getSelection().activeCol()
        // java if (isCtrlShift)
        // java activeCol = this.getSelectionRect().br.col
        // java if (activeCol == this.getMaxColumns() - 1)
        // java return null
        // java int activeRow = this.getSelection().activeRow()
        // java HashMap<Integer, Boolean> tmpMap = new LinkedHashMap(3)
        // java int dstCol
        // java for(dstCol = activeCol + 1; dstCol < this.getMaxColumns() - 1; ++dstCol)
        // java boolean preColBlank = this.isCellBlankBaseCol(tmpMap, activeRow, dstCol - 1)
        // java boolean curColBlank = this.isCellBlankBaseCol(tmpMap, activeRow, dstCol)
        // java boolean nextColBlank = this.isCellBlankBaseCol(tmpMap, activeRow, dstCol + 1)
        // java boolean leftBorder = !curColBlank && preColBlank
        // java boolean rightBorder = !curColBlank && nextColBlank
        // java if (rightBorder || leftBorder)
        // java break
        // java tmpMap.remove(dstCol - 1)
        // java return new KmoRect(activeRow, dstCol, activeRow, dstCol)
        */
    }

    // java private boolean isCellBlankBaseRow(HashMap<Integer, Boolean> tmpMap, int row, int col) {
    pub fn isCellBlankBaseRow(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean blank
        // java if (tmpMap.containsKey(row))
        // java blank = (Boolean)tmpMap.get(row)
        // java blank = this.isCellBlank(row, col)
        // java tmpMap.put(row, blank)
        // java return blank
        */
    }

    // java private boolean isCellBlankBaseCol(HashMap<Integer, Boolean> tmpMap, int row, int col) {
    pub fn isCellBlankBaseCol(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean blank
        // java if (tmpMap.containsKey(col))
        // java blank = (Boolean)tmpMap.get(col)
        // java blank = this.isCellBlank(row, col)
        // java tmpMap.put(col, blank)
        // java return blank
        */
    }

    // java public KmoRect expandUpSelection(KmoRect curRect) {
    pub fn expandUpSelection(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java curRect.copy(this.getSelectionRect())
        // java int activeRow = this.getSelection().activeRow()
        // java KmoRect tmpRect = SharedObjects.kmoRectPool.borrowObject()
        // java tmpRect.set(activeRow, curRect.tl.col, activeRow, curRect.br.col)
        // java KmoRect activeExpand = this.gridData.mergedDele().expandBounds(tmpRect)
        // java int upRow = activeExpand.tl.row
        // java int downRow = activeExpand.br.row
        // java if (downRow < curRect.br.row)
        // java tmpRect.set(curRect.br.row, curRect.tl.col, curRect.br.row, curRect.br.col)
        // java KmoRect brExpand = this.gridData.mergedDele().expandBounds(tmpRect)
        // java if (brExpand.tl.row - 1 >= downRow)
        // java curRect.tl.row = upRow
        // java curRect.br.row = brExpand.tl.row - 1
        // java return curRect
        // java tmpRect = SharedObjects.kmoRectPool.returnObject(tmpRect)
        // java if (curRect.tl.row == 0)
        // java return null
        // java --curRect.tl.row
        // java return this.gridData.mergedDele().expandBounds(curRect)
        */
    }

    // java public KmoRect expandDownSelection(KmoRect curRect) {
    pub fn expandDownSelection(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java curRect.copy(this.getSelectionRect())
        // java int activeRow = this.getSelection().activeRow()
        // java KmoRect tmpRect = SharedObjects.kmoRectPool.borrowObject()
        // java tmpRect.set(activeRow, curRect.tl.col, activeRow, curRect.br.col)
        // java KmoRect activeExpand = this.gridData.mergedDele().expandBounds(tmpRect)
        // java int upRow = activeExpand.tl.row
        // java int downRow = activeExpand.br.row
        // java if (upRow > curRect.tl.row)
        // java tmpRect.set(curRect.tl.row, curRect.tl.col, curRect.tl.row, curRect.br.col)
        // java KmoRect tlExpand = this.gridData.mergedDele().expandBounds(tmpRect)
        // java if (tlExpand.br.row + 1 <= upRow)
        // java curRect.tl.row = tlExpand.br.row + 1
        // java curRect.br.row = downRow
        // java return curRect
        // java tmpRect = SharedObjects.kmoRectPool.returnObject(tmpRect)
        // java if (curRect.br.row == this.getMaxRows() - 1)
        // java return null
        // java ++curRect.br.row
        // java return this.gridData.mergedDele().expandBounds(curRect)
        */
    }

    // java public KmoRect expandLeftSelection(KmoRect curRect) {
    pub fn expandLeftSelection(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java curRect.copy(this.getSelectionRect())
        // java int activeCol = this.getSelection().activeCol()
        // java KmoRect tmpRect = SharedObjects.kmoRectPool.borrowObject()
        // java tmpRect.set(curRect.tl.row, activeCol, curRect.br.row, activeCol)
        // java KmoRect activeExpand = this.gridData.mergedDele().expandBounds(tmpRect)
        // java int leftCol = activeExpand.tl.col
        // java int rightCol = activeExpand.br.col
        // java if (rightCol < curRect.br.col)
        // java tmpRect.set(curRect.tl.row, curRect.br.col, curRect.br.row, curRect.br.col)
        // java KmoRect brExpand = this.gridData.mergedDele().expandBounds(tmpRect)
        // java if (brExpand.tl.col - 1 >= rightCol)
        // java curRect.tl.col = leftCol
        // java curRect.br.col = brExpand.tl.col - 1
        // java return curRect
        // java tmpRect = SharedObjects.kmoRectPool.returnObject(tmpRect)
        // java if (curRect.tl.col == 0)
        // java return null
        // java --curRect.tl.col
        // java return this.gridData.mergedDele().expandBounds(curRect)
        */
    }

    // java public KmoRect expandRightSelection(KmoRect curRect) {
    pub fn expandRightSelection(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java curRect.copy(this.getSelectionRect())
        // java int activeCol = this.getSelection().activeCol()
        // java KmoRect tmpRect = SharedObjects.kmoRectPool.borrowObject()
        // java tmpRect.set(curRect.tl.row, activeCol, curRect.br.row, activeCol)
        // java KmoRect activeExpand = this.gridData.mergedDele().expandBounds(tmpRect)
        // java int leftCol = activeExpand.tl.col
        // java int rightCol = activeExpand.br.col
        // java if (leftCol > curRect.tl.col)
        // java tmpRect.set(curRect.tl.row, curRect.tl.col, curRect.br.row, curRect.tl.col)
        // java KmoRect tlExpand = this.gridData.mergedDele().expandBounds(tmpRect)
        // java if (tlExpand.br.col + 1 <= leftCol)
        // java curRect.tl.col = tlExpand.br.col + 1
        // java curRect.br.col = rightCol
        // java return curRect
        // java tmpRect = SharedObjects.kmoRectPool.returnObject(tmpRect)
        // java if (curRect.br.col == this.getMaxColumns() - 1)
        // java return null
        // java ++curRect.br.col
        // java return this.gridData.mergedDele().expandBounds(curRect)
        */
    }

    // java public void calcSubtotal() {
    pub fn calcSubtotal(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.getBook() != null)
        // java boolean isSynCalc = this.getBook().isSynCalc()
        // java final CalculationChain cc = this.getBook().getCalculationChain()
        // java if (cc != null)
        // java final KmoRect tmpRect = this.fitAndGetUsedRange()
        // java if (!isSynCalc)
        // java cc.calculate(new CalcChain.ICalculationCallback()
        // java if (false) public void onStart()
        // java cc.markSubtotalRecalculation(KmoSheet.this.getSheetIndex(), tmpRect.tl.row, tmpRect.br.row, tmpRect.tl.col, tmpRect.br.col)
        // java if (false) public void onEnd()
        // java KmoSheet.this.changeListeners.sheetContentsDidFinishChanging()
        // java KmoSheet.this.parent.areaWatcher().postContentChangedEvent(KmoSheet.this, tmpRect)
        // java catch (NullPointerException var2)
        // java cc.markSubtotalRecalculation(this.getSheetIndex(), tmpRect.tl.row, tmpRect.br.row, tmpRect.tl.col, tmpRect.br.col)
        // java cc.synCalculate()
        */
    }

    // java private void calcAfterHidden(final KmoRect rect) {
    pub fn calcAfterHidden(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean isSynCalc = this.getBook().isSynCalc()
        // java final CalculationChain cc = this.getBook().getCalculationChain()
        // java if (cc != null)
        // java if (!isSynCalc)
        // java cc.calculate(new CalcChain.ICalculationCallback()
        // java if (false) public void onStart()
        // java cc.markVolatile()
        // java if (false) public void onEnd()
        // java KmoSheet.this.changeListeners.sheetContentsDidFinishChanging()
        // java KmoSheet.this.parent.areaWatcher().postContentChangedEvent(KmoSheet.this, rect)
        // java catch (NullPointerException var2)
        // java cc.markVolatile()
        // java cc.synCalculate()
        */
    }

    // java private String getCustomResult(String numfmtstr, double value) {
    pub fn getCustomResult(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (numfmtstr != null && !numfmtstr.equals("General"))
        // java NumFmtResult formatResult = (NumFmtResult)this.shared.numFormatResult.get()
        // java INumFormatter numFormatter = KmoApp.sharedApp().numFormatter()
        // java if (numFormatter != null)
        // java numFormatter.format(value, numfmtstr, Integer.MAX_VALUE, this.parent.isUsing1904(), formatResult)
        // java return formatResult.numfmtStr()
        // java return null
        */
    }

    // java private boolean isValidKmoRect(KmoRect kmoRect) {
    pub fn isValidKmoRect(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (kmoRect == null)
        // java return false
        // java else if (kmoRect.height() > 0 && kmoRect.width() > 0)
        // java if (kmoRect.br.row <= this.getVersion().getMaxRows() && kmoRect.br.col <= this.getVersion().getMaxColumns())
        // java if (kmoRect.tl.col <= kmoRect.br.col && kmoRect.tl.row <= kmoRect.br.row)
        // java return kmoRect.tl.col >= 0 && kmoRect.tl.row >= 0 && kmoRect.br.col >= 0 && kmoRect.br.row >= 0
        // java return false
        // java return false
        // java return false
        */
    }

    // java public String[][] getSelectionText(KmoRect selection) {
    pub fn getSelectionText(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (!this.isValidKmoRect(selection))
        // java return null
        // java int startRow = selection.tl.row
        // java int startCol = selection.tl.col
        // java int height = selection.height()
        // java int width = selection.width()
        // java ArrayList<KmoRect> mergedRanges = new ArrayList()
        // java this.getGrid().mergedDele().getContainsMergedRanges(selection, mergedRanges)
        // java int mergedRangesSize = mergedRanges.size()
        // java String[][] result = new String[height][width]
        // java for(int i = 0; i < mergedRangesSize; ++i)
        // java KmoRect mergedRect = (KmoRect)mergedRanges.get(i)
        // java int mergedStartRow = mergedRect.tl.row
        // java int mergedStartCol = mergedRect.tl.col
        // java int mergedHeight = mergedRect.height()
        // java int mergedWidth = mergedRect.width()
        // java for(int r = mergedStartRow; r < mergedStartRow + mergedHeight; ++r)
        // java for(int c = mergedStartCol; c < mergedStartCol + mergedWidth; ++c)
        // java result[r - startRow][c - startCol] = this.getFormattedCellValue(mergedStartRow, mergedStartCol)
        // java for(int i = startRow; i < startRow + height; ++i)
        // java for(int j = startCol; j < startCol + width; ++j)
        // java if (result[i - startRow][j - startCol] == null)
        // java result[i - startRow][j - startCol] = this.getFormattedCellValue(i, j)
        // java return result
        */
    }

    // java private KmoRect expandMergeCell(KmoRect initialRect, HashSet<KmoRect> mergeCellSet) {
    pub fn expandMergeCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int minRow = initialRect.tl.row
        // java int minCol = initialRect.tl.col
        // java int maxRow = initialRect.br.row
        // java int maxCol = initialRect.br.col
        // java for(int c = initialRect.tl.col; c <= initialRect.br.col; ++c)
        // java for(int r = initialRect.tl.row; r <= initialRect.br.row; ++r)
        // java KmoRect mergeRect = this.gridData.mergedDele().getMergedCells(r, c)
        // java if (mergeRect != null && !mergeCellSet.contains(mergeRect))
        // java mergeCellSet.add(mergeRect)
        // java if (mergeRect.tl.row < minRow)
        // java minRow = mergeRect.tl.row
        // java if (mergeRect.tl.col < minCol)
        // java minCol = mergeRect.tl.col
        // java if (mergeRect.br.row > maxRow)
        // java maxRow = mergeRect.br.row
        // java if (mergeRect.br.col > maxCol)
        // java maxCol = mergeRect.br.col
        // java KmoRect newRect = new KmoRect(minRow, minCol, maxRow, maxCol)
        // java if (initialRect.equals(newRect))
        // java return initialRect
        // java return this.expandMergeCell(newRect, mergeCellSet)
        */
    }

    // java public boolean isReservedCellImgList() {
    pub fn isReservedCellImgList(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.isSheetVeryHidden() && "WpsReserved_CellImgList".equals(this.name())
        */
    }

    // java public void checkAutoFiler(CellRangeAddress range) throws EmptyRangeException {
    pub fn checkAutoFiler(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.getRangeManager().isCross(0, this, range) || this.getRangeManager().isCross(1, this, range))
        // java throw new EmptyRangeException()
        */
    }

    // java public boolean isInPivotEditorRange(KmoRect affectRange) {
    pub fn isInPivotEditorRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.getRangeManager().isCross(1, this, ExpandHelper.rect2range(affectRange))
        */
    }

    // java public void checkPivotEditorRange(KmoRect affectRange) throws KmoPivotEditException {
    pub fn checkPivotEditorRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.isInPivotEditorRange(affectRange))
        // java throw new KmoPivotEditException()
        */
    }

    // java public void checkEditorRange(KmoRect affectRange, int errorCode) throws KmoPivotEditException, KmoTableOpFailedException {
    pub fn checkEditorRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.getRangeManager().isCross(0, this, ExpandHelper.rect2range(affectRange)))
        // java throw new KmoTableOpFailedException(errorCode)
        // java this.checkPivotEditorRange(affectRange)
        */
    }

    // java public void checkEditorAffectRange(KmoRect affectRange, int errorCode) throws KmoPivotEditException, KmoTableOpFailedException {
    pub fn checkEditorAffectRange(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.getRangeManager().isCrossAffectedArea(0, this, ExpandHelper.rect2range(affectRange)))
        // java throw new KmoTableOpFailedException(errorCode)
        // java else if (this.getRangeManager().isCrossAffectedArea(1, this, ExpandHelper.rect2range(affectRange)))
        // java throw new KmoPivotEditException()
        */
    }

    // java void setValueForSlashHeader(int row, int col, String raw) {
    pub fn setValueForSlashHeader(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (raw == null)
        // java raw = ""
        // java KmoCellValueStore cellValueStore = this.getCellValueStore()
        // java KmoCellData cellData = this.getCellData(row, col)
        // java UnicodeString unicodeString = cellValueStore.runsStringValue(cellData.id)
        // java if (unicodeString != null)
        // java unicodeString = (UnicodeString)unicodeString.clone()
        // java String[] strs = raw.trim().split("\\s+", 2)
        // java String colTitle = strs.length > 0 ? strs[0] : ""
        // java String rowTitle = strs.length > 1 ? strs[1] : ""
        // java unicodeString.getFormatRun(unicodeString.getFormatRunCount() - 1).setCharPos((short)colTitle.length())
        // java unicodeString.setString(colTitle + " " + rowTitle)
        // java cellData.id = cellValueStore.newRunStringValue(unicodeString)
        // java this.setCellData(row, col, cellData)
        // java this.afterSetSlashHeader(new KmoRect(row, col, row, col))
        */
    }

    // java public String[] getSlashHeaderTitle(KmoRect kmoRect) {
    pub fn getSlashHeaderTitle(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int row = kmoRect.tl.row
        // java int col = kmoRect.tl.col
        // java if (!this.isSlashHeader(row, col))
        // java return null
        // java String[] strs = this.getCellStringValue(row, col).trim().split("\\s+", 2)
        // java String colTitle = strs.length > 0 ? strs[0] : ""
        // java String rowTitle = strs.length > 1 ? strs[1] : ""
        // java return new String[]{rowTitle, colTitle}
        */
    }

    // java public boolean isSlashHeader(int row, int col) {
    pub fn isSlashHeader(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellValueStore cellValueStore = this.getCellValueStore()
        // java KmoCellData cellData = this.getCellData(row, col)
        // java if (!cellValueStore.isRunsString2(cellData.id))
        // java return false
        // java UnicodeString unicodeString = cellValueStore.runsStringValue(cellData.id)
        // java if (unicodeString == null)
        // java return false
        // java List<FormatRun> formatRuns = unicodeString.getFormatRuns()
        // java if (formatRuns != null && formatRuns.size() != 0 && formatRuns.size() <= 2)
        // java KmoXF xf = this.getCellXF(row, col)
        // java if (xf.getDiag() != 1)
        // java return false
        // java KmoFont font = this.getBook().getXFManager().getFont(((FormatRun)formatRuns.get(0)).getFontIndex())
        // java if (formatRuns.size() == 1)
        // java KmoFont font2 = xf.getFont()
        // java return font.getSuperSubScript() == 1 && font2 != null && font2.getSuperSubScript() == 2
        // java else if (formatRuns.size() != 2)
        // java return true
        // java KmoFont font2 = this.getBook().getXFManager().getFont(((FormatRun)formatRuns.get(1)).getFontIndex())
        // java return font.getSuperSubScript() == 2 && font2.getSuperSubScript() == 1
        // java return false
        */
    }

    // java public void setSlashHeader(KmoRect range, String rowTitle, String colTitle) {
    pub fn setSlashHeader(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (rowTitle == null)
        // java rowTitle = ""
        // java if (colTitle == null)
        // java colTitle = ""
        // java KmoXFManager xfManager = this.getBook().getXFManager()
        // java KmoXF oldXf = this.getCellXF(range.tl.row, range.tl.col)
        // java KmoFont font = oldXf.getFont()
        // java if (font == null)
        // java font = xfManager.getFont(0)
        // java KmoFont subFont = new KmoFont()
        // java subFont.completeCopy(font)
        // java subFont.setFontHeight((short)320)
        // java subFont.setSuperSubScript((short)2)
        // java KmoFont superFont = new KmoFont()
        // java superFont.completeCopy(font)
        // java superFont.setFontHeight((short)320)
        // java superFont.setSuperSubScript((short)1)
        // java KmoXFMask xfMask = new KmoXFMask()
        // java xfMask.markDgDiagDown(true)
        // java xfMask.markAllFont(true)
        // java KmoXF xf = KmoXF.shareEmptyCellXF()
        // java xf = KmoXF.createXF(xf)
        // java xf.setDiag((byte)1)
        // java xf.setAdtlDiagLineStyle((short)1)
        // java xf.setAdtlDiag(64)
        // java xf.setFont(subFont)
        // java this.getGrid().getMatrix().addChangeRange(range)
        // java this.changeListeners.startMergeEvents()
        // java int newIXF = this.parent.getXFManager().getNewIXF(oldXf, xf, xfMask)
        // java this.setCellIXF(range.tl.row, range.tl.col, newIXF)
        // java KmoXF newXF = xfManager.getXF(newIXF)
        // java short subFontIndex = 0
        // java if (newXF != null)
        // java subFontIndex = newXF.getFontIndex()
        // java String strValue = colTitle + " " + rowTitle
        // java List<FormatRun> runs = new ArrayList()
        // java runs.add(new FormatRun((short)0, subFontIndex))
        // java runs.add(new FormatRun((short)colTitle.length(), (short)xfManager.addFont(superFont)))
        // java String newValue = EscapeUtil.unEscapeControlCharacter(strValue)
        // java UnicodeString unicodeStr = new UnicodeString(newValue)
        // java unicodeStr.resetFormatRuns(runs)
        // java this.setRunsCellData(unicodeStr, range.tl.row, range.tl.col)
        // java this.changeListeners.finishMergeEvents()
        // java this.afterSetSlashHeader(range)
        */
    }

    // java private void afterSetSlashHeader(final KmoRect range) {
    pub fn afterSetSlashHeader(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java final CalculationChain cc = this.parent.getCalculationChain()
        // java final int sheetIndex = this.getSheetIndex()
        // java this.parent.areaWatcher().startMergeEvents()
        // java if (cc != null)
        // java boolean isSynCalc = this.parent.isSynCalc()
        // java cc.interrupt()
        // java cc.registerFormula(sheetIndex, range.tl.row, range.br.row, range.tl.col, range.br.col)
        // java if (!isSynCalc)
        // java cc.calculate(new CalcChain.ICalculationCallback()
        // java if (false) public void onStart()
        // java cc.markRecalculation(sheetIndex, range.tl.row, range.br.row, range.tl.col, range.br.col)
        // java if (false) public void onEnd()
        // java KmoSheet.this.parent.setDirty(true)
        // java KmoSheet.this.parent.getActiveSheet().changeListenerGroup().sheetContentsDidFinishChanging()
        // java KmoSheet.this.parent.areaWatcher().postContentChangedEvent(KmoSheet.this, new KmoRect(range))
        // java cc.markRecalculation(sheetIndex, range.tl.row, range.br.row, range.tl.col, range.br.col)
        // java cc.synCalculate()
        // java this.parent.setDirty(true)
        // java this.changeListeners.sheetContentsDidFinishChanging()
        // java this.parent.areaWatcher().postContentChangedEvent(this, new KmoRect(range))
        // java this.parent.areaWatcher().finishMergeEvents()
        */
    }

    // java public void backFreezeParams(int[] topRowCol, float[] topOffset, float[] mainOffset) {
    pub fn backFreezeParams(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (topRowCol != null)
        // java this.freezeTopRowCol[0] = topRowCol[0]
        // java this.freezeTopRowCol[1] = topRowCol[1]
        // java if (topOffset != null)
        // java this.topOffset[0] = topOffset[0]
        // java this.topOffset[1] = topOffset[1]
        // java if (mainOffset != null)
        // java this.mainOffset[0] = mainOffset[0]
        // java this.mainOffset[1] = mainOffset[1]
        */
    }

    // java public void recoveryFreezeParams(int[] topRowCol, float[] topOffset, float[] mainOffset) {
    pub fn recoveryFreezeParams(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java topRowCol[0] = this.freezeTopRowCol[0]
        // java topRowCol[1] = this.freezeTopRowCol[1]
        // java topOffset[0] = this.topOffset[0]
        // java topOffset[1] = this.topOffset[1]
        // java mainOffset[0] = this.mainOffset[0]
        // java mainOffset[1] = this.mainOffset[1]
        */
    }

    // java public void saveFreeze() {
    pub fn saveFreeze(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.hasFrozen() && (this.freezeTopRowCol[0] != -1 || this.freezeTopRowCol[1] != -1))
        // java int splitRows = this.getPaneRowSplit()
        // java int splitCols = this.getPaneColSplit()
        // java if (splitRows > 0)
        // java int newSplitRows = splitRows - (this.freezeTopRowCol[0] - this.leftTopRow())
        // java this.setPaneRowSplit(newSplitRows)
        // java this.setLeftTopRow(this.freezeTopRowCol[0])
        // java if (splitCols > 0)
        // java int newSplitCols = splitCols - (this.freezeTopRowCol[1] - this.leftTopCol())
        // java this.setPaneColSplit(newSplitCols)
        // java this.setLeftTopCol(this.freezeTopRowCol[1])
        */
    }

    // java public UnicodeString getRunsStringValue(int row, int col) {
    pub fn getRunsStringValue(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData data = this.getCellData(row, col)
        // java UnicodeString src = this.getCellValueStore().runsStringValue(data.id)
        // java return src
        */
    }

    // java public String getStringValue(int row, int col) {
    pub fn getStringValue(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData kmoCellData = this.getCellData(row, col)
        // java if (kmoCellData.isBlank())
        // java return null
        // java else if (!KmoCellValueStore.isStringId(kmoCellData.id))
        // java return null
        // java String cellStr = this.getBook().getCellValueStore().stringValue(kmoCellData.id)
        // java return cellStr
        */
    }

    // java public boolean isPicCell(int row, int col) {
    pub fn isPicCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData cellData = SharedObjects.kmoCellDataPool.borrowObject()
        // java this.getCellData(row, col, cellData)
        // java return null != cellData && cellData.isPicInCell
        */
    }

    // java public int getCellType(CellIterator iterator) {
    pub fn getCellType__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoCellData cellData = iterator.cellData()
        // java int id = cellData.id
        // java int cellType = KmoCellValueStore.identifyTypeByID(id)
        // java if (cellType == 7)
        // java KmoFormulaData fmlData = SharedObjects.kmoFormulaDataPool.borrowObject()
        // java this.getCellValueStore().formulaValue(id, fmlData)
        // java id = fmlData.getCacheValueId()
        // java cellType = KmoCellValueStore.identifyTypeByID(id)
        // java fmlData = SharedObjects.kmoFormulaDataPool.returnObject(fmlData)
        // java return cellType
        */
    }

    // java public boolean hasDrawing() {
    pub fn hasDrawing(&self) -> bool {
        // java return this.hasDrawing;
        self.hasDrawing.get()
    }

    // java public void setHasDrawing(boolean hasDrawing) {
    pub fn setHasDrawing(&self, hasDrawing: bool) {
        // java this.hasDrawing = hasDrawing;
        self.hasDrawing.set(hasDrawing);
    }

    // java public CellIterator(KmoMatrixCellIterator iter) {
    pub fn CellIterator(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.iter = iter
        */
    }

    // java public boolean hasNext() {
    pub fn hasNext(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.iter.hasNext()
        */
    }

    // java public void next() {
    pub fn next(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.iter.next()
        // java this.dataNeedRefresh = true
        */
    }

    // java public int row() {
    pub fn row(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.iter.row()
        */
    }

    // java public int col() {
    pub fn col(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.iter.col()
        */
    }

    // java public KmoCellData cellData() {
    pub fn cellData(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.dataNeedRefresh)
        // java this.data.read(this.iter.data())
        // java this.dataNeedRefresh = false
        // java return this.data
        */
    }
}
