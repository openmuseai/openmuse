// java package cn.wps.et.ss.dependent.refer.base;

use crate::java_runtime::{Rc, RefCell};
use kfc::cn::wps::memor4j::AtomSnapshot::{SharedAtom, KmoAtom as SnapshotAtom, KmoAtomData as SnapshotData};
use std::sync::Mutex;


// java import cn.wps.et.ss.util.Area;
use crate::cn::wps::et::ss::util::Area::Area;
// java import cn.wps.et.ss.util.level.PointLevelMap;
use crate::cn::wps::et::ss::util::level::PointLevelMap::PointLevelMap;
// java import cn.wps.et.ss.util.primeval.objmap.IntObjectHashMap;
use crate::cn::wps::et::ss::util::primeval::objmap::IntObjectHashMap::IntObjectHashMap;
// java import cn.wps.memor4j.KmoAtom/KmoAtomData;
use kfc::cn::wps::memor4j::KmoAtom::KmoAtom;
use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData, KmoAtomDataSub};
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

// java protected static class Backup<T> extends KmoAtomData
pub(crate) struct Backup<T> {
    base: KmoAtomData,
    pub(crate) sheetUid: i32,
    pub(crate) row: i32,
    pub(crate) col: i32,
    pub(crate) data: Option<T>,
    pub(crate) opType: i32,
}

impl<T> Backup<T> {
    fn new() -> Self { Self { base: KmoAtomData::new(), sheetUid: 0, row: 0, col: 0, data: None, opType: 0 } }
}

impl<T> KmoAtomDataSub for Backup<T> {
    fn as_base(&self) -> &KmoAtomData { &self.base }
    fn as_base_mut(&mut self) -> &mut KmoAtomData { &mut self.base }
    // java Backup.copy(): new Backup(); ret.__forceOldest(); return ret;
    fn copy(&self) -> Self { let mut result = Self::new(); result.base.__forceOldest(); result }
}

