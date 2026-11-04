// java package cn.wps.et.ss.dependent.refer.base;

use crate::java_runtime::{Rc, RefCell};
use kfc::cn::wps::memor4j::AtomSnapshot::{SharedAtom, KmoAtom as SnapshotAtom, KmoAtomData as SnapshotData};
use std::sync::Mutex;


// java import cn.wps.et.ss.util.primeval.objmap.LongObjectHashMap;
use crate::cn::wps::et::ss::util::primeval::objmap::LongObjectHashMap::LongObjectHashMap;
// java import cn.wps.memor4j.KmoAtom/KmoAtomData;
use kfc::cn::wps::memor4j::KmoAtom::KmoAtom;
use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData, KmoAtomDataSub};
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

// java protected static class Backup<T> extends KmoAtomData
pub(crate) struct Backup<T> {
    base: KmoAtomData,
    pub(crate) nameNumber: i32,
    pub(crate) data: Option<T>,
    pub(crate) opType: i32,
}

impl<T> Backup<T> {
    fn new() -> Self { Self { base: KmoAtomData::new(), nameNumber: 0, data: None, opType: 0 } }
}

impl<T> KmoAtomDataSub for Backup<T> {
    fn as_base(&self) -> &KmoAtomData { &self.base }
    fn as_base_mut(&mut self) -> &mut KmoAtomData { &mut self.base }
    // java Backup.copy(): Backup<T> ret = new Backup<T>(); ret.__forceOldest(); return ret;
    fn copy(&self) -> Self { let mut result = Self::new(); result.base.__forceOldest(); result }
}

