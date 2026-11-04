#[derive(Default)]
struct OpenObserver;
impl ss_core::cn::wps::moss::app::event::KmoBookOpenEventListener::KmoBookOpenEventListener for OpenObserver{}
use ss_core::java_runtime::Rc;

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use kfc_kfc_alpha::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttrValue::XmlRoAttrValue;
use ss_core::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
use ss_core::cn::wps::et::ss::formula::parser::FormulaParser::parse;
use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{area_ptg, getEncodedSizeWithoutArrayData, ref_ptg, serializePtgs, Ptg, PtgKind};
use ss_core::cn::wps::et::ss::formula::parser::ParserOption::ParserOption;
use ss_core::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::app::KmoBookApi::KmoBookApi;
use ss_core::cn::wps::moss::app::KmoSheet::KmoSheet;
use ss_core::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use ss_core::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
use ss_core::cn::wps::moss::io::QuickInitCell::QuickInitCell;
use ss_core::cn::wps::moss::xlsxr::handler::sheet::CellFmlBackupHandler::{
    CellFmlBackupHandler, NormalFml,
};
use ss_core::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use ss_core::cn::wps::moss::xlsxr::xl::XlsxrWorkbook::XlsxrWorkbook;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn sample_book() -> String {
    format!(
        "{}/../../samples/ss-render/出入库管理明细表.xlsx",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn open_book(path: &str) -> (Rc<KmoBook>, Rc<XlsxrWorkbook>) {
    let book = KmoBook::KmoBook();
    book.init(false);
    let mut pack = OxfrPackage::OxfrPackage(path).unwrap();
    let part = XlsxrPackage::getBookPart(&mut pack);
    let zip = part.getZipFile();
    let entry = part.getZipEntry().unwrap();
    let owned = OpcrPackagePart::OpcrPackagePart(&zip, &entry);
    let reader = XlsxrWorkbook::XlsxrWorkbook(Rc::clone(&book),None,Some(Rc::new(OpenObserver)),None);
    reader.setBookPart(owned);
    XlsxrWorkbook::open(&reader).unwrap();
    let _ = pack;
    (book, reader)
}

fn count_formula_types(sheet: &KmoSheet) -> (i32, i32, i32) {
    let mut normal = 0;
    let mut array = 0;
    let mut shared = 0;
    let mut row = 0;
    while row < 64 {
        let mut col = 0;
        while col < 32 {
            match sheet.getCellData(row, col).formulaType {
                1 => normal += 1,
                2 => array += 1,
                3 => shared += 1,
                _ => {}
            }
            col += 1;
        }
        row += 1;
    }
    (normal, array, shared)
}

#[test]
fn fml1_unknown_function_stores_name_ptg_and_func_var() {
    let book = KmoBook::KmoBook__parent(false);
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let opt = ParserOption::createOOXMLParserOption();
    let tokens = parse(
        "CONCAT(\"a\",\"b\")",
        &workbook,
        6,
        0,
        SpreadsheetVersion::EXCEL2007,
        &opt,
    )
    .expect("CONCAT is an external function");
    let mut bytes = Vec::new();
    for token in &tokens {
        token.write(&mut bytes);
    }
    // java FunctionDataHolder index 255 uses paramR: NamePtg class 0; FuncVarPtg index 255 includes the name argument.
    assert_eq!(
        bytes,
        vec![35, 1, 0, 0, 0, 23, 1, 0, 97, 23, 1, 0, 98, 66, 3, 255, 0]
    );
}

#[test]
fn fixed_function_uses_java_func_ptg_sid_33_and_roundtrips() {
    let book = KmoBook::KmoBook__parent(false);
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let opt = ParserOption::createOOXMLParserOption();
    let tokens = parse("ABS(2)", &workbook, 0, 0, SpreadsheetVersion::EXCEL2007, &opt).unwrap();
    let bytes: Vec<u8> = tokens.iter().flat_map(|token| { let mut out = Vec::new(); token.write(&mut out); out }).collect();
    // java IntPtg(2), FuncPtg(index 24) in RPN.
    assert_eq!(bytes, [30, 2, 0, 65, 24, 0]);
    let decoded = Ptg::readTokens(bytes.len() as i32, &bytes, SpreadsheetVersion::EXCEL2007);
    assert_eq!(decoded.len(), tokens.len());
    assert!(matches!(decoded.last().unwrap().kind, PtgKind::Func { meta } if meta.getIndex() == 24));
}

#[test]
fn mem_tokens_keep_java_field_widths_and_operand_classes() {
    // java MemErrPtg: sid 39 + int reserved + short subex_len.
    let mem_err = [71, 0x78, 0x56, 0x34, 0x12, 0xfe, 0xff];
    // java MemFuncPtg: sid 41 + ushort len_ref_subexpression.
    let mem_func = [41, 0x34, 0x12];
    for bytes in [&mem_err[..], &mem_func[..]] {
        let tokens = Ptg::readTokens(bytes.len() as i32, bytes, SpreadsheetVersion::EXCEL2007);
        assert_eq!(tokens.len(), 1);
        let mut out = Vec::new();
        tokens[0].write(&mut out);
        assert_eq!(out, bytes);
    }
}

#[test]
fn mem_area_reads_tail_after_all_plain_tokens() {
    // java MemAreaPtg.Initial = 7 bytes, followed by IntPtg; area count and 8-byte entry follow the complete token region.
    let bytes = [38, 0, 0, 0, 0, 3, 0, 30, 2, 0, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8];
    let tokens = Ptg::readTokens(10, &bytes, SpreadsheetVersion::EXCEL2007);
    assert_eq!(tokens.len(), 2);
    assert!(matches!(&tokens[0].kind, PtgKind::MemArea { areas: Some(a), .. } if a == &[1,2,3,4,5,6,7,8]));
    assert_eq!(getEncodedSizeWithoutArrayData(&tokens), 10);
    assert_eq!(serializePtgs(&tokens), bytes);
}

#[test]
fn array_ptg_reads_constant_tail_after_plain_tokens() {
    // java ArrayPtg.Initial(8) + IntPtg(3), then 2x1 ConstantValueParser values.
    let mut bytes = vec![32, 0, 0, 0, 0, 0, 0, 0, 30, 2, 0, 1, 0, 0];
    // java ConstantValueParser.encodeSingleValue(Double(2.5)); Boolean(true).
    bytes.push(1);
    bytes.extend_from_slice(&2.5f64.to_le_bytes());
    bytes.push(4);
    bytes.extend_from_slice(&1u64.to_le_bytes());
    let tokens = Ptg::readTokens(11, &bytes, SpreadsheetVersion::EXCEL2007);
    assert_eq!(tokens.len(), 2);
    assert!(matches!(&tokens[0].kind, PtgKind::Array { columns: 2, rows: 1, values, .. } if values.as_ref().unwrap().len() == 2));
    assert_eq!(getEncodedSizeWithoutArrayData(&tokens), 11);
    assert_eq!(serializePtgs(&tokens), bytes);
}

#[test]
fn array_ptg_constant_parser_preserves_unicode_error_and_empty() {
    // java ArrayPtg.Initial(8), 3x1 values: StringUtil Unicode, ErrorConstant(7), EMPTY_REPRESENTATION.
    let mut bytes = vec![32, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0];
    bytes.extend_from_slice(&[2, 1, 0, 1, 0x2d, 0x4e]);
    bytes.push(16);
    bytes.extend_from_slice(&7u64.to_le_bytes());
    bytes.push(0);
    bytes.extend_from_slice(&0u64.to_le_bytes());
    let tokens = Ptg::readTokens(8, &bytes, SpreadsheetVersion::EXCEL2007);
    assert_eq!(tokens.len(), 1);
    assert_eq!(serializePtgs(&tokens), bytes);
}

#[test]
fn fml2_cache_book_marks_only_parsed_formulas() {
    let (book, reader) = open_book(&fixture("formula-cache.xlsx"));
    assert_eq!(reader.delayedImportOrder(), vec![0]);
    assert_eq!(
        reader.formulaImportKinds(0),
        vec!["shared", "array", "normal"]
    );
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    let a1 = sheet.getCellData(0, 0);
    assert_eq!(a1.formulaType, 1);
    assert_eq!(sheet.getCellValueType__2(0, 0), 1);
    let b1 = sheet.getCellData(0, 1);
    assert_eq!(b1.formulaType, 1);
    assert!(KmoCellValueStore::isFormulaId(b1.id));
    let store = sheet.getBook().getCellValueStore();
    let mut record = KmoFormulaData::KmoFormulaData();
    store.borrow().formulaValue(b1.id, &mut record);
    assert_eq!(store.borrow_mut().stringValue(record.getCacheValueId()), "ab");
    let c1 = sheet.getCellData(0, 2);
    assert_eq!(c1.formulaType, 1);
    assert!(!c1.isBlank());
    assert_eq!(sheet.getCellValueType__2(0, 2), 0);
    assert_eq!(sheet.getExpFmlManager().borrow().shared_count(), 0);
}

#[test]
fn fml3_sample_normal_and_shared_types_follow_delayed_import() {
    let (book, reader) = open_book(&sample_book());
    let active = book.whichSheetIsActivated();
    let order = reader.delayedImportOrder();
    assert_eq!(order.first().copied(), Some(active));
    assert!(order.windows(2).all(|pair| pair[0] != pair[1]));
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    assert_eq!(sheet.getCellData(3, 4).formulaType, 1);
    assert_eq!(sheet.getCellData(10, 7).formulaType, 3);
    let shared_cell = sheet.getCellData(10, 7);
    let store = sheet.getBook().getCellValueStore();
    let mut record = KmoFormulaData::KmoFormulaData();
    store.borrow().formulaValue(shared_cell.id, &mut record);
    // java Exp03Ptg：sid 1，主格 H10 行 9 列 7，Excel97 各 2 字节
    assert_eq!(
        store.borrow().formulaTokens(record.getTokensId()),
        vec![1, 9, 0, 7, 0]
    );
    assert_eq!(record.getTokensPlainSize(), 5);
    let (normal, array, shared) = count_formula_types(&sheet);
    assert_eq!(array, 0);
    assert_eq!(normal, 28);
    assert_eq!(shared, 25);
    assert!(sheet.getExpFmlManager().borrow().shared_count() >= 1);
    assert_eq!(
        reader.formulaImportKinds(0),
        vec!["shared", "array", "normal"]
    );
}

#[test]
fn fml4_formula_text_does_not_set_type_until_parse_stores_value() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    let mut qcell = QuickInitCell::QuickInitCell__sheet(&sheet);
    qcell.setRKCellData(0, 0, 0, 2.0);
    qcell.setLabelCellData(0, 1, 0, Some("ab"), false);
    qcell.setRKCellData(0, 2, 0, 3.0);
    let backup = CellFmlBackupHandler::CellFmlBackupHandler();
    backup.addNormal(NormalFml {
        row: 0,
        col: 0,
        fml: "1+1".to_string(),
    });
    backup.addNormal(NormalFml {
        row: 0,
        col: 1,
        fml: "CONCAT(\"a\",\"b\")".to_string(),
    });
    backup.addNormal(NormalFml {
        row: 0,
        col: 2,
        fml: "1+".to_string(),
    });
    assert_eq!(sheet.getCellData(0, 0).formulaType, 0);
    assert_eq!(sheet.getCellData(0, 1).formulaType, 0);
    backup.delayImportAllNormalFml(&sheet);
    let parsed = sheet.getCellData(0, 0);
    assert_eq!(parsed.formulaType, 1);
    assert!(KmoCellValueStore::isFormulaId(parsed.id));
    let store = sheet.getBook().getCellValueStore();
    let mut record = KmoFormulaData::KmoFormulaData();
    store.borrow().formulaValue(parsed.id, &mut record);
    let tokens = store.borrow().formulaTokens(record.getTokensId());
    // java IntPtg 30 + ushort 1，AddPtg sid 3
    assert_eq!(tokens, vec![30, 1, 0, 30, 1, 0, 3]);
    assert_eq!(record.getTokensPlainSize(), 7);
    let cache = store.borrow().formulaCachaId(parsed.id);
    assert_eq!(store.borrow_mut().doubleValue(cache), 2.0);
    let parsed_name = sheet.getCellData(0, 1);
    assert_eq!(parsed_name.formulaType, 1);
    assert!(KmoCellValueStore::isFormulaId(parsed_name.id));
    assert_eq!(
        stored_tokens(&sheet, 0, 1).0,
        vec![35, 1, 0, 0, 0, 23, 1, 0, 97, 23, 1, 0, 98, 66, 3, 255, 0]
    );
    let number = sheet.getCellData(0, 2);
    assert_eq!(number.formulaType, 0);
    assert!(!KmoCellValueStore::isFormulaId(number.id));
    assert_eq!(store.borrow_mut().doubleValue(number.id), 3.0);
}

fn stored_tokens(sheet: &KmoSheet, row: i32, col: i32) -> (Vec<u8>, i32) {
    let cell = sheet.getCellData(row, col);
    let store = sheet.getBook().getCellValueStore();
    let mut record = KmoFormulaData::KmoFormulaData();
    store.borrow().formulaValue(cell.id, &mut record);
    let tokens = store.borrow().formulaTokens(record.getTokensId());
    let plain = record.getTokensPlainSize();
    (tokens, plain)
}

struct AttrValue {
    name: i32,
    value: String,
}

impl XmlRoAttrValue for AttrValue {
    fn to_string(&self) -> String {
        self.value.clone()
    }
    fn to_int(&self) -> i32 {
        self.value.parse().unwrap_or(0)
    }
    fn to_byte(&self) -> i8 {
        self.to_int() as i8
    }
    fn to_short(&self) -> i16 {
        self.to_int() as i16
    }
    fn to_boolean(&self) -> bool {
        matches!(self.value.as_str(), "1" | "true")
    }
    fn to_float(&self) -> f32 {
        self.value.parse().unwrap_or(0.0)
    }
    fn to_double(&self) -> f64 {
        self.value.parse().unwrap_or(0.0)
    }
    fn to_long(&self) -> i64 {
        self.value.parse().unwrap_or(0)
    }
    fn to_percentage(&self) -> f64 {
        self.to_double()
    }
}

impl XmlRoAttr for AttrValue {
    fn get_by_name(&self, _: i32) -> Option<&dyn XmlRoAttr> {
        None
    }
    fn get_attr_at(&self, _: i32) -> Option<&dyn XmlRoAttr> {
        None
    }
    fn get_name_at(&self, _: i32) -> i32 {
        0
    }
    fn count(&self) -> i32 {
        0
    }
    fn get_characters(&self) -> String {
        String::new()
    }
    fn get_name(&self) -> i32 {
        self.name
    }
    fn get_by_name_with_flag(&self, _: i32, _: bool) -> Option<&dyn XmlRoAttr> {
        None
    }
    fn is_leaf(&self) -> bool {
        true
    }
}

struct NamedAttrs {
    items: Vec<AttrValue>,
}

impl NamedAttrs {
    fn new(pairs: &[(i32, &str)]) -> Self {
        Self {
            items: pairs
                .iter()
                .map(|(name, value)| AttrValue {
                    name: *name,
                    value: (*value).to_string(),
                })
                .collect(),
        }
    }
}

impl XmlRoAttrValue for NamedAttrs {
    fn to_string(&self) -> String {
        String::new()
    }
    fn to_int(&self) -> i32 {
        0
    }
    fn to_byte(&self) -> i8 {
        0
    }
    fn to_short(&self) -> i16 {
        0
    }
    fn to_boolean(&self) -> bool {
        false
    }
    fn to_float(&self) -> f32 {
        0.0
    }
    fn to_double(&self) -> f64 {
        0.0
    }
    fn to_long(&self) -> i64 {
        0
    }
    fn to_percentage(&self) -> f64 {
        0.0
    }
}

impl XmlRoAttr for NamedAttrs {
    fn get_by_name(&self, name: i32) -> Option<&dyn XmlRoAttr> {
        self.items
            .iter()
            .find(|item| item.name == name)
            .map(|item| item as &dyn XmlRoAttr)
    }
    fn get_attr_at(&self, index: i32) -> Option<&dyn XmlRoAttr> {
        self.items
            .get(index as usize)
            .map(|item| item as &dyn XmlRoAttr)
    }
    fn get_name_at(&self, index: i32) -> i32 {
        self.items
            .get(index as usize)
            .map(|item| item.name)
            .unwrap_or(0)
    }
    fn count(&self) -> i32 {
        self.items.len() as i32
    }
    fn get_characters(&self) -> String {
        String::new()
    }
    fn get_name(&self) -> i32 {
        0
    }
    fn get_by_name_with_flag(&self, name: i32, _: bool) -> Option<&dyn XmlRoAttr> {
        self.get_by_name(name)
    }
    fn is_leaf(&self) -> bool {
        false
    }
}

fn feed_f(handler: &CellFmlBackupHandler, row: i32, col: i32, attrs: &NamedAttrs, text: &str) {
    handler.init(row, col);
    handler.startElement(4359, attrs);
    if !text.is_empty() {
        handler.characters(text);
    }
}

#[test]
fn fml5_product_open_stores_parsed_tokens_not_f_marks() {
    let api = KmoBookApi::KmoBookApi();
    assert_eq!(api.open(&sample_book(), None, true), 1);
    let book = api.book();
    assert_eq!(book.getSpreadsheetVersion(), SpreadsheetVersion::EXCEL2007);
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    let (e4, e4_len) = stored_tokens(&sheet, 3, 4);
    // java IF(C4="",SUM($F$8:$F$2000),SUMIF(...))，Excel2007 引用宽 7/区域宽 13。
    // 条件 11，真分支 17，attrIf=21；假分支 37，skip=44。
    let expected = vec![
        68, 3, 0, 0, 0, 2, 192, 23, 0, 0, 11, 25, 2, 21, 0, 37, 7, 0, 0, 0, 207, 7, 0, 0, 5, 0,
        5, 0, 25, 16, 0, 0, 25, 8, 44, 0, 37, 7, 0, 0, 0, 207, 7, 0, 0, 2, 0, 2, 0, 68, 3, 0, 0,
        0, 2, 192, 37, 7, 0, 0, 0, 207, 7, 0, 0, 5, 0, 5, 0, 66, 3, 89, 1, 25, 8, 3, 0, 66, 3, 1,
        0,
    ];
    assert_eq!(e4, expected);
    assert_eq!(e4_len, expected.len() as i32);
    let (diff, diff_len) = stored_tokens(&sheet, 3, 12);
    assert_eq!(
        diff,
        vec![68, 3, 0, 0, 0, 4, 192, 68, 3, 0, 0, 0, 8, 192, 4]
    );
    assert_eq!(diff_len, 15);
    let shared = sheet
        .getExpFmlManager()
        .borrow()
        .getMatchedSharedFmls(9, 7);
    assert_eq!(shared.len(), 1);
    // java IF 的第二个参数是真分支。replaceWithNPtgs 后相对引用变成 RefN。
    // 真分支空串 3 字节，attrIf=7；假分支 F10*G10 为 15 字节，skip=22。
    assert_eq!(
        shared[0].getEncodedTokens(),
        vec![
            76, 0, 0, 0, 0, 251, 255, 23, 0, 0, 11, 25, 2, 7, 0, 23, 0, 0, 25, 8, 22, 0, 76, 0, 0,
            0, 0, 254, 255, 76, 0, 0, 0, 0, 255, 255, 5, 25, 8, 3, 0, 66, 3, 1, 0,
        ]
    );
    let (member, member_len) = stored_tokens(&sheet, 10, 7);
    assert_eq!(member, vec![1, 9, 0, 0, 0, 7, 0]);
    assert_eq!(member_len, 7);
    assert_eq!(sheet.getCellData(10, 7).formulaType, 3);
    assert!(!sheet.getCellData(3, 4).isPicInCell);
    assert!(!sheet.getCellData(10, 7).isPicInCell);
}

#[test]
fn fml9_dispimg_ref_stores_formula_and_continues() {
    let book = KmoBook::KmoBook__parent(false);
    book.setSpreadsheetVersion(SpreadsheetVersion::EXCEL2007);
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    let mut qcell = QuickInitCell::QuickInitCell__sheet(&sheet);
    qcell.setLabelCellData(0, 0, 0, Some("=DISPIMG(\"id\",1)"), false);
    qcell.setRKCellData(0, 1, 0, 2.0);
    qcell.setRKCellData(0, 2, 0, 3.0);
    let backup = CellFmlBackupHandler::CellFmlBackupHandler();
    backup.addNormal(NormalFml {
        row: 0,
        col: 1,
        fml: "A1".to_string(),
    });
    backup.addNormal(NormalFml {
        row: 0,
        col: 2,
        fml: "1+1".to_string(),
    });
    backup.delayImportAllNormalFml(&sheet);
    let b1 = sheet.getCellData(0, 1);
    assert_eq!(b1.formulaType, 1);
    assert!(b1.isPicInCell);
    assert_eq!(sheet.getCellData(0, 2).formulaType, 1);
    assert_eq!(stored_tokens(&sheet, 0, 2).0, vec![30, 1, 0, 30, 1, 0, 3]);
}

#[test]
fn fml6_shared_unknown_function_stores_exp_and_cache() {
    let book = KmoBook::KmoBook__parent(false);
    book.setSpreadsheetVersion(SpreadsheetVersion::EXCEL2007);
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    let mut qcell = QuickInitCell::QuickInitCell__sheet(&sheet);
    qcell.setLabelCellData(0, 0, 0, Some("ab"), false);
    qcell.setRKCellData(0, 1, 0, 3.0);
    let backup = CellFmlBackupHandler::CellFmlBackupHandler();
    feed_f(
        &backup,
        0,
        0,
        &NamedAttrs::new(&[(4155, "shared"), (4649, "0"), (4100, "A1:B1")]),
        "CONCAT(\"a\",\"b\")",
    );
    feed_f(
        &backup,
        0,
        1,
        &NamedAttrs::new(&[(4155, "shared"), (4649, "0")]),
        "",
    );
    assert_eq!(sheet.getCellData(0, 0).formulaType, 0);
    backup.delayImportAllSharedFml(&sheet);
    let a1 = sheet.getCellData(0, 0);
    let b1 = sheet.getCellData(0, 1);
    assert_eq!(a1.formulaType, 3);
    assert_eq!(b1.formulaType, 3);
    assert!(KmoCellValueStore::isFormulaId(a1.id));
    assert!(KmoCellValueStore::isFormulaId(b1.id));
    let store = sheet.getBook().getCellValueStore();
    let mut record = KmoFormulaData::KmoFormulaData();
    store.borrow().formulaValue(a1.id, &mut record);
    assert_eq!(store.borrow_mut().stringValue(record.getCacheValueId()), "ab");
    store.borrow().formulaValue(b1.id, &mut record);
    assert_eq!(store.borrow_mut().doubleValue(record.getCacheValueId()), 3.0);
    assert_eq!(sheet.getExpFmlManager().borrow().shared_count(), 1);
}

#[test]
fn fml7_array_parse_stores_exp_and_failure_leaves_type() {
    let book = KmoBook::KmoBook__parent(false);
    book.setSpreadsheetVersion(SpreadsheetVersion::EXCEL2007);
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    let mut qcell = QuickInitCell::QuickInitCell__sheet(&sheet);
    qcell.setRKCellData(0, 0, 0, 2.0);
    let backup = CellFmlBackupHandler::CellFmlBackupHandler();
    feed_f(
        &backup,
        0,
        0,
        &NamedAttrs::new(&[(4155, "array"), (4100, "A1:B1")]),
        "1+1",
    );
    backup.delayImportAllArrayFml(&sheet);
    assert_eq!(sheet.getCellData(0, 0).formulaType, 2);
    assert_eq!(sheet.getCellData(0, 1).formulaType, 2);
    let (a1, a1_len) = stored_tokens(&sheet, 0, 0);
    assert_eq!(a1, vec![1, 0, 0, 0, 0, 0, 0]);
    assert_eq!(a1_len, 7);
    let master = sheet.getExpFmlManager().borrow().query(0, 0, false);
    let master = master.expect("array formula");
    assert_eq!(master.getEncodedTokens(), vec![30, 1, 0, 30, 1, 0, 3]);
    let cache = sheet
        .getBook()
        .getCellValueStore()
        .borrow()
        .formulaCachaId(sheet.getCellData(0, 0).id);
    assert_eq!(
        sheet.getBook().getCellValueStore().borrow_mut().doubleValue(cache),
        2.0
    );

    let book = KmoBook::KmoBook__parent(false);
    book.setSpreadsheetVersion(SpreadsheetVersion::EXCEL2007);
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    let mut qcell = QuickInitCell::QuickInitCell__sheet(&sheet);
    qcell.setLabelCellData(1, 0, 0, Some("ab"), false);
    qcell.setRKCellData(1, 1, 0, 4.0);
    let backup = CellFmlBackupHandler::CellFmlBackupHandler();
    feed_f(
        &backup,
        1,
        0,
        &NamedAttrs::new(&[(4155, "array"), (4100, "A2:B2")]),
        "CONCAT(\"a\",\"b\")",
    );
    backup.delayImportAllArrayFml(&sheet);
    assert_eq!(sheet.getCellData(1, 0).formulaType, 2);
    assert_eq!(sheet.getCellData(1, 1).formulaType, 2);
    assert!(KmoCellValueStore::isFormulaId(sheet.getCellData(1, 0).id));
    let master = sheet.getExpFmlManager().borrow().query(1, 0, false);
    let master = master.expect("array formula");
    assert_eq!(
        master.getEncodedTokens(),
        vec![35, 1, 0, 0, 0, 23, 1, 0, 97, 23, 1, 0, 98, 66, 3, 255, 0]
    );
    let store = sheet.getBook().getCellValueStore();
    let mut record = KmoFormulaData::KmoFormulaData();
    store.borrow().formulaValue(sheet.getCellData(1, 1).id, &mut record);
    assert_eq!(store.borrow_mut().doubleValue(record.getCacheValueId()), 4.0);
}

#[test]
fn fml8_null_shared_range_stops_the_import() {
    let book = KmoBook::KmoBook__parent(false);
    book.setSpreadsheetVersion(SpreadsheetVersion::EXCEL2007);
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    let mut qcell = QuickInitCell::QuickInitCell__sheet(&sheet);
    qcell.setRKCellData(0, 0, 0, 1.0);
    qcell.setRKCellData(1, 0, 0, 2.0);
    let backup = CellFmlBackupHandler::CellFmlBackupHandler();
    feed_f(
        &backup,
        0,
        0,
        &NamedAttrs::new(&[(4155, "shared"), (4649, "0")]),
        "1+1",
    );
    feed_f(
        &backup,
        1,
        0,
        &NamedAttrs::new(&[(4155, "shared"), (4649, "1"), (4100, "A2")]),
        "1+1",
    );
    backup.delayImportAllSharedFml(&sheet);
    assert_eq!(sheet.getCellData(0, 0).formulaType, 0);
    assert_eq!(sheet.getCellData(1, 0).formulaType, 0);
    assert!(!KmoCellValueStore::isFormulaId(sheet.getCellData(1, 0).id));
}

#[test]
fn fml10_convert_keeps_in_range_and_rewrites_out_of_range() {
    let inside = ref_ptg(3, 2, true, true, SpreadsheetVersion::EXCEL2007);
    let kept = FormulaHelper::convertFromExcel2007ToExcel97(std::slice::from_ref(&inside), 0, true);
    assert_eq!(kept[0].getSid(), 36);
    assert_eq!(kept[0].getSize(), 7);

    let excel2007_only = ref_ptg(70_000, 2, true, true, SpreadsheetVersion::EXCEL2007);
    let still = FormulaHelper::convertFromExcel2007ToExcel97(std::slice::from_ref(&excel2007_only), 0, true);
    assert_eq!(still[0].getSid(), 36);
    let as97 = FormulaHelper::convertFromExcel2007ToExcel97(std::slice::from_ref(&excel2007_only), 0, false);
    assert_eq!(as97[0].getSid(), 42);
    assert_eq!(as97[0].getSize(), 5);

    let beyond = ref_ptg(1_048_576, 0, false, false, SpreadsheetVersion::EXCEL2007);
    let err = FormulaHelper::convertFromExcel2007ToExcel97(std::slice::from_ref(&beyond), 0, true);
    assert_eq!(err[0].getSid(), 42);

    let full = area_ptg(
        0,
        1_048_575,
        0,
        16_383,
        false,
        false,
        false,
        false,
        SpreadsheetVersion::EXCEL2007,
    );
    let shrunk = FormulaHelper::convertFromExcel2007ToExcel97(std::slice::from_ref(&full), 3, false);
    match &shrunk[0].kind {
        PtgKind::Area {
            last_row,
            last_col_raw,
            ..
        } => {
            assert_eq!(*last_row, 65_535);
            assert_eq!(last_col_raw & 0x3fff, 255);
        }
        _ => panic!("full sheet should stay an area"),
    }

    let tall = area_ptg(
        0,
        70_000,
        0,
        2,
        false,
        false,
        false,
        false,
        SpreadsheetVersion::EXCEL2007,
    );
    let area_err = FormulaHelper::convertFromExcel2007ToExcel97(std::slice::from_ref(&tall), 3, false);
    assert_eq!(area_err[0].getSid(), 43);
    assert_eq!(area_err[0].getSize(), 9);

    let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        FormulaHelper::convertFromExcel2007ToExcel97(std::slice::from_ref(&inside), 1, true);
    }));
    assert!(rejected.is_err());
}

