// java package cn.wps.moss.app;

// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::java_runtime::{Cell, RefCell};
use crate::java_runtime::{Rc, Weak};

use crate::cn::wps::moss::app::xf::KmoXFManager::KmoXFManager;
// java import cn.wps.et.ss.dependent.Dependent;
use crate::cn::wps::et::ss::dependent::Dependent::Dependent;
use crate::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use crate::cn::wps::moss::core::drawing::KmoCellPic::KmoCellPic;
use crate::cn::wps::moss::engine::linktb::LinkTable::LinkTable;
// java import cn.wps.moss.engine.areaevent.AreaWatcher;
use crate::cn::wps::moss::engine::areaevent::AreaWatcher::AreaWatcher;
use crate::cn::wps::moss::core::theme::WorkbookPr::WorkbookPr;
use crate::cn::wps::moss::util::KmoRect::KmoRect;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

use super::KmoSheet::KmoSheet;
use super::FileVersion::FileVersion;
use super::CalcPr::CalcPr;
use super::KmoSheets::KmoSheets;

pub use super::TransactionManager::TransactionManager;

pub use crate::cn::wps::moss::core::book::Window1::Window1;

// rust: captures of KmoBook.reCalcInBackground's original anonymous callback.
struct BookRecalculation {
    book: Rc<KmoBook>,
    calculationChain: Rc<crate::cn::wps::moss::engine::eval::CalculationChain::CalculationChain>,
    activateSheet: Rc<RefCell<KmoSheet>>,
    markList: RefCell<std::collections::BTreeMap<i32, Vec<KmoRect>>>,
}
impl crate::cn::wps::et::ss::calcchain::CalcChain::ICalculationCallback for BookRecalculation {
    // java public void onStart() {
    fn onStart(&self) {
        // java calculationChain.resumeInterruptMark();
        self.calculationChain.resumeInterruptMark();
        // java KmoBook.this.markRecalWithTransaction(markList);
        self.book.markRecalWithTransaction(&self.markList.borrow());
    }
    // java public void onEnd() {
    fn onEnd(&self) {
        // java KmoBook.this.setDirty(true);
        self.book.setDirty(true);
        // java KmoBook.this.getActiveSheet().changeListenerGroup().sheetContentsDidFinishChanging();
        self.book.getActiveSheet().borrow().changeListenerGroup().sheetContentsDidFinishChanging();
        // java KmoBook.this.areaWatcher().postContentChangedEvent(activateSheet, activateSheet.getUsedRange());
        let sheet = self.activateSheet.borrow();
        self.book.areaWatcher().borrow_mut().postContentChangedEvent__rect(&sheet, &sheet.getUsedRange());
        // java markList.clear();
        self.markList.borrow_mut().clear();
    }
}

// java public class KmoBook
pub struct KmoBook {
    mUUID:String,
    palette:RefCell<Rc<crate::cn::wps::moss::core::book::KmoPalette::KmoPalette>>,
    me: Weak<KmoBook>,
    pub(crate) sheets: RefCell<KmoSheets>,
    value_store: RefCell<Option<Rc<RefCell<KmoCellValueStore>>>>,
    ss_version: Cell<SpreadsheetVersion>,
    // java private Window1 window1 = new Window1();
    window1: RefCell<Option<Rc<Window1>>>,
    // java private KmoBookProtect bookProtect = new KmoBookProtect(this);
    bookProtect:Rc<crate::cn::wps::moss::core::book::KmoBookProtect::KmoBookProtect>,
    // java private KmoBookCrypt cryptInfo = new KmoBookCrypt(this);
    cryptInfo:Rc<crate::cn::wps::moss::core::book::KmoBookCrypt::KmoBookCrypt>,
    trans_mgr: Rc<TransactionManager>,
    // java private KmoBookChangeListenerGroup changeListeners = new KmoBookChangeListenerGroup();
    changeListeners: super::KmoBookChangeListenerGroup::KmoBookChangeListenerGroup,
    // java private KmoBookTransListenerGroup transListeners = new KmoBookTransListenerGroup();
    transListeners: super::event::KmoBookTransListenerGroup::KmoBookTransListenerGroup,
    // java private boolean markInTransaction;
    markInTransaction: Cell<bool>,
    // rust: original synchronized KmoBook monitor.
    monitor: crate::cn::wps::et::ss::calcchain::RecalcLock,
    dirty: Cell<bool>,
    mShouldDirty: Cell<bool>,
    isIOState: Cell<bool>,
    // java private boolean isAutoCalc;
    isAutoCalc: Cell<bool>,
    // java private boolean isCompletelyReaded;
    isCompletelyReaded: Cell<bool>,
    // java private boolean calcChainInitialized = false;
    calcChainInitialized: Cell<bool>,
    isMakingSnapshot: Cell<bool>,
    interruptAutoSave: Cell<bool>,
    isSilentSaveWithoutDirty: Cell<bool>,
    lastModify: RefCell<[i32; 3]>,
    // java private KmoAppSettings mAppSettings = null;
    mAppSettings:RefCell<Option<Rc<RefCell<crate::cn::wps::moss::core::book::KmoAppSettings::KmoAppSettings>>>>,
    // java private KmoXFManager xfManager = new KmoXFManager();
    xfManager: Rc<RefCell<KmoXFManager>>,
    filePath: RefCell<Option<String>>,
    fileFormat: Cell<i32>,
    bReadOnly: Cell<bool>,
    workbookPr: Rc<RefCell<WorkbookPr>>,
    fileVersion: RefCell<Option<Rc<RefCell<FileVersion>>>>,
    calcPr: Rc<RefCell<CalcPr>>,
    using1904: Cell<bool>,
    // java private LinkTable linktable;
    linktable: RefCell<Option<Rc<RefCell<LinkTable>>>>,
    // java private AreaWatcher areaWatcher = new AreaWatcher();
    areaWatcher: RefCell<AreaWatcher>,
    // java private Dependent dep;
    dep: RefCell<Option<Rc<RefCell<Dependent>>>>,
    // java private CalculationChain calcChain;
    calcChain: RefCell<Option<Rc<crate::cn::wps::moss::engine::eval::CalculationChain::CalculationChain>>>,
    // java private boolean isOom = false;
    isOom: Cell<bool>,
    // java private ThreadAffinityCallback threadAffinityCallback;
    threadAffinityCallback: RefCell<Option<Rc<dyn super::ThreadAffinityCallback::ThreadAffinityCallback>>>,
    // rust: the current XLSX reader clears bookReader before returning.
    isLargestFile: Cell<bool>,
    bookReader:RefCell<Option<Rc<dyn crate::cn::wps::moss::io::BookReader::BookReader>>>,
}

impl KmoBook {
    pub(crate) fn me_weak(&self) -> Weak<KmoBook> {
        self.me.clone()
    }

    pub(crate) fn open_force_quit(&self) -> bool {
        false
    }

    // java KmoBook()
    pub fn KmoBook() -> Rc<Self> {
        // java this.ssVersion = SpreadsheetVersion.EXCEL97;
        // java this.curIOVersion = null;
        // java this.interruptAutoSave = false;
        // java this.workbookPr = new WorkbookPr();
        // java this.calcPr = new CalcPr();
        // rust: calcPr is initialized in the struct below and read by EvaluationWorkbook.
        // java this.recoveryLogs = new KmoRecoveryLogs();
        // rust: no-stmt — 恢复日志不参与这一档的读回。
        // java this.mIsLegal = true;
        // java this.mShouldDirty = true;
        // java this.fmlHyperLinkManager = new KmoFmlHyperLinkManager();
        // rust: no-stmt — 超链接管理器不参与格子读回。
        // java this.isReadTable = true;
        // java this.isAutoCalc = true;
        // java this.lastModify = new int[3];
        // java this.mSaveCompleted = false;
        // java this.isMakingSnapshot = false;
        // java this.tip = null;
        Rc::new_cyclic(|me| Self {
            mUUID:util::cn::wps::moffice::util::UUIDUtil::UUIDUtil::generate32ID(),
            palette:RefCell::new(Rc::new(crate::cn::wps::moss::core::book::KmoPalette::KmoPalette::new())),
            me: me.clone(),
            sheets: RefCell::new(KmoSheets::KmoSheets()),
            value_store: RefCell::new(None),
            ss_version: Cell::new(SpreadsheetVersion::EXCEL97),
            window1: RefCell::new(Some(Rc::new(Window1::new()))),
            bookProtect:Rc::new(crate::cn::wps::moss::core::book::KmoBookProtect::KmoBookProtect::new(me.clone())),
            cryptInfo:Rc::new(crate::cn::wps::moss::core::book::KmoBookCrypt::KmoBookCrypt::new(me.clone())),
            trans_mgr: Rc::new(TransactionManager::TransactionManager(me.clone())),
            changeListeners: super::KmoBookChangeListenerGroup::KmoBookChangeListenerGroup::new(),
            transListeners: super::event::KmoBookTransListenerGroup::KmoBookTransListenerGroup::new(),
            markInTransaction: Cell::new(false),
            monitor: crate::cn::wps::et::ss::calcchain::RecalcLock::new(),
            dirty: Cell::new(false),
            mShouldDirty: Cell::new(true),
            isIOState: Cell::new(false),
            // java this.isAutoCalc = true;
            isAutoCalc: Cell::new(true),
            isCompletelyReaded: Cell::new(false),
            calcChainInitialized: Cell::new(false),
            isMakingSnapshot: Cell::new(false),
            interruptAutoSave: Cell::new(false),
            isSilentSaveWithoutDirty: Cell::new(false),
            lastModify: RefCell::new([0, 0, 0]),
            mAppSettings:RefCell::new(None),
            xfManager: Rc::new(RefCell::new(KmoXFManager::KmoXFManager())),
            filePath: RefCell::new(None),
            fileFormat: Cell::new(0),
            bReadOnly: Cell::new(false),
            workbookPr: Rc::new(RefCell::new(WorkbookPr::WorkbookPr())),
            fileVersion: RefCell::new(None),
            calcPr: Rc::new(RefCell::new(CalcPr::default())),
            using1904: Cell::new(false),
            linktable: RefCell::new(None),
            // java private AreaWatcher areaWatcher = new AreaWatcher();
            areaWatcher: RefCell::new(AreaWatcher::new()),
            dep: RefCell::new(None),
            calcChain: RefCell::new(None),
            isOom: Cell::new(false),
            threadAffinityCallback:RefCell::new(None),
            isLargestFile: Cell::new(false),bookReader:RefCell::new(None),
        })
    }

    // java KmoBook(KmoBooks parent, boolean createDefaultSheet)
    pub fn KmoBook__parent(create_default_sheet: bool) -> Rc<Self> {
        // java this.ssVersion = SpreadsheetVersion.EXCEL97;
        // java this.curIOVersion = null;
        // java this.interruptAutoSave = false;
        // java this.workbookPr = new WorkbookPr();
        // java this.calcPr = new CalcPr();
        // java this.recoveryLogs = new KmoRecoveryLogs();
        // java this.mIsLegal = true;
        // java this.mShouldDirty = true;
        // java this.fmlHyperLinkManager = new KmoFmlHyperLinkManager();
        // java this.isReadTable = true;
        // java this.isAutoCalc = true;
        // java this.lastModify = new int[3];
        // java this.mSaveCompleted = false;
        // java this.isMakingSnapshot = false;
        // java this.tip = null;
        let book = Self::KmoBook();
        // java this.init(parent, createDefaultSheet);
        book.init(create_default_sheet);
        book
    }
}

impl KmoBook {
    // java public SpreadsheetVersion getCurIOVersion() {
    pub fn getCurIOVersion(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.curIOVersion
        */
    }

    // java public void setCurIOVersion(SpreadsheetVersion curIOVersion) {
    pub fn setCurIOVersion(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.curIOVersion = curIOVersion
        */
    }

    // java public int[] getLastModify() {
    pub fn getLastModify(&self) -> [i32; 3] {
        // java return this.lastModify;
        *self.lastModify.borrow()
    }

    // java public void setLastModify(int sheetIndex, int row, int col) {
    pub fn setLastModify(&self, sheet_index: i32, row: i32, col: i32) {
        // java this.lastModify[0] = sheetIndex;
        // java this.lastModify[1] = row;
        // java this.lastModify[2] = col;
        let mut last = self.lastModify.borrow_mut();
        last[0] = sheet_index;
        last[1] = row;
        last[2] = col;
    }

    // java public CustomWoProps getCustomWoProps() {
    pub fn getCustomWoProps(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.customWoProps
        */
    }

    // java public void setCustomWoProps(CustomWoProps customWoProps) {
    pub fn setCustomWoProps(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.customWoProps = customWoProps
        */
    }

    // java public void setLayoutInfo(ILayoutInfo layoutInfo) {
    pub fn setLayoutInfo(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.layoutInfo = layoutInfo
        */
    }

    // java public ILayoutInfo getLayoutInfo() {
    pub fn getLayoutInfo(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.layoutInfo
        */
    }

    // java public void setStatAnalytics(StatAnalytics statAnalytics) {
    pub fn setStatAnalytics(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.statAnalytics = statAnalytics
        */
    }

    // java public StatAnalytics getStatAnalytics() {
    pub fn getStatAnalytics(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.statAnalytics
        */
    }

    // java public void setIsReadTable(boolean isReadTable) {
    pub fn setIsReadTable(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.isReadTable = isReadTable
        */
    }

    // java public boolean isReadTable() {
    pub fn isReadTable(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.isReadTable
        */
    }

    // java public KmoCellValueStore getCellValueStore() {
    pub fn getCellValueStore(&self) -> Rc<RefCell<KmoCellValueStore>> {
        // java return this.valueStore;
        self.value_store
            .borrow()
            .clone()
            .unwrap_or_else(|| panic!("NullPointerException"))
    }

    // java public KmoTransaction transaction() {
    pub fn transaction(&self) -> &TransactionManager {
        // java return this.transMgr;
        &self.trans_mgr
    }
    // Native reference transport of the same Java transaction field (no copied manager).
    pub(crate) fn transactionReference(&self)->Rc<TransactionManager>{self.trans_mgr.clone()}

