// java package cn.wps.moss.engine.eval;

// java import cn.wps.moss.app.KmoSheet;
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
// java import cn.wps.moss.engine.grid.KmoCellData;
use crate::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;
// java import cn.wps.moss.engine.grid.KmoFormulaData;
use crate::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
use crate::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use std::sync::{Condvar, Mutex};
use std::thread::ThreadId;
use crate::cn::wps::{et::ss::formula::ptg::Ptg::{Ptg,PtgKind,PtgArray},moss::{engine::grid::ExpFmlManager::ExpFmlManager,util::objpool::SharedObjects::{kmoCellDataPool,kmoFormulaDataPool}}};
// java public static long time = 0L;
pub static time:std::sync::atomic::AtomicI64=std::sync::atomic::AtomicI64::new(0);
#[cfg(test)]
#[path="EvaluationPresent.tokens.review.rs"]
mod tokens_review;

// java public class EvaluationPresent {
pub struct EvaluationPresent {
    // java private ReentrantLock lock = new ReentrantLock();
    lock: Mutex<LockState>,
    available: Condvar,
}

struct LockState {
    owner: Option<ThreadId>,
    depth: usize,
}

// java public class EvaluationPresent {
impl EvaluationPresent {
    // java private ReentrantLock lock = new ReentrantLock();
    pub fn new() -> Self {
        Self { lock: Mutex::new(LockState { owner: None, depth: 0 }), available: Condvar::new() }
    }