#[test]
fn fml11_comparison_power_percent_and_unary_store_java_sids() {
    let book = KmoBook::KmoBook__parent(false);
    book.setSpreadsheetVersion(SpreadsheetVersion::EXCEL2007);
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    let mut qcell = QuickInitCell::QuickInitCell__sheet(&sheet);
    let formulas = [
        "A1<B1",
        "A1<=B1",
        "A1>B1",
        "A1>=B1",
        "A1<>B1",
        "1^2",
        "2^3^2",
        "1%",
        "-1",
        "-A1",
        "-1%",
        "+1",
        "+A1",
        "2×3",
        "A1≤B1",
        "1〓1",
        "1<",
    ];
    for (col, _) in formulas.iter().enumerate() {
        qcell.setRKCellData(0, col as i32, 0, 1.0);
    }
    let backup = CellFmlBackupHandler::CellFmlBackupHandler();
    for (col, text) in formulas.iter().enumerate() {
        backup.addNormal(NormalFml {
            row: 0,
            col: col as i32,
            fml: (*text).to_string(),
        });
    }
    assert_eq!(sheet.getCellData(0, 0).formulaType, 0);
    backup.delayImportAllNormalFml(&sheet);

    let a1 = vec![68, 0, 0, 0, 0, 0, 192];
    let b1 = vec![68, 0, 0, 0, 0, 1, 192];
    let pair = |sid: u8| {
        let mut bytes = a1.clone();
        bytes.extend(b1.clone());
        bytes.push(sid);
        bytes
    };
    assert_eq!(stored_tokens(&sheet, 0, 0).0, pair(9));
    assert_eq!(sheet.getCellData(0, 0).formulaType, 1);
    assert_eq!(stored_tokens(&sheet, 0, 1).0, pair(10));
    assert_eq!(stored_tokens(&sheet, 0, 2).0, pair(13));
    assert_eq!(stored_tokens(&sheet, 0, 3).0, pair(12));
    assert_eq!(stored_tokens(&sheet, 0, 4).0, pair(14));
    assert_eq!(stored_tokens(&sheet, 0, 5).0, vec![30, 1, 0, 30, 2, 0, 7]);
    assert_eq!(
        stored_tokens(&sheet, 0, 6).0,
        vec![30, 2, 0, 30, 3, 0, 7, 30, 2, 0, 7]
    );
    assert_eq!(stored_tokens(&sheet, 0, 7).0, vec![30, 1, 0, 20]);
    let mut neg = vec![31];
    neg.extend((-1.0f64).to_le_bytes());
    assert_eq!(stored_tokens(&sheet, 0, 8).0, neg);
    let mut neg_ref = a1.clone();
    neg_ref.push(19);
    assert_eq!(stored_tokens(&sheet, 0, 9).0, neg_ref);
    assert_eq!(stored_tokens(&sheet, 0, 10).0, vec![30, 1, 0, 20, 19]);
    assert_eq!(stored_tokens(&sheet, 0, 11).0, vec![30, 1, 0]);
    let mut plus_ref = a1.clone();
    plus_ref.push(18);
    assert_eq!(stored_tokens(&sheet, 0, 12).0, plus_ref);
    assert_eq!(stored_tokens(&sheet, 0, 13).0, vec![30, 2, 0, 30, 3, 0, 5]);
    assert_eq!(stored_tokens(&sheet, 0, 14).0, pair(10));
    assert_eq!(stored_tokens(&sheet, 0, 15).0, vec![30, 1, 0, 30, 1, 0, 11]);
    assert_eq!(sheet.getCellData(0, 16).formulaType, 0);
    assert!(!KmoCellValueStore::isFormulaId(sheet.getCellData(0, 16).id));
}

