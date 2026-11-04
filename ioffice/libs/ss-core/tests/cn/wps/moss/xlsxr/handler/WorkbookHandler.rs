#[derive(Default)]
struct OpenObserver;
impl ss_core::cn::wps::moss::app::event::KmoBookOpenEventListener::KmoBookOpenEventListener for OpenObserver{}
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use ss_core::java_runtime::Rc;

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use ss_core::cn::wps::moss::xlsxr::xl::XlsxrWorkbook::XlsxrWorkbook;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn declare(path: &str, xml: Option<&str>) -> (Rc<KmoBook>, Rc<XlsxrWorkbook>) {
    let book = KmoBook::KmoBook();
    let mut pack = OxfrPackage::OxfrPackage(path).unwrap();
    let part = XlsxrPackage::getBookPart(&mut pack);
    let mut bytes = Vec::new();
    part.getInputStream_default()
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();
    let rels = part.getRelationships().unwrap().unwrap();
    let reader = XlsxrWorkbook::XlsxrWorkbook(Rc::clone(&book),None,Some(Rc::new(OpenObserver)),None);
    let payload = xml.map(|text| text.as_bytes().to_vec()).unwrap_or(bytes);
    XlsxrWorkbook::readDeclaredSheets(&reader, rels, &mut std::io::Cursor::new(payload)).unwrap();
    (book, reader)
}

fn sheet_line(book: &KmoBook, index: i32) -> String {
    let sheet = book.sheetAt(index);
    let sheet = sheet.borrow();
    format!(
        "{} id={} type={} hidden={} very={} selected={}",
        sheet.name(),
        sheet.getSheetId(),
        sheet.sheetType(),
        sheet.isSheetHidden(),
        sheet.isSheetVeryHidden(),
        sheet.isSelected()
    )
}

fn write_book(path: &PathBuf, sheets_xml: &str, rels: &[( &str, &str)]) {
    let mut rel_body = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n",
    );
    for (i, (typ, target)) in rels.iter().enumerate() {
        rel_body.push_str(&format!(
            "  <Relationship Id=\"rId{}\" Type=\"{typ}\" Target=\"{target}\"/>\n",
            i + 1
        ));
    }
    rel_body.push_str("</Relationships>\n");
    let workbook = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<workbook xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"><sheets>\n{sheets_xml}</sheets></workbook>\n"
    );
    let types = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/><Default Extension=\"xml\" ContentType=\"application/xml\"/><Override PartName=\"/xl/workbook.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml\"/></Types>\n";
    let root = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"xl/workbook.xml\"/></Relationships>\n";
    let file = File::create(path).unwrap();
    let mut zip = ZipWriter::new(file);
    let opt = SimpleFileOptions::default();
    for (name, body) in [
        ("[Content_Types].xml", types),
        ("_rels/.rels", root),
        ("xl/workbook.xml", workbook.as_str()),
        ("xl/_rels/workbook.xml.rels", rel_body.as_str()),
    ] {
        zip.start_file(name, opt).unwrap();
        zip.write_all(body.as_bytes()).unwrap();
    }
    zip.finish().unwrap();
}

#[test]
fn decl1_basic_types_sheet() {
    let (book, reader) = declare(&fixture("basic-types.xlsx"), None);
    assert_eq!(book.sheetCount(), 1);
    assert_eq!(
        sheet_line(&book, 0),
        "Sheet1 id=1 type=0 hidden=false very=false selected=false"
    );
    assert_eq!(reader.relationshipId(0).as_deref(), Some("rId1"));
    assert_eq!(reader.sheetReaderSheetCount(), 1);
    assert!(reader.getSheetBySheetId(1).is_some());
    assert_eq!(book.whichSheetIsActivated(), 0);
}

#[test]
fn decl2_sparse_order_and_hidden() {
    let (book, reader) = declare(&fixture("sparse-multisheet.xlsx"), None);
    assert_eq!(book.sheetCount(), 3);
    assert_eq!(
        sheet_line(&book, 0),
        "Visible id=1 type=0 hidden=false very=false selected=false"
    );
    assert_eq!(
        sheet_line(&book, 1),
        "Hidden id=2 type=0 hidden=true very=false selected=false"
    );
    assert_eq!(
        sheet_line(&book, 2),
        "Other id=3 type=0 hidden=false very=false selected=false"
    );
    assert_eq!(reader.relationshipId(0).as_deref(), Some("rId1"));
    assert_eq!(reader.relationshipId(1).as_deref(), Some("rId2"));
    assert_eq!(reader.relationshipId(2).as_deref(), Some("rId3"));
    assert_eq!(reader.sheetReaderSheetCount(), 3);
    assert!(reader.getSheetBySheetId(2).unwrap().borrow().isSheetHidden());
}

