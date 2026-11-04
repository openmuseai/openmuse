//! 03_function_dgstore_update — DgStore.add / update / query

use ppt_core::cn::wps::drawing::app::DgStore::DgStore;
use ppt_core::cn::wps::drawing::app::DgStoreData::DgStoreData;
use ppt_core::cn::wps::kso::kproto::KoBin::Null;

#[test]
fn dg1_add_type1_query_type() {
    let mut store = DgStore::DgStore_new();
    let id = store.add(1);
    assert_eq!(DgStoreData::queryType(id), 1);
    assert_eq!(store.helper_getIndexById(id), 0);
}

#[test]
fn dg2_second_add_increments_index() {
    let mut store = DgStore::DgStore_new();
    let id0 = store.add(1);
    let id1 = store.add(1);
    assert_eq!(store.helper_getIndexById(id0), 0);
    assert_eq!(store.helper_getIndexById(id1), 1);
    assert_eq!(DgStoreData::queryType(id0), 1);
    assert_eq!(DgStoreData::queryType(id1), 1);
}

#[test]
fn dg3_types_split_stores() {
    let mut store = DgStore::DgStore_new();
    for t in [0, 2, 3, 4] {
        let id = store.add(t);
        assert_eq!(DgStoreData::queryType(id), t, "type {t}");
        assert_eq!(store.helper_getIndexById(id), 0);
    }
}

#[test]
fn dg4_update_good_spid() {
    let mut store = DgStore::DgStore_new();
    let id = store.add(1);
    let bin = Null::value1();
    store.update(id, bin.as_ref());
}

#[test]
fn dg5_update_bad_spid_is_noop() {
    let mut store = DgStore::DgStore_new();
    let bin = Null::value1();
    store.update((1 << 20) | 99, bin.as_ref());
}

#[test]
fn dg6_add_illegal_type_panics() {
    let mut store = DgStore::DgStore_new();
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        store.add(99);
    }));
    assert!(r.is_err());
}
