// function_id=03_function_table_infos
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_table_infos

use std::path::PathBuf;

use word_core::cn::wps::moffice::writer::api::DocumentOpen::open_boxed_text_document_from_docx_outcome;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};

fn sample() -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../samples/docx-mvp-render/flutter/assets/sample2/机械制造与自动化毕业论文.docx",
    );
    std::fs::read(path).expect("complex sample")
}

#[test]
fn product_table_infos_size_comes_from_papx_rows() {
    let bytes = sample();
    let (doc, outcome) = open_boxed_text_document_from_docx_outcome(&bytes, "")
        .expect("open docx");
    assert!(outcome.panic_kind.is_none());
    let main = doc.getMainDocument().expect("main story");
    let infos = main.getTableInfos().expect("table infos");
    let size = infos.size();
    let again = infos.size();
    let nested = infos.nestedTableCount();
    assert_eq!(size, again);
    // Java product dump on PGU110 (2026-10-07): one root with six rows and
    // four child tables. This catches updateInNormal losing scanNext=true.
    assert_eq!(size, 1, "root table count");
    assert_eq!(nested, 4, "cell-nested table count");
    assert_eq!(nested, infos.nestedTableCount());
    assert!(infos.getTableByIndex(-1).is_none());
    assert!(infos.getTableByIndex(size).is_none());
    let root = infos.getTableByIndex(0).expect("root table");
    assert_eq!(root.getLevel(), 1);
    assert_eq!(root.row_count(), 6);
    assert_eq!(root.getStartCP(), 6334);
    assert!(root.getStartCP() < root.getEndCP());
    let by_level = infos
        .getTableInfoByLevel(root.getStartCP(), 1)
        .expect("level 1 table");
    assert_eq!(by_level.getStartCP(), root.getStartCP());
    assert_eq!(by_level.getEndCP(), root.getEndCP());
    let row = infos
        .getRowInfoByLevel(root.getStartCP(), 1)
        .expect("level 1 row");
    assert!(row.getStartCP() <= root.getStartCP() && root.getStartCP() < row.getEndCP());
    let cell = infos
        .getCellInfoByLevel(root.getStartCP(), 1)
        .expect("level 1 cell");
    assert!(cell.getStartCP() <= root.getStartCP() && root.getStartCP() < cell.getEndCP());
    let mut level2_cp = None;
    if let Some(papx) = main.getPapxTable() {
        let mut entry = IPapxTable::begin(papx);
        while let Some(current) = entry {
            if current.isEnd() {
                break;
            }
            if let Some(prop) = PapxEntry::getProperty(&*current) {
                if prop.getInt__int_int(224, 1) >= 2 && prop.getBoolean__int_boolean(223, false) {
                    level2_cp = Some(current.position());
                    break;
                }
            }
            entry = PapxEntry::getNext(&*current);
        }
    }
    let level2_cp = level2_cp.expect("level-2 paragraph");
    let nested_table = infos
        .getTableInfoByLevel(level2_cp, 2)
        .expect("level 2 table");
    assert_eq!(nested_table.getLevel(), 2);
    assert!(nested_table.getStartCP() <= level2_cp && level2_cp < nested_table.getEndCP());
}
