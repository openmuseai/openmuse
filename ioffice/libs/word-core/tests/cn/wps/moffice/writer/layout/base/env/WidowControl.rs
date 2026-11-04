use word_core::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use word_core::cn::wps::moffice::writer::layout::base::env::WidowControl::{PageLine, WidowControl};

fn l(cp: i32, para: i32, widow: bool) -> PageLine {
    let mut line = TypoTextLine::new(); line.setStartCP(cp); line.setEndCP(cp + 1);
    PageLine { line, para_id: para, widow_control: widow, force_page_after: false, cell: None }
}
#[test]
fn WC1_widow_pulls_one_predecessor() {
    let mut pages = vec![vec![l(0, 1, true), l(1, 1, true), l(2, 1, true)], vec![l(3, 1, true)]];
    WidowControl::rebalance(&mut pages);
    assert_eq!(pages[0].len(), 2); assert_eq!(pages[1].len(), 2); assert_eq!(pages[1][0].line.getStartCP(), 2);
}
#[test]
fn WC2_orphan_moves_to_next_page() {
    let mut pages = vec![vec![l(0, 9, true)], vec![l(1, 9, true), l(2, 9, true)]];
    WidowControl::rebalance(&mut pages);
    assert_eq!(pages.len(), 1); assert_eq!(pages[0].len(), 3);
}
#[test]
fn WC3_disabled_does_not_move() {
    let mut pages = vec![vec![l(0, 1, false), l(1, 1, false)], vec![l(2, 1, false)]];
    WidowControl::rebalance(&mut pages); assert_eq!(pages[0].len(), 2);
}
#[test]
fn WC4_different_paragraphs_do_not_move() {
    let mut pages = vec![vec![l(0, 1, true)], vec![l(1, 2, true)]];
    WidowControl::rebalance(&mut pages); assert_eq!(pages.len(), 2);
}
#[test]
fn WC5_empty_pages_are_removed() {
    let mut pages = vec![vec![], vec![l(0, 1, true)]];
    WidowControl::rebalance(&mut pages); assert_eq!(pages.len(), 1);
}
