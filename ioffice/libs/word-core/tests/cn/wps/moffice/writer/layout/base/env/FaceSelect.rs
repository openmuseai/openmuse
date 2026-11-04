// function_id=03_function_face_select
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::env::FaceSelect::{
    is_east_asia, FaceSelect,
};
use word_core::cn::wps::moffice::writer::layout::base::env::TypoFontStyle::TypoFontStyle;

fn style() -> TypoFontStyle {
    let mut s = TypoFontStyle::TypoFontStyle();
    s.ascii = "Times New Roman".into();
    s.eastAsia = "宋体".into();
    s.fontName = "Times New Roman".into();
    s
}

/// FS1
#[test]
fn FS1_latin_uses_ascii() {
    assert_eq!(FaceSelect::selectFace(&style(), 'A'), "Times New Roman");
}

/// FS2
#[test]
fn FS2_han_uses_east_asia() {
    assert_eq!(FaceSelect::selectFace(&style(), '中'), "宋体");
}

/// FS3
#[test]
fn FS3_hiragana() {
    assert_eq!(FaceSelect::selectFace(&style(), 'あ'), "宋体");
}

/// FS4
#[test]
fn FS4_hangul() {
    assert_eq!(FaceSelect::selectFace(&style(), '한'), "宋体");
}

/// FS5
#[test]
fn FS5_space_ascii() {
    assert_eq!(FaceSelect::selectFace(&style(), ' '), "Times New Roman");
}

/// FS6
#[test]
fn FS6_digit_ascii() {
    assert_eq!(FaceSelect::selectFace(&style(), '3'), "Times New Roman");
}

/// FS7
#[test]
fn FS7_cjk_punct() {
    assert_eq!(FaceSelect::selectFace(&style(), '。'), "宋体");
}

/// FS8
#[test]
fn FS8_empty_east_asia_falls_back_font_name() {
    let mut s = style();
    s.eastAsia.clear();
    assert_eq!(FaceSelect::selectFace(&s, '中'), "Times New Roman");
}

/// FS9
#[test]
fn FS9_mixed_per_char() {
    let s = style();
    let names: Vec<&str> = "A中B"
        .chars()
        .map(|c| FaceSelect::selectFace(&s, c))
        .collect();
    assert_eq!(names, ["Times New Roman", "宋体", "Times New Roman"]);
}

/// FS10
#[test]
fn FS10_katakana_and_helper() {
    assert!(is_east_asia('ア'));
    assert!(!is_east_asia('A'));
    assert_eq!(FaceSelect::selectFace(&style(), 'ア'), "宋体");
}
