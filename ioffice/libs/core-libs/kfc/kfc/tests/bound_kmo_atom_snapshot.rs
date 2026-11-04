use core_libs_kfc_main::cn::wps::memor4j::AtomSnapshot::{KmoAtom as SnapshotAtom, KmoAtomData as SnapshotData, SharedAtom};
use core_libs_kfc_main::cn::wps::memor4j::KmoAtom::KmoAtom;
use core_libs_kfc_main::cn::wps::memor4j::KmoAtomData::{KmoAtomData, KmoAtomDataSub};
use core_libs_kfc_main::cn::wps::memor4j::KmoAtomStore::KmoAtomStore;
use std::sync::{Arc, Mutex};

struct Data { base: KmoAtomData, value: i32 }
impl Data { fn new(value: i32) -> Self { Self { base: KmoAtomData::new(), value } } }
impl KmoAtomDataSub for Data {
    fn as_base(&self) -> &KmoAtomData { &self.base }
    fn as_base_mut(&mut self) -> &mut KmoAtomData { &mut self.base }
    fn copy(&self) -> Self { Self::new(self.value) }
}
struct State { atom: KmoAtom<Data>, events: Vec<String> }
struct Owner { state: Arc<Mutex<State>> }
impl SnapshotAtom for Owner {
    fn willSwap(&mut self, undo: bool) {
        let mut state = self.state.lock().unwrap();
        let value = state.atom.rawData().value;
        state.events.push(format!("will:{undo}:{value}"));
    }
    fn swapData(&mut self, data: &mut Box<dyn SnapshotData>) { self.state.lock().unwrap().atom.swapSnapshotData(data); }
    fn didFinishSwapping(&mut self, undo: bool) {
        let mut state = self.state.lock().unwrap();
        let value = state.atom.rawData().value;
        state.events.push(format!("did:{undo}:{value}"));
    }
    fn isNeedSupportNonSequence(&self) -> bool { false }
    fn cloneAtom(&self) -> SharedAtom { panic!("non-sequence cloning is not requested") }
}

#[test]
fn bound_generic_before_write_matches_the_original_java_atom_probe() {
    KmoAtomStore::enable();
    KmoAtomStore::clearUp();
    let state = Arc::new(Mutex::new(State { atom: KmoAtom::new(Data::new(0)), events: Vec::new() }));
    let owner: SharedAtom = Arc::new(Mutex::new(Box::new(Owner { state: state.clone() })));
    state.lock().unwrap().atom.bindSnapshotOwner(&owner);
    KmoAtomStore::sharedStore().as_mut().unwrap().start();
    for value in [9, 12] {
        let mut state = state.lock().unwrap();
        state.atom.beforeWrite();
        state.atom.rawData_mut().value = value;
    }
    assert!(KmoAtomStore::sharedStore().as_mut().unwrap().commit());
    assert_eq!(state.lock().unwrap().atom.rawData().value, 12);
    KmoAtomStore::sharedStore().as_mut().unwrap().undo();
    assert_eq!(state.lock().unwrap().atom.rawData().value, 0);
    KmoAtomStore::sharedStore().as_mut().unwrap().redo();
    let state = state.lock().unwrap();
    assert_eq!(state.atom.rawData().value, 12);
    // java AtomSnapshotParityProbe's observed callback sequence; second write
    // in the same tip must not register another AtomImage for ordinary Data.
    assert_eq!(state.events, ["will:true:12", "did:true:0", "will:false:0", "did:false:12"]);
    drop(state);
    KmoAtomStore::clearUp();
}