    // java public Ptg[] getCellFmlTokens(KmoSheet sheet, int row, int col, boolean process_exp) { return this.getCellFmlTokens(sheet, row, col, process_exp, process_exp); }
    pub fn getCellFmlTokens(&self,sheet:Option<&KmoSheet>,row:i32,col:i32,process_exp:bool)->Option<PtgArray>{self.getCellFmlTokens__5(sheet,row,col,process_exp,process_exp)}
    // java public Ptg[] getCellFmlTokens(KmoSheet sheet, int row, int col, boolean process_sharefml, boolean process_arrayfml) {
    pub fn getCellFmlTokens__5(&self,sheet:Option<&KmoSheet>,row:i32,col:i32,process_sharefml:bool,process_arrayfml:bool)->Option<PtgArray>{
        // java KmoCellData cellBuf = SharedObjects.kmoCellDataPool.borrowObject(); KmoFormulaData fmlbuf = SharedObjects.kmoFormulaDataPool.borrowObject();
        let cellBuf=kmoCellDataPool.borrowObject();let fmlbuf=kmoFormulaDataPool.borrowObject();
        // java try {
        let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
            // java sheet.getCellData(row, col, cellBuf);
            let sheet=sheet.expect("NullPointerException");sheet.getCellData__2(row,col,&mut cellBuf.borrow_mut());
            // java if (cellBuf.formulaType != 0) {
            if cellBuf.borrow().formulaType!=0{
                // java KmoCellValueStore cvs = sheet.getCellValueStore(); cvs.formulaValue(cellBuf.id, fmlbuf);
                let cvs=sheet.getCellValueStore();cvs.borrow().formulaValue(cellBuf.borrow().id,&mut fmlbuf.borrow_mut());
                // java Ptg[] ret = Ptg.readTokens(fmlbuf.getTokensPlainSize(), cvs.formulaTokens(fmlbuf.getTokensId()), sheet.getVersion());
                let size=fmlbuf.borrow().getTokensPlainSize();let bytes=cvs.borrow().formulaTokens(fmlbuf.borrow().getTokensId());let mut ret=Ptg::readTokens__array(size,&bytes,sheet.getVersion());
                // java if ((process_sharefml && cellBuf.formulaType == 3 || process_arrayfml && cellBuf.formulaType == 2) && ret.length == 1 && ret[0].getSid() == 1) {
                let process=(process_sharefml&&cellBuf.borrow().formulaType==3||process_arrayfml&&cellBuf.borrow().formulaType==2)&&ret.borrow().len()==1;
                if process&&ret.borrow()[0].as_ref().expect("NullPointerException").borrow().getSid()==1{
                    // java ExpPtg eptg = (ExpPtg)ret[0];
                    let eptg=ret.borrow()[0].clone().expect("NullPointerException");if !matches!(eptg.borrow().kind,PtgKind::Exp{..}){panic!("ClassCastException");}
                    // java ExpFmlManager.ExpFml shrfml = sheet.getExpFmlManager().query(eptg.getRow(), (short)eptg.getColumn(), cellBuf.formulaType == 3);
                    let masterRow=eptg.borrow().getExpRow();let masterCol=eptg.borrow().getExpColumn();let shrfml=sheet.getExpFmlManager().borrow().query(masterRow,masterCol as i16 as i32,cellBuf.borrow().formulaType==3);
                    // java if (shrfml == null) { Ptg[] var12 = ret; return var12; }
                    let Some(shrfml)=shrfml else{let var12=ret;return Some(var12);};
                    // java if (cellBuf.formulaType == 3) { ret = ExpFmlManager.adjustSharedFormula(shrfml, row, col, sheet.getBook().getSpreadsheetVersion()); }
                    if cellBuf.borrow().formulaType==3{ret=ExpFmlManager::adjustSharedFormula__reference(Some(&shrfml),row,col,Some(sheet.getBook().getSpreadsheetVersion()));}
                    // java else { ret = shrfml.getPtgs(); }
                    else{ret=shrfml.getPtgs__reference();}
                }
                // java Ptg[] var19 = ret; return var19;
                let var19=ret;return Some(var19);
            }
            // java return null; (after finally)
            None
        }));
        // java finally { cellBuf = SharedObjects.kmoCellDataPool.returnObject(cellBuf); fmlbuf = SharedObjects.kmoFormulaDataPool.returnObject(fmlbuf); }
        let _cellBuf=kmoCellDataPool.returnObject(Some(cellBuf));let _fmlbuf=kmoFormulaDataPool.returnObject(Some(fmlbuf));
        match result{Ok(ret)=>ret,Err(error)=>std::panic::resume_unwind(error)}
    }

    // java public boolean isLock()
    pub fn isLock(&self) -> bool {
        // java return this.lock.isLocked();
        self.lock.lock().unwrap().depth != 0
    }

    // java public void lock()
    pub fn lock(&self) {
        // java this.lock.lock();
        let current = std::thread::current().id();
        let mut state = self.lock.lock().unwrap();
        while state.owner.is_some() && state.owner != Some(current) {
            state = self.available.wait(state).unwrap();
        }
        state.owner = Some(current);
        state.depth += 1;
    }

    // java public void unlock()
    pub fn unlock(&self) {
        // java this.lock.unlock();
        let current = std::thread::current().id();
        let mut state = self.lock.lock().unwrap();
        if state.owner != Some(current) {
            drop(state);
            panic!("IllegalMonitorStateException");
        }
        state.depth -= 1;
        if state.depth == 0 {
            state.owner = None;
            self.available.notify_one();
        }
    }

    // java public static final int NEED_CALCULATE_FLAG = 1;
    pub const NEED_CALCULATE_FLAG: i8 = 1;

    // java public static boolean isNeedRecalculationFlagSet(KmoSheet sheet, int row, int col)
    pub fn isNeedRecalculationFlagSet(sheet: &KmoSheet, row: i32, col: i32) -> bool {
        // java int id = sheet.getGrid().getMatrix().getId(row, col);
        let id = sheet.getGrid().getMatrix().borrow().getId(row, col);
        // java return (sheet.getCellValueStore().getFormulaCalcFlag(id) & 1) != 0;
        (sheet.getCellValueStore().borrow().getFormulaCalcFlag(id) & Self::NEED_CALCULATE_FLAG) != 0
        // java catch (IndexOutOfBoundsException var4) { return true; }
        // java FormulaDataStore.getCalcFlag returns 1 for an out-of-range id.
    }

    // java public static synchronized void setNeedRecalculationFlag(KmoSheet sheet, int row, int col, boolean f)
    pub fn setNeedRecalculationFlag(sheet: &KmoSheet, row: i32, col: i32, f: bool) {
        // java KmoCellData cellBuf = SharedObjects.kmoCellDataPool.borrowObject();
        let mut cellBuf = KmoCellData::default();
        // java if (f != isNeedRecalculationFlagSet(sheet, row, col)) {
        if f != Self::isNeedRecalculationFlagSet(sheet, row, col) {
            // java sheet.getCellData(row, col, cellBuf);
            sheet.getCellData__2(row, col, &mut cellBuf);
            // java byte flags = sheet.getCellValueStore().getFormulaCalcFlag(cellBuf.id);
            let store = sheet.getCellValueStore();
            let flags = store.borrow().getFormulaCalcFlag(cellBuf.id);
            // java if (f) { flags = (byte)(flags | 1); } else { flags = (byte)(flags & -2); }
            let flags = if f { flags | Self::NEED_CALCULATE_FLAG } else { flags & !Self::NEED_CALCULATE_FLAG };
            // java sheet.getCellValueStore().setFormulaCalcFlag(cellBuf.id, flags);
            store.borrow_mut().setFormulaCalcFlag(cellBuf.id, flags);
        }
        // java SharedObjects.kmoCellDataPool.returnObject(cellBuf);
    }

    // java public static synchronized void loadBuffer(KmoSheet sheet, int row, int col, KmoCellData cellBuf, KmoFormulaData fmlbuf)
    pub fn loadBuffer(sheet: &KmoSheet, row: i32, col: i32, cellBuf: &mut KmoCellData, fmlbuf: &mut KmoFormulaData) {
        // java sheet.getCellData(row, col, cellBuf);
        sheet.getCellData__2(row, col, cellBuf);
        // java if (cellBuf.formulaType == 0) { throw new IllegalStateException("Formula expacted"); }
        if cellBuf.formulaType == 0 {
            panic!("Formula expacted");
        }
        // java sheet.getCellValueStore().formulaValue(cellBuf.id, fmlbuf);
        sheet.getCellValueStore().borrow().formulaValue(cellBuf.id, fmlbuf);
    }

    // java public static synchronized void storeBuffer(KmoSheet sheet, int row, int col, KmoCellData cellBuf, KmoFormulaData fmlbuf)
    pub fn storeBuffer(sheet: &KmoSheet, row: i32, col: i32, cellBuf: &mut KmoCellData, fmlbuf: &KmoFormulaData) {
        // java cellBuf.id = sheet.getCellValueStore().newFormulaValue(fmlbuf);
        cellBuf.id = sheet.getCellValueStore().borrow_mut().newFormulaValue(fmlbuf);
        // java sheet.setCellDataCalcResult(row, col, cellBuf);
        sheet.setCellDataCalcResult(row, col, cellBuf);
    }

    // java public static void setNumberResult(KmoSheet sheet, int row, int col, double v)
    pub fn setNumberResult(sheet: &KmoSheet, row: i32, col: i32, v: f64) {
        let mut cellBuf = KmoCellData::default();
        let mut fmlbuf = KmoFormulaData::KmoFormulaData();
        // java loadBuffer(sheet, row, col, cellBuf, fmlbuf);
        Self::loadBuffer(sheet, row, col, &mut cellBuf, &mut fmlbuf);
        // java int oldid = fmlbuf.getCacheValueId();
        let oldid = fmlbuf.getCacheValueId();
        let store = sheet.getCellValueStore();
        // java if (identifyTypeByID(oldid) != 1 || v != doubleValue(oldid))
        let changed = KmoCellValueStore::identifyTypeByID(oldid) != 1 || v != store.borrow_mut().doubleValue(oldid);
        if changed {
            // java int newid = sheet.getCellValueStore().newDoubleValue(v);
            let newid = store.borrow_mut().newDoubleValue(v);
            // java fmlbuf.setCacheValueId(newid); storeBuffer(...);
            fmlbuf.setCacheValueId(newid);
            Self::storeBuffer(sheet, row, col, &mut cellBuf, &fmlbuf);
            // java sheet.getBook().setDirty(true);
            sheet.getBook().setDirty(true);
            // java sheet.getBook().areaWatcher().postContentChangedEvent(sheet, row, col);
            sheet.getBook().areaWatcher().borrow_mut().postContentChangedEvent(sheet, row, col);
        }
    }

    // java public static void setStringResult(KmoSheet sheet, int row, int col, String s)
    pub fn setStringResult(sheet: &KmoSheet, row: i32, col: i32, s: &str) {
        let mut cellBuf = KmoCellData::default();
        let mut fmlbuf = KmoFormulaData::KmoFormulaData();
        // java loadBuffer(sheet, row, col, cellBuf, fmlbuf);
        Self::loadBuffer(sheet, row, col, &mut cellBuf, &mut fmlbuf);
        // java if (cellBuf.isPicInCell) { ... getCellPic ... }
        // getCellPic(row,col) is not yet in the Rust facade; fail closed for that branch.
        assert!(!cellBuf.isPicInCell, "getCellPic(row,col) is required for picture formula results");
        // java int oldid = fmlbuf.getCacheValueId();
        let oldid = fmlbuf.getCacheValueId();
        let store = sheet.getCellValueStore();
        // java if (identifyTypeByID(oldid) != 2 || !s.equals(stringValue(oldid)))
        let changed = KmoCellValueStore::identifyTypeByID(oldid) != 2 || s != store.borrow_mut().stringValue(oldid);
        if changed {
            // java fmlbuf.setCacheValueId(sheet.getCellValueStore().newFmlString(s)); storeBuffer(...);
            fmlbuf.setCacheValueId(store.borrow_mut().newFmlString(s));
            Self::storeBuffer(sheet, row, col, &mut cellBuf, &fmlbuf);
            // java sheet.getBook().setDirty(true); areaWatcher().postContentChangedEvent(sheet,row,col);
            sheet.getBook().setDirty(true);
            sheet.getBook().areaWatcher().borrow_mut().postContentChangedEvent(sheet, row, col);
        }
    }

    // java public static void setBoolResult(KmoSheet sheet, int row, int col, boolean b)
    pub fn setBoolResult(sheet: &KmoSheet, row: i32, col: i32, b: bool) {
        let mut cellBuf = KmoCellData::default();
        let mut fmlbuf = KmoFormulaData::KmoFormulaData();
        // java loadBuffer(sheet, row, col, cellBuf, fmlbuf);
        Self::loadBuffer(sheet, row, col, &mut cellBuf, &mut fmlbuf);
        let oldid = fmlbuf.getCacheValueId();
        let store = sheet.getCellValueStore();
        // java if (identifyTypeByID(oldid) != 5 || b != booleanValue(oldid))
        if KmoCellValueStore::identifyTypeByID(oldid) != 5 || b != store.borrow().booleanValue(oldid) {
            // java fmlbuf.setCacheValueId(newBooleanValue(b)); storeBuffer(...);
            fmlbuf.setCacheValueId(store.borrow().newBooleanValue(b));
            Self::storeBuffer(sheet, row, col, &mut cellBuf, &fmlbuf);
            // java setDirty(true); areaWatcher().postContentChangedEvent(sheet,row,col);
            sheet.getBook().setDirty(true);
            sheet.getBook().areaWatcher().borrow_mut().postContentChangedEvent(sheet, row, col);
        }
    }

    // java public static void setErrorResult(KmoSheet sheet, int row, int col, byte e)
    pub fn setErrorResult(sheet: &KmoSheet, row: i32, col: i32, e: i8) {
        let mut cellBuf = KmoCellData::default();
        let mut fmlbuf = KmoFormulaData::KmoFormulaData();
        // java loadBuffer(sheet, row, col, cellBuf, fmlbuf);
        Self::loadBuffer(sheet, row, col, &mut cellBuf, &mut fmlbuf);
        let oldid = fmlbuf.getCacheValueId();
        let old_type = KmoCellValueStore::identifyTypeByID(oldid);
        // java if (e != 29 || identifyTypeByID(oldid) == 0)
        if e != 29 || old_type == 0 {
            let store = sheet.getCellValueStore();
            // java if (identifyTypeByID(oldid) == 6 && e == errorValue(oldid)) return;
            if old_type == 6 && e == store.borrow().errorValue(oldid) { return; }
            // java fmlbuf.setCacheValueId(newErrorValue(e)); storeBuffer(...);
            fmlbuf.setCacheValueId(store.borrow().newErrorValue(e as i32));
            Self::storeBuffer(sheet, row, col, &mut cellBuf, &fmlbuf);
            // java setDirty(true); areaWatcher().postContentChangedEvent(sheet,row,col);
            sheet.getBook().setDirty(true);
            sheet.getBook().areaWatcher().borrow_mut().postContentChangedEvent(sheet, row, col);
        }
    }

    // java public static void setZeroResultIfBlank(KmoSheet sheet, int row, int col)
    pub fn setZeroResultIfBlank(sheet: &KmoSheet, row: i32, col: i32) {
        let mut cellBuf = KmoCellData::default();
        let mut fmlbuf = KmoFormulaData::KmoFormulaData();
        // java loadBuffer(sheet, row, col, cellBuf, fmlbuf);
        Self::loadBuffer(sheet, row, col, &mut cellBuf, &mut fmlbuf);
        // java if (identifyTypeByID(fmlbuf.getCacheValueId()) == 0)
        if KmoCellValueStore::identifyTypeByID(fmlbuf.getCacheValueId()) == 0 {
            // java fmlbuf.setCacheValueId(newDoubleValue(0.0F)); storeBuffer(...);
            fmlbuf.setCacheValueId(sheet.getCellValueStore().borrow_mut().newDoubleValue(0.0));
            Self::storeBuffer(sheet, row, col, &mut cellBuf, &fmlbuf);
            // java setDirty(true); areaWatcher().postContentChangedEvent(sheet,row,col);
            sheet.getBook().setDirty(true);
            sheet.getBook().areaWatcher().borrow_mut().postContentChangedEvent(sheet, row, col);
        }
    }
}
