use std::{fs, path::Path};
use word_core::cn::wps::moffice::writer::api::DocumentOpen;

#[test]
fn restart_at_same_grid_edges_begins_a_new_java_merge_chain() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/docx-mvp-render/flutter/assets/sample3/签到表格.docx");
    let bytes = fs::read(path).expect("Sample 3 DOCX");
    let (doc, _) = DocumentOpen::open_boxed_text_document_from_docx_outcome(&bytes, "")
        .expect("real DOCX import");
    let main = doc.getMainDocument().expect("main story");
    let mut table = main.getTableInfos().and_then(|infos| infos.getTableByIndex(0))
        .expect("root table");
    table.getJC(); // Java TableInfo is validated before TableRowLayouter.prepareEnvs.

    for row in [4, 9, 14, 18, 22] {
        assert_eq!(table.cell_env_flags(row, 0), (true, true, false), "restart row {row}");
        assert_eq!(table.first_vert_merge_cell(row, 0), Some((row, 0)));
    }
    for row in [8, 13, 17, 21, 23] {
        assert_eq!(table.cell_env_flags(row, 0), (true, false, true), "last row {row}");
    }
}