// java public class CellRefMap<T> extends KmoAtom<CellRefMap.Backup<T>> {
pub(crate) struct CellRefMapState<T = ()> {
    pub(crate) atom: KmoAtom<Backup<T>>,
    // java protected final SpreadsheetVersion sv;
    pub(crate) sv: SpreadsheetVersion,
    // java protected final IntObjectHashMap<PointLevelMap<T>> points = new IntObjectHashMap();
    pub(crate) points: IntObjectHashMap<PointLevelMap<T>>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> CellRefMapState<T> {
    // java public CellRefMap(SpreadsheetVersion sv)
    pub fn CellRefMap(sv: SpreadsheetVersion) -> Self {
        // java super(new Backup()); this.sv = sv;
        Self { atom: KmoAtom::new(Backup::new()), sv, points: IntObjectHashMap::IntObjectHashMap() }
    }

    // java public void addToMap(int sheetUid, int row, int col, T data, boolean inTrans)
    pub fn addToMap(&mut self, sheetUid: i32, row: i32, col: i32, data: T, inTrans: bool) {
        // java if (inTrans) beforeWrite(); Backup records sheetUid,row,col,data,opType=ADD.
        if inTrans {
            self.atom.beforeWrite();
            let mut backup = self.atom.rawData_mut();
            backup.sheetUid = sheetUid; backup.row = row; backup.col = col; backup.data = Some(data.clone()); backup.opType = 1;
        }
        // java PointLevelMap<T> plm = points.get(sheetUid); if null, create and put.
        if self.points.get(sheetUid).is_none() { self.points.put(sheetUid, PointLevelMap::PointLevelMap(self.sv)); }
        // java plm.insert(row,col,data).
        self.points.get_mut(sheetUid).unwrap().insert(row, col, data);
    }

    // java public T removeFromMap(int sheetUid, int row, int col, T data)
    pub fn removeFromMap(&mut self, sheetUid: i32, row: i32, col: i32, data: &T) -> Option<T> {
        // java beforeWrite(); Backup records sheetUid,row,col,data,opType=DEL.
        self.atom.beforeWrite();
        let mut backup = self.atom.rawData_mut();
        backup.sheetUid = sheetUid; backup.row = row; backup.col = col; backup.data = Some(data.clone()); backup.opType = 2;
        // java return plm == null ? null : plm.remove(row,col,data).
        self.points.get_mut(sheetUid)?.remove(row, col, data)
    }

    // java public void queryReferred(int sheetUid, int row, int col, Collection<T> out)
    pub fn queryReferred(&self, sheetUid: i32, row: i32, col: i32, out: &mut Vec<T>) {
        // java if (plm != null) plm.queryAreaRefered(row,col,out).
        if let Some(plm) = self.points.get(sheetUid) { plm.queryAreaRefered(row, col, out); }
    }

    // java public void queryReferred(int sheetUid, Area area, Collection<T> out)
    pub fn queryReferred__area(&self, sheetUid: i32, area: &Area, out: &mut Vec<T>) {
        // java if (plm != null) plm.queryAreaReferred(area,out).
        if let Some(plm) = self.points.get(sheetUid) { plm.queryAreaReferred(area, out); }
    }

    // java public void queryAll(Collection<T> out)
    pub fn queryAll(&self, out: &mut Vec<T>) {
        // java for each IntObjectHashMap.Entry, e.getValue().queryAll(out).
        for (_, plm) in self.points.iterator() { plm.queryAll(out); }
    }

    // java public void willSwap(boolean isUndo)
    pub fn willSwap(&mut self, isUndo: bool) {
        // java if (isUndo) this.reverseOp(isUndo, (Backup)this.rawData());
        if isUndo { self.reverseOp(isUndo); }
    }

    // java public void didFinishSwapping(boolean isUndo)
    pub fn didFinishSwapping(&mut self, isUndo: bool) {
        // java if (!isUndo) this.reverseOp(isUndo, (Backup)this.rawData());
        if !isUndo { self.reverseOp(isUndo); }
    }

    // java protected void reverseOp(boolean isUndo, Backup<T> data)
    fn reverseOp(&mut self, isUndo: bool) {
        // java CalcChain.recalcLock.lock(); try { ... } finally { unlock(); }
        let _lock = crate::cn::wps::et::ss::calcchain::recalcLock.lock();
        let backup = self.atom.rawData();
        let (op_type, sheet_uid, row, col, data) =
            (backup.opType, backup.sheetUid, backup.row, backup.col, backup.data.clone());
        // java if (data.opType != 0)
        if op_type == 0 { return; }
        // java invalid opType throws IllegalArgumentException.
        assert!(op_type == 1 || op_type == 2, "IllegalArgumentException");
        let data = data.expect("Backup.data");
        // java ADD+undo or DEL+redo removes the recorded reference.
        if (op_type == 1 && isUndo) || (op_type == 2 && !isUndo) {
            // java if (plm != null) plm.remove(data.row, data.col, data.data);
            if let Some(plm) = self.points.get_mut(sheet_uid) { plm.remove(row, col, &data); }
        } else {
            // java if (plm == null) create and put; plm.insert(data.row, data.col, data.data).
            if self.points.get(sheet_uid).is_none() {
                self.points.put(sheet_uid, PointLevelMap::PointLevelMap(self.sv));
            }
            self.points.get_mut(sheet_uid).unwrap().insert(row, col, data);
        }
    }

}

impl CellRefMap<()> {
    // java public static long cellPos2Long(int sheetUid, int row, int col)
    pub fn cellPos2Long(sheetUid: i32, row: i32, col: i32) -> i64 {
        // java return (long)sheetUid << 48 | ((long)row & 4294967295L) << 16 | (long)col & 65535L;
        ((sheetUid as i64) << 48) | (((row as i64) & 0xffff_ffff) << 16) | ((col as i64) & 0xffff)
    }

