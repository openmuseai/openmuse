use word_core::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
use word_core::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;

#[test]
fn java_graph_planes_and_cp_map_survive_snapshot_commit() {
    let mut snapshot = TypoSnapshot::default();
    let mut para = TypoPara::default();
    para.alloc(&mut snapshot);
    para.allocGraphs(2);
    para.putChars__chararr_int_int_int(&['A', 'B'], 0, 2, 0);
    para.putWidths(0, 2, &[120, 240]);
    para.allocGraphs(1);
    para.putChar(2, 'C');
    para.putWidth(2, 360);

    let mut map = [100, 103, 0, 3];
    para.commit(&mut map, 1, None, 0);
    let pointer = para.getPointer();

    assert_eq!(map, [0, 3, 0, 3]);
    assert_eq!(TypoPara::getStartCP__int_int_typosnapshot(0, pointer, &snapshot), 100);
    assert_eq!(TypoPara::getStartCP__int_int_typosnapshot(2, pointer, &snapshot), 102);
    assert_eq!(TypoPara::getEndCP__int_int_typosnapshot(2, pointer, &snapshot), 103);
    assert_eq!(TypoPara::getWidth__int_int_typosnapshot(0, pointer, &snapshot), 120);
    assert_eq!(TypoPara::getWidth__int_int_typosnapshot(1, pointer, &snapshot), 240);
    assert_eq!(TypoPara::getWidth__int_int_typosnapshot(2, pointer, &snapshot), 360);
}
