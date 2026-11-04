use word_core::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use word_core::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers;
use word_core::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;

#[test]
fn wrap_table_lookup_matches_cp_and_optional_parent_page() {
    let mut snapshot = TypoSnapshot::new();
    let first_page = snapshot.alloc(19);
    let second_page = snapshot.alloc(19);
    let mut layers = TypoDrawingLayers::newInstance();
    layers.alloc(&mut snapshot);

    let mut first = TypoDrawing::new();
    first.alloc(&mut snapshot);
    first.setParent(first_page);
    let first_cp = snapshot.addCPRef(None, 6341);
    snapshot.set__int_int(first.getPointer() + 11, first_cp);
    layers.addToList(6, first.getPointer());

    let mut second = TypoDrawing::new();
    second.alloc(&mut snapshot);
    second.setParent(second_page);
    let second_cp = snapshot.addCPRef(None, 6341);
    snapshot.set__int_int(second.getPointer() + 11, second_cp);
    layers.addToList(6, second.getPointer());

    assert_eq!(layers.getWrapTableByCP(6341, first_page), first.getPointer());
    assert_eq!(layers.getWrapTableByCP(6341, second_page), second.getPointer());
    assert_eq!(layers.getWrapTableByCP(6341, 0), first.getPointer());
    assert_eq!(layers.getWrapTableByCP(6353, 0), 0);
}
