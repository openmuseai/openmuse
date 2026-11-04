use word_core::cn::wps::moffice::writer::layout::FlowLayout::FlowLayout;

/// LC1
#[test]
fn lc1_invalidate_not_paintable() {
    let mut fl = FlowLayout::new();
    fl.layout();
    assert!(fl.can_paint());
    let v = fl.version();
    fl.invalidate();
    assert!(!fl.can_paint());
    assert_eq!(fl.version(), v);
}
