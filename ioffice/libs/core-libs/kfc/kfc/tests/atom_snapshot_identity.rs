use core_libs_kfc_main::cn::wps::memor4j::AtomSnapshot::{KmoAtom, KmoAtomData, SharedAtom};
use core_libs_kfc_main::cn::wps::memor4j::KmoAtomData::KmoAtomData as BaseData;
use core_libs_kfc_main::cn::wps::memor4j::KmoAtomStore::KmoAtomStore;
use std::sync::{Arc, Mutex};

struct TestData { base: BaseData, value: i32 }

impl TestData { fn new(value: i32) -> Self { Self { base: BaseData::new(), value } } }

impl KmoAtomData for TestData {
    fn tip(&self) -> i32 { self.base.tip() }
    fn copy(&self) -> Box<dyn KmoAtomData> { Box::new(Self::new(self.value)) }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

struct TestState {
    current: Box<dyn KmoAtomData>,
    events: Vec<String>,
}

impl TestState {
    fn value(&self) -> i32 {
        self.current.as_any().downcast_ref::<TestData>().unwrap().value
    }
    fn set_value(&mut self, value: i32) {
        self.current.as_any_mut().downcast_mut::<TestData>().unwrap().value = value;
    }
}

struct TestAtom { state: Arc<Mutex<TestState>> }

impl KmoAtom for TestAtom {
    fn willSwap(&mut self, is_undo: bool) {
        let mut state = self.state.lock().unwrap();
        let value = state.value();
        state.events.push(format!("will:{is_undo}:{value}"));
    }
    fn swapData(&mut self, data: &mut Box<dyn KmoAtomData>) {
        let mut state = self.state.lock().unwrap();
        std::mem::swap(&mut state.current, data);
    }
    fn didFinishSwapping(&mut self, is_undo: bool) {
        let mut state = self.state.lock().unwrap();
        let value = state.value();
        state.events.push(format!("did:{is_undo}:{value}"));
    }
    fn isNeedSupportNonSequence(&self) -> bool { false }
    fn cloneAtom(&self) -> SharedAtom {
        Arc::new(Mutex::new(Box::new(Self { state: self.state.clone() })))
    }
}

#[test]
fn need_copy_snapshot_swaps_the_original_atom_on_undo_and_redo() {
    KmoAtomStore::clearUp();
    let state = Arc::new(Mutex::new(TestState { current: Box::new(TestData::new(0)), events: Vec::new() }));
    let atom: SharedAtom = Arc::new(Mutex::new(Box::new(TestAtom { state: state.clone() })));
    // java AtomStoreImpl.start() sets tip=1 and creates workingSnap.
    KmoAtomStore::sharedStore().as_mut().unwrap().start();
    // java KmoAtom.beforeWrite() calls needCopy(this, rawData), keeping this atom's identity.
    let original_data = {
        let mut state = state.lock().unwrap();
        std::mem::replace(&mut state.current, Box::new(TestData::new(-1)))
    };
    let copied = KmoAtomStore::sharedStore().as_mut().unwrap().needCopy(atom.clone(), original_data);
    assert_eq!(copied.tip(), 1);
    state.lock().unwrap().current = copied;
    state.lock().unwrap().set_value(9);
    // java second beforeWrite in the same tip returns data without appending
    // another AtomImage, because data.tip() is no longer below store.tip().
    let current_data = {
        let mut state = state.lock().unwrap();
        std::mem::replace(&mut state.current, Box::new(TestData::new(-1)))
    };
    let unchanged = KmoAtomStore::sharedStore().as_mut().unwrap().needCopy(atom.clone(), current_data);
    assert_eq!(unchanged.tip(), 1);
    state.lock().unwrap().current = unchanged;
    state.lock().unwrap().set_value(12);
    assert!(KmoAtomStore::sharedStore().as_mut().unwrap().commit());
    assert_eq!(state.lock().unwrap().value(), 12);
    KmoAtomStore::sharedStore().as_mut().unwrap().undo();
    assert_eq!(state.lock().unwrap().value(), 0);
    assert_eq!(state.lock().unwrap().current.tip(), 0);
    KmoAtomStore::sharedStore().as_mut().unwrap().redo();
    let state = state.lock().unwrap();
    assert_eq!(state.value(), 12);
    assert_eq!(state.current.tip(), 1);
    assert_eq!(state.events, ["will:true:12", "did:true:0", "will:false:0", "did:false:12"]);
    KmoAtomStore::clearUp();
}
