// function_id=01_function_property_set
// cargo test --manifest-path libs/core-libs/util/util/Cargo.toml --test 01_function_property_set
// src:  libs/core-libs/util/util/src/cn/wps/moffice/property/{PropertySet,MutablePropertySet}.rs
// test: libs/core-libs/util/util/tests/cn/wps/moffice/property/PropertySet.rs

use core_libs_util::cn::wps::moffice::property::MutablePropertySet::MutablePropertySet;
use core_libs_util::cn::wps::moffice::property::PropertySet::{empty as PROPERTYSET_EMPTY, PropertySet, PropertyValue};
use core_libs_util::cn::wps::moffice::property::SetPropertyMode;
use std::sync::Arc;

/// Kernel propId used by papx (`SubDocument`); not an OOXML number.
const PROP_ID: i32 = 193;
const PROP_ID_B: i32 = 191;
const OTHER_BUCKET: i32 = 8; // first slot of bucket 1 (id >> 3)

fn freeze_int(id: i32, v: i32) -> PropertySet {
    let mut mps = MutablePropertySet::new();
    mps.setInt(id, v);
    mps.freeze()
}

fn ids_of(ps: &PropertySet) -> Vec<i32> {
    let n = ps.size() as usize;
    let mut ids = vec![0; n];
    ps.getAll(&mut ids, None);
    ids
}

#[test]
fn ps1_set_int_roundtrip() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_ID, 1);
    let frozen = mps.freeze();
    assert_eq!(frozen.getInt(PROP_ID), 1);
}

#[test]
fn ps2_missing_id_is_empty() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_ID, 1);
    let frozen = mps.freeze();
    assert!(!frozen.contains(0));
    assert!(!frozen.contains(194));
    assert!(frozen.getValue(194).is_none());
    assert_eq!(frozen.getInt_default(194, 0), 0);
}

#[test]
fn ps3_freeze_identity() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_ID, 1);
    let frozen = mps.freeze();
    let published = mps.get();
    assert!(frozen.isFrozen());
    assert!(published.isFrozen());
    assert_eq!(frozen.getInt(PROP_ID), 1);
    assert_eq!(published.getInt(PROP_ID), 1);
    assert_eq!(ids_of(&frozen), ids_of(published));
}

#[test]
fn ps4_typed_setters_roundtrip() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(1, 42);
    mps.setFloat(2, 1.5);
    mps.setBoolean(3, true);
    mps.setString(4, "hi".to_string());
    let frozen = mps.freeze();
    assert_eq!(frozen.getInt(1), 42);
    assert_eq!(frozen.getFloat(2), 1.5);
    assert!(frozen.getBoolean(3));
    assert_eq!(frozen.getString(4), "hi");
    assert_eq!(frozen.size(), 4);
}

#[test]
fn ps5_overwrite_same_id_keeps_size() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_ID, 1);
    mps.setInt(PROP_ID, 2);
    assert_eq!(mps.size(), 1);
    assert_eq!(mps.getInt(PROP_ID), 2);
    let frozen = mps.freeze();
    assert_eq!(frozen.size(), 1);
    assert_eq!(frozen.getInt(PROP_ID), 2);
}

#[test]
fn ps6_remove_clears_slot() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_ID, 1);
    mps.remove(PROP_ID);
    assert!(mps.isEmpty());
    assert!(!mps.contains(PROP_ID));
    assert!(mps.getValue(PROP_ID).is_none());
}

#[test]
fn ps7_clear_empties_all() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(0, 1);
    mps.setInt(PROP_ID, 2);
    mps.clear();
    assert!(mps.isEmpty());
    assert_eq!(mps.size(), 0);
}

#[test]
fn ps8_multi_bucket_ids() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(0, 10);
    mps.setInt(7, 11);
    mps.setInt(OTHER_BUCKET, 12);
    mps.setInt(PROP_ID, 13);
    let frozen = mps.freeze();
    assert_eq!(frozen.size(), 4);
    assert_eq!(ids_of(&frozen), vec![0, 7, OTHER_BUCKET, PROP_ID]);
    assert_eq!(frozen.getInt(0), 10);
    assert_eq!(frozen.getInt(7), 11);
    assert_eq!(frozen.getInt(OTHER_BUCKET), 12);
    assert_eq!(frozen.getInt(PROP_ID), 13);
}

#[test]
fn ps9_get_defaults_and_getters_on_mps() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(1, 9);
    mps.setBoolean(2, false);
    mps.setFloat(3, 0.25);
    mps.setString(4, "x".to_string());
    assert_eq!(mps.getInt_with_default(99, -1), -1);
    assert_eq!(mps.getBoolean_with_default(99, true), true);
    assert_eq!(mps.getFloat_with_default(99, 3.0), 3.0);
    assert_eq!(mps.getString_with_default(99, "d"), "d");
    assert_eq!(mps.getInt(1), 9);
    assert!(!mps.getBoolean(2));
    assert_eq!(mps.getFloat(3), 0.25);
    assert_eq!(mps.getString(4), "x");
}

