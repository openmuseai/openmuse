// function_id=02_function_builtin_typesti
// cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_builtin_typesti
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/BuildinStyleSheet.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/BuildinStyleSheet.rs

use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BuildinStyleSheet::BuildinStyleSheet;
use std::sync::Mutex;

// ★修复既有 flake（本轮全量跑命中 1 次 `BS2_normal_type_sti` 失败；单跑 5/5 通过）：
//   `BuildinStyleSheet.mMapName2TypeSti` 是**进程级共享静态**，而 `BS9` 会调
//   `clearMapName2TypeSti()` **清空**它 ⇒ 并行执行时 `BS2` 可能在「已清空、尚未惰性重建」
//   的窗口里读到 `None`（`getTypeSti` 的 check 与 act 不在同一把锁内）。
//   ⇒ 与 `50_function_xwpf_ns_mode` 同一处置：用一把文件内 `Mutex` 把共享态访问**串行化**。
static SERIAL: Mutex<()> = Mutex::new(());

#[test]
fn BS1_heading1_type_sti() {
    let _g = SERIAL.lock().unwrap();
    let ts = BuildinStyleSheet::getTypeSti("heading 1").expect("heading 1 is a builtin name");
    assert_eq!(ts.r#type, 1);
    assert_eq!(ts.sti, 1);
}

#[test]
fn BS2_normal_type_sti() {
    let _g = SERIAL.lock().unwrap();
    let ts = BuildinStyleSheet::getTypeSti("Normal").expect("Java map has Normal → TypeSti(1,0)");
    assert_eq!(ts.r#type, 1);
    assert_eq!(ts.sti, 0);
}

#[test]
fn BS3_default_paragraph_font() {
    let _g = SERIAL.lock().unwrap();
    let ts = BuildinStyleSheet::getTypeSti("Default Paragraph Font")
        .expect("Default Paragraph Font is a builtin name");
    assert_eq!(ts.r#type, 2);
    assert_eq!(ts.sti, 65);
}

#[test]
fn BS4_heading1_char() {
    let _g = SERIAL.lock().unwrap();
    let ts = BuildinStyleSheet::getTypeSti("Heading 1 Char").expect("Heading 1 Char is builtin");
    assert_eq!(ts.r#type, 2);
    assert_eq!(ts.sti, 12289);
}

#[test]
fn BS5_unknown_name_is_null() {
    let _g = SERIAL.lock().unwrap();
    assert!(BuildinStyleSheet::getTypeSti("not-a-builtin").is_none());
}

#[test]
fn BS6_is_reserved_name() {
    let _g = SERIAL.lock().unwrap();
    assert!(BuildinStyleSheet::isReservedName("heading 1"));
    assert!(!BuildinStyleSheet::isReservedName("not-a-builtin"));
}

#[test]
fn BS7_is_reserved_index_bounds() {
    let _g = SERIAL.lock().unwrap();
    assert!(!BuildinStyleSheet::isReservedIndex(-1));
    assert!(BuildinStyleSheet::isReservedIndex(0));
    assert!(BuildinStyleSheet::isReservedIndex(267));
    assert!(!BuildinStyleSheet::isReservedIndex(268));
}

#[test]
fn BS8_get_name_zero_and_one() {
    let _g = SERIAL.lock().unwrap();
    assert_eq!(BuildinStyleSheet::getName(0), "Normal");
    assert_eq!(BuildinStyleSheet::getName(1), "heading 1");
}

#[test]
fn BS9_clear_then_reinit() {
    let _g = SERIAL.lock().unwrap();
    let _ = BuildinStyleSheet::getTypeSti("heading 1");
    BuildinStyleSheet::clearMapName2TypeSti();
    let ts = BuildinStyleSheet::getTypeSti("heading 1").expect("lazy re-init after clear");
    assert_eq!(ts.r#type, 1);
    assert_eq!(ts.sti, 1);
}
