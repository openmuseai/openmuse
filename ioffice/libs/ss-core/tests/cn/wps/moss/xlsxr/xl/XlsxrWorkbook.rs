#[derive(Default)]
struct OpenObserver;
impl ss_core::cn::wps::moss::app::event::KmoBookOpenEventListener::KmoBookOpenEventListener for OpenObserver{}
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use ss_core::java_runtime::Rc;

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use kfc_kfc_alpha::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use ss_core::cn::wps::moss::xlsxr::xl::XlsxrWorkbook::{memoryToolKitCpuNumber, XlsxrWorkbook};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

const WORKSHEET: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
const STYLES: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
const SHARED: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings";
const THEME: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn open_book(path: &str) -> (Rc<KmoBook>, Rc<XlsxrWorkbook>, Result<(), std::io::Error>) {
    let book = KmoBook::KmoBook();
    book.init(false);
    let mut pack = OxfrPackage::OxfrPackage(path).unwrap();
    let part = XlsxrPackage::getBookPart(&mut pack);
    let zip = part.getZipFile();
    let entry = part.getZipEntry().unwrap();
    let owned = OpcrPackagePart::OpcrPackagePart(&zip, &entry);
    let reader = XlsxrWorkbook::XlsxrWorkbook(Rc::clone(&book),None,Some(Rc::new(OpenObserver)),None);
    reader.setBookPart(owned);
    let result = XlsxrWorkbook::open(&reader);
    let _ = pack;
    (book, reader, result)
}

fn sst(strings: &[&str]) -> String {
    let body = strings
        .iter()
        .map(|text| format!("<si><t>{text}</t></si>"))
        .collect::<String>();
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<sst xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" count=\"{}\" uniqueCount=\"{}\">{body}</sst>\n",
        strings.len(),
        strings.len()
    )
}

fn styles_xml(inner: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<styleSheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">{inner}</styleSheet>\n"
    )
}