    // java public KmoCutCopyPasteManager pasteMgr() {
    pub fn pasteMgr(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.pasteMgr
        */
    }

    // java public WorkbookPr getWorkbookPr() {
    pub fn getWorkbookPr(&self) -> Rc<RefCell<WorkbookPr>> {
        // java return this.workbookPr;
        Rc::clone(&self.workbookPr)
    }

    // java public CalcPr getCalcPr() {
    pub fn getCalcPr(&self) -> Rc<RefCell<CalcPr>> {
        // java return this.calcPr;
        Rc::clone(&self.calcPr)
    }

    // java public void setFileVersion(FileVersion val) {
    pub fn setFileVersion(&self, val: Rc<RefCell<FileVersion>>) {
        // java this.fileVersion = val;
        *self.fileVersion.borrow_mut() = Some(val);
    }

    // java public void setUnitsConverter(ICoreUnitsConverter unitsConverter) {
    pub fn setUnitsConverter(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.unitsConverter = unitsConverter
        */
    }

    // java public ICoreUnitsConverter getUnitsConverter() {
    pub fn getUnitsConverter(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.unitsConverter
        */
    }

    // java public FileVersion fileVersion() {
    pub fn fileVersion(&self) -> Option<Rc<RefCell<FileVersion>>> {
        // java return this.fileVersion;
        self.fileVersion.borrow().clone()
    }

    // java public KmoTheme theme() {
    pub fn theme(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.kmoTheme
        */
    }

