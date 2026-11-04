// function_id=05_function_line_layouter
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::env::LineLayout::{
    layout_lines, LayoutRun, ParaPapx,
};

fn run(text: &str, adv: &[i32]) -> LayoutRun {
    LayoutRun {
        text: text.to_string(),
        advances: adv.to_vec(),
        ascent: 180,
        descent: 60,
    }
}

fn papx() -> ParaPapx {
    ParaPapx::default()
}

/// LL1
#[test]
fn LL1_single_line_first_and_last() {
    let lines = layout_lines(&[run("Hi", &[100, 100])], 500, &papx());
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].getStartCP(), 0);
    assert_eq!(lines[0].getEndCP(), 2);
    assert!(lines[0].isFirstLineOfPara());
    assert!(lines[0].isLastLineOfPara());
}

/// LL2
#[test]
fn LL2_multi_line_cps_abut() {
    let lines = layout_lines(&[run("你好", &[400, 400])], 500, &papx());
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].getStartCP(), 0);
    assert_eq!(lines[0].getEndCP(), 1);
    assert_eq!(lines[1].getStartCP(), 1);
    assert_eq!(lines[1].getEndCP(), 2);
    assert_eq!(lines[0].getEndCP(), lines[1].getStartCP());
}

/// LL3
#[test]
fn LL3_only_first_first_only_last_last() {
    let lines = layout_lines(&[run("你好", &[400, 400])], 500, &papx());
    assert!(lines[0].isFirstLineOfPara());
    assert!(!lines[0].isLastLineOfPara());
    assert!(!lines[1].isFirstLineOfPara());
    assert!(lines[1].isLastLineOfPara());
}

/// LL4
#[test]
fn LL4_first_line_indent_shrinks_first_line() {
    let mut p = papx();
    p.first_line_indent = 200;
    let lines = layout_lines(&[run("abcd", &[100, 100, 100, 100])], 300, &p);
    assert!(lines.len() >= 2);
    assert_eq!(lines[0].getEndCP() - lines[0].getStartCP(), 1);
}

/// LL5
#[test]
fn LL5_left_center_right_x() {
    let mut p = papx();
    p.jc = 0;
    let left = layout_lines(&[run("ab", &[100, 100])], 500, &p);
    assert_eq!(left[0].getLeft(), 0);

    p.jc = 1;
    let center = layout_lines(&[run("ab", &[100, 100])], 500, &p);
    assert_eq!(center.len(), 1);
    assert_eq!(center[0].getLeft(), 150);

    p.jc = 2;
    let right = layout_lines(&[run("ab", &[100, 100])], 500, &p);
    assert_eq!(right[0].getLeft(), 300);
}

/// LL6
#[test]
fn LL6_para_before_after_only_on_first_last() {
    let mut p = papx();
    p.para_before = 200;
    p.para_after = 120;
    let lines = layout_lines(&[run("你好", &[400, 400])], 500, &p);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].getTextRectHeight(), 440);
    assert_eq!(lines[1].getTextRectHeight(), 360);
}

/// LL7
#[test]
fn LL7_cr_one_last_line() {
    let lines = layout_lines(&[run("a\r", &[100, 0])], 500, &papx());
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].getEndCP(), 2);
    assert!(lines[0].isLastLineOfPara());
}

/// LL8
#[test]
fn LL8_larger_advance_more_lines() {
    let small = layout_lines(&[run("abcd", &[50, 50, 50, 50])], 150, &papx());
    let large = layout_lines(&[run("abcd", &[200, 200, 200, 200])], 150, &papx());
    assert!(large.len() >= small.len());
}

/// LL9
#[test]
fn LL9_justify_x_matches_left() {
    let mut left = papx();
    left.jc = 0;
    let mut just = papx();
    just.jc = 3;
    let a = layout_lines(&[run("ab", &[100, 100])], 500, &left);
    let b = layout_lines(&[run("ab", &[100, 100])], 500, &just);
    assert_eq!(a[0].getLeft(), b[0].getLeft());
    assert_eq!(a[0].getLeft(), 0);
}

/// LL10
#[test]
fn LL10_no_page_height_truncation() {
    let lines = layout_lines(&[run("你好世界", &[400, 400, 400, 400])], 500, &papx());
    assert_eq!(lines.len(), 4);
}

/// LL11
#[test]
fn LL11_src_does_not_call_wr04_wrap() {
    let src = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/cn/wps/moffice/writer/layout/base/env/LineLayout.rs"
    ));
    assert!(
        !src.contains("text::Wrap") && !src.contains("layout::base::text::Wrap"),
        "layout_lines must not call wr-04 Wrap.rs"
    );
}

/// LL12
#[test]
fn LL12_product_type_is_typo_text_line() {
    let lines = layout_lines(&[run("Hi", &[100, 100])], 500, &papx());
    let _ = lines[0].getStartCP();
    let _ = lines[0].getEndCP();
}