fn write_book(path: &PathBuf, sheets_xml: &str, rels: &[(&str, &str, &str)]) {
    let mut rel_body = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n",
    );
    for (id, typ, target) in rels {
        rel_body.push_str(&format!(
            "  <Relationship Id=\"{id}\" Type=\"{typ}\" Target=\"{target}\"/>\n"
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
    let mut parts: Vec<(&str, String)> = vec![
        ("[Content_Types].xml", types.to_string()),
        ("_rels/.rels", root.to_string()),
        ("xl/workbook.xml", workbook),
        ("xl/_rels/workbook.xml.rels", rel_body),
    ];
    for (_id, typ, _target) in rels {
        if *typ == STYLES {
            parts.push((
                "xl/styles.xml",
                styles_xml("<cellStyleXfs count=\"1\"><xf numFmtId=\"0\"/></cellStyleXfs><cellXfs count=\"2\"><xf numFmtId=\"0\" xfId=\"0\"/><xf numFmtId=\"0\" xfId=\"0\"/></cellXfs>"),
            ));
        } else if *typ == SHARED {
            parts.push(("xl/sharedStrings.xml", sst(&["Hello", "主题后"])));
        } else if *typ == THEME {
            parts.push((
                "xl/theme/theme1.xml",
                "<a:theme xmlns:a=\"http://schemas.openxmlformats.org/drawingml/2006/main\"/>".to_string(),
            ));
        }
    }
    for (name, body) in &parts {
        zip.start_file(*name, opt).unwrap();
        zip.write_all(body.as_bytes()).unwrap();
    }
    zip.finish().unwrap();
}

fn style_xf_size(reader: &XlsxrWorkbook) -> i32 {
    reader
        .stylesTable()
        .expect("StylesTable")
        .borrow()
        .getStyleXfSize()
}

fn store_count(book: &KmoBook) -> i32 {
    book.getCellValueStore().borrow().getUniqueCount()
}

fn store_string(book: &KmoBook, id: i32) -> String {
    book.getCellValueStore()
        .borrow_mut()
        .getStringBySSTId(id)
}

#[test]
fn glob1_basic_types_global_data_before_cells() {
    let (book, reader, result) = open_book(&fixture("basic-types.xlsx"));
    result.unwrap();
    assert_eq!(book.sheetCount(), 1);
    assert_eq!(book.sheetAt(0).borrow().name(), "Sheet1");
    assert_eq!(reader.relationshipId(0).as_deref(), Some("rId1"));
    assert_eq!(reader.sheetReaderSheetCount(), 1);
    assert_eq!(style_xf_size(&reader), 1);
    assert_eq!(
        reader.stylesTable().unwrap().borrow().convertDocumentXF2CoreXF(0),
        15
    );
    assert_eq!(store_string(&book, 0), "Hello");
    assert_eq!(store_string(&book, 1), "Latin");
    assert_eq!(store_string(&book, 2), "中文");
    // inlineStr「世界」再入库一次，共享字符串 0..2 不变。
    assert_eq!(store_count(&book), 4);
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    assert_eq!(sheet.getCellValueType__2(0, 0), 1);
    assert_eq!(
        sheet
            .getCellValueStore()
            .borrow_mut()
            .doubleValue(sheet.getCellData(0, 0).id),
        1.5
    );
    if !std::path::Path::new("/sys/devices/system/cpu/").is_dir() {
        assert_eq!(memoryToolKitCpuNumber(), 0);
    }
}

#[test]
fn glob2_sparse_sheets_without_shared_strings() {
    let (book, reader, result) = open_book(&fixture("sparse-multisheet.xlsx"));
    result.unwrap();
    assert_eq!(book.sheetCount(), 3);
    assert_eq!(book.sheetAt(0).borrow().name(), "Visible");
    assert!(book.sheetAt(1).borrow().isSheetHidden());
    assert_eq!(book.sheetAt(2).borrow().name(), "Other");
    assert_eq!(reader.sheetReaderSheetCount(), 3);
    assert_eq!(style_xf_size(&reader), 1);
    assert_eq!(store_count(&book), 0);
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    assert_eq!(
        sheet
            .getCellValueStore()
            .borrow_mut()
            .doubleValue(sheet.getCellData(0, 0).id),
        1.0
    );
}

#[test]
fn glob3_empty_sst_still_opens_sheet_decl() {
    let (book, reader, result) = open_book(&fixture("empty.xlsx"));
    result.unwrap();
    assert_eq!(book.sheetCount(), 1);
    assert_eq!(reader.sheetReaderSheetCount(), 1);
    assert_eq!(style_xf_size(&reader), 1);
    assert_eq!(store_count(&book), 0);
    assert!(book.sheetAt(0).borrow().getCellData(0, 0).isNull());
}

#[test]
fn glob4_sheet_without_rid_is_damaged_after_styles_and_strings() {
    let path = std::env::temp_dir().join(format!("m1-open-norid-{}.xlsx", std::process::id()));
    write_book(
        &path,
        "    <sheet name=\"NoRid\" sheetId=\"4\"/>\n",
        &[
            ("rId1", STYLES, "styles.xml"),
            ("rId2", SHARED, "sharedStrings.xml"),
        ],
    );
    let (book, reader, result) = open_book(path.to_str().unwrap());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("FileDamagedException"));
    assert_eq!(book.sheetCount(), 0);
    assert_eq!(reader.sheetReaderSheetCount(), 0);
    assert_eq!(style_xf_size(&reader), 1);
    assert_eq!(store_count(&book), 2);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn glob5_large_styles_do_not_skip_shared_strings() {
    let path = std::env::temp_dir().join(format!("m1-open-large-xf-{}.xlsx", std::process::id()));
    write_book(
        &path,
        "    <sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/>\n",
        &[
            ("rId1", WORKSHEET, "worksheets/sheet1.xml"),
            ("rId2", STYLES, "styles.xml"),
            ("rId3", SHARED, "sharedStrings.xml"),
        ],
    );
    let bytes = std::fs::read(&path).unwrap();
    let pad = format!(" {}", " ".repeat(102401));
    let mut out = Vec::new();
    {
        let mut zip = ZipWriter::new(std::io::Cursor::new(&mut out));
        let opt = SimpleFileOptions::default();
        let mut reader = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
        for i in 0..reader.len() {
            let mut file = reader.by_index(i).unwrap();
            let name = file.name().to_string();
            let mut body = Vec::new();
            std::io::Read::read_to_end(&mut file, &mut body).unwrap();
            if name == "xl/styles.xml" {
                body = pad.as_bytes().to_vec();
            }
            zip.start_file(name, opt).unwrap();
            zip.write_all(&body).unwrap();
        }
        zip.finish().unwrap();
    }
    std::fs::write(&path, out).unwrap();
    let (book, reader, result) = open_book(path.to_str().unwrap());
    result.unwrap();
    assert_eq!(book.sheetCount(), 1);
    assert_eq!(style_xf_size(&reader), 0);
    assert_eq!(store_count(&book), 2);
    assert_eq!(store_string(&book, 0), "Hello");
    assert!(book.sheetAt(0).borrow().getCellData(0, 0).isNull());
    let _ = std::fs::remove_file(&path);
}

#[test]
fn glob6_no_sheet_is_file_damaged_after_global_data() {
    let path = std::env::temp_dir().join(format!("m1-open-nosheet-{}.xlsx", std::process::id()));
    write_book(
        &path,
        "",
        &[
            ("rId1", STYLES, "styles.xml"),
            ("rId2", SHARED, "sharedStrings.xml"),
        ],
    );
    let (book, reader, result) = open_book(path.to_str().unwrap());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("FileDamagedException"));
    assert_eq!(book.sheetCount(), 0);
    assert_eq!(reader.sheetReaderSheetCount(), 0);
    assert_eq!(style_xf_size(&reader), 1);
    assert_eq!(store_count(&book), 2);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn glob7_theme_relationship_does_not_block_styles_or_strings() {
    let path = std::env::temp_dir().join(format!("m1-open-theme-{}.xlsx", std::process::id()));
    write_book(
        &path,
        "    <sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/>\n",
        &[
            ("rId1", WORKSHEET, "worksheets/sheet1.xml"),
            ("rId2", THEME, "theme/theme1.xml"),
            ("rId3", STYLES, "styles.xml"),
            ("rId4", SHARED, "sharedStrings.xml"),
        ],
    );
    let (book, reader, result) = open_book(path.to_str().unwrap());
    result.unwrap();
    assert_eq!(book.sheetCount(), 1);
    assert_eq!(style_xf_size(&reader), 1);
    assert_eq!(store_string(&book, 1), "主题后");
    assert!(book.sheetAt(0).borrow().getCellData(0, 0).isNull());
    let _ = std::fs::remove_file(&path);
}

#[test]
fn glob8_missing_sheet_part_still_declares_and_loads_styles() {
    let (book, reader, result) = open_book(&fixture("broken-rel.xlsx"));
    result.unwrap();
    assert_eq!(book.sheetCount(), 1);
    // 部件缺失时 openActiveSheet 吃掉空指针并移除 reader，表声明还在。
    assert_eq!(reader.sheetReaderSheetCount(), 0);
    assert_eq!(style_xf_size(&reader), 1);
    assert_eq!(store_count(&book), 0);
    assert!(book.sheetAt(0).borrow().getCellData(0, 0).isNull());
}
