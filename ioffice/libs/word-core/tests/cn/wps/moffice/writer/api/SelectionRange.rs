// function_id=01_function_sel_clamp
use word_core::cn::wps::moffice::writer::api::SelectionRange::clamp;

#[test]
fn cl1_ordered_range() {
    assert_eq!(clamp(2, 5, 10), Some((2, 5)));
}

#[test]
fn cl2_swaps_inverted() {
    assert_eq!(clamp(5, 2, 10), Some((2, 5)));
}

#[test]
fn cl3_clamps_negative_start() {
    assert_eq!(clamp(-1, 3, 10), Some((0, 3)));
}

#[test]
fn cl4_clamps_end_past_len() {
    assert_eq!(clamp(8, 20, 10), Some((8, 10)));
}

#[test]
fn cl5_empty_is_none() {
    assert_eq!(clamp(3, 3, 10), None);
}

#[test]
fn cl6_zero_len_is_none() {
    assert_eq!(clamp(0, 1, 0), None);
}
