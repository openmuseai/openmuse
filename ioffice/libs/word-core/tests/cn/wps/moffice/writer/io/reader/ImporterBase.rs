// function_id=01_function_importer_mode
// cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_importer_mode
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/ImporterBase.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/ImporterBase.rs

#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::io::reader::ImporterBase::ImporterBase;
use word_core::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::Xml03Context;
use word_core::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;

fn empty_xml07() -> Xml07Context {
    Xml07Context::default()
}

fn empty_xml03() -> Xml03Context {
    Xml03Context::default()
}

#[test]
fn IM1_docx_null_contexts() {
    let importer = ImporterBase::ImporterBase(0, None, None);
    assert!(importer.isDocx());
    assert!(!importer.isXml07());
    assert!(!importer.isXml03());
    assert!(!importer.isGVml());
    assert_eq!(importer.mode(), 0);
}

#[test]
fn IM2_docx_ignores_present_context() {
    let importer = ImporterBase::ImporterBase(0, Some(empty_xml07()), Some(empty_xml03()));
    assert!(importer.isDocx());
    assert!(!importer.isXml07());
    assert!(!importer.isXml03());
}

#[test]
fn IM3_xml07_requires_mode_and_context() {
    let importer = ImporterBase::ImporterBase(1, Some(empty_xml07()), None);
    assert!(importer.isXml07());
    assert!(!importer.isDocx());
    assert!(importer.xml07Context().is_some());
}

#[test]
fn IM4_xml07_mode_without_context_is_false() {
    let importer = ImporterBase::ImporterBase(1, None, None);
    assert!(!importer.isXml07());
}

#[test]
fn IM5_xml03_requires_mode_and_context() {
    let importer = ImporterBase::ImporterBase(3, None, Some(empty_xml03()));
    assert!(importer.isXml03());
}

#[test]
fn IM6_xml03_mode_without_context_is_false() {
    let importer = ImporterBase::ImporterBase(3, None, None);
    assert!(!importer.isXml03());
}

#[test]
fn IM7_gvml_mode_two() {
    let importer = ImporterBase::ImporterBase(2, None, None);
    assert!(importer.isGVml());
    assert!(!importer.isDocx());
}

#[test]
fn IM8_mode_roundtrip() {
    assert_eq!(ImporterBase::ImporterBase(0, None, None).mode(), 0);
    assert_eq!(ImporterBase::ImporterBase(2, None, None).mode(), 2);
}

#[test]
fn IM9_context_accessors() {
    let docx = ImporterBase::ImporterBase(0, None, None);
    assert!(docx.xml07Context().is_none());
    assert!(docx.xml03Context().is_none());

    let xml07 = ImporterBase::ImporterBase(1, Some(empty_xml07()), None);
    assert!(xml07.xml07Context().is_some());
    assert!(xml07.xml03Context().is_none());
}

#[test]
fn IM10_xml07_does_not_set_xml03() {
    let importer = ImporterBase::ImporterBase(1, Some(empty_xml07()), None);
    assert!(!importer.isXml03());
}
