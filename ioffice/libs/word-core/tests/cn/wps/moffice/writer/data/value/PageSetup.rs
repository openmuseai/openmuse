// function_id=01_function_page_setup
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;

fn default_page() -> PageSetup {
    PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440)
}

/// PS1 PAGE-DEFAULT
#[test]
fn PS1_default() {
    let p = default_page();
    assert_eq!(p.getWidth(), 11906);
    assert_eq!(p.getHeight(), 16838);
    assert_eq!(p.getMarginLeft(), 1800);
    assert_eq!(p.getMarginRight(), 1800);
    assert_eq!(p.getMarginTop(), 1440);
    assert_eq!(p.getMarginBottom(), 1440);
}

/// PS2 PAGE-CONTENT
#[test]
fn PS2_content() {
    let p = default_page();
    assert_eq!(p.contentWidth(), 8306);
    assert_eq!(p.contentHeight(), 13958);
}

/// PS3 PAGE-CUSTOM
#[test]
fn PS3_custom() {
    let p = PageSetup::PageSetup(10000, 20000, 1000, 1000, 2000, 2000);
    assert_eq!(p.contentWidth(), 8000);
    assert_eq!(p.contentHeight(), 16000);
}

/// PS4 PAGE-ZERO
#[test]
fn PS4_zero() {
    let p = PageSetup::PageSetup(11906, 16838, 0, 0, 0, 0);
    assert_eq!(p.contentWidth(), 11906);
    assert_eq!(p.contentHeight(), 16838);
}

/// PS5 PAGE-ROUND odd twips stay integer
#[test]
fn PS5_round() {
    let p = PageSetup::PageSetup(10001, 20001, 1, 2, 3, 4);
    assert_eq!(p.contentWidth(), 9998);
    assert_eq!(p.contentHeight(), 19994);
}