#[test]
fn ps10_empty_is_frozen_and_empty() {
    let empty = PropertySet::createEmpty();
    assert!(empty.isFrozen());
    assert!(empty.isEmpty());
    assert_eq!(empty.size(), 0);
    assert!(!empty.contains(PROP_ID));
    assert!(PROPERTYSET_EMPTY.isFrozen());
    assert!(PROPERTYSET_EMPTY.isEmpty());
}

#[test]
fn ps11_value_of_and_int_ctor() {
    let v: PropertyValue = Arc::new(7i32);
    let a = PropertySet::valueOf(5, v.clone());
    assert!(a.isFrozen());
    assert_eq!(a.getInt(5), 7);
    let b = PropertySet::new__int_object(6, Arc::new(8i32));
    assert_eq!(b.getInt(6), 8);
    let ids = [1, 2];
    let vals = [Some(Arc::new(1i32) as PropertyValue), Some(Arc::new(2i32) as PropertyValue)];
    let c = PropertySet::valueOf__intarr_objectarr(&ids, &vals);
    assert_eq!(c.getInt(1), 1);
    assert_eq!(c.getInt(2), 2);
}

#[test]
fn ps12_add_later_wins_same_key() {
    let old = freeze_int(PROP_ID, 1);
    let newp = freeze_int(PROP_ID, 2);
    let mut mps = MutablePropertySet::new();
    mps.add(&old);
    mps.add(&newp);
    assert_eq!(mps.getInt(PROP_ID), 2);
    assert_eq!(mps.size(), 1);
}

#[test]
fn ps13_add_merges_distinct_keys() {
    let a = freeze_int(PROP_ID, 1);
    let b = freeze_int(PROP_ID_B, 2);
    let mut mps = MutablePropertySet::new();
    mps.add(&a);
    mps.add(&b);
    let frozen = mps.freeze();
    assert_eq!(frozen.getInt(PROP_ID), 1);
    assert_eq!(frozen.getInt(PROP_ID_B), 2);
    assert_eq!(frozen.size(), 2);
}

#[test]
fn ps14_add_limit_copies_half_open_range() {
    let mut src = MutablePropertySet::new();
    src.setInt(0, 1);
    src.setInt(7, 2);
    src.setInt(8, 3);
    let src = src.freeze();
    let mut mps = MutablePropertySet::new();
    mps.addLimit(&src, 0, 8);
    let frozen = mps.freeze();
    assert!(frozen.contains(0));
    assert!(frozen.contains(7));
    assert!(!frozen.contains(8));
}

#[test]
fn ps15_new_from_ps_and_mutable() {
    let src = freeze_int(PROP_ID, 4);
    let from_ps = MutablePropertySet::new_from_ps(&src);
    assert_eq!(from_ps.getInt(PROP_ID), 4);
    let from_m = MutablePropertySet::new_from_mutable(&from_ps);
    assert_eq!(from_m.getInt(PROP_ID), 4);
}

#[test]
fn ps16_clone_mps_is_independent() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_ID, 1);
    let mut cloned = mps.clone_mps();
    cloned.setInt(PROP_ID, 9);
    assert_eq!(mps.getInt(PROP_ID), 1);
    assert_eq!(cloned.getInt(PROP_ID), 9);
}

#[test]
fn ps17_compose_modes() {
    let old = freeze_int(PROP_ID, 1);
    let mut np = MutablePropertySet::new();
    np.setInt(PROP_ID, 2);
    np.setInt(PROP_ID_B, 3);
    let newp = np.freeze();
    let nop = PropertySet::compose(&old, &newp, SetPropertyMode::NOP);
    assert_eq!(nop.getInt(PROP_ID), 1);
    assert!(!nop.contains(PROP_ID_B));
    let replace = PropertySet::compose(&old, &newp, SetPropertyMode::REPLACE);
    assert_eq!(replace.getInt(PROP_ID), 2);
    assert_eq!(replace.getInt(PROP_ID_B), 3);
    let merge = PropertySet::compose(&old, &newp, SetPropertyMode::MERGE);
    assert_eq!(merge.getInt(PROP_ID), 2);
    assert_eq!(merge.getInt(PROP_ID_B), 3);
    let clear = PropertySet::compose(&old, &newp, SetPropertyMode::CLEAR);
    assert!(!clear.contains(PROP_ID));
    assert!(clear.isEmpty() || !clear.contains(PROP_ID));
    let fallback = PropertySet::compose(&old, &newp, SetPropertyMode::FB_REPLACE);
    assert_eq!(fallback.getInt(PROP_ID), 1);
}