#[test]
fn fml12_boolean_and_error_literals_store_scalar_ptgs() {
    let book = KmoBook::KmoBook__parent(false);
    book.setSpreadsheetVersion(SpreadsheetVersion::EXCEL2007);
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    let mut qcell = QuickInitCell::QuickInitCell__sheet(&sheet);
    let formulas = [
        "TRUE", "FALSE", "#DIV/0!", "#N/A", "#VALUE!", "#REF!", "#NAME?", "#NUM!", "#NULL!", "#FOO",
    ];
    for (col, _) in formulas.iter().enumerate() {
        qcell.setRKCellData(0, col as i32, 0, 1.0);
    }
    let backup = CellFmlBackupHandler::CellFmlBackupHandler();
    for (col, text) in formulas.iter().enumerate() {
        backup.addNormal(NormalFml {
            row: 0,
            col: col as i32,
            fml: (*text).to_string(),
        });
    }
    backup.delayImportAllNormalFml(&sheet);
    assert_eq!(stored_tokens(&sheet, 0, 0).0, vec![29, 1]);
    assert_eq!(sheet.getCellData(0, 0).formulaType, 1);
    assert_eq!(stored_tokens(&sheet, 0, 1).0, vec![29, 0]);
    assert_eq!(stored_tokens(&sheet, 0, 2).0, vec![28, 7]);
    assert_eq!(stored_tokens(&sheet, 0, 3).0, vec![28, 42]);
    assert_eq!(stored_tokens(&sheet, 0, 4).0, vec![28, 15]);
    assert_eq!(stored_tokens(&sheet, 0, 5).0, vec![28, 23]);
    assert_eq!(stored_tokens(&sheet, 0, 6).0, vec![28, 29]);
    assert_eq!(stored_tokens(&sheet, 0, 7).0, vec![28, 36]);
    assert_eq!(stored_tokens(&sheet, 0, 8).0, vec![28, 0]);
    assert_eq!(sheet.getCellData(0, 9).formulaType, 0);
    assert!(!KmoCellValueStore::isFormulaId(sheet.getCellData(0, 9).id));
}

