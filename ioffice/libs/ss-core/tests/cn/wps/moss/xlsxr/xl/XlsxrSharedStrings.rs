use std::fs::File;
use std::io::Write;
use ss_core::java_runtime::Rc;

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use ss_core::cn::wps::moss::xlsxr::xl::XlsxrSharedStrings::XlsxrSharedStrings;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
// The original reader exposes one static uniqueCount for all workbooks. Keep
// each open/count observation together while cargo runs these tests in parallel.
static SST_SERIAL:std::sync::Mutex<()>=std::sync::Mutex::new(());

const SST_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings";

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn texts(book: &KmoBook) -> Vec<String> {
    let n = XlsxrSharedStrings::getUniqueCount();
    let store = book.getCellValueStore();
    let mut guard = store.borrow_mut();
    (0..n).map(|i| guard.getStringBySSTId(i)).collect()
}

fn open_xlsx(path: &str) -> Rc<KmoBook> {
    let book = KmoBook::KmoBook();
    book.init(false);
    let mut pack = OxfrPackage::OxfrPackage(path).unwrap();
    let part = XlsxrPackage::getBookPart(&mut pack);
    let rels = part.getRelationships_mut().unwrap().unwrap();
    let mut found = None;
    for i in 0..rels.size() {
        if rels.getRelationship(i).getType().as_deref() == Some(SST_REL) {
            found = Some(i);
            break;
        }
    }
    let rel = rels.getRelationship_mut(found.expect("shared strings relationship"));
    XlsxrSharedStrings::open(&book, Some(rel)).unwrap();
    book
}

fn write_sst(path: &std::path::Path, sst_xml: &str) {
    let rels = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n  <Relationship Id=\"rId1\" Type=\"{SST_REL}\" Target=\"sharedStrings.xml\"/>\n</Relationships>\n"
    );
    let workbook = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<workbook xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"><sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>\n";
    let types = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/><Default Extension=\"xml\" ContentType=\"application/xml\"/><Override PartName=\"/xl/workbook.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml\"/></Types>\n";
    let root = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"xl/workbook.xml\"/></Relationships>\n";
    let file = File::create(path).unwrap();
    let mut zip = ZipWriter::new(file);
    let opt = SimpleFileOptions::default();
    for (name, body) in [
        ("[Content_Types].xml", types),
        ("_rels/.rels", root),
        ("xl/workbook.xml", workbook),
        ("xl/_rels/workbook.xml.rels", rels.as_str()),
        ("xl/sharedStrings.xml", sst_xml),
    ] {
        zip.start_file(name, opt).unwrap();
        zip.write_all(body.as_bytes()).unwrap();
    }
    zip.finish().unwrap();
}

#[test]
fn sst1_missing_relationship() {
    let _serial=SST_SERIAL.lock().unwrap();
    let book = KmoBook::KmoBook();
    XlsxrSharedStrings::open(&book, None).unwrap();
    assert_eq!(XlsxrSharedStrings::getUniqueCount(), 0);
}

#[test]
fn sst2_basic_types_order() {
    let _serial=SST_SERIAL.lock().unwrap();
    let book = open_xlsx(&fixture("basic-types.xlsx"));
    assert_eq!(XlsxrSharedStrings::getUniqueCount(), 3);
    assert_eq!(
        texts(&book),
        vec!["Hello".to_string(), "Latin".to_string(), "中文".to_string()]
    );
}

#[test]
fn sst3_empty_sst() {
    let _serial=SST_SERIAL.lock().unwrap();
    let book = open_xlsx(&fixture("empty.xlsx"));
    assert_eq!(XlsxrSharedStrings::getUniqueCount(), 0);
    assert!(texts(&book).is_empty());
}

#[test]
fn sst4_empty_and_duplicate_strings() {
    let _serial=SST_SERIAL.lock().unwrap();
    let path = std::env::temp_dir().join(format!("m1-sst-dup-{}.xlsx", std::process::id()));
    write_sst(
        &path,
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="3" uniqueCount="3"><si><t>Hello</t></si><si><t></t></si><si><t>Hello</t></si></sst>"#,
    );
    let book = open_xlsx(path.to_str().unwrap());
    assert_eq!(
        texts(&book),
        vec!["Hello".to_string(), String::new(), "Hello".to_string()]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn sst5_large_empty_sst_uses_original_zero_count_fallback() {
    let _serial=SST_SERIAL.lock().unwrap();
    let path = std::env::temp_dir().join(format!("m1-sst-large-{}.xlsx", std::process::id()));
    let pad = format!("<sst>{}</sst>", " ".repeat(16385));
    write_sst(&path, &pad);
    let book = KmoBook::KmoBook();
    book.init(false);
    let mut pack = OxfrPackage::OxfrPackage(path.to_str().unwrap()).unwrap();
    let part = XlsxrPackage::getBookPart(&mut pack);
    let rels = part.getRelationships_mut().unwrap().unwrap();
    let mut found = None;
    for i in 0..rels.size() {
        if rels.getRelationship(i).getType().as_deref() == Some(SST_REL) {
            found = Some(i);
            break;
        }
    }
    let rel = rels.getRelationship_mut(found.unwrap());
    // Fresh original Android proof for this exact no-attribute XML, R49.
    let proof=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/sst-fallback-r0b49");
    let input:serde_json::Value=serde_json::from_slice(&std::fs::read(proof.join("inputs.json")).unwrap()).unwrap();
    let java:serde_json::Value=serde_json::from_slice(&std::fs::read(proof.join("java.json")).unwrap()).unwrap();
    assert_eq!(input["sst"][0]["xml"],pad);
    assert_eq!(java["pipelines"][0]["open"],serde_json::json!({"result":"ok","value":null}));
    XlsxrSharedStrings::open(&book, Some(rel)).unwrap();
    assert_eq!(serde_json::json!(XlsxrSharedStrings::getUniqueCount()),java["pipelines"][0]["afterOpen"]["unique"]);
    assert_eq!(serde_json::json!(book.getCellValueStore().borrow_mut().getStringBySSTId(0)),java["pipelines"][0]["afterOpen"]["strings"][0]["value"]);
    let _ = std::fs::remove_file(&path);
}
