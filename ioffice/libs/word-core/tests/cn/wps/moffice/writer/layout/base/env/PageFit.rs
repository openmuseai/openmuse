use word_core::cn::wps::moffice::writer::layout::base::env::PageFit::PageFit;

#[test]
fn PF1_fits_exactly() { assert!(!PageFit::shouldBreakBefore(60, 40, 100, true)); }
#[test]
fn PF2_overflow_breaks() { assert!(PageFit::shouldBreakBefore(61, 40, 100, true)); }
#[test]
fn PF3_first_oversized_line_progresses() { assert!(!PageFit::shouldBreakBefore(0, 120, 100, false)); }
#[test]
fn PF4_negative_dimensions_are_clamped() { assert!(!PageFit::shouldBreakBefore(-1, -2, -3, true)); }
#[test]
fn PF5_saturating_add_cannot_wrap() { assert!(PageFit::shouldBreakBefore(i32::MAX, 1, i32::MAX, true)); }
