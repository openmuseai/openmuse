use std::sync::Mutex;
static STYLES_TEST_LOCK:Mutex<()>=Mutex::new(());
use std::fs::File;
use std::io::Write;
use ss_core::java_runtime::Rc;
use ss_core::java_runtime::RefCell;

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use ss_core::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use ss_core::cn::wps::moss::xlsxr::xl::XlsxrStyles::XlsxrStyles;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

const STYLES_REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn open_xlsx(path: &str) -> (Rc<KmoBook>, Rc<RefCell<StylesTable>>) {
    let book = KmoBook::KmoBook();
    // Java KmoBooks.loadTemplates precedes XlsxrStyles.open in the real reader.
    ss_core::cn::wps::moss::app::KmoBooks::KmoBooks::loadTemplates(&book);
    let table = Rc::new(RefCell::new(StylesTable::StylesTable()));
    let mut pack = OxfrPackage::OxfrPackage(path).unwrap();
    let part = XlsxrPackage::getBookPart(&mut pack);
    let rels = part.getRelationships_mut().unwrap().unwrap();
    let mut found = None;
    for i in 0..rels.size() {
        if rels.getRelationship(i).getType().as_deref() == Some(STYLES_REL) {
            found = Some(i);
            break;
        }
    }
    let rel = rels.getRelationship_mut(found.expect("styles relationship"));
    XlsxrStyles::open(&book, Some(rel), &table).unwrap();
    (book, table)
}

fn write_styles(path: &std::path::Path, styles_xml: &str) {
    let rels = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n  <Relationship Id=\"rId1\" Type=\"{STYLES_REL}\" Target=\"styles.xml\"/>\n</Relationships>\n"
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
        ("xl/styles.xml", styles_xml),
    ] {
        zip.start_file(name, opt).unwrap();
        zip.write_all(body.as_bytes()).unwrap();
    }
    zip.finish().unwrap();
}

fn sheet(styles_inner: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<styleSheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">{styles_inner}</styleSheet>\n"
    )
}

#[test]
fn xf1_missing_styles_relationship() {
    let _guard=STYLES_TEST_LOCK.lock().unwrap();
    let book = KmoBook::KmoBook();
    let table = Rc::new(RefCell::new(StylesTable::StylesTable()));
    XlsxrStyles::open(&book, None, &table).unwrap();
    assert_eq!(book.getXFManager().borrow().getNumExFormats(), 0);
    assert_eq!(table.borrow().getStartCoreXF(), 0);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(0), 15);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(1), 15);
}

#[test]
fn xf2_basic_types_default_xf() {
    let _guard=STYLES_TEST_LOCK.lock().unwrap();
    let (book, table) = open_xlsx(&fixture("basic-types.xlsx"));
    assert_eq!(book.getXFManager().borrow().getNumExFormats(), 22);
    assert_eq!(table.borrow().getStartCoreXF(), 22);
    assert_eq!(table.borrow().getStyleXfSize(), 1);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(0), 15);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(1), 15);
}

