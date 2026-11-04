use ss_core::java_runtime::RefCell;
use ss_core::java_runtime::Rc;

use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::grid::KmoMergeDelegate::KmoMergeDelegate;
use ss_core::cn::wps::moss::util::KmoRect::KmoRect;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

fn rect(r1: i32, c1: i32, r2: i32, c2: i32) -> KmoRect {
    KmoRect::KmoRect__bounds(r1, c1, r2, c2)
}

fn bounds(range: &KmoRect) -> (i32, i32, i32, i32) {
    (range.tl.row, range.tl.col, range.br.row, range.br.col)
}

fn delegate() -> KmoMergeDelegate {
    KmoMergeDelegate::KmoMergeDelegate(
        Rc::new(RefCell::new(KmoRect::KmoRect())),
        SpreadsheetVersion::EXCEL2007,
    )
}

fn listed(dele: &KmoMergeDelegate) -> Vec<(i32, i32, i32, i32)> {
    let mut out = Vec::new();
    dele.getIntersectMergedRanges(&rect(0, 0, 100, 100), &mut out);
    out.iter().map(bounds).collect()
}

#[test]
fn mrg1_intersect_keeps_insertion_order_and_skips_gaps() {
    let dele = delegate();
    dele.merge(&rect(0, 0, 1, 1));
    dele.merge(&rect(4, 4, 5, 5));
    dele.merge(&rect(1, 2, 2, 3));
    let mut hit = Vec::new();
    dele.getIntersectMergedRanges(&rect(1, 1, 2, 2), &mut hit);
    assert_eq!(
        hit.iter().map(bounds).collect::<Vec<_>>(),
        vec![(0, 0, 1, 1), (1, 2, 2, 3)]
    );
    assert!(dele.getMergedCells(4, 4).is_some());
    assert!(dele.getMergedCells(3, 3).is_none());
}

#[test]
fn mrg2_contains_is_reverse_and_unmerge_splits_the_two_modes() {
    let dele = delegate();
    dele.merge(&rect(0, 0, 3, 3));
    dele.merge(&rect(1, 1, 1, 1));
    let mut contained = Vec::new();
    dele.getContainsMergedRanges(&rect(1, 1, 2, 2), &mut contained);
    assert_eq!(contained.iter().map(bounds).collect::<Vec<_>>(), vec![(1, 1, 1, 1)]);
    assert!(!dele.isContainAllIntersectMergedRange(&rect(1, 1, 2, 2)));
    dele.unmerge__flag(&rect(1, 1, 2, 2), true);
    assert_eq!(listed(&dele), vec![(0, 0, 3, 3)]);
    dele.unmerge(&rect(0, 0, 0, 0));
    assert!(listed(&dele).is_empty());
}

#[test]
fn mrg3_expand_and_copy_follow_the_stored_ranges() {
    let dele = delegate();
    dele.merge(&rect(0, 0, 4, 1));
    let grown = dele.expandBounds(&rect(2, 0, 2, 0));
    assert_eq!(bounds(&grown), (0, 0, 4, 1));
    let mut other = delegate();
    other.copy(&dele);
    assert_eq!(listed(&other), vec![(0, 0, 4, 1)]);
}

#[test]
fn mrg4_sheet_facade_reports_direction_and_point() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    sheet.merge(&rect(2, 2, 2, 5));
    sheet.merge(&rect(6, 1, 8, 1));
    assert!(sheet.isContainsMergedCells(&rect(2, 4, 2, 4)));
    assert!(sheet.isContainsHorMergedCells(&rect(0, 0, 9, 9)));
    assert!(sheet.isContainsVecMergedCells(&rect(0, 0, 9, 9)));
    assert!(!sheet.isContainsHorMergedCells(&rect(6, 1, 8, 1)));
    assert_eq!(bounds(&sheet.getMergedRange(7, 1).unwrap()), (6, 1, 8, 1));
    let expanded = sheet.expandMergeBounds(&rect(2, 3, 2, 3));
    assert_eq!(bounds(&expanded), (2, 2, 2, 5));
    sheet.unmerge(&rect(2, 2, 2, 5));
    assert!(sheet.getMergedRange(2, 3).is_none());
    assert_eq!(bounds(&sheet.getMergedRange(6, 1).unwrap()), (6, 1, 8, 1));
}
