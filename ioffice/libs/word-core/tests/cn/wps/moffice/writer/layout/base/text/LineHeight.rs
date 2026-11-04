use word_core::cn::wps::moffice::writer::layout::base::text::LineHeight::{
    first_line_top, last_line_after, line_height, ParaLineMetrics,
};

#[test]
fn h1_12pt_single() {
    let p = ParaLineMetrics::default_para();
    assert_eq!(line_height(&p), 240);
}

#[test]
fn h2_half_pt_ceil() {
    let p = ParaLineMetrics {
        font_size_pt: 10.5,
        line_spacing: 1.0,
        ..Default::default()
    };
    assert_eq!(line_height(&p), 210);
}

#[test]
fn h3_space_before() {
    let p = ParaLineMetrics {
        space_before_twip: 240,
        ..ParaLineMetrics::default_para()
    };
    assert_eq!(first_line_top(&p), 240);
}

#[test]
fn h4_space_after() {
    let p = ParaLineMetrics {
        space_after_twip: 240,
        ..ParaLineMetrics::default_para()
    };
    assert_eq!(last_line_after(&p), 240);
}

#[test]
fn h5_default_no_ppr() {
    assert_eq!(line_height(&ParaLineMetrics::default_para()), 240);
}