#[test]
fn xf3_second_cell_xf_maps_to_zero() {
    let _guard=STYLES_TEST_LOCK.lock().unwrap();
    let path = std::env::temp_dir().join(format!("m1-styles-cell-{}.xlsx", std::process::id()));
    write_styles(
        &path,
        &sheet("<cellStyleXfs count=\"1\"><xf numFmtId=\"0\"/></cellStyleXfs><cellXfs count=\"2\"><xf numFmtId=\"0\" xfId=\"0\"/><xf numFmtId=\"0\" xfId=\"0\"/></cellXfs>"),
    );
    let (book, table) = open_xlsx(path.to_str().unwrap());
    assert_eq!(book.getXFManager().borrow().getNumExFormats(), 23);
    assert_eq!(table.borrow().getStartCoreXF(), 22);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(0), 15);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(1), 22);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(2), 15);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn xf4_second_style_shifts_core_index() {
    let _guard=STYLES_TEST_LOCK.lock().unwrap();
    let path = std::env::temp_dir().join(format!("m1-styles-style-{}.xlsx", std::process::id()));
    write_styles(
        &path,
        &sheet("<cellStyleXfs count=\"2\"><xf numFmtId=\"0\"/><xf numFmtId=\"0\"/></cellStyleXfs><cellXfs count=\"2\"><xf numFmtId=\"0\" xfId=\"0\"/><xf numFmtId=\"0\" xfId=\"0\"/></cellXfs>"),
    );
    let (book, table) = open_xlsx(path.to_str().unwrap());
    assert_eq!(book.getXFManager().borrow().getNumExFormats(), 24);
    assert_eq!(table.borrow().getStartCoreXF(), 23);
    assert_eq!(table.borrow().getStyleXfSize(), 2);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(0), 15);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(1), 23);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(2), 15);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn xf5_cell_xf_without_style_xfs() {
    let _guard=STYLES_TEST_LOCK.lock().unwrap();
    let path = std::env::temp_dir().join(format!("m1-styles-nostyle-{}.xlsx", std::process::id()));
    write_styles(
        &path,
        &sheet("<cellXfs count=\"2\"><xf numFmtId=\"0\" xfId=\"0\"/><xf numFmtId=\"0\" xfId=\"0\"/></cellXfs>"),
    );
    let (book, table) = open_xlsx(path.to_str().unwrap());
    assert_eq!(book.getXFManager().borrow().getNumExFormats(), 23);
    assert_eq!(table.borrow().getStartCoreXF(), 22);
    assert_eq!(table.borrow().getStyleXfSize(), 1);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(1), 22);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn xf6_large_styles_lazy_blocks_and_active_sheet_completion() {
    let _guard=STYLES_TEST_LOCK.lock().unwrap();
    let path = std::env::temp_dir().join(format!("m2-styles-large-{}.xlsx", std::process::id()));
    // A valid >100KiB part with 301 XFs takes Java's block path.
    let xfs=(0..301).map(|i|format!("<xf numFmtId=\"{}\" xfId=\"0\"/>",i%10)).collect::<String>();
    let xml=format!("<styleSheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">{}<cellStyleXfs><xf numFmtId=\"0\"/></cellStyleXfs><cellXfs count=\"301\">{xfs}</cellXfs></styleSheet>"," ".repeat(102401));
    write_styles(&path,&xml);
    let (book,table)=open_xlsx(path.to_str().unwrap());
    assert_eq!(book.getXFManager().borrow().getNumExFormats(),322);
    assert_eq!(table.borrow().convertDocumentXF2CoreXF(300),321);
    XlsxrStyles::onXFIndex(&table.borrow(),200);
    XlsxrStyles::onActiveSheetAfter(&table.borrow()).unwrap();
    for i in 1..301 {let xf=book.getXFManager().borrow().getXF(21+i).unwrap();assert_eq!(xf.getFormatIndex(),(i%10) as i16);}
    XlsxrStyles::clear();
    let _=std::fs::remove_file(path);
}

#[test]
fn xf7_original_java_traversal_byte_offsets() {
    let _guard=STYLES_TEST_LOCK.lock().unwrap();
    use ss_core::cn::wps::moss::xlsxr::xl::StylesTraversal::StylesTraversal;
    let root=format!("{}/rewrite-review/milestones/m2-xf-display/fixtures/traversal",env!("CARGO_MANIFEST_DIR"));
    for line in std::fs::read_to_string(format!("{root}/expected.tsv")).unwrap().lines(){
        let v=line.split('\t').collect::<Vec<_>>();let t=StylesTraversal::new(&format!("{root}/{}",v[0])).unwrap();
        assert_eq!(t.needBlocks().to_string(),v[1],"{}",v[0]);
        for (got,i) in [(t.getFirstXfStartOffset(),2),(t.getCellXfsEndOffset(),3),(t.getXFCount(),4),(t.getFileSize(),5)]{assert_eq!(got.to_string(),v[i],"{} field {i}",v[0]);}
        let b=t.getXfsOffset().borrow().iter().map(|b|format!("{}:{}",b.mBeginOffset,b.mEndOffset)).collect::<Vec<_>>();assert_eq!(b,v[6..],"{}",v[0]);
    }
}
