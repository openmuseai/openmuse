use word_core::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use word_core::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;

#[test]
fn line_commit_records_java_snapshot_mem_end() {
    let mut snapshot = TypoSnapshot::new();
    let mut line = TypoTextLine::new();
    let pointer = line.alloc(&mut snapshot);
    let mem_end = snapshot.getAllocSize() as i32;

    line.setMemEnd(mem_end);

    assert_eq!(line.getMemEnd(), mem_end);
    assert_eq!(TypoTextLine::getMemEnd__int_typosnapshot(pointer, &snapshot), mem_end);
}
