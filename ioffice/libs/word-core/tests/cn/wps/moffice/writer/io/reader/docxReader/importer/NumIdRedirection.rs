// function_id=07_function_numid_redirection
// cargo test --manifest-path libs/word-core/Cargo.toml --test 07_function_numid_redirection
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/NumIdRedirection.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/NumIdRedirection.rs

use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::{
    NumIdRedirection, UUNumberingId,
};

#[test]
fn NR1_first_next_id_is_one() {
    let mut maker = UUNumberingId::new();
    assert_eq!(maker.getNextId(), 1);
}

#[test]
fn NR2_put_records_first_kernel_id() {
    let mut r = NumIdRedirection::NumIdRedirection(UUNumberingId::new());
    assert_eq!(r.put(10), 1);
    assert_eq!(r.getId(10), Some(1));
}

#[test]
fn NR3_second_key_gets_two() {
    let mut r = NumIdRedirection::NumIdRedirection(UUNumberingId::new());
    r.put(10);
    assert_eq!(r.put(20), 2);
    assert_eq!(r.getId(10), Some(1));
}

#[test]
fn NR4_get_id_miss_is_null() {
    let r = NumIdRedirection::NumIdRedirection(UUNumberingId::new());
    assert_eq!(r.getId(99), None);
}

#[test]
fn NR5_put_same_key_overwrites() {
    let mut r = NumIdRedirection::NumIdRedirection(UUNumberingId::new());
    assert_eq!(r.put(10), 1);
    assert_eq!(r.put(10), 2);
    assert_eq!(r.getId(10), Some(2));
}

#[test]
fn NR6_get_current_id_tracks_next() {
    let mut maker = UUNumberingId::new();
    maker.getNextId();
    assert_eq!(maker.getCurrentId(), 1);
}