#[test]
fn fml13_sheet_ref_stores_ref3d_unknown_sheet_does_not_mark_type() {
    let book = KmoBook::KmoBook__parent(false);
    book.setSpreadsheetVersion(SpreadsheetVersion::EXCEL2007);
    let main = book.appendSheet__2(Some("Main".to_string()));
    let src = book.appendSheet__2(Some("Src".to_string()));
    {
        let src = src.borrow();
        let mut qcell = QuickInitCell::QuickInitCell__sheet(&src);
        qcell.setLabelCellData(0, 0, 0, Some("=DISPIMG(\"id\",1)"), false);
        qcell.setRKCellData(0, 1, 0, 5.0);
    }
    let main = main.borrow();
    let mut qcell = QuickInitCell::QuickInitCell__sheet(&main);
    qcell.setRKCellData(0, 0, 0, 2.0);
    qcell.setRKCellData(0, 1, 0, 6.0);
    qcell.setLabelCellData(0, 2, 0, Some("ab"), false);
    qcell.setRKCellData(0, 3, 0, 3.0);
    qcell.setRKCellData(0, 4, 0, 4.0);
    let backup = CellFmlBackupHandler::CellFmlBackupHandler();
    backup.addNormal(NormalFml {
        row: 0,
        col: 0,
        fml: "Src!B1".to_string(),
    });
    backup.addNormal(NormalFml {
        row: 0,
        col: 1,
        fml: "'Src'!$A$1:$B$2".to_string(),
    });
    backup.addNormal(NormalFml {
        row: 0,
        col: 2,
        fml: "Missing!A1".to_string(),
    });
    backup.addNormal(NormalFml {
        row: 0,
        col: 3,
        fml: "Src!A1".to_string(),
    });
    backup.addNormal(NormalFml {
        row: 0,
        col: 4,
        fml: "1+1".to_string(),
    });
    assert_eq!(main.getCellData(0, 0).formulaType, 0);
    assert_eq!(main.getCellData(0, 2).formulaType, 0);
    backup.delayImportAllNormalFml(&main);
    assert_eq!(main.getCellData(0, 0).formulaType, 1);
    // java Ref3D10：sid 58 + value class 32，extern index 0，B1 相对。
    assert_eq!(
        stored_tokens(&main, 0, 0).0,
        vec![90, 0, 0, 0, 0, 0, 0, 1, 0xC0]
    );
    let store = main.getBook().getCellValueStore();
    let mut record = KmoFormulaData::KmoFormulaData();
    store
        .borrow()
        .formulaValue(main.getCellData(0, 0).id, &mut record);
    assert_eq!(store.borrow_mut().doubleValue(record.getCacheValueId()), 2.0);
    assert_eq!(main.getCellData(0, 1).formulaType, 1);
    // java Area3D10：sid 59 + value class 32，$A$1:$B$2 绝对。
    assert_eq!(
        stored_tokens(&main, 0, 1).0,
        vec![91, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0]
    );
    let failed = main.getCellData(0, 2);
    assert_eq!(failed.formulaType, 0);
    assert!(!KmoCellValueStore::isFormulaId(failed.id));
    assert_eq!(store.borrow_mut().stringValue(failed.id), "ab");
    let disp = main.getCellData(0, 3);
    assert_eq!(disp.formulaType, 1);
    assert!(disp.isPicInCell);
    store.borrow().formulaValue(disp.id, &mut record);
    assert_eq!(store.borrow_mut().doubleValue(record.getCacheValueId()), 3.0);
    assert_eq!(main.getCellData(0, 4).formulaType, 1);
    assert_eq!(stored_tokens(&main, 0, 4).0, vec![30, 1, 0, 30, 1, 0, 3]);
    store.borrow().formulaValue(main.getCellData(0, 4).id, &mut record);
    assert_eq!(store.borrow_mut().doubleValue(record.getCacheValueId()), 4.0);
}