#[test]
fn decl3_missing_rid_still_appends() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets><sheet name="Orphan" sheetId="9" r:id="rIdMissing"/></sheets>
</workbook>"#;
    let (book, reader) = declare(&fixture("basic-types.xlsx"), Some(xml));
    assert_eq!(book.sheetCount(), 1);
    assert_eq!(book.sheetAt(0).borrow().name(), "Orphan");
    assert_eq!(book.sheetAt(0).borrow().getSheetId(), 9);
    assert_eq!(book.sheetAt(0).borrow().sheetType(), 0);
    assert_eq!(reader.sheetReaderSheetCount(), 0);
    assert!(reader.relationshipId(0).is_none());
    assert!(reader.getSheetBySheetId(9).is_some());
}

#[test]
fn decl4_no_rid_or_name_skips_sheet() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets>
    <sheet name="NoRid" sheetId="4"/>
    <sheet r:id="rId1" sheetId="5"/>
    <sheet name="Kept" sheetId="6" r:id="rId1"/>
  </sheets>
</workbook>"#;
    let (book, reader) = declare(&fixture("basic-types.xlsx"), Some(xml));
    assert_eq!(book.sheetCount(), 1);
    assert_eq!(book.sheetAt(0).borrow().name(), "Kept");
    assert_eq!(reader.relationshipId(0).as_deref(), Some("rId1"));
}

#[test]
fn decl5_id_fallback_very_hidden_and_active_tab() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <bookViews><workbookView activeTab="1"/></bookViews>
  <sheets>
    <sheet name="A" sheetId="1" id="rId1"/>
    <sheet name="B" sheetId="2" state="veryHidden" r:id="rId1"/>
  </sheets>
</workbook>"#;
    let (book, reader) = declare(&fixture("basic-types.xlsx"), Some(xml));
    assert_eq!(book.sheetCount(), 2);
    assert_eq!(reader.relationshipId(0).as_deref(), Some("rId1"));
    assert!(!book.sheetAt(0).borrow().isSheetHidden());
    assert!(book.sheetAt(1).borrow().isSheetVeryHidden());
    assert!(book.sheetAt(1).borrow().isSheetHidden());
    assert_eq!(book.whichSheetIsActivated(), 1);
    assert!(!book.sheetAt(0).borrow().isSelected());
    assert!(book.sheetAt(1).borrow().isSelected());
}

#[test]
fn decl6_defined_names_flag_and_bad_active_tab() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <definedNames><definedName name="A">Sheet1!$A$1</definedName></definedNames>
  <bookViews><workbookView activeTab="4"/></bookViews>
  <sheets><sheet name="Only" sheetId="1" r:id="rId1"/></sheets>
</workbook>"#;
    let (book, reader) = declare(&fixture("basic-types.xlsx"), Some(xml));
    assert!(reader.hasDefinedNames());
    assert_eq!(book.sheetCount(), 1);
    // activateSheet 先写 window1，sheets.activate 越界才抛。catch 吞掉之后活动索引留在 4，选中位没有改。
    assert_eq!(book.whichSheetIsActivated(), 4);
    assert!(!book.sheetAt(0).borrow().isSelected());
}

#[test]
fn decl7_chart_and_macro_types() {
    let path = std::env::temp_dir().join(format!("m1-sheet-decl-{}.xlsx", std::process::id()));
    write_book(
        &path,
        "    <sheet name=\"Grid\" sheetId=\"1\" r:id=\"rId1\"/>\n    <sheet name=\"Chart\" sheetId=\"2\" r:id=\"rId2\"/>\n    <sheet name=\"Macro\" sheetId=\"3\" r:id=\"rId3\"/>\n",
        &[
            (
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
                "worksheets/sheet1.xml",
            ),
            (
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet",
                "chartsheets/sheet1.xml",
            ),
            (
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/macrosheet",
                "macrosheets/sheet1.xml",
            ),
        ],
    );
    let (book, reader) = declare(path.to_str().unwrap(), None);
    let _ = std::fs::remove_file(&path);
    assert_eq!(book.sheetAt(0).borrow().sheetType(), 0);
    assert_eq!(book.sheetAt(1).borrow().sheetType(), 2);
    assert_eq!(book.sheetAt(2).borrow().sheetType(), 1);
    assert_eq!(reader.sheetReaderSheetCount(), 3);
    assert_eq!(reader.relationshipId(1).as_deref(), Some("rId2"));
}

#[test]
fn decl8_omitted_sheet_id_is_not_indexed() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets><sheet name="Bare" r:id="rId1"/></sheets>
</workbook>"#;
    let (book, reader) = declare(&fixture("basic-types.xlsx"), Some(xml));
    let id = book.sheetAt(0).borrow().getSheetId();
    assert_eq!(id, 0);
    assert!(reader.getSheetBySheetId(id).is_none());
    assert_eq!(reader.relationshipId(0).as_deref(), Some("rId1"));
}
