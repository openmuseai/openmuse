use word_core::cn::wps::moffice::writer::layout::base::line::LineLayouter::cache_run_widths;

/// RW1
#[test]
fn rw1_empty() {
    let w = cache_run_widths(&[], |_| 10);
    assert!(w.is_empty());
}

/// RW2
#[test]
fn rw2_n_runs() {
    let w = cache_run_widths(&["a", "bb"], |_| 10);
    assert_eq!(w, vec![10, 10]);
}
