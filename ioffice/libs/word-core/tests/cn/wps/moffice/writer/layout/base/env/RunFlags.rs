// function_id=02_function_run_flags
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::empty;
use word_core::cn::wps::moffice::writer::layout::base::env::RunFlags::RunFlags;

fn ps_bool(id: i32, v: bool) -> util::moffice::property::PropertySet::PropertySet {
    let mut m = MutablePropertySet::new();
    m.setBoolean(id, v);
    m.freeze()
}

fn ps_bi() -> util::moffice::property::PropertySet::PropertySet {
    let mut m = MutablePropertySet::new();
    m.setBoolean(5, true);
    m.setBoolean(6, true);
    m.freeze()
}

#[test]
fn RF1_defaults_false() {
    assert!(!RunFlags::getBold(&*empty, 0));
    assert!(!RunFlags::getItalic(&*empty, 0));
}

#[test]
fn RF2_bold() {
    assert!(RunFlags::getBold(&ps_bool(5, true), 0));
    assert!(!RunFlags::getItalic(&ps_bool(5, true), 0));
}

#[test]
fn RF3_italic() {
    assert!(!RunFlags::getBold(&ps_bool(6, true), 0));
    assert!(RunFlags::getItalic(&ps_bool(6, true), 0));
}

#[test]
fn RF4_bold_and_italic() {
    let ps = ps_bi();
    assert!(RunFlags::getBold(&ps, 0));
    assert!(RunFlags::getItalic(&ps, 0));
}

#[test]
fn RF5_complex_bold_prop_38() {
    assert!(RunFlags::getBold(&ps_bool(38, true), 5));
    assert!(!RunFlags::getItalic(&ps_bool(38, true), 5));
}

#[test]
fn RF6_complex_ignores_latin_italic() {
    assert!(!RunFlags::getBold(&ps_bool(6, true), 5));
    assert!(!RunFlags::getItalic(&ps_bool(6, true), 5));
}