#[test]
fn ps18_merge_static() {
    let a = freeze_int(1, 10);
    let b = freeze_int(2, 20);
    let m = PropertySet::merge(&a, &b);
    assert!(m.isFrozen());
    assert_eq!(m.getInt(1), 10);
    assert_eq!(m.getInt(2), 20);
}

#[test]
fn ps19_get_all_with_values() {
    let frozen = freeze_int(PROP_ID, 1);
    let n = frozen.size() as usize;
    let mut ids = vec![0; n];
    let mut vals: Vec<Option<PropertyValue>> = vec![None; n];
    let got = frozen.getAll(&mut ids, Some(&mut vals));
    assert_eq!(got, 1);
    assert_eq!(ids[0], PROP_ID);
    assert!(vals[0].is_some());
}

#[test]
fn ps20_contain_empty_and_self() {
    let frozen = freeze_int(PROP_ID, 1);
    assert!(frozen.contain(None));
    assert!(frozen.contain(Some(&frozen)));
    let other = freeze_int(0, 1);
    assert!(!frozen.contain(Some(&other)));
}

#[test]
fn ps21_dispose_drops_map() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_ID, 1);
    let mut frozen = mps.freeze();
    frozen.dispose();
    assert_eq!(frozen.size(), 0);
    assert!(!frozen.contains(PROP_ID));
}

#[test]
fn ps22_difference_set_shared_add() {
    let base = freeze_int(PROP_ID, 1);
    let mut mps = MutablePropertySet::new_from_ps(&base);
    mps.setInt(PROP_ID_B, 2);
    mps.differenceSet(&base);
    assert!(!mps.contains(PROP_ID));
    assert_eq!(mps.getInt(PROP_ID_B), 2);
}

#[test]
fn ps23_intersect_shared_add() {
    let a = freeze_int(PROP_ID, 1);
    let mut b = MutablePropertySet::new_from_ps(&a);
    b.setInt(PROP_ID_B, 2);
    let b = b.freeze();
    let inter = PropertySet::intersect(&a, &b);
    assert_eq!(inter.getInt(PROP_ID), 1);
    assert!(!inter.contains(PROP_ID_B));
}

#[test]
fn ps24_equals_same_primitive_values() {
    let a = freeze_int(PROP_ID, 1);
    let b = freeze_int(PROP_ID, 1);
    assert!(a.equals(Some(&b)));
    let c = freeze_int(PROP_ID, 2);
    assert!(!a.equals(Some(&c)));
    assert!(PropertySet::createEmpty().equals(Some(&PropertySet::createEmpty())));
}

#[test]
fn ps25_hash_code_frozen_stable() {
    let mut a = freeze_int(PROP_ID, 1);
    let h1 = a.hashCode();
    let h2 = a.hashCode();
    assert_eq!(h1, h2);
    let mut b = freeze_int(PROP_ID, 1);
    assert_eq!(a.hashCode(), b.hashCode());
}

#[test]
#[should_panic(expected = "No such property")]
fn ps26_get_int_missing_panics() {
    let frozen = freeze_int(PROP_ID, 1);
    let _ = frozen.getInt(0);
}

#[test]
#[should_panic(expected = "This PropertySet is frozen")]
fn ps27_set_after_freeze_panics() {
    let mut mps = MutablePropertySet::new();
    mps.setInt(PROP_ID, 1);
    let _ = mps.freeze();
    mps.setInt(PROP_ID, 2);
}

#[test]
fn ps28_is_empty_new() {
    let mps = MutablePropertySet::new();
    assert!(mps.isEmpty());
    assert_eq!(mps.size(), 0);
    assert!(!mps.contains(PROP_ID));
}

#[test]
fn ps29_get_object_present() {
    let frozen = freeze_int(PROP_ID, 1);
    let obj = frozen.getObject(PROP_ID);
    assert!(obj.as_ref().as_any().downcast_ref::<i32>().copied() == Some(1));
}

#[test]
fn ps30_is_property_equal_and_object_equal() {
    let a = freeze_int(PROP_ID, 1);
    let b = freeze_int(PROP_ID, 1);
    assert!(PropertySet::isPropertyEqual(Some(&a), Some(&b)));
    assert!(PropertySet::isPropertyEqual(None, None));
    let va: PropertyValue = Arc::new(1i32);
    let vb: PropertyValue = Arc::new(1i32);
    assert!(PropertySet::isObjectEqual(Some(&va), Some(&vb)));
    assert!(PropertySet::isObjectEqual(None, None));
    assert!(!PropertySet::isObjectEqual(Some(&va), None));
}
