use word_core::cn::wps::moffice::writer::layout::base::RelayoutCache::{
    relayout_range, shifted_cp_after_insert,
};

#[test]
fn r1_full_doc() {
    let r = relayout_range(10, 1, 2, 99);
    assert_eq!(r.start, 0);
    assert_eq!(r.end, 99);
}

#[test]
fn r2_empty_edit() {
    let r = relayout_range(0, 0, 0, 99);
    assert_eq!(r.end, 0);
}

#[test]
fn r3_insert_shift_smoke() {
    assert_eq!(shifted_cp_after_insert(10, 5), 15);
    let r = relayout_range(10, 0, 5, 20);
    assert_eq!(r.start, 0);
    assert_eq!(r.end, 20);
}
