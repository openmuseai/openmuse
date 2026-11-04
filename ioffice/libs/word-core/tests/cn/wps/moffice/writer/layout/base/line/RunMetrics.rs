use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::layout::base::line::RunMetrics::{
    measure_run, RunMeasure,
};
use word_core::cn::wps::moffice::writer::layout::text::FontStack::{FontErrorKind, FontStack};

fn fixture_stack() -> FontStack {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/01_function_font_stack/TestFont.ttf");
    FontStack::from_font_file(path).expect("fixture")
}

/// RM1 / RM-EMPTY
#[test]
fn rm1_empty_width_zero() {
    let stack = fixture_stack();
    let m = measure_run(&stack, "", 10.5).unwrap();
    assert_eq!(m.width_twip, 0);
    assert!(m.ascent_twip != 0 || m.descent_twip != 0);
}

/// RM2 / RM-ASCII
#[test]
fn rm2_hello_positive_width() {
    let stack = fixture_stack();
    let m = measure_run(&stack, "Hello", 10.5).unwrap();
    assert!(m.width_twip > 0);
}

/// RM3 / RM-MONO
#[test]
fn rm3_prefix_width_monotonic() {
    let stack = fixture_stack();
    let he = measure_run(&stack, "He", 10.5).unwrap();
    let hello = measure_run(&stack, "Hello", 10.5).unwrap();
    assert!(he.width_twip <= hello.width_twip);
}

/// RM4 / RM-CTRL — control chars do not add width
#[test]
fn rm4_control_chars_skipped() {
    let stack = fixture_stack();
    let a = measure_run(&stack, "A", 12.0).unwrap();
    let with_nl = measure_run(&stack, "A\n\r\t", 12.0).unwrap();
    assert_eq!(a.width_twip, with_nl.width_twip);
}

/// RM5 / RM-SCALE — 24pt ≈ 2× 12pt (linear, no hinting)
#[test]
fn rm5_pt_scales_linearly() {
    let stack = fixture_stack();
    let w12 = measure_run(&stack, "Hello", 12.0).unwrap().width_twip;
    let w24 = measure_run(&stack, "Hello", 24.0).unwrap().width_twip;
    assert_eq!(w24, w12 * 2);
}

/// RM6 / RM-MISS — missing cmap must not silently use width=size
#[test]
fn rm6_missing_glyph_is_error_not_size() {
    let stack = fixture_stack();
    match measure_run(&stack, "我", 12.0) {
        Err(e) => assert_eq!(e.kind, FontErrorKind::MissingGlyph),
        Ok(RunMeasure { width_twip, .. }) => {
            panic!("must not fake width={width_twip} from size")
        }
    }
}
