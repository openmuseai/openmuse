use word_core::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;

#[test]
fn extend_alloc_keeps_a_tail_allocation_in_place() {
    let mut snapshot = TypoSnapshot::new();
    let pointer = snapshot.alloc(3);
    for (offset, value) in [11, 22, 33].into_iter().enumerate() {
        snapshot.set__int_int(pointer + offset as i32, value);
    }
    assert_eq!(snapshot.extendAlloc(pointer, 3, 2), pointer);
    assert_eq!((0..5).map(|i| snapshot.get__int(pointer + i)).collect::<Vec<_>>(), [11, 22, 33, 0, 0]);
}

#[test]
fn extend_alloc_moves_an_earlier_allocation_and_preserves_the_source() {
    let mut snapshot = TypoSnapshot::new();
    let source = snapshot.alloc(3);
    for (offset, value) in [4, 5, 6].into_iter().enumerate() {
        snapshot.set__int_int(source + offset as i32, value);
    }
    let guard = snapshot.alloc(2);
    snapshot.set__int_int(guard, 90);
    snapshot.set__int_int(guard + 1, 91);

    let moved = snapshot.extendAlloc(source, 3, 2);
    assert_eq!(moved, guard + 2);
    assert_eq!((0..5).map(|i| snapshot.get__int(moved + i)).collect::<Vec<_>>(), [4, 5, 6, 0, 0]);
    assert_eq!((0..3).map(|i| snapshot.get__int(source + i)).collect::<Vec<_>>(), [4, 5, 6]);
    assert_eq!([snapshot.get__int(guard), snapshot.get__int(guard + 1)], [90, 91]);
}

#[test]
fn copy_preserves_java_arraycopy_overlap_direction() {
    let mut snapshot = TypoSnapshot::new();
    let pointer = snapshot.alloc(6);
    for (offset, value) in [1, 2, 3, 4, 5, 6].into_iter().enumerate() {
        snapshot.set__int_int(pointer + offset as i32, value);
    }
    snapshot.copy__int_int_int(pointer, pointer + 2, 4);
    assert_eq!((0..6).map(|i| snapshot.get__int(pointer + i)).collect::<Vec<_>>(), [1, 2, 1, 2, 3, 4]);
    snapshot.copy__int_int_int(pointer + 2, pointer, 4);
    assert_eq!((0..6).map(|i| snapshot.get__int(pointer + i)).collect::<Vec<_>>(), [1, 2, 3, 4, 3, 4]);
}

#[test]
fn copy_with_allocation_keeps_an_independent_source() {
    let mut snapshot = TypoSnapshot::new();
    let source = snapshot.alloc(2);
    snapshot.set__int_int(source, 17);
    snapshot.set__int_int(source + 1, 23);
    let copied = snapshot.copy__int_int(source, 2);
    assert_ne!(copied, source);
    assert_eq!([snapshot.get__int(copied), snapshot.get__int(copied + 1)], [17, 23]);
    snapshot.set__int_int(source, 99);
    assert_eq!(snapshot.get__int(copied), 17);
}
