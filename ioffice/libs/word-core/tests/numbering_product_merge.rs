use std::fs;

use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::data::IPapxTable::{Entry, IPapxTable};
use word_core::cn::wps::moffice::writer::layout::base::env::PropertyTool::PropertyTool;

#[test]
fn complex_sample_numbering_reaches_final_paragraph_properties() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/docx-mvp-render/flutter/assets/sample2/机械制造与自动化毕业论文.docx");
    let bytes = fs::read(path).unwrap();
    let (mut document, outcome) = DocumentOpen::open_boxed_text_document_from_docx_outcome(&bytes, "").unwrap();
    assert_eq!(format!("{:?}", outcome.source), "Facade");

    let lfo = document.getAutoNumTable().getLfoDataTable().getDataByIlfo(1).unwrap();
    let level = document.getAutoNumTable().getLstDataTable().getDataByLsid(lfo.getLsid())
        .unwrap().getLvlfDataByIlvl(1).unwrap();
    let level_para = level.getParaProp().unwrap();
    assert_eq!(level_para.getInt__int_int(210, 0), 960);
    assert_eq!(level_para.getInt__int_int(208, 0), -420);

    let main = document.getMainDocument().unwrap();
    let papx = IPapxTable::seek(main.getPapxTable().unwrap(), 1850).unwrap();
    let direct = Entry::getProperty(&*papx).unwrap();
    assert_eq!(direct.getInt__int_int(235, -1), 1);
    assert_eq!(direct.getInt__int_int(234, -1), 1);
    let mut tool = PropertyTool::PropertyTool(main, 256, 256);
    let merged = tool.getFinalParaProp(&*papx, true);
    assert_eq!(merged.getInt__int_int(210, 0), 960);
    assert_eq!(merged.getInt__int_int(208, 0), -420);
}
