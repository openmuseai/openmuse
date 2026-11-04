// java package cn.wps.et.ss.dependent.refer.base;

use crate::java_runtime::{Rc, RefCell};
use kfc::cn::wps::memor4j::AtomSnapshot::{SharedAtom, KmoAtom as SnapshotAtom, KmoAtomData as SnapshotData};
use std::sync::Mutex;


// java import cn.wps.et.ss.util.Area;
use crate::cn::wps::et::ss::util::Area::Area;
// java import cn.wps.et.ss.util.level.LineLevelMap/RectLevelMap;
use crate::cn::wps::et::ss::util::level::LineLevelMap::LineLevelMap;
use crate::cn::wps::et::ss::util::level::RectLevelMap::RectLevelMap;
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
    pub(crate) firstRow: i32,
    pub(crate) lastRow: i32,
    pub(crate) firstCol: i32,
    pub(crate) lastCol: i32,
    pub(crate) data: Option<T>,
    pub(crate) opType: i32,
}

impl<T> Backup<T> {
    fn new() -> Self { Self { base: KmoAtomData::new(), sheetUid: 0, firstRow: 0, lastRow: 0, firstCol: 0, lastCol: 0, data: None, opType: 0 } }
    fn record(&mut self, sheetUid: i32, area: &Area, data: T, opType: i32) {
        self.sheetUid = sheetUid;
        self.firstRow = area.getFirstRow(); self.lastRow = area.getLastRow();
        self.firstCol = area.getFirstCol(); self.lastCol = area.getLastCol();
        self.data = Some(data); self.opType = opType;
    }
}

impl<T> KmoAtomDataSub for Backup<T> {
    fn as_base(&self) -> &KmoAtomData { &self.base }
    fn as_base_mut(&mut self) -> &mut KmoAtomData { &mut self.base }
    // java Backup.copy(): new Backup(); ret.__forceOldest(); return ret;
    fn copy(&self) -> Self { let mut result = Self::new(); result.base.__forceOldest(); result }
}