// java public class NameRefMap<T> extends KmoAtom<NameRefMap.Backup<T>>
pub(crate) struct NameRefMapState<T> {
    pub(crate) atom: KmoAtom<Backup<T>>,
    // java private final SpreadsheetVersion sv;
    _sv: SpreadsheetVersion,
    // java protected LongObjectHashMap<ArrayList<T>> map = new LongObjectHashMap();
    pub(crate) map: LongObjectHashMap<Vec<T>>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> NameRefMapState<T> {
    // java public NameRefMap(SpreadsheetVersion sv)
    pub fn NameRefMap(sv: SpreadsheetVersion) -> Self {
        // java super(new Backup()); this.sv = sv;
        Self { atom: KmoAtom::new(Backup::new()), _sv: sv, map: LongObjectHashMap::LongObjectHashMap() }
    }

    // java public void addToMap(int nameNumber, T data, boolean inTrans)
    pub fn addToMap(&mut self, name_number: i32, data: T, in_trans: bool) {
        // java if (inTrans) { beforeWrite(); Backup.nameNumber/data/opType = nameNumber/data/1; }
        if in_trans {
            self.atom.beforeWrite();
            let mut backup = self.atom.rawData_mut();
            backup.nameNumber = name_number;
            backup.data = Some(data.clone());
            backup.opType = 1;
        }
        // java if (objects == null) { objects = new ArrayList(); map.put(nameNumber, objects); }
        if self.map.get(name_number as i64).is_none() { self.map.put(name_number as i64, Vec::new()); }
        // java objects.add(data);
        self.map.get_mut(name_number as i64).unwrap().push(data);
    }

    // java public T removeFromMap(int nameNumber, T data)
    pub fn removeFromMap(&mut self, name_number: i32, data: &T) -> Option<T> {
        // java beforeWrite(); Backup.nameNumber/data/opType = nameNumber/data/2;
        self.atom.beforeWrite();
        let mut backup = self.atom.rawData_mut();
        backup.nameNumber = name_number;
        backup.data = Some(data.clone());
        backup.opType = 2;
        // java if (objects == null) return null; else indexOf(data), remove(index).
        let objects = self.map.get_mut(name_number as i64)?;
        let index = objects.iter().position(|value| data == value)?;
        Some(objects.remove(index))
    }

    // java public void queryReferred(int nameNumber, Collection<T> out)
    pub fn queryReferred(&self, name_number: i32, out: &mut Vec<T>) {
        // java if (objects != null) out.addAll(objects);
        if let Some(objects) = self.map.get(name_number as i64) { out.extend(objects.iter().cloned()); }
    }

    // java public void willSwap(boolean isUndo)
    pub fn willSwap(&mut self, is_undo: bool) {
        // java if (isUndo) reverseOp(isUndo, (Backup)rawData());
        if is_undo { self.reverseOp(is_undo); }
    }

    // java public void didFinishSwapping(boolean isUndo)
    pub fn didFinishSwapping(&mut self, is_undo: bool) {
        // java if (!isUndo) reverseOp(isUndo, (Backup)rawData());
        if !is_undo { self.reverseOp(is_undo); }
    }

    // java protected void reverseOp(boolean isUndo, Backup<T> data)
    fn reverseOp(&mut self, is_undo: bool) {
        let backup = self.atom.rawData();
        let (op_type, name_number, data) = (backup.opType, backup.nameNumber, backup.data.clone());
        // java if (data.opType != 0)
        if op_type == 0 { return; }
        // java invalid opType throws IllegalArgumentException.
        assert!(op_type == 1 || op_type == 2, "IllegalArgumentException");
        let data = data.expect("Backup.data");
        // java ADD+undo or DEL+redo removes the first matching entry.
        if (op_type == 1 && is_undo) || (op_type == 2 && !is_undo) {
            // java if (objects != null) objects.remove(data.data);
            if let Some(objects) = self.map.get_mut(name_number as i64) {
                if let Some(index) = objects.iter().position(|value| &data == value) { objects.remove(index); }
            }
        } else {
            // java if (objects == null) create; objects.add(data.data);
            if self.map.get(name_number as i64).is_none() { self.map.put(name_number as i64, Vec::new()); }
            self.map.get_mut(name_number as i64).unwrap().push(data);
        }
    }

    // java public void queryAll(Collection<T> out)
    pub fn queryAll(&self, out: &mut Vec<T>) {
        // java for(LongObjectHashMap.Entry<ArrayList<T>> entry : map) out.addAll(entry.getValue());
        for (_, objects) in self.map.iterator() { out.extend(objects.iter().cloned()); }
    }
}

// rust: one stable handle for the Java NameRefMap object. Snapshot views share
// this exact state, including its KmoAtom field and live reference indexes.
pub struct NameRefMap<T = ()> {
    pub(crate) state: Rc<RefCell<NameRefMapState<T>>>,
    owner: Option<SharedAtom>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> NameRefMap<T> {
    // java public NameRefMap(SpreadsheetVersion sv)
    pub fn NameRefMap(sv: SpreadsheetVersion) -> Self {
        let state = Rc::new(RefCell::new(NameRefMapState::NameRefMap(sv)));
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
    // java NameRefMap.addToMap: delegate through the retained object identity.
    pub fn addToMap(&mut self, name_number: i32, data: T, in_trans: bool) { self.state.borrow_mut().addToMap(name_number, data, in_trans) }
    // java NameRefMap.removeFromMap: delegate through the retained object identity.
    pub fn removeFromMap(&mut self, name_number: i32, data: &T) -> Option<T> { self.state.borrow_mut().removeFromMap(name_number, data) }
    // java NameRefMap.queryReferred: delegate through the retained object identity.
    pub fn queryReferred(&self, name_number: i32, out: &mut Vec<T>) { self.state.borrow().queryReferred(name_number, out) }
    // java NameRefMap.queryAll: delegate through the retained object identity.
    pub fn queryAll(&self, out: &mut Vec<T>) { self.state.borrow().queryAll(out) }
    // java public void willSwap(boolean isUndo)
    pub fn willSwap(&mut self, is_undo: bool) { self.state.borrow_mut().willSwap(is_undo); }
    // java public void didFinishSwapping(boolean isUndo)
    pub fn didFinishSwapping(&mut self, is_undo: bool) { self.state.borrow_mut().didFinishSwapping(is_undo); }
}

impl<T: Clone + PartialEq + Send + Sync + 'static> SnapshotAtom for NameRefMap<T> {
    fn willSwap(&mut self, is_undo: bool) { self.willSwap(is_undo); }
    fn swapData(&mut self, data: &mut Box<dyn SnapshotData>) { self.state.borrow_mut().atom.swapSnapshotData(data); }
    fn didFinishSwapping(&mut self, is_undo: bool) { self.didFinishSwapping(is_undo); }
    // java KmoAtom.isNeedSupportNonSequence() returns false.
    fn isNeedSupportNonSequence(&self) -> bool { false }
    fn cloneAtom(&self) -> SharedAtom { panic!("non-sequence clone is not used by NameRefMap") }
}
