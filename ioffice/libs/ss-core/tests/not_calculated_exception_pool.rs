use ss_core::java_runtime::Rc;
use ss_core::cn::wps::et::ss::formula::evaluator::NotCalculatedException;

#[test]
fn java_not_calculated_signal_pool_preserves_identity_coordinates_and_eleven_entry_boundary() {
    let originals: Vec<_> = (0..12).map(|index| NotCalculatedException::obtain(index, index + 1, index + 2)).collect();
    for signal in &originals { signal.recycle(); }
    // java recycleCount <= 10 accepts the eleventh entry; the twelfth is skipped.
    let reused: Vec<_> = (0..12).map(|index| NotCalculatedException::obtain(100 + index, 200 + index, 300 + index)).collect();
    for index in 0..11 {
        assert!(Rc::ptr_eq(&reused[index], &originals[10 - index]));
        assert_eq!((reused[index].getSheetUid(), reused[index].getRow(), reused[index].getCol()), (100 + index as i32, 200 + index as i32, 300 + index as i32));
    }
    assert!(!originals.iter().any(|original| Rc::ptr_eq(original, &reused[11])));
    let transferred = reused[11].clone();
    std::thread::spawn(move || transferred.recycle()).join().unwrap();
    let obtained = NotCalculatedException::obtain(7, 8, 9);
    assert!(Rc::ptr_eq(&obtained, &reused[11]));
    assert_eq!((obtained.getSheetUid(), obtained.getRow(), obtained.getCol()), (7, 8, 9));
}
