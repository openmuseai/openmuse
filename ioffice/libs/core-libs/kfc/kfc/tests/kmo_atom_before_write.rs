use core_libs_kfc_main::cn::wps::memor4j::EmptyAtomData::EmptyAtomData;
use core_libs_kfc_main::cn::wps::memor4j::KmoAtom::KmoAtom;
use core_libs_kfc_main::cn::wps::memor4j::KmoAtomStore::KmoAtomStore;
use core_libs_kfc_main::cn::wps::memor4j::KmoAtomData::KmoAtomDataSub;
static STORE_TEST: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn before_write_after_snapshot_without_working_snapshot_returns_original_data() {
    let _serial = STORE_TEST.lock().unwrap();
    KmoAtomStore::clearUp();
    let mut atom = KmoAtom::new(EmptyAtomData::new());
    // java AtomStoreImpl.snapshot() advances tip without creating workingSnap.
    KmoAtomStore::sharedStore().as_mut().unwrap().snapshot();
    // java needCopy: data.tip < tip, but workingSnap == null => return data.
    atom.beforeWrite();
    assert_eq!(atom.rawData().as_base().tip(), 0);
    KmoAtomStore::clearUp();
}

#[test]
fn before_write_when_atom_store_is_disabled_returns_original_data() {
    let _serial = STORE_TEST.lock().unwrap();
    KmoAtomStore::clearUp();
    let mut atom = KmoAtom::new(EmptyAtomData::new());
    KmoAtomStore::sharedStore().as_mut().unwrap().start();
    KmoAtomStore::disable();
    // java needCopy: if (disable) return data before inspecting workingSnap.
    atom.beforeWrite();
    assert_eq!(atom.rawData().as_base().tip(), 0);
    KmoAtomStore::enable();
    KmoAtomStore::clearUp();
}

#[test]
fn local_subclass_keeps_the_before_write_facade_without_a_send_bound() {
    use core_libs_kfc_main::cn::wps::memor4j::KmoAtomData::KmoAtomData;
    use std::rc::Rc;
    struct LocalData { base: KmoAtomData, value: Rc<std::cell::Cell<i32>> }
    impl KmoAtomDataSub for LocalData {
        fn as_base(&self) -> &KmoAtomData { &self.base }
        fn as_base_mut(&mut self) -> &mut KmoAtomData { &mut self.base }
        fn copy(&self) -> Self { Self { base: KmoAtomData::new(), value: self.value.clone() } }
    }
    let _serial = STORE_TEST.lock().unwrap();
    KmoAtomStore::enable();
    KmoAtomStore::clearUp();
    let value = Rc::new(std::cell::Cell::new(7));
    let mut atom = KmoAtom::new(LocalData { base: KmoAtomData::new(), value: value.clone() });
    KmoAtomStore::sharedStore().as_mut().unwrap().snapshot();
    atom.beforeWrite();
    assert!(Rc::ptr_eq(&atom.rawData().value, &value));
    assert_eq!(value.get(), 7);
    KmoAtomStore::clearUp();
}