// java public class AreaRefMap<T> extends KmoAtom<AreaRefMap.Backup<T>>
pub(crate) struct AreaRefMapState<T> {
    pub(crate) atom: KmoAtom<Backup<T>>,
    // java protected final SpreadsheetVersion sv;
    pub(crate) sv: SpreadsheetVersion,
    // java protected final IntObjectHashMap<LineLevelMap<T>> lines = new IntObjectHashMap();
    pub(crate) lines: IntObjectHashMap<LineLevelMap<T>>,
    // java protected final IntObjectHashMap<RectLevelMap<T>> rects = new IntObjectHashMap();
    pub(crate) rects: IntObjectHashMap<RectLevelMap<T>>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> AreaRefMapState<T> {
    // java public AreaRefMap(SpreadsheetVersion sv)
    pub fn AreaRefMap(sv: SpreadsheetVersion) -> Self {
        // java super(new Backup()); this.sv = sv;
        Self { atom: KmoAtom::new(Backup::new()), sv, lines: IntObjectHashMap::IntObjectHashMap(), rects: IntObjectHashMap::IntObjectHashMap() }
    }

    // java public void addToMap(int sheetUid, Area area, T data, boolean inTrans)
    pub fn addToMap(&mut self, sheetUid: i32, area: &Area, data: T, inTrans: bool) {
        // java if (inTrans) beforeWrite(); Backup records bounds/data/opType=ADD.
        if inTrans { self.atom.beforeWrite(); self.atom.rawData_mut().record(sheetUid, area, data.clone(), 1); }
        // java if (area.isSingleRow()) use LineLevelMap.insertHorizontalLine.
        if area.isSingleRow() {
            if self.lines.get(sheetUid).is_none() { self.lines.put(sheetUid, LineLevelMap::LineLevelMap(self.sv)); }
            self.lines.get_mut(sheetUid).unwrap().insertHorizontalLine(area.getFirstRow(), area.getFirstCol(), area.getLastCol(), data);
        } else if area.isSingleCol() {
            // java else if (area.isSingleCol()) use LineLevelMap.insertVerticalLine.
            if self.lines.get(sheetUid).is_none() { self.lines.put(sheetUid, LineLevelMap::LineLevelMap(self.sv)); }
            self.lines.get_mut(sheetUid).unwrap().insertVerticalLine(area.getFirstRow(), area.getLastRow(), area.getFirstCol(), data);
        } else {
            // java else use RectLevelMap.insert.
            if self.rects.get(sheetUid).is_none() { self.rects.put(sheetUid, RectLevelMap::RectLevelMap(self.sv)); }
            self.rects.get_mut(sheetUid).unwrap().insert(area.getFirstRow(), area.getLastRow(), area.getFirstCol(), area.getLastCol(), data);
        }
    }

    // java public T removeFromMap(int sheetUid, Area area, T data)
    pub fn removeFromMap(&mut self, sheetUid: i32, area: &Area, data: &T) -> Option<T> {
        // java beforeWrite(); Backup records bounds/data/opType=DEL.
        self.atom.beforeWrite();
        self.atom.rawData_mut().record(sheetUid, area, data.clone(), 2);
        // java choose horizontal first, then vertical, then rectangle, matching addToMap.
        if area.isSingleRow() {
            self.lines.get_mut(sheetUid)?.removeHorizontalLine(area.getFirstRow(), area.getFirstCol(), area.getLastCol(), data)
        } else if area.isSingleCol() {
            self.lines.get_mut(sheetUid)?.removeVerticalLine(area.getFirstRow(), area.getLastRow(), area.getFirstCol(), data)
        } else {
            self.rects.get_mut(sheetUid)?.remove(area.getFirstRow(), area.getLastRow(), area.getFirstCol(), area.getLastCol(), data)
        }
    }

    // java public void queryAreaReferred(int sheetUid, int row, int col, Collection<T> out)
    pub fn queryAreaReferred(&self, sheetUid: i32, row: i32, col: i32, out: &mut Vec<T>) {
        // java query lines first, then rects.
        if let Some(lines) = self.lines.get(sheetUid) { lines.queryReferred(row, col, out); }
        if let Some(rects) = self.rects.get(sheetUid) { rects.queryAreaReferred(row, col, out); }
    }

    // java public void queryAreaReferred(int sheetUid, Area area, Collection<T> out)
    pub fn queryAreaReferred__area(&self, sheetUid: i32, area: &Area, out: &mut Vec<T>) {
        // java query lines first, then rects.
        if let Some(lines) = self.lines.get(sheetUid) { lines.queryReferred__area(area, out); }
        if let Some(rects) = self.rects.get(sheetUid) { rects.queryAreaReferred__area(area, out); }
    }

    // java public void queryAll(Collection<T> out)
    pub fn queryAll(&self, out: &mut Vec<T>) {
        // java for each lines entry queryAll, then each rects entry queryAll; preserve IntObjectHashMap order.
        for (_, lines) in self.lines.iterator() { lines.queryAll(out); }
        for (_, rects) in self.rects.iterator() { rects.queryAll(out); }
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
        let backup = self.atom.rawData();
        let (op_type, sheet_uid, first_row, last_row, first_col, last_col, data) =
            (backup.opType, backup.sheetUid, backup.firstRow, backup.lastRow,
             backup.firstCol, backup.lastCol, backup.data.clone());
        // java if (data.opType != 0)
        if op_type == 0 { return; }
        // java invalid opType throws IllegalArgumentException.
        assert!(op_type == 1 || op_type == 2, "IllegalArgumentException");
        let data = data.expect("Backup.data");
        // java ADD+undo or DEL+redo removes; DEL+undo or ADD+redo inserts.
        let remove = (op_type == 1 && isUndo) || (op_type == 2 && !isUndo);
        // java firstRow == lastRow is tested before firstCol == lastCol.
        if first_row == last_row {
            if remove {
                // java if (llm != null) llm.removeHorizontalLine(...).
                if let Some(llm) = self.lines.get_mut(sheet_uid) {
                    llm.removeHorizontalLine(first_row, first_col, last_col, &data);
                }
            } else {
                // java if (llm == null) create and put; llm.insertHorizontalLine(...).
                if self.lines.get(sheet_uid).is_none() {
                    self.lines.put(sheet_uid, LineLevelMap::LineLevelMap(self.sv));
                }
                self.lines.get_mut(sheet_uid).unwrap().insertHorizontalLine(first_row, first_col, last_col, data);
            }
        } else if first_col == last_col {
            if remove {
                // java if (llm != null) llm.removeVerticalLine(...).
                if let Some(llm) = self.lines.get_mut(sheet_uid) {
                    llm.removeVerticalLine(first_row, last_row, first_col, &data);
                }
            } else {
                // java if (llm == null) create and put; llm.insertVerticalLine(...).
                if self.lines.get(sheet_uid).is_none() {
                    self.lines.put(sheet_uid, LineLevelMap::LineLevelMap(self.sv));
                }
                self.lines.get_mut(sheet_uid).unwrap().insertVerticalLine(first_row, last_row, first_col, data);
            }
        } else if remove {
            // java if (rlm != null) rlm.remove(...).
            if let Some(rlm) = self.rects.get_mut(sheet_uid) {
                rlm.remove(first_row, last_row, first_col, last_col, &data);
            }
        } else {
            // java if (rlm == null) create and put; rlm.insert(...).
            if self.rects.get(sheet_uid).is_none() {
                self.rects.put(sheet_uid, RectLevelMap::RectLevelMap(self.sv));
            }
            self.rects.get_mut(sheet_uid).unwrap().insert(first_row, last_row, first_col, last_col, data);
        }
    }
}

// rust: one stable handle for the Java AreaRefMap object. Snapshot views share
// this exact state, including its KmoAtom field and live reference indexes.
pub struct AreaRefMap<T = ()> {
    pub(crate) state: Rc<RefCell<AreaRefMapState<T>>>,
    owner: Option<SharedAtom>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> AreaRefMap<T> {
    // java public AreaRefMap(SpreadsheetVersion sv)
    pub fn AreaRefMap(sv: SpreadsheetVersion) -> Self {
        let state = Rc::new(RefCell::new(AreaRefMapState::AreaRefMap(sv)));
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
    // java AreaRefMap.addToMap: delegate through the retained object identity.
    pub fn addToMap(&mut self, sheetUid: i32, area: &Area, data: T, inTrans: bool) { self.state.borrow_mut().addToMap(sheetUid, area, data, inTrans) }
    // java AreaRefMap.removeFromMap: delegate through the retained object identity.
    pub fn removeFromMap(&mut self, sheetUid: i32, area: &Area, data: &T) -> Option<T> { self.state.borrow_mut().removeFromMap(sheetUid, area, data) }
    // java AreaRefMap.queryAreaReferred: delegate through the retained object identity.
    pub fn queryAreaReferred(&self, sheetUid: i32, row: i32, col: i32, out: &mut Vec<T>) { self.state.borrow().queryAreaReferred(sheetUid, row, col, out) }
    // java AreaRefMap.queryAreaReferred__area: delegate through the retained object identity.
    pub fn queryAreaReferred__area(&self, sheetUid: i32, area: &Area, out: &mut Vec<T>) { self.state.borrow().queryAreaReferred__area(sheetUid, area, out) }
    // java AreaRefMap.queryAll: delegate through the retained object identity.
    pub fn queryAll(&self, out: &mut Vec<T>) { self.state.borrow().queryAll(out) }
    // java public void willSwap(boolean isUndo)
    pub fn willSwap(&mut self, is_undo: bool) { self.state.borrow_mut().willSwap(is_undo); }
    // java public void didFinishSwapping(boolean isUndo)
    pub fn didFinishSwapping(&mut self, is_undo: bool) { self.state.borrow_mut().didFinishSwapping(is_undo); }
}

impl<T: Clone + PartialEq + Send + Sync + 'static> SnapshotAtom for AreaRefMap<T> {
    fn willSwap(&mut self, is_undo: bool) { self.willSwap(is_undo); }
    fn swapData(&mut self, data: &mut Box<dyn SnapshotData>) { self.state.borrow_mut().atom.swapSnapshotData(data); }
    fn didFinishSwapping(&mut self, is_undo: bool) { self.didFinishSwapping(is_undo); }
    // java KmoAtom.isNeedSupportNonSequence() returns false.
    fn isNeedSupportNonSequence(&self) -> bool { false }
    fn cloneAtom(&self) -> SharedAtom { panic!("non-sequence clone is not used by AreaRefMap") }
}
