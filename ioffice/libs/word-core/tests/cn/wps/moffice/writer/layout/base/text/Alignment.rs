use word_core::cn::wps::moffice::writer::layout::base::text::Alignment::{
    align_x, JC_CENTER, JC_JUSTIFY, JC_LEFT, JC_RIGHT,
};

#[test]
fn a1_left() {
    assert_eq!(align_x(JC_LEFT, 100, 200), 0);
}

#[test]
fn a2_center() {
    assert_eq!(align_x(JC_CENTER, 100, 200), 50);
}

#[test]
fn a3_right() {
    assert_eq!(align_x(JC_RIGHT, 100, 200), 100);
}

#[test]
fn a4_full_width() {
    assert_eq!(align_x(JC_LEFT, 200, 200), 0);
    assert_eq!(align_x(JC_CENTER, 200, 200), 0);
    assert_eq!(align_x(JC_RIGHT, 200, 200), 0);
}

#[test]
fn a5_justify_is_left() {
    assert_eq!(align_x(JC_JUSTIFY, 100, 200), 0);
}