    // java public BuildInTableStyleFactory getBuildInTableStyleFactory() {
    pub fn getBuildInTableStyleFactory(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null == this.mTableStyleFactory)
        // java this.mTableStyleFactory = new BuildInTableStyleFactory(this.kmoTheme)
        // java return this.mTableStyleFactory
        */
    }

    // java public boolean hasThemeOverride() {
    pub fn hasThemeOverride(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.kmoThemeOverrideLst.size() != 0
        */
    }

    // java public int themeOverrideSize() {
    pub fn themeOverrideSize(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.kmoThemeOverrideLst.size()
        */
    }

    // java public KmoTheme themeOverrideAt(int index) {
    pub fn themeOverrideAt(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return (KmoTheme)this.kmoThemeOverrideLst.get(index)
        */
    }

    // java public void addThemeOverride(KmoTheme theme) {
    pub fn addThemeOverride(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (!this.kmoThemeOverrideLst.contains(theme))
        // java this.kmoThemeOverrideLst.add(theme)
        */
    }

    // java public void setFindList(List<FindResult> list) {
    pub fn setFindList(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.findResults = list
        */
    }

    // java public List<FindResult> getFindList() {
    pub fn getFindList(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.findResults
        */
    }

    // java public SpreadsheetVersion getSpreadsheetVersion() {
    pub fn getSpreadsheetVersion(&self) -> SpreadsheetVersion {
        // java return this.ssVersion;
        self.ss_version.get()
    }

    // java public int getMaxRows() {
    pub fn getMaxRows(&self) -> i32 {
        // java return this.ssVersion.getMaxRows();
        self.ss_version.get().getMaxRows()
    }

    // java public int getMaxColumns() {
    pub fn getMaxColumns(&self) -> i32 {
        // java return this.ssVersion.getMaxColumns();
        self.ss_version.get().getMaxColumns()
    }

    // java public void setSpreadsheetVersion(SpreadsheetVersion version) {
    pub fn setSpreadsheetVersion(&self, version: SpreadsheetVersion) {
        // java this.ssVersion = version;
        self.ss_version.set(version);
    }

    // java public void setInterruptAutoSave(boolean interrupt) {
    pub fn setInterruptAutoSave(&self, interrupt: bool) {
        // java this.interruptAutoSave = interrupt;
        self.interruptAutoSave.set(interrupt);
    }

    // java public boolean isInterruptAutoSave() {
    pub fn isInterruptAutoSave(&self) -> bool {
        // java return this.interruptAutoSave;
        self.interruptAutoSave.get()
    }

    // java public boolean isSaveCompleted() {
    pub fn isSaveCompleted(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.mSaveCompleted
        */
    }

    // java public void initBookStringStoreByStrs(Iterator<UnicodeString> strs, int size) {
    pub fn initBookStringStoreByStrs(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.valueStore.initStrStore(strs, size)
        */
    }

    // java void init(KmoBooks parent, boolean createDefaultSheet) {
    pub fn init(&self, create_default_sheet: bool) {
        // java this.valueStore = new KmoCellValueStore();
        *self.value_store.borrow_mut() = Some(Rc::new(RefCell::new(KmoCellValueStore::KmoCellValueStore())));
        // java this.parent = parent;
        // rust: no-stmt — KmoBooks 不在这一档。parent 只被保存，表顺序和格子不读它。
        let _parent = ();
        // java if (createDefaultSheet)
        if create_default_sheet {
            // java for(int i = 0; i < 1; ++i)
            for i in 0..1 {
                let _i = i;
                // java this.sheets.append();
                self.appendSheet();
            }
        }
        // java this.mUUID = UUIDUtil.generate32ID();
        // rust: no-stmt — UUID 不参与表顺序和单元格读回。
        // java this.mFileManager = new FileManager(this.getTmpPath());
        // rust: no-stmt — 临时文件管理器不参与格子读回。
    }

    // java public void subscribeChangeEvents(KmoBookChangeEventsListener rhs) {
    pub fn subscribeChangeEvents(&self, rhs: Option<Rc<dyn super::event::KmoBookChangeEventsListener::KmoBookChangeEventsListener>>) {
        // java this.changeListeners.add(rhs)
        self.changeListeners.add(rhs);
    }

    // java public void unsubscribeChangeEvents(KmoBookChangeEventsListener rhs) {
    pub fn unsubscribeChangeEvents(&self, rhs: Option<&Rc<dyn super::event::KmoBookChangeEventsListener::KmoBookChangeEventsListener>>) {
        // java this.changeListeners.remove(rhs)
        self.changeListeners.remove(rhs);
    }

    // java public int whichSheetIsActivated() {
    pub fn whichSheetIsActivated(&self) -> i32 {
        // java return this.window1.getActiveSheet();
        self.window1.borrow().clone().expect("NullPointerException").getActiveSheet()
    }

    // java public void setAutoCalcOn(boolean isTurnOn) {
    pub fn setAutoCalcOn(&self, isTurnOn: bool) {
        // java this.isAutoCalc = isTurnOn;
        self.isAutoCalc.set(isTurnOn);
    }

    // java public boolean isAutoCalcOn() {
    pub fn isAutoCalcOn(&self) -> bool {
        // java return this.isAutoCalc;
        self.isAutoCalc.get()
    }

    // java public int getNumSelectedTabs() {
    pub fn getNumSelectedTabs(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int count = 0
        // java int i = 0
        // java for(int len = this.sheetCount(); i < len; ++i)
        // java if (this.sheetAt(i).isSelected())
        // java ++count
        // java return count
        */
    }

    // java public KmoFmlHyperLinkManager getFmlHyperLinkManager() {
    pub fn getFmlHyperLinkManager(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.fmlHyperLinkManager
        */
    }

    // java public void activateSheet(int index) {
    pub fn activateSheet(&self, index: i32) {
        // java this.window1.setActiveSheet(index);
        self.window1.borrow().clone().expect("NullPointerException").setActiveSheet(index);
        // java this.sheets.activate(index);
        KmoSheets::activate(self, index);
        // java this.changeListeners.sheetDidFinishActivating();
        self.changeListeners.sheetDidFinishActivating();
    }

    // java public void activateSheetWithNoInvalidate(int index) {
    pub fn activateSheetWithNoInvalidate(&self, index: i32) {
        // java this.window1.setActiveSheet(index);
        self.window1.borrow().clone().expect("NullPointerException").setActiveSheet(index);
        // java this.sheets.activate(index);
        KmoSheets::activate(self, index);
    }

    // java public KmoSheet sheetAt(int index) {
    pub fn sheetAt(&self, index: i32) -> Rc<RefCell<KmoSheet>> {
        // java return this.sheets.sheetAt(index);
        KmoSheets::sheetAt(self, index)
    }

    // java public void removeSheet(KmoSheet sheet) {
    pub fn removeSheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheets.remove(sheet)
        */
    }

    // java public int sheetUid2SheetIndex(int sheetUid) {
    pub fn sheetUid2SheetIndex(&self, uid: i32) -> i32 {
        // java for (int i=0,len=sheetCount(); i<len; ++i) if (sheets.sheetAt(i).getSheetUid()==sheetUid) return i; return -1;
        for i in 0..self.sheetCount() {
            if self.sheetAt(i).borrow().getSheetUid() == uid {
                return i;
            }
        }
        -1
    }

    // java public boolean isDefaultSheetName(String sheetName) {
    pub fn isDefaultSheetName(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheets.isDefaultSheetName(sheetName)
        */
    }

    // java public KmoSheet sheetUid2Sheet(int sheetUid) {
    pub fn sheetUid2Sheet(&self, uid: i32) -> Option<Rc<RefCell<KmoSheet>>> {
        // java for (int i=0,len=sheetCount(); i<len; ++i) if (sheets.sheetAt(i).getSheetUid()==sheetUid) return sheets.sheetAt(i); return null;
        for i in 0..self.sheetCount() {
            let sheet = self.sheetAt(i);
            if sheet.borrow().getSheetUid() == uid {
                return Some(sheet);
            }
        }
        None
    }

    // java public int sheetIndex(KmoSheet sheet) {
    pub fn sheetIndex(&self, sheet: &KmoSheet) -> i32 {
        // java return this.sheets.sheetIndex(sheet);
        KmoSheets::sheetIndex(self, sheet)
    }

    // java public KmoSheet getActiveSheet() {
    pub fn getActiveSheet(&self) -> Rc<RefCell<KmoSheet>> {
        // java return this.sheetAt(this.whichSheetIsActivated());
        self.sheetAt(self.whichSheetIsActivated())
    }

    // java public int sheetIndex(String sheet) {
    pub fn sheetIndex__2(&self, sheet: &str) -> i32 {
        // java return this.sheets.sheetIndex(sheet);
        KmoSheets::sheetIndex__2(self, sheet)
    }

    // java public KmoSheet insertSheet(int index) {
    pub fn insertSheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoSheet sheet = this.sheets.insertAt(index)
        // java this.setDirty(true)
        // java this.changeListeners.sheetDidFinishActivating()
        // java return sheet
        */
    }

    // java public KmoSheet insertSheet(int index, String sheetName) {
    pub fn insertSheet__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoSheet sheet = this.sheets.insertAt(-1, index, sheetName)
        // java this.setDirty(true)
        // java this.changeListeners.sheetDidFinishActivating()
        // java return sheet
        */
    }

    // java public KmoSheet insertSheet(int sheetId, int index, String sheetName, byte sheetType) {
    pub fn insertSheet__3(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheets.insertAt(sheetId, index, sheetName, sheetType)
        */
    }

    // java public KmoSheet appendSheetTest() {
    pub fn appendSheetTest(&self) -> Rc<RefCell<KmoSheet>> {
        // java KmoSheet sheet = this.sheets.append((String)null);
        let sheet = KmoSheets::append(self, None);
        // java this.transMgr.reset();
        self.trans_mgr.reset();
        // java this.setDirty(true);
        self.setDirty(true);
        // java this.changeListeners.sheetDidFinishActivating();
        // rust: no-stmt — 监听器不改变表顺序。
        // java return sheet;
        sheet
    }

    // java public KmoSheet appendSheet() {
    pub fn appendSheet(&self) -> Rc<RefCell<KmoSheet>> {
        // java KmoSheet sheet = this.sheets.append();
        let sheet = KmoSheets::append__2(self);
        // java this.transMgr.reset();
        self.trans_mgr.reset();
        // java this.setDirty(true);
        self.setDirty(true);
        // java this.changeListeners.sheetDidFinishActivating();
        // rust: no-stmt — 监听器不改变表顺序。
        // java return sheet;
        sheet
    }

    // java public KmoSheet appendSheet(String sheetName) {
    pub fn appendSheet__2(&self, sheet_name: Option<String>) -> Rc<RefCell<KmoSheet>> {
        // java KmoSheet sheet = this.sheets.append(sheetName);
        let sheet = KmoSheets::append(self, sheet_name);
        // java this.transMgr.reset();
        self.trans_mgr.reset();
        // java this.setDirty(true);
        self.setDirty(true);
        // java this.changeListeners.sheetDidFinishActivating();
        // rust: no-stmt — 监听器不改变表顺序。
        // java return sheet;
        sheet
    }

    // java public KmoSheet appendSheet(KmoSheet sheet) {
    pub fn appendSheet__3(&self, sheet: Rc<RefCell<KmoSheet>>) -> Rc<RefCell<KmoSheet>> {
        // java this.sheets.appendSheet(sheet);
        KmoSheets::appendSheet(self, Rc::clone(&sheet));
        // java this.transMgr.reset();
        self.trans_mgr.reset();
        // java this.setDirty(true);
        self.setDirty(true);
        // java this.changeListeners.sheetDidFinishActivating();
        // rust: no-stmt — 监听器不改变表顺序。
        // java return sheet;
        sheet
    }

    // java public KmoSheet appendSheetWithNoInvalidate(KmoSheet sheet) {
    pub fn appendSheetWithNoInvalidate(&self, sheet: Rc<RefCell<KmoSheet>>) -> Rc<RefCell<KmoSheet>> {
        // java this.sheets.appendSheet(sheet);
        KmoSheets::appendSheet(self, Rc::clone(&sheet));
        // java this.transMgr.reset();
        self.trans_mgr.reset();
        // java this.setDirty(true);
        self.setDirty(true);
        // java return sheet;
        sheet
    }

    // java public KmoSheet appendSheetWithNoInvalidate() {
    pub fn appendSheetWithNoInvalidate__2(&self) -> Rc<RefCell<KmoSheet>> {
        // java KmoSheet sheet = this.sheets.append();
        let sheet = KmoSheets::append__2(self);
        // java this.transMgr.reset();
        self.trans_mgr.reset();
        // java this.setDirty(true);
        self.setDirty(true);
        // java return sheet;
        sheet
    }

    // java public KmoSheet appendSheet(String name, byte sheetType) {
    pub fn appendSheet__4(&self, name: Option<String>, sheet_type: i8) -> Rc<RefCell<KmoSheet>> {
        // java return this.appendSheet(-1, name, sheetType);
        self.appendSheet__5(-1, name, sheet_type)
    }

    // java public KmoSheet appendSheet(int sheetId, String name, byte sheetType) {
    pub fn appendSheet__5(
        &self,
        sheet_id: i32,
        name: Option<String>,
        sheet_type: i8,
    ) -> Rc<RefCell<KmoSheet>> {
        // java KmoSheet sheet = this.sheets.insertAt(sheetId, -1, name, sheetType);
        let sheet = KmoSheets::insertAt__2(self, sheet_id, -1, name, sheet_type);
        // java this.setDirty(true);
        self.setDirty(true);
        // java this.changeListeners.sheetDidFinishActivating();
        // rust: no-stmt — 监听器不改变表顺序。
        // java return sheet;
        sheet
    }

    // java public int sheetCount() {
    pub fn sheetCount(&self) -> i32 {
        // java return this.sheets.size();
        self.sheets.borrow().size()
    }

    // java public int getNextSheetIndex(int curSheetIndex, boolean isForward) {
    pub fn getNextSheetIndex(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int newSheetIndex = curSheetIndex + (isForward ? -1 : 1)
        // java newSheetIndex = (newSheetIndex + this.sheetCount()) % this.sheetCount()
        // java return newSheetIndex
        */
    }

    // java public int getNextSheetId() {
    pub fn getNextSheetId(&self) -> i32 {
        // java return this.sheets.helper_generateNewSheetId();
        self.sheets.borrow_mut().helper_generateNewSheetId()
    }

    // java public void removeActivatedSheet() {
    pub fn removeActivatedSheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.removeActivatedSheet(true)
        */
    }

    // java public void removeActivatedSheet(boolean isActiveHidden) {
    pub fn removeActivatedSheet__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int activeSheet = this.whichSheetIsActivated()
        // java KmoSheet sheet = this.sheetAt(activeSheet)
        // java if (this.calcChain != null)
        // java this.calcChain.markRecalculation(activeSheet, 0, this.ssVersion.getLastRowIndex(), 0, this.ssVersion.getLastColumnIndex(), false)
        // java this.calcChain.unregisterFormula(activeSheet, 0, this.ssVersion.getLastRowIndex(), 0, this.ssVersion.getLastColumnIndex())
        // java this.getLinkTable().localSheetRemoved(activeSheet)
        // java this.sheets.remove(activeSheet)
        // java if (this.calcChain != null)
        // java this.calcChain.synCalculate()
        // java catch (CalcChain.CircleReferenceException var5)
        // java activeSheet = this.getNewActiveSheetIndexAfterDel(activeSheet, isActiveHidden)
        // java this.window1.setActiveSheet(activeSheet)
        // java this.sheets.activate(activeSheet)
        // java this.setDirty(true)
        // java this.pasteMgr.cancelPasteState()
        // java KmoRect rect = new KmoRect(0, 0, this.ssVersion.getLastRowIndex(), this.ssVersion.getLastColumnIndex())
        // java this.areaWatcher.postRemoveRectUpwardEvent(sheet, rect)
        // java this.changeListeners.sheetDidFinishActivating()
        */
    }

    // java private int getNewActiveSheetIndexAfterDel(int curActiveIndex, boolean isActiveHidden) {
    pub fn getNewActiveSheetIndexAfterDel(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java for(int newActiveSheetIndex = curActiveIndex; newActiveSheetIndex < this.sheets.size(); ++newActiveSheetIndex)
        // java if (isActiveHidden)
        // java return newActiveSheetIndex
        // java if (!this.sheets.sheetAt(newActiveSheetIndex).isSheetHidden())
        // java return newActiveSheetIndex
        // java for(int var4 = curActiveIndex - 1; var4 >= 0; --var4)
        // java if (isActiveHidden)
        // java return var4
        // java if (!this.sheets.sheetAt(var4).isSheetHidden())
        // java return var4
        // java throw new RuntimeException()
        */
    }

    // java public void moveSheet(int srcSheetIndex, int newIndex) {
    pub fn moveSheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.getLinkTable().localSheetMoved(srcSheetIndex, newIndex)
        // java this.sheets.moveSheet(srcSheetIndex, newIndex)
        // java this.window1.setActiveSheet(newIndex)
        // java this.sheets.activate(newIndex)
        // java if (this.pasteMgr.isInPasteState())
        // java this.pasteMgr.cancelPasteState()
        // java this.getActiveSheet().changeListenerGroup().sheetContentsDidFinishChanging()
        // java this.setDirty(true)
        // java this.changeListeners.sheetDidFinishActivating()
        */
    }

    // java public void checkChartLink() {
    pub fn checkChartLink(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int sheetCount = this.sheetCount()
        // java for(int i = 0; i < sheetCount; ++i)
        // java KmoSheet sheet = this.sheetAt(i)
        // java if (!(sheet instanceof KmoChartSheet))
        // java SheetDrawingAgg sheetDrawingAgg = sheet.getSheetDrawingAgg()
        // java for(KmoShape kmoShape : sheetDrawingAgg.getShapes())
        // java this.attachShape(kmoShape)
        // java KmoChartSheet kmoChartSheet = (KmoChartSheet)sheet
        // java if (kmoChartSheet.chart() != null)
        // java kmoChartSheet.chart().tryAttachSource()
        */
    }

    // java private void attachShape(KmoShape kmoShape) {
    pub fn attachShape(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (kmoShape.isGroupShape())
        // java for(int j = 0; j < kmoShape.getChildCount(); ++j)
        // java KmoShape kmoChildShape = kmoShape.getChildShape(j)
        // java this.attachShape(kmoChildShape)
        // java else if (kmoShape instanceof KmoCTChart)
        // java ((KmoCTChart)kmoShape).tryAttachSource()
        */
    }

    // java void helper_shutdown() {
    pub fn helper_shutdown(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.getCalculationChain() != null)
        // java this.getCalculationChain().interrupt()
        // java this.getCalculationChain().turnOff()
        // java KPictureRegistry.instance().clear()
        // java this.bookDrawingAgg.onDestory()
        // java catch (Exception var3)
        // java this.parent = null
        // java this.changeListeners.clear()
        // java this.transMgr.clearListeners()
        // java this.sheets.destroy()
        // java this.areaWatcher.destroy()
        // java this.getCellValueStore().releaseStoreData()
        // java if (this.bookReader != null)
        // java this.bookReader.close()
        // java catch (IOException var2)
        // java this.bookReader = null
        // java if (this.mOleMgr != null)
        // java this.mOleMgr.destroy(this.getFilePath(), this.getRaf())
        // java RendererOption.destroy()
        */
    }

    // java public LinkTable getLinkTable() {
    pub fn getLinkTable(&self) -> Rc<RefCell<LinkTable>> {
        // java if (this.linktable == null)
        if self.linktable.borrow().is_none() {
            // java this.linktable = new LinkTable(this.sheetCount(), this)
            let created = LinkTable::LinkTable(self.sheetCount(), &self.me);
            *self.linktable.borrow_mut() = Some(Rc::new(RefCell::new(created)));
        }
        // java return this.linktable
        Rc::clone(self.linktable.borrow().as_ref().unwrap())
    }

    // java public void setLinkTable(LinkTable lt) {
    pub fn setLinkTable(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoConstraintCheck.checkNotNull(lt)
        // java if (this.linktable == null)
        // java this.linktable = lt
        */
    }

    // java public KmoPalette getPalette() {
    pub fn getPalette(&self)->Rc<crate::cn::wps::moss::core::book::KmoPalette::KmoPalette> {
self.palette.borrow().clone()
    }

    // java public KmoAppSettings getAppSettings() {
    pub fn getAppSettings(&self)->Rc<RefCell<crate::cn::wps::moss::core::book::KmoAppSettings::KmoAppSettings>> {
        // java if (this.mAppSettings == null) { this.mAppSettings = new KmoAppSettings(this); }
        let mut settings=self.mAppSettings.borrow_mut();
        if settings.is_none(){*settings=Some(Rc::new(RefCell::new(crate::cn::wps::moss::core::book::KmoAppSettings::KmoAppSettings::KmoAppSettings(self.me.upgrade().expect("KmoBook")))));}
        // java return this.mAppSettings;
        settings.as_ref().unwrap().clone()
    }

    // java public void setPalette(KmoPalette palette) {
    pub fn setPalette(&self,palette:Rc<crate::cn::wps::moss::core::book::KmoPalette::KmoPalette>) {
*self.palette.borrow_mut()=palette;
    }

    // java public Dependent getDependent() {
    pub fn getDependent(&self) -> Option<Rc<RefCell<Dependent>>> {
        // java if (!this.isOom && !this.isLargeFile())
        if !self.isOOM() && !self.isLargeFile() {
            // java if (this.dep == null) this.dep = new Dependent(this.getSpreadsheetVersion());
            if self.dep.borrow().is_none() {
                *self.dep.borrow_mut() = Some(Rc::new(RefCell::new(Dependent::Dependent(self.getSpreadsheetVersion()))));
            }
            // java return this.dep;
            self.dep.borrow().clone()
        } else {
            // java return null;
            None
        }
    }

    // java public boolean isSynCalc() {
    pub fn isSynCalc(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.isSynCalc
        */
    }

    // java public void setCalcWay(boolean isSynCalc) {
    pub fn setCalcWay(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.isSynCalc = isSynCalc
        */
    }

    // java public CalculationChain getCalculationChain() {
    pub fn getCalculationChain(
        &self,
    ) -> Option<Rc<crate::cn::wps::moss::engine::eval::CalculationChain::CalculationChain>> {
        // java if (!this.isOom && !this.isLargeFile())
        if self.isOOM() || self.isLargeFile() {
            return None;
        }
        let mut chain = self.calcChain.borrow_mut();
        // java if (this.calcChain == null) this.calcChain = new CalculationChain(this,...);
        if chain.is_none() {
            let cc = Rc::new(crate::cn::wps::moss::engine::eval::CalculationChain::CalculationChain::CalculationChain(
                &self.me.upgrade().unwrap(),
                Rc::new(BookCalcChainListener(self.me.clone())),
            ));
            // java calcChain.setThreadAffinityCallback(this.threadAffinityCallback);
            cc.setThreadAffinityCallback(self.threadAffinityCallback.borrow().clone());
            *chain = Some(cc);
        }
        chain.clone()
    }

    // java public void markRegionAndCalc(final int sheetIndex, final KmoRect selection) {
    pub fn markRegionAndCalc(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java final CalculationChain cc = this.getCalculationChain()
        // java final KmoSheet sheet = this.sheetAt(sheetIndex)
        // java final KmoRect activeSel = sheet.getSelectionRect()
        // java if (cc != null)
        // java boolean isSynCalc = this.isSynCalc()
        // java cc.interrupt()
        // java if (!isSynCalc)
        // java cc.calculate(new CalcChain.ICalculationProgressCallback()
        // java if (false) public void onStart()
        // java cc.markRecalculation(sheetIndex, selection.tl.row, selection.br.row, selection.tl.col, selection.br.col)
        // java if (false) public boolean onProgress(int sheetx, int row, int col)
        // java return sheetx == sheetIndex && row == activeSel.tl.row && col == activeSel.tl.col
        // java if (false) public void onEnd()
        // java KmoBook.this.setDirty(true)
        // java KmoBook.this.getActiveSheet().changeListenerGroup().sheetContentsDidFinishChanging()
        // java KmoBook.this.areaWatcher.postContentChangedEvent(sheet, new KmoRect(activeSel))
        // java cc.markRecalculation(sheetIndex, selection.tl.row, selection.br.row, selection.tl.col, selection.br.col)
        // java cc.synCalculate()
        */
    }

    // java public void manageCellPic(int sheetIdex, Ptg[] ptgs, int mode) {
    pub fn manageCellPic(
        &self,
        sheetIndex: i32,
        ptgs: &[crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg],
        _mode: i32,
    ) {
        use crate::cn::wps::et::ss::formula::ptg::Ptg::PtgKind;
        // java resolve ExpPtg before inspecting NamePtg.
        let mut ptgs = ptgs.to_vec();
        if let Some(crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg {
            kind: PtgKind::Exp { row, col, .. },
            ..
        }) = ptgs.first()
        {
            let formula = self
                .sheetAt(sheetIndex)
                .borrow()
                .getExpFmlManager()
                .borrow()
                .query(*row, *col as i32, true);
            let Some(formula) = formula else {
                return;
            };
            ptgs = formula.getPtgs();
        }
        // java DISPIMG-only body; other formula changes have no side effects.
        if ptgs.len() >= 3 {
            if let PtgKind::Name { index } = ptgs[0].kind {
                if let Some(name) = self.getLinkTable().borrow().tryNameRecord(index).unwrap_or(None) {
                    if name.borrow().getNameText() == "_xlfn.DISPIMG"
                        && matches!(ptgs[1].kind, PtgKind::Str(_) | PtgKind::StrRaw { .. })
                    {
                        todo!("DEFERRED_NON_FUNCTION: manageCellPic DISPIMG");
                    }
                }
            }
        }

        /* DEFERRED_NON_FUNCTION
        // java if (ptgs != null && ptgs.length != 0)
        // java if (ptgs[0] instanceof ExpPtg)
        // java ExpPtg eptg = (ExpPtg)ptgs[0]
        // java ExpFmlManager.ExpFml evalfml = this.sheetAt(sheetIdex).getExpFmlManager().query(eptg.getRow(), eptg.getColumn(), true)
        // java if (evalfml == null)
        // java return
        // java ptgs = evalfml.getPtgs()
        // java if (ptgs.length >= 3 && ptgs[0] instanceof NamePtg)
        // java NamePtg ptg = (NamePtg)ptgs[0]
        // java NameRecord nameRecord = null
        // java nameRecord = this.getLinkTable().getNameRecord(ptg.getIndex())
        // java catch (Exception var10)
        // java if (null == nameRecord)
        // java return
        // java String formuleName = nameRecord.getNameText()
        // java if ("_xlfn.DISPIMG".equals(formuleName) && ptgs[1] instanceof StringPtg)
        // java String piName = ((StringPtg)ptgs[1]).getValue()
        // java KmoCellPic kmoCellPic = this.getDispingKmoPicture(piName.hashCode())
        // java if (kmoCellPic != null)
        // java if (mode == 2)
        // java kmoCellPic.del()
        // java else if (mode == 1)
        // java kmoCellPic.add()
        // java catch (Exception var9)
         */
    }

    // java private void managerHyperlinkForFml(int sheetIndex, int row, int col, Ptg[] ptgs, int mode) {
    pub fn managerHyperlinkForFml(
        &self,
        _sheet: i32,
        _row: i32,
        _col: i32,
        ptgs: &[crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg],
        _mode: i32,
    ) {
        // java only trailing FuncVarPtg with functionIndex==359 enters the hyperlink branch.
        if ptgs.last().is_some_and(|p| match &p.kind {
            crate::cn::wps::et::ss::formula::ptg::Ptg::PtgKind::FuncVar { meta, .. } => {
                meta.getIndex() == 359
            }
            crate::cn::wps::et::ss::formula::ptg::Ptg::PtgKind::FuncVarExt { index, .. } => {
                *index == 359
            }
            _ => false,
        }) {
            todo!("DEFERRED_NON_FUNCTION: formula hyperlink manager");
        }

        /* DEFERRED_NON_FUNCTION
        // java if (ptgs != null && ptgs.length != 0)
        // java if (ptgs[ptgs.length - 1] instanceof FuncVarPtg)
        // java KmoSheet sheet = this.sheetAt(sheetIndex)
        // java if (((FuncVarPtg)ptgs[ptgs.length - 1]).getFunctionIndex() == 359)
        // java if (this.fmlHyperLinkManager == null)
        // java this.fmlHyperLinkManager = new KmoFmlHyperLinkManager()
        // java if (mode == 1)
        // java String address = ""
        // java String label = null
        // java String markText = null
        // java Ptg[] tmpPtgs = new Ptg[ptgs.length - 1]
        // java for(int i = 0; i < ptgs.length - 1; ++i)
        // java tmpPtgs[i] = ptgs[i]
        // java int encodedLen = Ptg.getEncodedSizeWithoutArrayData(tmpPtgs)
        // java byte[] array = new byte[Ptg.getEncodedSize(tmpPtgs)]
        // java Ptg.serializePtgs(tmpPtgs, array, 0)
        // java EvaluationWorkbook workbook = new EvaluationWorkbook(this)
        // java WorkbookEvaluator workbookEvaluator = new WorkbookEvaluator(workbook)
        // java WorkbookBytesEvaluator workbookBytesEvaluator = new WorkbookBytesEvaluator(workbookEvaluator)
        // java EvaluationContext ec = new EvaluationContext(workbookEvaluator, workbook, sheet.getSheetIndex(), -1, -1, false)
        // java LinkedList<IValueEval> evalList = workbookBytesEvaluator.evaluateFormula(ec, array, encodedLen)
        // java if (evalList.size() == 1)
        // java IValueEval addressEval = (IValueEval)evalList.removeFirst()
        // java if (addressEval instanceof StringEval)
        // java address = ((StringEval)addressEval).getStringValue()
        // java if (evalList.size() != 2)
        // java return
        // java IValueEval labelEval = (IValueEval)evalList.removeFirst()
        // java IValueEval addressEval = (IValueEval)evalList.removeFirst()
        // java if (labelEval instanceof StringEval)
        // java label = ((StringEval)labelEval).getStringValue()
        // java if (addressEval instanceof StringEval)
        // java address = ((StringEval)addressEval).getStringValue()
        // java if (KmoHyperLinkManager.isContainIllegalBlank(address))
        // java return
        // java KmoRect hyperRange = new KmoRect(row, col, row, col)
        // java sheet.hyperlinkMgr().clearRangeHyperlinks(hyperRange, true)
        // java KmoHyperlink hyperlink = new KmoHyperlink()
        // java if (sheet.hyperlinkMgr().isFileLinkString(address))
        // java hyperlink.setHyperlinkType(0)
        // java else if (sheet.hyperlinkMgr().isHyperlinkString(address))
        // java hyperlink.setHyperlinkType(1)
        // java else if (address.startsWith("#"))
        // java hyperlink.setHyperlinkType(2)
        // java address = address.substring(1)
        // java markText = address
        // java else if (address.indexOf(91) == 0 && address.lastIndexOf(93) > 0)
        // java char[] tmp = address.toCharArray()
        // java int stack = 0
        // java for(int i = 0; i < tmp.length; ++i)
        // java if (tmp[i] == '[')
        // java ++stack
        // java else if (tmp[i] == ']')
        // java --stack
        // java if (stack == 0)
        // java stack = i
        // java break
        // java address = address.substring(1, stack)
        // java hyperlink.setHyperlinkType(0)
        // java hyperlink.setShortFileName(address)
        // java hyperlink.setHyperlinkType(0)
        // java hyperlink.setRange(hyperRange)
        // java hyperlink.setAddress(address)
        // java hyperlink.setLabel(label == null ? address : label)
        // java hyperlink.setTextMark(markText)
        // java this.fmlHyperLinkManager.addHLink(hyperlink, row, col, sheetIndex)
        // java else if (mode == 2)
        // java this.fmlHyperLinkManager.removeLink(row, col, sheetIndex)
         */
    }

    // java public void interruptCalc() {
    pub fn interruptCalc(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.getCalculationChain().interrupt()
        */
    }

    // java public void setOOM() {
    pub fn setOOM(&self) {
        // java this.isOom = true; this.bReadOnly = true; this.calcChain = null; this.dep = null;
        self.isOom.set(true);
        self.bReadOnly.set(true);
        self.dep.borrow_mut().take();
        self.calcChain.borrow_mut().take();
    }

    // java public boolean isOOM() {
    pub fn isOOM(&self) -> bool {
        // java return this.isOom;
        self.isOom.get()
    }

    // java public void setIsLegal(boolean isLegal) {
    pub fn setIsLegal(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mIsLegal = isLegal
        */
    }

    // java public boolean isLegal() {
    pub fn isLegal(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.mIsLegal
        */
    }

    // java public void setCompletelyReaded() {
    pub fn setCompletelyReaded(&self) {
        // java this.isCompletelyReaded = true;
        self.isCompletelyReaded.set(true);
    }

    // java public boolean isCompletelyReaded() {
    pub fn isCompletelyReaded(&self) -> bool {
        // java return this.isCompletelyReaded;
        self.isCompletelyReaded.get()
    }

    // java public boolean isLargeFile() {
    pub fn isLargeFile(&self) -> bool {
        // java return this.bookReader != null && this.bookReader.isLargestFile();
        self.isLargestFile.get()
    }

    // java public void setCalcChainInitialized() {
    pub fn setCalcChainInitialized(&self) {
        // java this.calcChainInitialized = true;
        self.calcChainInitialized.set(true);
    }

    // java public boolean isCalcChainInitialized() {
    pub fn isCalcChainInitialized(&self) -> bool {
        // java return this.calcChainInitialized;
        self.calcChainInitialized.get()
    }

    // rust: Owned handle for Java object identity across the IGridSheet facade.
    pub fn xfManagerHandle(&self)->Rc<RefCell<KmoXFManager>>{self.xfManager.clone()}
    // java public KmoXFManager getXFManager() {
    pub fn getXFManager(&self) -> &RefCell<KmoXFManager> {
        // java return this.xfManager;
        &self.xfManager
    }

    // java public void setBookReader(BookReader reader) {
    pub fn setBookReader(&self,r:Option<Rc<dyn crate::cn::wps::moss::io::BookReader::BookReader>>){*self.bookReader.borrow_mut()=r;}

    // java public BookReader getBookReader() {
    pub fn getBookReader(&self)->Option<Rc<dyn crate::cn::wps::moss::io::BookReader::BookReader>>{self.bookReader.borrow().clone()}

    // java public Set<Integer> getNotEmptySelectIndex(IDocument iDocument, Set<Integer> selectedIndex) {
    pub fn getNotEmptySelectIndex(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java MergeKmoBookProvider mergeKmoBookProvider = new MergeKmoBookProvider()
        // java return mergeKmoBookProvider.getNotEmptySelectIndex(iDocument, selectedIndex)
        */
    }

    // java public boolean hasEmptySheet(IDocument iDocument, boolean allSelected, Set<Integer> index) {
    pub fn hasEmptySheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java MergeKmoBookProvider mergeKmoBookProvider = new MergeKmoBookProvider()
        // java return mergeKmoBookProvider.hasEmptySheet(iDocument, allSelected, index)
        */
    }

    // java public boolean isAllSheetEmpty(IDocument iDocument, boolean allSelected, Set<Integer> index, boolean hasEmptySheet) {
    pub fn isAllSheetEmpty(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java MergeKmoBookProvider mergeKmoBookProvider = new MergeKmoBookProvider()
        // java return mergeKmoBookProvider.isAllSheetEmpty(iDocument, allSelected, index, hasEmptySheet)
        */
    }

    // java public Set<Integer> getSelectedIndex(IDocument iDocument, boolean allSelected, Set<Integer> index, Set<Integer> selectedIndex) {
    pub fn getSelectedIndex(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java MergeKmoBookProvider mergeKmoBookProvider = new MergeKmoBookProvider()
        // java return mergeKmoBookProvider.getSelectedIndex(iDocument, allSelected, index, selectedIndex)
        */
    }

    // java public Set<Integer> getNoVeryHiddenSelectedIndex(IDocument iDocument, Set<Integer> selectedIndex) {
    pub fn getNoVeryHiddenSelectedIndex(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoBook kmoBook = null
        // java if (iDocument instanceof KmoBook)
        // java kmoBook = (KmoBook)iDocument
        // java if (null == kmoBook)
        // java return selectedIndex
        // java ArrayList<Integer> veryHiddenIndexes = new ArrayList()
        // java for(int index : selectedIndex)
        // java KmoSheet kmoSheet = kmoBook.sheetAt(index)
        // java if (null != kmoSheet && kmoSheet.isSheetVeryHidden())
        // java veryHiddenIndexes.add(index)
        // java if (veryHiddenIndexes.size() > 0)
        // java for(Integer deletedIndex : veryHiddenIndexes)
        // java selectedIndex.remove(deletedIndex)
        // java return selectedIndex
        */
    }

    // java public String getSelectSheetName(IDocument iDocument, boolean allSelected, Set<Integer> index) {
    pub fn getSelectSheetName(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java MergeKmoBookProvider mergeKmoBookProvider = new MergeKmoBookProvider()
        // java return mergeKmoBookProvider.getSelectSheetName(iDocument, allSelected, index)
        */
    }

    // java public KmoConnections getKmoConnections() {
    pub fn getKmoConnections(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.kmoConnections
        */
    }

    // java public KmoPivotRecordsStore getPivotRecordsStore() {
    pub fn getPivotRecordsStore(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.pivotRecordsStore == null)
        // java this.pivotRecordsStore = new KmoPivotRecordsStore()
        // java return this.pivotRecordsStore
        */
    }

    // java public ThreadAffinityCallback getThreadAffinityCallback() {
    pub fn getThreadAffinityCallback(
        &self,
    ) -> Option<Rc<dyn super::ThreadAffinityCallback::ThreadAffinityCallback>> {
        // java return this.threadAffinityCallback;
        self.threadAffinityCallback.borrow().clone()
    }
    // java public void setThreadAffinityCallback(ThreadAffinityCallback threadAffinityCallback)
    pub fn setThreadAffinityCallback(
        &self,
        callback: Option<Rc<dyn super::ThreadAffinityCallback::ThreadAffinityCallback>>,
    ) {
        // java this.threadAffinityCallback=threadAffinityCallback;
        *self.threadAffinityCallback.borrow_mut() = callback;
    }

    // java public boolean hasDashboard() {
    pub fn hasDashboard(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null != this.webExtensionManager && this.webExtensionManager.size() > 0)
        // java int i = 0
        // java for(int len = this.sheetCount(); i < len; ++i)
        // java if (this.sheets.sheetAt(i).isDashboard())
        // java return true
        // java return false
        */
    }

    // java public List<FeatureLost> detectFeaturesLost(int suffix) {
    pub fn detectFeaturesLost(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java List<FeatureLost> losted = new ArrayList()
        // java if (suffix == 1)
        // java return losted
        // java if (suffix == 0)
        // java boolean autofilter_fl = false
        // java int sheetCount = this.sheetCount()
        // java for(int i = 0; i < sheetCount; ++i)
        // java KmoSheet sheet = this.sheetAt(i)
        // java if (!autofilter_fl)
        // java KmoAutoFilter af = sheet.sheetOp().autoFilter()
        // java if (af != null)
        // java for(FilterColumn fc : af.getRecords())
        // java switch (fc.getFilterType())
        // java case COLOR:
        // java case ICON:
        // java case DYNAMIC:
        // java case EXTLST:
        // java autofilter_fl = true
        // java case CUSTOM:
        // java case TOP10:
        // java default:
        // java break
        // java case FILTERS:
        // java FiltersFilter ff = (FiltersFilter)fc
        // java if (ff.dateGroupItems != null && ff.dateGroupItems.size() > 0)
        // java autofilter_fl = true
        // java int count = 0
        // java if (ff.blanks)
        // java ++count
        // java if (ff.filters != null)
        // java count += ff.filters.size()
        // java if (count > 2)
        // java autofilter_fl = true
        // java if (autofilter_fl)
        // java break
        // java if (autofilter_fl)
        // java losted.add(KmoBook.FeatureLost.AUTOFILTER)
        // java return losted
        */
    }

    // java public boolean saveImpl(File file, File dir, int suffix) throws IOException {
    pub fn saveImpl(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return false
        */
    }

    // java private void beforeSaving(String outpath) {
    pub fn beforeSaving(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoLog.logPoint()
        // java String dir = StringUtil.getNameDelLastPath(outpath)
        // java if (!(new File(dir)).exists())
        // java (new File(dir)).mkdirs()
        // java String strSuffix = StringUtil.pathExtension(outpath).toLowerCase()
        // java if (!strSuffix.equals("xlt") && !strSuffix.equals("xltx") && !strSuffix.equals("xltm"))
        // java if (this.isTemplate())
        // java this.setTemplate(false)
        // java this.setDirty(true)
        // java else if (!this.isTemplate())
        // java this.setTemplate(true)
        // java this.setDirty(true)
        // java if (!strSuffix.equals("xltm") && !strSuffix.equals("xlsm"))
        // java if (this.isMacroEnabled())
        // java this.setMacroEnabled(false)
        // java this.setDirty(true)
        // java else if (!this.isMacroEnabled())
        // java this.setMacroEnabled(true)
        // java this.setDirty(true)
        // java this.dealHpyerlinkRelativePath(outpath)
        // java KmoLog.logPoint()
        */
    }

    // java private void dealHpyerlinkRelativePath(String outpath) {
    pub fn dealHpyerlinkRelativePath(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int size = this.sheets.size()
        // java boolean isDirty = false
        // java for(int i = 0; i < size; ++i)
        // java KmoSheet sheet = this.sheetAt(i)
        // java KmoHyperLinkManager linkManager = sheet.hyperlinkMgr()
        // java boolean isChange = linkManager.turnRelativePath(this.filePath, outpath)
        // java if (isChange)
        // java isDirty = true
        // java if (isDirty)
        // java this.setDirty(true)
        */
    }

    // java private void copySave(File srcfile, File destFile) throws IOException {
    pub fn copySave(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoLog.logPoint()
        // java if (!this.filePath.equalsIgnoreCase(destFile.getAbsolutePath()))
        // java if (destFile.exists())
        // java KmoLog.logPoint("backupFile")
        // java SaveHelper.backupFile(AppType.SpreadSheet, destFile)
        // java KmoLog.logPoint("backupFile end")
        // java KmoLog.logPoint("copyFile")
        // java SaveHelper.copyFile(srcfile, destFile)
        // java this.filePath = destFile.getAbsolutePath()
        // java KmoLog.logPoint("copyFile end")
        // java else if (!this.filePath.equals(destFile.getAbsolutePath()))
        // java KmoLog.logPoint("moveFile")
        // java FileUtil.moveFile(srcfile, destFile)
        // java KmoLog.logPoint("moveFile end")
        // java if (this.pasteMgr.isInPasteState())
        // java this.pasteMgr.cancelPasteState()
        // java KmoLog.logPoint()
        */
    }

    // java public void saveFreeze() {
    pub fn saveFreeze(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int count = this.sheetCount()
        // java for(int i = 0; i < count; ++i)
        // java this.sheets.sheetAt(i).saveFreeze()
        */
    }

    // java private boolean realSave(File destFile, int suffix) throws IOException, RuntimeException {
    pub fn realSave(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoLog.logPoint()
        // java boolean result = false
        // java if (null != destFile && destFile.exists())
        // java KmoLog.logPoint("#realSave start destFile: " + destFile.getAbsoluteFile() + " length: " + destFile.length())
        // java String globalTemp = Platform.getTempDirectory(this.filePath)
        // java File tempDirFile = new File(globalTemp, "Spreadsheet")
        // java if (!tempDirFile.exists())
        // java tempDirFile.mkdirs()
        // java result = this.saveImpl(destFile, tempDirFile, suffix)
        // java if (result)
        // java this.filePath = destFile.getAbsolutePath()
        // java this.fileFormat = suffix
        // java catch (Throwable e)
        // java throw new RuntimeException(e)
        // java if (null != destFile && destFile.exists())
        // java KmoLog.logPoint("#realSave end destFile: " + destFile.getAbsoluteFile() + " length: " + destFile.length())
        // java KmoLog.logPoint("end")
        // java return result
        */
    }

    // java private void afterSaving() {
    pub fn afterSaving(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoLog.logPoint()
        // java this.transMgr.reset()
        // java this.isNewBook = false
        // java this.isNewOpenDocument = false
        // java this.isSilentSaveWithoutDirty = false
        // java this.setDirty(false)
        // java this.changeListeners.bookSettingsDidFinishChanging()
        // java ISlimToolManager.instance().clearCache()
        // java KmoLog.logPoint("end")
        */
    }

    // java public synchronized boolean save(String outpath, int suffix) throws IOException {
    pub fn save(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.save(outpath, suffix, (FileSaveCallback)null, (OnlineSecurityTool)null)
        */
    }

    // java public synchronized boolean save(String destPath, int suffix, FileSaveCallback callback, OnlineSecurityTool tool) throws IOException {
    pub fn save__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoLog.startSaveLog(destPath)
        // java boolean result
        // java if (destPath == null)
        // java return false
        // java if (suffix != 0 && suffix != 1 && suffix != 4)
        // java return false
        // java boolean tmpResult = false
        // java this.beforeSaving(destPath)
        // java this.mSaveCompleted = false
        // java boolean isOnlineEncrypt = null != tool && tool.isEnable()
        // java String oldFilePath = this.filePath
        // java File destFile = new File(destPath)
        // java File srcFile = null
        // java if (this.filePath != null && this.filePath.length() != 0)
        // java srcFile = new File(this.filePath)
        // java if (srcFile != null)
        // java if (isOnlineEncrypt)
        // java if (destFile.getName().equals(srcFile.getName()))
        // java this.copyWhenSaveAsNoDirty(suffix, destFile, srcFile)
        // java tmpResult = true
        // java this.copyWhenSaveAsNoDirty(suffix, destFile, srcFile)
        // java tmpResult = true
        // java if (!this.mSaveCompleted)
        // java tmpResult = this.realSave(destFile, suffix)
        // java this.mSaveCompleted = true
        // java if (null != callback)
        // java tmpResult = callback.extend(destFile.getAbsolutePath())
        // java if (oldFilePath != null)
        // java KmoLog.logPoint("oldFilePath file length:" + (new File(oldFilePath)).length())
        // java this.afterSaving()
        // java result = tmpResult
        // java catch (IOException e)
        // java if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // java throw new NoSpaceLeftException(e)
        // java this.exportDebugInfomation(destPath)
        // java throw e
        // java catch (RuntimeException e)
        // java if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // java throw new NoSpaceLeftException(e)
        // java this.exportDebugInfomation(destPath)
        // java throw e
        // java KmoLog.endSaveLog()
        // java return result
        */
    }

    // java private void copyWhenSaveAsNoDirty(int suffix, File destFile, File srcFile) throws IOException {
    pub fn copyWhenSaveAsNoDirty(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.fileFormat == suffix && !this.isDirty() && suffix != 4 && !this.isNewBook() && srcFile.exists())
        // java this.copySave(srcFile, destFile)
        // java this.mSaveCompleted = true
        */
    }

    // java void startMonitor() {
    pub fn startMonitor(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.logFileObserver == null)
        // java this.logFileObserver = (new KLogFileObserverUtil()).newObserver()
        // java if (this.logFileObserver != null)
        // java this.logFileObserver.start()
        */
    }

    // java void stopMonitor() {
    pub fn stopMonitor(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.logFileObserver != null)
        // java this.logFileObserver.stop()
        // java this.logFileObserver = null
        */
    }

    // java private String getSDPath() {
    pub fn getSDPath(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java File sdDir = null
        // java boolean sdCardExist = Environment.getExternalStorageState().equals("mounted")
        // java if (sdCardExist)
        // java sdDir = KfsFileUtil.convertOrgToKfsFile(Environment.getExternalStorageDirectory())
        // java if (null != sdDir)
        // java return sdDir.toString()
        // java return "NotExist"
        */
    }

    // java private void exportDebugInfomation(String outpath) {
    pub fn exportDebugInfomation(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java String dir = StringUtil.getNameDelLastPath(outpath)
        // java File dirFile = new File(dir)
        // java String perm
        // java for(perm = null; dirFile != null && !dirFile.exists(); dirFile = dirFile.getParentFile())
        // java if (dirFile != null)
        // java perm = "workdir:" + dirFile.getPath() + " r:" + dirFile.canRead() + " w:" + dirFile.canWrite() + " e:" + dirFile.canExecute() + " SdCard:" + this.getSDPath()
        // java GlobalDebugInformation.DebugInfomation = "excel outpath:" + outpath + " " + perm
        */
    }

    // java public void updateCore4Save(boolean isSavedAs) {
    pub fn updateCore4Save(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoAppSettings appSettings = this.getAppSettings()
        // java if (null != appSettings)
        // java appSettings.updateIcv(isSavedAs)
        */
    }

    // java public boolean needMakeSnapshot() {
    pub fn needMakeSnapshot(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return !this.bReadOnly && this.filePath != null && this.isDirty() && !this.isMakingSnapshot && !KmoAtomStore.sharedStore().isTheSameTipStamp(this.tip)
        */
    }

    // java public boolean makeSnapshot(String destPath, FileSaveCallback callback) throws IOException {
    pub fn makeSnapshot(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (!this.needMakeSnapshot())
        // java return false
        // java this.tip = KmoAtomStore.sharedStore().getCurrentTipStamp()
        // java return this.makeCommonShot(destPath, callback)
        */
    }

    // java public boolean makeTvSnapshot(String destPath) throws IOException {
    pub fn makeTvSnapshot(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.makeCommonShot(destPath, (FileSaveCallback)null)
        */
    }

    // java private boolean makeCommonShot(String destPath, FileSaveCallback callback) throws IOException {
    pub fn makeCommonShot(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.isMakingSnapshot = true
        // java KmoLog.logPoint("#makeCommonShot#head isMakingSnapshot: true")
        // java File destFile = null
        // java boolean success
        // java boolean tempResult = false
        // java destFile = new File(destPath)
        // java if (null != destFile)
        // java int ff = 0
        // java if (this.getSpreadsheetVersion() == SpreadsheetVersion.EXCEL2007)
        // java ff = 1
        // java String tempDir = Platform.getTempDirectory(destFile.getAbsolutePath())
        // java tempResult = this.saveImpl(destFile, new File(tempDir), ff)
        // java if (null != callback)
        // java tempResult = callback.extend(destFile.getAbsolutePath())
        // java success = tempResult
        // java this.isMakingSnapshot = false
        // java this.interruptAutoSave = false
        // java KmoLog.logPoint("#makeCommonShot#end isMakingSnapshot: false")
        // java return success
        */
    }

    // java private static String getExternalSnapshot(String srcfileStr) {
    pub fn getExternalSnapshot(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (srcfileStr != null && SnapshotMgr.hasSnapshot(srcfileStr))
        // java java.io.File snapshot = SnapshotMgr.getSnapshotPath(srcfileStr)
        // java if (!snapshot.exists())
        // java return null
        // java java.io.File srcfile = new File(srcfileStr)
        // java return srcfile.exists() && snapshot.lastModified() <= srcfile.lastModified() ? null : snapshot.getAbsolutePath()
        // java return null
        */
    }

    // java public void setIsNewBook(boolean b) {
    pub fn setIsNewBook(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.isNewBook = b
        */
    }

    // java public boolean isNewBook() {
    pub fn isNewBook(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.isNewBook
        */
    }

    // java public boolean getPureDirtyValue() {
    pub fn getPureDirtyValue(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.dirty
        */
    }

    // java public boolean isSilentSaveWithoutDirty() {
    pub fn isSilentSaveWithoutDirty(&self)->bool {
        // java return this.isSilentSaveWithoutDirty;
        self.isSilentSaveWithoutDirty.get()
    }

    // java public void setSilentSaveWithoutDirty(boolean silentSaveWithoutDirty) {
    pub fn setSilentSaveWithoutDirty(&self,silentSaveWithoutDirty:bool) {
        // java this.isSilentSaveWithoutDirty = silentSaveWithoutDirty;
        self.isSilentSaveWithoutDirty.set(silentSaveWithoutDirty);
    }

    // java public boolean isNewOpenDocument() {
    pub fn isNewOpenDocument(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.isNewOpenDocument
        */
    }

    // java public void setIsNewOpenDocument(boolean isOrNot) {
    pub fn setIsNewOpenDocument(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.isNewOpenDocument = isOrNot
        */
    }

    // java public void setIsIOState(boolean val) {
    pub fn setIsIOState(&self, val: bool) {
        // java this.isIOState = val;
        self.isIOState.set(val);
    }

    // java public boolean isIOState() {
    pub fn isIOState(&self) -> bool {
        // java return this.isIOState;
        self.isIOState.get()
    }

    // java public void setShouldDirty(boolean shouldDirty) {
    pub fn setShouldDirty(&self, should_dirty: bool) {
        // java this.mShouldDirty = shouldDirty;
        self.mShouldDirty.set(should_dirty);
    }

    // java public void setDirty(boolean d) {
    pub fn setDirty(&self, d: bool) {
        // java if (this.mShouldDirty)
        if self.mShouldDirty.get() {
            // java if (this.isMakingSnapshot && d)
            if self.isMakingSnapshot.get() && d {
                // java this.setInterruptAutoSave(true);
                self.setInterruptAutoSave(true);
                // java KmoLog.logPoint("#setDirty setInterruptAutoSave(true)");
                // rust: no-stmt — KmoLog 不在这一档，中断自动保存的标志已经写上。
            }
            // java if (this.dirty != d)
            if self.dirty.get() != d {
                // java if (!this.isIOState)
                if !self.isIOState.get() {
                    // java this.dirty = d;
                    self.dirty.set(d);
                }
                // java this.changeListeners.bookSettingsDidFinishChanging();
                self.changeListeners.bookSettingsDidFinishChanging();
            }
        }
    }

    // java public boolean isDirty() {
    pub fn isDirty(&self) -> bool {
        // java return this.dirty || this.isSilentSaveWithoutDirty;
        self.dirty.get() || self.isSilentSaveWithoutDirty.get()
    }

    // java public boolean isSecurityFile() {
    pub fn isSecurityFile(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return false
        */
    }

    // java public void setWindow1(Window1 win1) {
    pub fn setWindow1(&self,win1:Option<Rc<Window1>>) {
        // java this.window1 = win1;
        *self.window1.borrow_mut()=win1;
    }

    // java public Window1 getWindow1() {
    pub fn getWindow1(&self)->Option<Rc<Window1>> {
        // java return this.window1;
        self.window1.borrow().clone()
    }

    // java public boolean showSheetTabs() {
    pub fn showSheetTabs(&self)->bool {
        // java return this.window1.showTabs();
        self.window1.borrow().clone().expect("NullPointerException").showTabs()
    }

    // java public void setShowSheetTabs(boolean s) {
    pub fn setShowSheetTabs(&self,s:bool) {
        // java this.window1.setShowTabs(s);
        self.window1.borrow().clone().expect("NullPointerException").setShowTabs(s);
    }

    // java public boolean getShowSheetTabs() {
    pub fn getShowSheetTabs(&self)->bool {
        // java return this.window1.showTabs();
        self.window1.borrow().clone().expect("NullPointerException").showTabs()
    }

    // java public boolean isUsing1904() {
    pub fn isUsing1904(&self) -> bool {
        // java return this.using1904;
        self.using1904.get()
    }

    // java public void setUsing1904(boolean u) {
    pub fn setUsing1904(&self, u: bool) {
        // java this.using1904 = u;
        self.using1904.set(u);
    }

    // java public boolean isTemplate() {
    pub fn isTemplate(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.isTemplate
        */
    }

    // java public void setTemplate(boolean template) {
    pub fn setTemplate(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.isTemplate = template
        */
    }

    // java public boolean isMacroEnabled() {
    pub fn isMacroEnabled(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.bMacroEnabled
        */
    }

    // java public void setMacroEnabled(boolean macroEnabled) {
    pub fn setMacroEnabled(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.bMacroEnabled = macroEnabled
        */
    }

    // java public BookDrawingAgg getBookDrawingAgg() {
    pub fn getBookDrawingAgg(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.bookDrawingAgg
        */
    }

    // java public KmoBookChangeEventsListener changeListenerGroup() {
    pub fn changeListenerGroup(&self) -> &super::KmoBookChangeListenerGroup::KmoBookChangeListenerGroup {
        // java return this.changeListeners
        &self.changeListeners
    }

    // java public KmoBookTransListenerGroup transListenerGroup() {
    pub fn transListenerGroup(&self) -> &super::event::KmoBookTransListenerGroup::KmoBookTransListenerGroup {
        // java return this.transListeners
        &self.transListeners
    }

    // java public AreaWatcher areaWatcher() {
    pub fn areaWatcher(&self) -> &RefCell<AreaWatcher> {
        // java return this.areaWatcher;
        &self.areaWatcher
    }

    // java public void setReadOnly(boolean bOnly) {
    pub fn setReadOnly(&self, bOnly: bool) {
        // java this.bReadOnly = bOnly
        self.bReadOnly.set(bOnly);
    }

    // java public boolean getReadOnly() {
    pub fn getReadOnly(&self) -> bool {
        // java return this.bReadOnly
        self.bReadOnly.get()
    }

    // java private boolean markRecalWithTransaction(HashMap<Integer, List<KmoRect>> rectMap) {
    pub fn markRecalWithTransaction(&self, rectMap: &std::collections::BTreeMap<i32, Vec<KmoRect>>) -> bool {
        // java synchronized(this)
        let _monitor = self.monitor.lock();
        // java boolean ret = false
        let mut ret = false;
        // java this.markInTransaction = true
        self.markInTransaction.set(true);
        // rust: preserve finally on exceptions from marking.
        struct Restore<'a>(&'a Cell<bool>); impl Drop for Restore<'_>{fn drop(&mut self){self.0.set(true);}}
        let _finally = Restore(&self.markInTransaction);
        // java boolean hasMark = false
        let mut hasMark = false;
        // java for(Integer key : rectMap.keySet())
        // rust: getCurMarkRect supplies dense nonnegative sheet indices, visited
        // in this same order by Java HashMap's buckets.
        for (key, rects) in rectMap {
        // java if (!this.markInTransaction)
        // java break
            if !self.markInTransaction.get() { break; }
        // java List<KmoRect> rects = rectMap.get(key)
        // java if (rects == null)
        // java continue
            // rust: getCurMarkRect always creates a non-null list.
        // java for(KmoRect tRect : rects)
            for tRect in rects {
        // java if (!this.markInTransaction)
        // java break
                if !self.markInTransaction.get() { break; }
        // java if (tRect != null)
                // rust: recorded ranges are non-null KmoRect values.
        // java this.getCalculationChain().markRecalculation(key, tRect.tl.row, tRect.br.row, tRect.tl.col, tRect.br.col)
                self.getCalculationChain().expect("CalculationChain").markRecalculation__area(*key,tRect.tl.row,tRect.br.row,tRect.tl.col,tRect.br.col,true);
        // java hasMark = true
                hasMark = true;
        // java ret = true
                ret = true;
            }
        }
        // java if (!hasMark)
        // java this.getCalculationChain().markVolatile()
        if !hasMark { self.getCalculationChain().expect("CalculationChain").markVolatile(); }
        // java this.markInTransaction = true
        // rust: restored by the finally guard above.
        // java return ret
        ret
    }

    // java private HashMap<Integer, List<KmoRect>> getCurMarkRect() {
    pub fn getCurMarkRect(&self) -> std::collections::BTreeMap<i32, Vec<KmoRect>> {
        // java HashMap<Integer, List<KmoRect>> retList = new HashMap(this.sheetCount());
        let mut retList = std::collections::BTreeMap::new();
        // java for(int i = 0; i < this.sheetCount(); ++i) {
        for i in 0..self.sheetCount() {
            // java KmoSheet sheet = this.sheetAt(i);
            let sheet = self.sheetAt(i); let sheet = sheet.borrow();
            // java retList.put(sheet.getSheetIndex(), new ArrayList(sheet.getUTransactionList()));
            retList.insert(sheet.getSheetIndex(), sheet.getUTransactionList());
            // java sheet.cleanUTransactionList();
            sheet.cleanUTransactionList();
        }
        // java return retList;
        retList
    }

    // java public synchronized void undo() {
    pub fn undo(&self) {
        let _monitor = self.monitor.lock();
        // java if (this.canUndo())
        if self.canUndo() {
        // java if (this.transMgr.isTransStarted())
        // java this.transMgr.commit()
            if self.trans_mgr.isTransStarted() { self.trans_mgr.commit(); }
        // java this.activateSheet(this.transMgr.back())
            self.activateSheet(self.trans_mgr.back());
        // java this.preperReCalc()
            self.preperReCalc();
        // java KmoAtomStore.sharedStore().undo()
            super::TransactionManager::withStore(|s| s.undo());
        // java this.changeListeners.bookSettingsDidFinishChanging()
            self.changeListeners.bookSettingsDidFinishChanging();
        // java this.transListeners.onUndo()
            self.transListeners.onUndo();
        // java int i = 0
        // java for(int len = this.sheetCount(); i < len; ++i)
            for i in 0..self.sheetCount() {
        // java KmoSheet sheet = this.sheetAt(i)
                let sheet = self.sheetAt(i);
        // java sheet.changeListenerGroup().sheetContentsDidFinishChanging()
                sheet.borrow().changeListenerGroup().sheetContentsDidFinishChanging();
            }
        // java this.interruptAutoSave = true
            self.interruptAutoSave.set(true);
        // java this.reCalcInBackground()
            self.reCalcInBackground();
        }
    }

    // java public synchronized void redo() {
    pub fn redo(&self) {
        let _monitor = self.monitor.lock();
        // java if (this.canRedo())
        if self.canRedo() {
        // java this.activateSheet(this.transMgr.forward())
            self.activateSheet(self.trans_mgr.forward());
        // java this.preperReCalc()
            self.preperReCalc();
        // java KmoAtomStore.sharedStore().redo()
            super::TransactionManager::withStore(|s| s.redo());
        // java this.changeListeners.bookSettingsDidFinishChanging()
            self.changeListeners.bookSettingsDidFinishChanging();
        // java this.transListeners.onRedo()
            self.transListeners.onRedo();
        // java int i = 0
        // java for(int len = this.sheetCount(); i < len; ++i)
            for i in 0..self.sheetCount() {
        // java KmoSheet sheet = this.sheetAt(i)
                let sheet = self.sheetAt(i);
        // java sheet.changeListenerGroup().sheetContentsDidFinishChanging()
                sheet.borrow().changeListenerGroup().sheetContentsDidFinishChanging();
            }
        // java this.interruptAutoSave = true
            self.interruptAutoSave.set(true);
        // java this.reCalcInBackground()
            self.reCalcInBackground();
        }
    }

    // java private void preperReCalc() {
    pub fn preperReCalc(&self) {
        // java CalculationChain calculationChain = this.getCalculationChain()
        let calculationChain = self.getCalculationChain();
        // java if (calculationChain != null)
        if let Some(calculationChain) = calculationChain {
        // java calculationChain.interrupt()
            calculationChain.interrupt();
        // java calculationChain.interruptMark()
            calculationChain.interruptMark();
        // java this.markInTransaction = false
            self.markInTransaction.set(false);
        }
    }

    // java private void reCalcInBackground() {
    pub fn reCalcInBackground(&self) {
        // java final CalculationChain calculationChain = this.getCalculationChain()
        let calculationChain = self.getCalculationChain();
        // java if (calculationChain != null)
        if let Some(calculationChain) = calculationChain {
        // java final KmoSheet activateSheet = this.getActiveSheet()
            let activateSheet = self.getActiveSheet();
        // java final HashMap<Integer, List<KmoRect>> markList = this.getCurMarkRect()
            let markList = RefCell::new(self.getCurMarkRect());
        // java calculationChain.calculate(new CalcChain.ICalculationCallback()
            let callback = BookRecalculation { book: self.me.upgrade().expect("KmoBook"), calculationChain: calculationChain.clone(), activateSheet, markList };
            calculationChain.calculate__callback(Some(Rc::new(callback)));
        }
    }

    // java public boolean canUndo() {
    pub fn canUndo(&self) -> bool {
        // java return KmoAtomStore.sharedStore().canUndo()
        super::TransactionManager::withStore(|s| s.canUndo())
    }

    // java public boolean canRedo() {
    pub fn canRedo(&self) -> bool {
        // java return KmoAtomStore.sharedStore().canRedo()
        super::TransactionManager::withStore(|s| s.canRedo())
    }

    // java public boolean isOOXmlBook() {
    pub fn isOOXmlBook(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.fileFormat == 1
        */
    }

    // java public KmoBookProtect getBookProtect() {
    pub fn getBookProtect(&self)->Rc<crate::cn::wps::moss::core::book::KmoBookProtect::KmoBookProtect> {
        // java return this.bookProtect;
        self.bookProtect.clone()
    }

    // java public String getFilePath() {
    pub fn getFilePath(&self)->Option<String>{
        // java return this.filePath;
        self.filePath.borrow().clone()
    }

    // java public RandomAccessFile getRaf() {
    pub fn getRaf(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.raf
        */
    }

    // java public int sheetMergeIndex(EtSheet sheet) {
    pub fn sheetMergeIndex(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoSheet kmoSheet = (KmoSheet)sheet
        // java return this.sheetIndex(kmoSheet)
        */
    }

    // java public void setRaf(RandomAccessFile randomAccessFile) {
    pub fn setRaf(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.raf = randomAccessFile
        */
    }

    // java public void setFilePath(String path) {
    pub fn setFilePath(&self, path: &str) {
        // java this.filePath = path
        *self.filePath.borrow_mut() = Some(path.to_string());
    }

    // java public void setFileFormat(int type) {
    pub fn setFileFormat(&self, type_: i32) {
        // java this.fileFormat = type
        self.fileFormat.set(type_);
    }

    // java public int getFileFormat() {
    pub fn getFileFormat(&self) -> i32 {
        // java return this.fileFormat
        self.fileFormat.get()
    }

    // java public KmoBookOp bookOp() {
    pub fn bookOp(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.bookOp
        */
    }

    // java public boolean isSheetNameUnique(int sheetIndex, String newSheetName) {
    pub fn isSheetNameUnique(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (newSheetName != null && newSheetName.length() != 0)
        // java newSheetName = TextHelper.full2Half(newSheetName.toLowerCase())
        // java int i = 0
        // java for(int size = this.sheets.size(); i < size; ++i)
        // java String sn = this.sheets.sheetAt(i).name()
        // java sn = TextHelper.full2Half(sn.toLowerCase())
        // java if (i != sheetIndex && newSheetName.equals(sn))
        // java return false
        // java return true
        // java return true
        */
    }

    // java public KmoPivotTable getPivotTable() {
    pub fn getPivotTable(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.pivotTable
        */
    }

    // java public KmoVBAProject getVBAProject() {
    pub fn getVBAProject(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.vba
        */
    }

    // java public KmoControlStream getControl() {
    pub fn getControl(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.control
        */
    }

    // java public KmoExtStream getExt() {
    pub fn getExt(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.extStream
        */
    }

    // java public KmoBookCrypt getCryptInfo() {
    pub fn getCryptInfo(&self)->Rc<crate::cn::wps::moss::core::book::KmoBookCrypt::KmoBookCrypt> {
        // java return this.cryptInfo;
        self.cryptInfo.clone()
    }

    // java public void setModifyPassword(String passwd) {
    pub fn setModifyPassword(&self,passwd:Option<&str>) {
        // java this.cryptInfo.setModifyPasswd(passwd);
        self.cryptInfo.setModifyPasswd(passwd);
        // java FileSharingRecord record = this.cryptInfo.getFileSharing();
        let record=self.cryptInfo.getFileSharing();
        // java if (record != null) {
        if let Some(record)=record{
            // java String userName = record.getUsername();
            let userName=record.getUsername();
            // java if (userName == null) { record.setUsername(" "); }
            if userName.is_none(){record.setUsername(Some(" "));}
        }
    }

    // java public void manualCalculate() {
    pub fn manualCalculate(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.calcChain != null)
        // java int sheetCount = this.sheetCount()
        // java for(int i = 0; i < sheetCount; ++i)
        // java this.sheetAt(i).changeListenerGroup().startMergeEvents()
        // java this.areaWatcher.startMergeEvents()
        // java boolean var6 = false
        // java var6 = true
        // java this.calcChain.manualCalculate()
        // java this.changeListeners.bookSettingsDidFinishChanging()
        // java for(int var8 = 0; var8 < sheetCount; ++var8)
        // java this.sheetAt(var8).changeListenerGroup().sheetContentsDidFinishChanging()
        // java var6 = false
        // java if (var6)
        // java this.areaWatcher.finishMergeEvents()
        // java for(int i = 0; i < sheetCount; ++i)
        // java this.sheetAt(i).changeListenerGroup().finishMergeEvents()
        // java this.areaWatcher.finishMergeEvents()
        // java for(int i = 0; i < sheetCount; ++i)
        // java this.sheetAt(i).changeListenerGroup().finishMergeEvents()
        */
    }

    // java public KmoSheet[] getHiddenOrUnhiddenSheets(boolean isHidden) {
    pub fn getHiddenOrUnhiddenSheets(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java ArrayList<KmoSheet> tmpSheets = new ArrayList(this.sheets.size())
        // java int i = 0
        // java for(int size = this.sheets.size(); i < size; ++i)
        // java KmoSheet sheet = this.sheets.sheetAt(i)
        // java if (isHidden && sheet.isSheetHidden())
        // java tmpSheets.add(sheet)
        // java else if (!isHidden && !sheet.isSheetHidden())
        // java tmpSheets.add(sheet)
        // java KmoSheet[] retSheets = new KmoSheet[tmpSheets.size()]
        // java return (KmoSheet[])tmpSheets.toArray(retSheets)
        */
    }

    // java public KmoRecoveryLogs getRecoveryLogs() {
    pub fn getRecoveryLogs(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.recoveryLogs
        */
    }

    // java public ArrayList<CustomItem> items() {
    pub fn items(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null == this.items)
        // java this.items = new ArrayList()
        // java return this.items
        */
    }

    // java public WebExtensionManager webExtensionManager() {
    pub fn webExtensionManager(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null == this.webExtensionManager)
        // java this.webExtensionManager = new WebExtensionManager()
        // java return this.webExtensionManager
        */
    }

    // java public void fillFirstPageFont() {
    pub fn fillFirstPageFont(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int activeSheetIndex = this.window1.getActiveSheet()
        // java KmoSheet activeSheet = this.sheets.sheetAt(activeSheetIndex)
        // java if (null != activeSheet)
        // java KmoRect usedRange = activeSheet.getUsedRange()
        // java int firstRow = usedRange.tl.row
        // java int firstCol = usedRange.tl.col
        // java int maxLastRow = usedRange.tl.row + 100
        // java int maxLastCol = usedRange.tl.col + 100
        // java int lastRow = usedRange.br.row > maxLastRow ? maxLastRow : usedRange.br.row
        // java int lastCol = usedRange.br.col > maxLastCol ? maxLastCol : usedRange.br.col
        // java int cellCount = 0
        // java Set<String> firstPageFontSet = new HashSet()
        // java for(int row = firstRow; row < lastRow; ++row)
        // java for(int col = firstCol; col < lastCol && cellCount < 100; ++col)
        // java if (activeSheet.isCellExist(row, col) && !activeSheet.isBlankCellValue(row, col))
        // java KmoXF kmoXF = activeSheet.getCellXF(row, col)
        // java if (null != kmoXF)
        // java int fontIndex = kmoXF.getFontIndex()
        // java KmoFont font = this.xfManager.getFont(fontIndex)
        // java String name = null == font ? null : font.getFontName()
        // java if (null != name)
        // java firstPageFontSet.add(name)
        // java ++cellCount
        // java KmoCellData cellData = activeSheet.getCellData(row, col)
        // java if (null != cellData && this.valueStore.isRunsString2(cellData.id))
        // java UnicodeString unicodeString = this.valueStore.runsStringValue(cellData.id)
        // java Iterator<FormatRun> formatRunIterator = unicodeString.formatIterator()
        // java while(formatRunIterator.hasNext())
        // java FormatRun run = (FormatRun)formatRunIterator.next()
        // java int subStrFontIndex = run.getFontIndex()
        // java KmoFont subStrFont = this.xfManager.getFont(subStrFontIndex)
        // java String subStrname = null == subStrFont ? null : subStrFont.getFontName()
        // java if (null != subStrname)
        // java firstPageFontSet.add(subStrname)
        // java this.mFirstPageFontList = new ArrayList(firstPageFontSet)
        */
    }

    // java public ArrayList<String> getFirstPageFontList() {
    pub fn getFirstPageFontList(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.fillFirstPageFont()
        // java if (null == this.mFirstPageFontList)
        // java this.mFirstPageFontList = new ArrayList()
        // java return this.mFirstPageFontList
        */
    }

    // java public int type() {
    pub fn type_(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return 2
        */
    }

    // java public void setSlimDirty() {
    pub fn setSlimDirty(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.setDirty(true)
        // java Platform.getPicturePool().clear()
        // java this.getActiveSheet().changeListenerGroup().sheetContentsDidFinishChanging()
        */
    }

    // java public int open(String path, String password, Object context, IDLProvider dlProvider, boolean ignoreSecurity) {
    pub fn open(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return 2
        */
    }

    // java public boolean isEmptySlides() {
    pub fn isEmptySlides(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return false
        */
    }

    // java public void setWritePassword(String password) {
    pub fn setWritePassword(&self,password:Option<&str>) {
        // java this.getCryptInfo().setPasswd(password);
        self.getCryptInfo().setPasswd(password);
    }

    // java public boolean hasWritePassword() {
    pub fn hasWritePassword(&self)->bool {
        // java return this.getCryptInfo().hasModifyPasswd();
        self.getCryptInfo().hasModifyPasswd()
    }

    // java public boolean save(String path) {
    pub fn save__3(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java String pathExtension = StringUtil.pathExtension(path).toUpperCase()
        // java int format = this.getFileFormat(pathExtension)
        // java return this.save(path, format)
        // java catch (IOException var5)
        // java return false
        */
    }

    // java public IDocProtection getDocProtection() {
    pub fn getDocProtection(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return null
        */
    }

    // java public IKmoBookProtect getBookProtection() {
    pub fn getBookProtection(&self)->Rc<dyn util_adapter::cn::wps::moffice::io::IKmoBookProtect::IKmoBookProtect> {
        // java return this.getBookProtect();
        self.getBookProtect()
    }

    // java public void setDLProvider(IDLProvider dlProvider) {
    pub fn setDLProvider(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mDlProvider = dlProvider
        */
    }

    // java public IDLProvider getDlProvider() {
    pub fn getDlProvider(&self) -> Option<()> {
        // java return this.mDlProvider;
        None
    }

    // java public String getDocContent(int maxLen) {
    pub fn getDocContent(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java StringBuilder sb = new StringBuilder()
        // java int i = 0
        // java for(int sheetCount = this.sheetCount(); i < sheetCount; ++i)
        // java KmoSheet kmoSheet = this.sheetAt(i)
        // java if (kmoSheet != null)
        // java KmoRect usedRange = kmoSheet.getUsedRange()
        // java if (usedRange != null)
        // java int firstRow = usedRange.tl.row
        // java int lastRow = usedRange.br.row
        // java int firstCol = usedRange.tl.col
        // java int lastCol = usedRange.br.col
        // java for(int row = firstRow; row < lastRow; ++row)
        // java for(int col = firstCol; col < lastCol; ++col)
        // java int type = kmoSheet.getCellValueType(row, col)
        // java if (type == 2)
        // java String value = kmoSheet.getCellStringValue(row, col)
        // java sb.append(value)
        // java if (sb.length() >= maxLen)
        // java return sb.toString().substring(0, Math.min(sb.length(), maxLen))
        // java return sb.toString().substring(0, Math.min(sb.length(), maxLen))
        */
    }

    // java private int getFileFormat(String pathExtension) {
    pub fn getFileFormat__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if ("XLS".equals(pathExtension))
        // java return 0
        // java else if (!"XLSX".equals(pathExtension) && !"XLTX".equals(pathExtension))
        // java if ("CSV".equals(pathExtension))
        // java return 4
        // java return "PDF".equals(pathExtension) ? 15 : 0
        // java return 1
        */
    }

    // java public boolean hasHiddenSheet() {
    pub fn hasHiddenSheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java boolean hasHiddenSheet = false
        // java for(int i = 0; i < this.sheetCount(); ++i)
        // java KmoSheet sheet = this.sheetAt(i)
        // java if (sheet.isSheetHidden() && !sheet.isSheetVeryHidden())
        // java hasHiddenSheet = true
        // java break
        // java return hasHiddenSheet
        */
    }

    // java public int getActiveSheetContentSize() {
    pub fn getActiveSheetContentSize(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java String content = this.getActiveSheetContent()
        // java return null != content ? content.length() : 0
        */
    }

    // java public String getActiveSheetContent() {
    pub fn getActiveSheetContent(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoSheet kmoSheet = this.getActiveSheet()
        // java if (kmoSheet == null)
        // java return null
        // java KmoRect usedRange = kmoSheet.getUsedRange()
        // java if (usedRange == null)
        // java return null
        // java int firstRow = usedRange.tl.row
        // java int lastRow = usedRange.br.row
        // java int firstCol = usedRange.tl.col
        // java int lastCol = usedRange.br.col
        // java return this.getString(kmoSheet, firstRow, lastRow, firstCol, lastCol)
        */
    }

    // java private String getString(KmoSheet kmoSheet, int firstRow, int lastRow, int firstCol, int lastCol) {
    pub fn getString(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java StringBuilder sb = new StringBuilder()
        // java KmoMatrixCellIterator iterator = kmoSheet.getGrid().getMatrix().getCellDataIterator(new KmoRect(firstRow, firstCol, lastRow, lastCol), true, true, true)
        // java while(iterator.hasNext())
        // java iterator.next()
        // java String value = kmoSheet.getFormattedCellValue(iterator.row(), iterator.col())
        // java sb.append(value)
        // java return sb.toString()
        */
    }

    // java public String getActiveSheetContent(int maxLength) {
    pub fn getActiveSheetContent__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoSheet kmoSheet = this.getActiveSheet()
        // java if (kmoSheet == null)
        // java return null
        // java KmoRect usedRange = kmoSheet.getUsedRange()
        // java if (usedRange == null)
        // java return null
        // java int firstRow = usedRange.tl.row
        // java int lastRow = usedRange.br.row
        // java int firstCol = usedRange.tl.col
        // java int lastCol = usedRange.br.col
        // java return this.getString(kmoSheet, firstRow, lastRow, firstCol, lastCol, maxLength)
        */
    }

    // java private String getString(KmoSheet kmoSheet, int firstRow, int lastRow, int firstCol, int lastCol, int maxLength) {
    pub fn getString__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java StringBuilder sb = new StringBuilder()
        // java KmoMatrixCellIterator iterator = kmoSheet.getGrid().getMatrix().getCellDataIterator(new KmoRect(firstRow, firstCol, lastRow, lastCol), true, true, true)
        // java while(iterator.hasNext())
        // java iterator.next()
        // java String value = kmoSheet.getFormattedCellValue(iterator.row(), iterator.col())
        // java sb.append(value)
        // java if (sb.length() >= maxLength)
        // java return sb.toString()
        // java return sb.toString()
        */
    }

    // java public String getActiveSheetXRowColContent(int maxRow, int maxCol) {
    pub fn getActiveSheetXRowColContent(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoSheet kmoSheet = this.getActiveSheet()
        // java if (kmoSheet == null)
        // java return null
        // java KmoRect usedRange = kmoSheet.getUsedRange()
        // java if (usedRange == null)
        // java return null
        // java int firstRow = usedRange.tl.row
        // java int firstCol = usedRange.tl.col
        // java return this.getString(kmoSheet, firstRow, maxRow, firstCol, maxCol)
        */
    }

    // java public ArrayList<String> getAllPicPathes(KmoShape currentKmoShape) {
    pub fn getAllPicPathes(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java ArrayList<String> result = new ArrayList()
        // java if (null != currentKmoShape)
        // java ArrayList<String> pathes = this.getShapePicPath(currentKmoShape, (KmoShape)null)
        // java if (null != pathes)
        // java result.addAll(pathes)
        // java int i = 0
        // java for(int size = this.sheetCount(); i < size; ++i)
        // java KmoSheet kmoSheet = this.sheetAt(i)
        // java for(KmoShape kmoShape : kmoSheet.getSheetDrawingAgg().getShapes())
        // java ArrayList<String> pathes = this.getShapePicPath(kmoShape, currentKmoShape)
        // java if (null != pathes)
        // java result.addAll(pathes)
        // java return result
        */
    }

    // java private ArrayList<String> getShapePicPath(KmoShape kmoShape, KmoShape currentKmoShape) {
    pub fn getShapePicPath(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java ArrayList<String> pathes = new ArrayList()
        // java if (null == kmoShape)
        // java return null
        // java else if (kmoShape.equals(currentKmoShape))
        // java return null
        // java if (kmoShape instanceof KmoPicture)
        // java KmoPicture picture = (KmoPicture)kmoShape
        // java String path = this.getPicturePath(picture)
        // java pathes.add(path)
        // java int childCount = kmoShape.getChildCount()
        // java if (childCount != 0)
        // java for(int i = 0; i < childCount; ++i)
        // java KmoShape childShape = kmoShape.getChildShape(i)
        // java ArrayList<String> shapePicPathes = this.getShapePicPath(childShape, currentKmoShape)
        // java if (null != shapePicPathes)
        // java pathes.addAll(shapePicPathes)
        // java return pathes
        */
    }

    // java private String getPicturePath(KmoPicture pic) {
    pub fn getPicturePath(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null == pic)
        // java return null
        // java PictureData picData = pic.getPicData()
        // java if (null == picData)
        // java return null
        // java FileDataStorage fileDataStorage = picData.getData()
        // java if (null == fileDataStorage)
        // java return null
        // java File file = fileDataStorage.getFile()
        // java if (null == file)
        // java return null
        // java return file.exists() ? file.getAbsolutePath() : null
        */
    }

    // java public KmoOleMgr getOleMgr() {
    pub fn getOleMgr(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null == this.mOleMgr)
        // java RandomAccessFile raf = this.getRaf()
        // java if (null != raf)
        // java this.mOleMgr = KmoOleMgr.create(raf, this.getTmpPath())
        // java this.mOleMgr = KmoOleMgr.create(this.getFilePath(), this.getTmpPath())
        // java return this.mOleMgr
        */
    }

    // java public IMediaLib getOleMediaLib() {
    pub fn getOleMediaLib(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.getOleMgr().getOleMediaLib()
        */
    }

    // java public void addOle(int mediaId, KmoOle kmoOle) {
    pub fn addOle(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.getOleMgr().addOle(mediaId, kmoOle)
        */
    }

    // java public KmoCellPic getDispingKmoPicture(int nameCode) {
    pub fn getDispingKmoPicture(&self, _name_code: i32) -> Option<KmoCellPic> {
        // java if (null == this.mDispingInfoMap) this.initDisimageMap();
        // java if (null != this.mDispingInfoMap) return map.get(nameCode);
        // 图片表还没有装入。map 为空时 Java 返回 null，getCellPic 再换成 emptyKmoCellPic。
        None
    }

    // java public IntObjectHashMap<KmoCellPic> getCellPicMap() {
    pub fn getCellPicMap(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.mDispingInfoMap == null)
        // java this.mDispingInfoMap = new IntObjectHashMap<KmoCellPic>()
        // java return this.mDispingInfoMap
        */
    }

    // java public void initDisimageMap() {
    pub fn initDisimageMap(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int i = 0
        // java for(int size = this.sheetCount(); i < size; ++i)
        // java KmoSheet kmoSheet = this.sheetAt(i)
        // java String name = kmoSheet.name()
        // java if (this.isDisImageSheetName(name))
        // java this.initDisimagePicMap(kmoSheet)
        */
    }

    // java public boolean isDisImageSheetName(String sheetName) {
    pub fn isDisImageSheetName(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return sheetName.equals("WpsReserved_CellImgList")
        */
    }

    // java private void initDisimagePicMap(KmoSheet kmoSheet) {
    pub fn initDisimagePicMap(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java for(KmoShape kmoShape : kmoSheet.getSheetDrawingAgg().getShapes())
        // java if (null != kmoShape && 8 == kmoShape.getShapeObjType() && kmoShape instanceof KmoPicture)
        // java KmoPicture kmoPicture = (KmoPicture)kmoShape
        // java String name = kmoPicture.getCNvPrName()
        // java if (null != name)
        // java int id = name.hashCode()
        // java if (null == this.mDispingInfoMap)
        // java this.mDispingInfoMap = new IntObjectHashMap<KmoCellPic>()
        // java if (this.mDispingInfoMap.containsKey(id))
        // java return
        // java KmoCellPic kmoCellPic = new KmoCellPic(kmoPicture)
        // java this.mDispingInfoMap.put(id, kmoCellPic)
        */
    }

    // java public void addDispingInfo(int nameCode, KmoPicture kmoPicture) {
    pub fn addDispingInfo(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (null == this.mDispingInfoMap)
        // java this.mDispingInfoMap = new IntObjectHashMap<KmoCellPic>()
        // java KmoCellPic kmoCellPic = new KmoCellPic(kmoPicture)
        // java this.mDispingInfoMap.put(nameCode, kmoCellPic)
        */
    }

    // java public void appendSheet(cn.wps.moffice.et.EtSheet etSheet) {
    pub fn appendSheet__6(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoSheet sheet = this.sheetAt(0)
        // java for(EtCell cell : etSheet.cells)
        // java sheet.setCellRawValue(cell.row, cell.col, cell.data)
        // java KmoXF xf = KmoXF.shareEmptyCellXF()
        // java xf = KmoXF.createXF(xf)
        // java KmoXFMask mask = new KmoXFMask()
        // java xf.copyFromOutside(cell.etXF, mask)
        // java sheet.setRangeXF(new KmoRect(cell.row, cell.col, cell.row, cell.col), xf, mask)
        // java for(EtMerge merge : etSheet.merges)
        // java sheet.merge(new KmoRect(merge.tRow, merge.tCol, merge.bRow, merge.bCol))
        // java for(EtColInfo colInfo : etSheet.colInfos)
        // java sheet.getColInfos().setColInfo(new KmoColInfo(colInfo.min, colInfo.max, colInfo.width, (short)15, (short)6))
        // java for(EtRowInfo rowInfo : etSheet.rowInfos)
        // java sheet.setRowHeight(rowInfo.index, rowInfo.height, false)
        */
    }

    // java public boolean importExportTable(String path) {
    pub fn importExportTable(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java HtmlReader htmlReader = new HtmlReader(this)
        // java return htmlReader.read(path)
        // java catch (Exception var4)
        // java return false
        */
    }

    // java public boolean importExportTables(String[] pathArray, IImportExportTableCallback callback) {
    pub fn importExportTables(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (pathArray == null)
        // java return false
        // java int activeSheetIndex = this.sheetIndex(this.getActiveSheet())
        // java for(int i = 0; i < pathArray.length; ++i)
        // java HtmlReader htmlReader = new HtmlReader(this)
        // java activeSheetIndex = this.sheetIndex(this.getActiveSheet())
        // java this.pickSheet()
        // java if (!htmlReader.read(pathArray[i]))
        // java if (callback != null)
        // java callback.error(i, pathArray.length, 1)
        // java if (this.sheetCount() > 1)
        // java this.removeSheet(this.getActiveSheet())
        // java this.activateSheet(activeSheetIndex)
        // java continue
        // java catch (Exception var7)
        // java if (callback != null)
        // java callback.error(i, pathArray.length, 2)
        // java if (this.sheetCount() > 1)
        // java this.removeSheet(this.getActiveSheet())
        // java this.activateSheet(activeSheetIndex)
        // java continue
        // java if (callback != null)
        // java callback.progress(i, pathArray.length, this.getActiveSheet())
        // java return true
        */
    }

    // java public boolean importExportToComboTables(String[] pathArray, IImportExportTableCallback callback) {
    pub fn importExportToComboTables(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (pathArray == null)
        // java return false
        // java int activeSheetIndex = this.sheetIndex(this.getActiveSheet())
        // java this.pickSheet()
        // java for(int i = 0; i < pathArray.length; ++i)
        // java HtmlReader htmlReader = new HtmlReader(this)
        // java activeSheetIndex = this.sheetIndex(this.getActiveSheet())
        // java if (!htmlReader.read(pathArray[i]))
        // java if (callback != null)
        // java callback.error(i, pathArray.length, 1)
        // java continue
        // java catch (Exception var12)
        // java if (callback != null)
        // java callback.error(i, pathArray.length, 2)
        // java continue
        // java KmoRect rect = this.getActiveSheet().getUsedRange()
        // java rect.tl.row = rect.br.row + 2
        // java rect.br.row = rect.tl.row
        // java if (!rect.isValid())
        // java callback.error(i, pathArray.length, 3)
        // java return true
        // java this.getActiveSheet().setSelection(rect)
        // java if (callback != null)
        // java callback.progress(i, pathArray.length, this.getActiveSheet())
        // java KmoSheet newSheet = this.getActiveSheet()
        // java if (newSheet != null && newSheet.isEmptyInImportTable() && this.sheetCount() > 1)
        // java this.removeSheet(this.getActiveSheet())
        // java this.activateSheet(activeSheetIndex)
        // java return true
        */
    }

    // java public boolean importExportTables(String[] pathArray) {
    pub fn importExportTables__2(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.importExportTables(pathArray, (IImportExportTableCallback)null)
        */
    }

    // java private void pickSheet() {
    pub fn pickSheet(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int sheetCount = this.sheetCount()
        // java for(int i = 0; i < sheetCount; ++i)
        // java KmoSheet sheet = this.sheetAt(i)
        // java if (sheet.isEmptyInImportTable())
        // java this.activateSheet(i)
        // java return
        // java this.appendSheet()
        // java this.activateSheet(sheetCount)
        */
    }

    // java public boolean hasAllowUserItem() {
    pub fn hasAllowUserItem(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int sheetCount = this.sheetCount()
        // java for(int i = 0; i < sheetCount; ++i)
        // java KmoSheet kmoSheet = this.sheetAt(i)
        // java if (null != kmoSheet.getSheetProtect())
        // java KmoSheetProtect kmoSheetProtect = kmoSheet.getSheetProtect()
        // java if (null != kmoSheetProtect.getAllowEditUserInfo())
        // java return true
        // java List<KmoSheetProtectEditable> kmoSheetProtectEditables = kmoSheetProtect.protectEditables()
        // java if (null != kmoSheetProtectEditables)
        // java for(KmoSheetProtectEditable editable : kmoSheetProtectEditables)
        // java if (null != editable.editableUserInfo())
        // java return true
        // java return false
        */
    }

    // java public String getTmpPath() {
    pub fn getTmpPath(&self)->String {
util_adapter::cn::wps::core::runtime::Platform::Platform::getCurrentSsTmpDir(&self.mUUID)
    }

    // java public FileManager getFileManager() {
    pub fn getFileManager(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.mFileManager
        */
    }

    // java public int getFormulaSize() {
    pub fn getFormulaSize(&self) -> i32 {
        // java getCalculationChain(); null guard; original CalculationChain call.
        self.getCalculationChain()
            .map_or(0, |cc| cc.getFormulaSize())
    }

    // java public int validateFormula(String text, int sheetIndex, List<String> funcNameList, Set<String> unsupportedFuncSet, Boolean[] hasRef) {
    pub fn validateFormula(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java EvaluationWorkbook ew = new EvaluationWorkbook(this)
        // java Ptg[] ptgs = FormulaParser.parse(text.substring(1), ew, 0, sheetIndex, this.getSpreadsheetVersion(), ParserOption.createByLanguage(KmoBooks.getLanguageType()))
        // java if (ptgs[0] instanceof NamePtg)
        // java NamePtg nptg = (NamePtg)ptgs[0]
        // java IEvalName en = ew.getNameByNamePtg(nptg.getIndex())
        // java if (en != null && en.isFunctionName())
        // java String funcName = en.getNameText()
        // java if (funcName.startsWith("_xlfn."))
        // java funcName = funcName.substring(6)
        // java IFunction udf = AggregatingUDFFinder.DEFAULT.findFunction(funcName)
        // java if (udf instanceof DISPIMG && (ptgs.length != 4 || ptgs[1] instanceof MissingArgPtg || ptgs[2] instanceof MissingArgPtg))
        // java return 1
        // java funcNameList.addAll(FormulaHelper.listAllFunctions(ptgs, ew, hasRef))
        // java if (FormulaHelper.hasUnimplementedFunctions(ptgs, ew))
        // java unsupportedFuncSet.addAll(FormulaHelper.listUnimplementedFunctions(ptgs, ew))
        // java return 2
        // java return 0
        // java catch (Exception var12)
        // java return 1
        */
    }

    // java public boolean findKeyCell(KmoSheet sheet, KmoRect rect) {
    pub fn findKeyCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java Dependent dep = this.getDependent()
        // java ArrayList<IEvalFormulaBase> supported = new ArrayList()
        // java ArrayList<SharedFmlReferredArea> supportedShr = new ArrayList()
        // java dep.queryStaticReferred(sheet.getSheetUid(), new Area(rect.tl.row, rect.br.row, rect.tl.col, rect.br.col), supported, supportedShr)
        // java return supported.size() > 0 || supportedShr.size() > 0
        */
    }

    // java public void initCalculationChain() {
    pub fn initCalculationChain(&self) {
        // java getCalculationChain(); null guard; original CalculationChain call.
        if let Some(cc) = self.getCalculationChain() {
            cc.init();
        }
    }

    // java public void markRecalculation() {
    pub fn markRecalculation(&self) {
        // java getCalculationChain(); null guard; original CalculationChain call.
        if let Some(cc) = self.getCalculationChain() {
            cc.markRecalculation(-1, -1, -1);
        }
    }

    // java public void synCalculate() {
    pub fn synCalculate(&self) {
        // java getCalculationChain(); null guard; original CalculationChain call.
        if let Some(cc) = self.getCalculationChain() {
            cc.synCalculate();
        }
    }

    // java public List<String> getNameList(int sheetIndex) {
    pub fn getNameList(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (sheetIndex < 0)
        // java return null
        // java List<NameRecord> nameRecords = this.getLinkTable().getNameAvailbleInSheet(sheetIndex)
        // java List<String> names = new ArrayList()
        // java for(NameRecord nameRecord : nameRecords)
        // java names.add(nameRecord.getNameText())
        // java return names
        */
    }

    // java public FileSaveCallbackException(Exception e) {
    pub fn FileSaveCallbackException(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java super("File save callback exception")
        // java this.mInnerException = e
        */
    }

    // java public Exception getInnerException() {
    pub fn getInnerException(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.mInnerException
        */
    }
}

// java KmoBook.getCalculationChain's anonymous CalcChain.ICalcChainListener.
struct BookCalcChainListener(Weak<KmoBook>);
impl crate::cn::wps::et::ss::calcchain::CalcChain::ICalcChainListener for BookCalcChainListener {
    fn onChange(
        &self,
        sheet: i32,
        row: i32,
        col: i32,
        tokens: &[crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg],
        mode: i32,
    ) {
        if let Some(book) = self.0.upgrade() {
            // java KmoBook.this.manageCellPic(sheetIdex,tokens,mode);
            book.manageCellPic(sheet, tokens, mode);
            // java KmoBook.this.managerHyperlinkForFml(sheetIdex,row,col,tokens,mode);
            book.managerHyperlinkForFml(sheet, row, col, tokens, mode);
        }
    }
}