    // java public static int[] long2CellPos(long key)
    pub fn long2CellPos(key: i64) -> [i32; 3] {
        // java ret[0] = (int)(key >> 48 & 65535L);
        let sheet_uid = ((key >> 48) & 0xffff) as i32;
        // java ret[1] = (int)(key >> 16 & 4294967295L);
        let row = ((key >> 16) & 0xffff_ffff) as i32;
        // java ret[2] = (int)(key & 65535L);
        let col = (key & 0xffff) as i32;
        [sheet_uid, row, col]
    }
}

// rust: one stable handle for the Java CellRefMap object. Snapshot views share
// this exact state, including its KmoAtom field and live reference indexes.
pub struct CellRefMap<T = ()> {
    pub(crate) state: Rc<RefCell<CellRefMapState<T>>>,
    owner: Option<SharedAtom>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> CellRefMap<T> {
    // java public CellRefMap(SpreadsheetVersion sv)
    pub fn CellRefMap(sv: SpreadsheetVersion) -> Self {
        let state = Rc::new(RefCell::new(CellRefMapState::CellRefMap(sv)));
        let mut result = Self { state, owner: None };
        result.bindSnapshotOwner(Box::new(result.snapshot_view()));
        result
    }

    // rust: derived Java classes bind their overriding swap callbacks here.
    pub(crate) fn bindSnapshotOwner(&mut self, proxy: Box<dyn SnapshotAtom>) {
        let owner: SharedAtom = Rc::new(Mutex::new(proxy));
        self.state.borrow_mut().atom.bindSnapshotOwner(&owner);
        self.owner = Some(owner);
    }

    pub(crate) fn snapshot_view(&self) -> Self {
        Self { state: Rc::clone(&self.state), owner: None }
    }
    // java CellRefMap.addToMap: delegate through the retained object identity.
    pub fn addToMap(&mut self, sheetUid: i32, row: i32, col: i32, data: T, inTrans: bool) { self.state.borrow_mut().addToMap(sheetUid, row, col, data, inTrans) }
    // java CellRefMap.removeFromMap: delegate through the retained object identity.
    pub fn removeFromMap(&mut self, sheetUid: i32, row: i32, col: i32, data: &T) -> Option<T> { self.state.borrow_mut().removeFromMap(sheetUid, row, col, data) }
    // java CellRefMap.queryReferred: delegate through the retained object identity.
    pub fn queryReferred(&self, sheetUid: i32, row: i32, col: i32, out: &mut Vec<T>) { self.state.borrow().queryReferred(sheetUid, row, col, out) }
    // java CellRefMap.queryReferred__area: delegate through the retained object identity.
    pub fn queryReferred__area(&self, sheetUid: i32, area: &Area, out: &mut Vec<T>) { self.state.borrow().queryReferred__area(sheetUid, area, out) }
    // java CellRefMap.queryAll: delegate through the retained object identity.
    pub fn queryAll(&self, out: &mut Vec<T>) { self.state.borrow().queryAll(out) }
    // java public void willSwap(boolean isUndo)
    pub fn willSwap(&mut self, is_undo: bool) { self.state.borrow_mut().willSwap(is_undo); }
    // java public void didFinishSwapping(boolean isUndo)
    pub fn didFinishSwapping(&mut self, is_undo: bool) { self.state.borrow_mut().didFinishSwapping(is_undo); }
}

impl<T: Clone + PartialEq + Send + Sync + 'static> SnapshotAtom for CellRefMap<T> {
    fn willSwap(&mut self, is_undo: bool) { self.willSwap(is_undo); }
    fn swapData(&mut self, data: &mut Box<dyn SnapshotData>) { self.state.borrow_mut().atom.swapSnapshotData(data); }
    fn didFinishSwapping(&mut self, is_undo: bool) { self.didFinishSwapping(is_undo); }
    // java KmoAtom.isNeedSupportNonSequence() returns false.
    fn isNeedSupportNonSequence(&self) -> bool { false }
    fn cloneAtom(&self) -> SharedAtom { panic!("non-sequence clone is not used by CellRefMap") }
}
