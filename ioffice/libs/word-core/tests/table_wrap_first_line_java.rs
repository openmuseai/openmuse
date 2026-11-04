use word_core::cn::wps::moffice::writer::cache::TypoList::TypoList;
use word_core::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use word_core::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell;
use word_core::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::table::TableLayouter::{ParentLayouterTask, TableLayouter};

fn cell_with_first_line(snapshot: &mut TypoSnapshot, line: i32) -> i32 {
    let mut cell = TypoTableCell::new();
    let cell_pointer = cell.alloc(snapshot);
    let lines = TypoList::add__int_int_typosnapshot(line, 0, snapshot);
    snapshot.set__int_int(cell_pointer + 17, lines);
    cell_pointer
}

#[test]
fn overflow_relayout_uses_tallest_nested_first_line_instead_of_parent_row_height() {
    let mut snapshot = TypoSnapshot::new();
    let text = snapshot.alloc(18);
    snapshot.set__int_int(text, 3);
    snapshot.set__int_int(text + 4, 0);
    snapshot.set__int_int(text + 6, 250);

    let mut inner = TypoTableRow::new();
    inner.alloc(1, &mut snapshot);
    snapshot.set__int_int(inner.getPointer() + 4, 0);
    snapshot.set__int_int(inner.getPointer() + 6, 400);
    inner.setCell(0, cell_with_first_line(&mut snapshot, text));

    let short_text = snapshot.alloc(18);
    snapshot.set__int_int(short_text, 3);
    snapshot.set__int_int(short_text + 4, 0);
    snapshot.set__int_int(short_text + 6, 100);

    let mut outer = TypoTableRow::new();
    outer.alloc(2, &mut snapshot);
    snapshot.set__int_int(outer.getPointer() + 4, 0);
    snapshot.set__int_int(outer.getPointer() + 6, 800);
    outer.setCell(0, cell_with_first_line(&mut snapshot, short_text));
    outer.setCell(1, cell_with_first_line(&mut snapshot, inner.getPointer()));

    let layouter = TableLayouter::TableLayouter(
        ParentLayouterTask::default(), DocumentEnv::default(), ViewEnv::default(), MainState::default(),
    );
    assert_eq!(layouter.getEmbedMinFirstLine(0, &snapshot), 0);
    assert_eq!(layouter.getEmbedMinFirstLine(outer.getPointer(), &snapshot), 250);
}