#[test]
fn fml14_defined_name_stores_name_ptg_missing_name_keeps_type() {
    use ss_core::cn::wps::et::ss::formula::ptg::Ptg::int_ptg;
    use ss_core::cn::wps::moss::engine::linktb::NameRecord::NameRecord;

    let book = KmoBook::KmoBook__parent(false);
    book.setSpreadsheetVersion(SpreadsheetVersion::EXCEL2007);
    let mut named = NameRecord::NameRecord(SpreadsheetVersion::EXCEL2007);
    named.setNameText("Total");
    named.setNameDefinition(vec![int_ptg(1)]);
    book.getLinkTable().borrow_mut().addName(named);
    let mut skipped = NameRecord::NameRecord(SpreadsheetVersion::EXCEL2007);
    skipped.setNameText("Blank");
    book.getLinkTable().borrow_mut().addName(skipped);
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    let mut qcell = QuickInitCell::QuickInitCell__sheet(&sheet);
    qcell.setRKCellData(0, 0, 0, 2.0);
    qcell.setLabelCellData(0, 1, 0, Some("ab"), false);
    let backup = CellFmlBackupHandler::CellFmlBackupHandler();
    backup.addNormal(NormalFml {
        row: 0,
        col: 0,
        fml: "Total".to_string(),
    });
    backup.addNormal(NormalFml {
        row: 0,
        col: 1,
        fml: "Missing".to_string(),
    });
    assert_eq!(sheet.getCellData(0, 0).formulaType, 0);
    backup.delayImportAllNormalFml(&sheet);
    assert_eq!(sheet.getCellData(0, 0).formulaType, 1);
    assert_eq!(stored_tokens(&sheet, 0, 0).0, vec![67, 1, 0, 0, 0]);
    let missed = sheet.getCellData(0, 1);
    assert_eq!(missed.formulaType, 0);
    assert!(!KmoCellValueStore::isFormulaId(missed.id));
    assert_eq!(
        sheet.getBook().getCellValueStore().borrow_mut().stringValue(missed.id),
        "ab"
    );
}
