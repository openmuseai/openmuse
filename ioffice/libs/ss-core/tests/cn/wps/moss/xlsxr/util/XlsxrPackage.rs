use std::panic::{catch_unwind, AssertUnwindSafe};

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use ss_core::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;

const WORKSHEET: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
const STYLES: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
const SHARED_STRINGS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings";

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

#[test]
fn opc1_workbook_child_parts() {
    let mut pack = OxfrPackage::OxfrPackage(&fixture("basic-types.xlsx")).unwrap();
    let book = XlsxrPackage::getBookPart(&mut pack);
    let name = book.getPartName();
    assert!(name.ends_with("xl/workbook.xml"), "{name}");
    assert!(!XlsxrPackage::isXlsb(book));
    let sheet = XlsxrPackage::getChildPartByRid(Some(book), "rId1")
        .unwrap()
        .expect("sheet part");
    let sheet_name = sheet.getPartName();
    assert!(
        sheet_name.ends_with("xl/worksheets/sheet1.xml"),
        "{sheet_name}"
    );
    drop(sheet_name);
    let book = XlsxrPackage::getBookPart(&mut pack);
    let styles = XlsxrPackage::getChildPartByType(Some(book), STYLES)
        .unwrap()
        .expect("styles");
    assert!(styles.getPartName().ends_with("xl/styles.xml"));
    let book = XlsxrPackage::getBookPart(&mut pack);
    let sst = XlsxrPackage::getChildRelationshipByType(Some(book), SHARED_STRINGS)
        .unwrap()
        .expect("sst rel");
    assert_eq!(sst.getId().as_deref(), Some("rId3"));
    assert_eq!(sst.getTarget().as_deref(), Some("sharedStrings.xml"));
    let book = XlsxrPackage::getBookPart(&mut pack);
    let rids = XlsxrPackage::getRidsByType(book, WORKSHEET).unwrap();
    assert_eq!(rids, vec![Some("rId1".to_string())]);
    assert!(XlsxrPackage::getChildPartByRid(None, "rId1")
        .unwrap()
        .is_none());
}

#[test]
fn opc2_missing_target_and_not_workbook() {
    let mut pack = OxfrPackage::OxfrPackage(&fixture("broken-rel.xlsx")).unwrap();
    let book = XlsxrPackage::getBookPart(&mut pack);
    assert!(XlsxrPackage::getChildPartByRid(Some(book), "rId1")
        .unwrap()
        .is_none());
    let book = XlsxrPackage::getBookPart(&mut pack);
    let styles = XlsxrPackage::getChildPartByRid(Some(book), "rId2")
        .unwrap()
        .expect("styles still present");
    assert!(styles.getPartName().ends_with("xl/styles.xml"));

    let mut multi = OxfrPackage::OxfrPackage(&fixture("sparse-multisheet.xlsx")).unwrap();
    let book = XlsxrPackage::getBookPart(&mut multi);
    let rids = XlsxrPackage::getRidsByType(book, WORKSHEET).unwrap();
    assert_eq!(
        rids,
        vec![
            Some("rId1".to_string()),
            Some("rId2".to_string()),
            Some("rId3".to_string())
        ]
    );

    let path = fixture("not-workbook.zip");
    let opened = catch_unwind(AssertUnwindSafe(|| {
        let mut pack = OxfrPackage::OxfrPackage(&path).expect("zip opens");
        XlsxrPackage::getBookPart(&mut pack).getPartName()
    }));
    let err = opened.expect_err("not a workbook");
    let message = err
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| err.downcast_ref::<&str>().map(|s| (*s).to_string()))
        .unwrap_or_default();
    assert!(
        message.starts_with("FileNotSupportException"),
        "{message}"
    );
}
