use word_core::cn::wps::moffice::writer::layout::base::drawing::ShapeCollision::ClipRect;
use word_core::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;

fn rect(left: i32, right: i32) -> TypoRectCoreImpl {
    let mut rect = TypoRectCoreImpl::default();
    rect.set__int_int_int_int(left, 100, right, 200);
    rect
}

#[test]
fn java_clip_rect_subtract_preserves_order_and_wrap_width_gates() {
    let mut clip = ClipRect::default();
    clip.init(0, 100, 2000, 200);
    // Java ShapeCollision.ClipRect.subtract: a square object in the middle
    // creates two strips with a wrap boundary on their touching edge.
    clip.subtract(&rect(500, 900), ClipRect::SQUARE);
    let mut output = Vec::new();
    assert!(clip.getRects(&mut output));
    assert_eq!(output.iter().map(|r| (r.left, r.right, r.getWrapType())).collect::<Vec<_>>(),
        vec![(0, 500, ClipRect::SQUARE), (900, 2000, ClipRect::SQUARE)]);
    // Java's TABLESQUARE 380 twip minimum drops the 300 twip inner strip.
    clip.subtract(&rect(1350, 1700), ClipRect::TABLESQUARE);
    let mut output = Vec::new();
    assert!(clip.getRects(&mut output));
    assert_eq!(output.iter().map(|r| (r.left, r.right)).collect::<Vec<_>>(),
        vec![(0, 500), (900, 1350)]);
    // A top-and-bottom drawing clears the line rather than subtracting X.
    clip.clear();
    assert!(clip.isEmpty());
    assert!(!clip.getRects(&mut Vec::new()));
}
