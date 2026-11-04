use std::{fs, path::Path};
use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState;

#[test]
fn cell_state_reads_borders_from_its_java_cell_info_without_table_lookup() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/docx-mvp-render/flutter/assets/sample3/签到表格.docx");
    let bytes = fs::read(path).expect("Sample 3 DOCX");
    let (doc, _) = DocumentOpen::open_boxed_text_document_from_docx_outcome(&bytes, "")
        .expect("real DOCX import");
    let main = doc.getMainDocument().expect("main story");
    let mut table = main
        .getTableInfos()
        .and_then(|infos| infos.getTableByIndex(0))
        .expect("root table");
    table.getJC();

    let mut checked = 0;
    for row in table.rows() {
        for index in 0..row.size() {
            let cell = row.getCellByIndex(index).expect("cell index");
            let expected = [
                cell.getBrcTop(),
                cell.getBrcLeft(),
                cell.getBrcBottom(),
                cell.getBrcRight(),
            ]
            .map(|brc| brc.map(|value| (value.getBorderType(), value.getLineWidth())));
            if expected.iter().all(Option::is_none) {
                continue;
            }
            let mut state = TableCellLayoutState::TableCellLayoutState();
            state.setCellnfo(cell.clone());
            // Java getters use the stored cellInfo directly. No document/table is attached here.
            let actual = [
                state.getBrcTop(),
                state.getBrcLeft(),
                state.getBrcBottom(),
                state.getBrcRight(),
            ]
            .map(|brc| brc.map(|value| (value.getBorderType(), value.getLineWidth())));
            assert_eq!(actual, expected);
            checked += 1;
        }
    }
    assert!(checked > 0, "the fixture must exercise stored cell borders");
}
