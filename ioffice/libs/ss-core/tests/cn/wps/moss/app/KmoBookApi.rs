// Java ColorScheme and XlsxrStyles keep process-wide static read state; these book-open cases run serially.
static OPEN_SERIAL:std::sync::Mutex<()>=std::sync::Mutex::new(());
use ss_core::cn::wps::moss::app::KmoBookApi::KmoBookApi;
use ss_core::cn::wps::moss::app::KmoSheet::KmoSheet;
use ss_core::cn::wps::moss::engine::eval::EvaluationPresent::EvaluationPresent;
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;
use ss_core::cn::wps::et::ss::formula::ptg::ExpPtg::ExpPtg;
use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{Ptg, serializePtgs};
use ss_core::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;
use ss_core::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use ss_core::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
use ss_core::api::SpreadsheetSession::SpreadsheetSession;
use ss_core::api::SpreadsheetSession::SpreadsheetLoadConfig;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn number(sheet: &KmoSheet, row: i32, col: i32) -> f64 {
    let data = sheet.getCellData(row, col);
    let id = if KmoCellValueStore::isFormulaId(data.id) {
        sheet.getCellValueStore().borrow().formulaCachaId(data.id)
    } else {
        data.id
    };
    sheet.getCellValueStore().borrow_mut().doubleValue(id)
}

#[test]
fn open1_basic_types_returns_success_and_cells() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    let code = api.open(&fixture("basic-types.xlsx"), None, true);
    assert_eq!(code, 1);
    let book = api.book();
    assert!(book.isDirty());
    assert!(!book.getReadOnly());
    assert_eq!(book.getFileFormat(), 1);
    assert_eq!(book.getSpreadsheetVersion(), SpreadsheetVersion::EXCEL2007);
    assert_eq!(book.sheetCount(), 1);
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    assert_eq!(sheet.name(), "Sheet1");
    assert_eq!(number(&sheet, 0, 0), 1.5);
    assert_eq!(sheet.getCellValueType__2(0, 1), 5);
    assert_eq!(
        sheet
            .getCellValueStore()
            .borrow_mut()
            .stringValue(sheet.getCellData(1, 0).id),
        "中文"
    );
}

#[test]
fn open2_sparse_sheets_keep_their_own_values() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&fixture("sparse-multisheet.xlsx"), None, true), 1);
    let book = api.book();
    assert_eq!(book.sheetCount(), 3);
    assert_eq!(book.sheetAt(0).borrow().getSheetIndex(), 0);
    assert_eq!(book.sheetAt(1).borrow().getSheetIndex(), 1);
    assert_eq!(book.sheetAt(2).borrow().getSheetIndex(), 2);
    assert_eq!(number(&book.sheetAt(0).borrow(), 0, 0), 1.0);
    assert_eq!(number(&book.sheetAt(1).borrow(), 0, 0), 2.0);
    assert!(book.sheetAt(2).borrow().getCellData(0, 0).isNull());
    assert_eq!(number(&book.sheetAt(2).borrow(), 1, 1), 3.0);
}

#[test]
fn open3_formula_without_cache_is_blank() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&fixture("formula-cache.xlsx"), None, true), 1);
    let sheet = api.book().sheetAt(0);
    let sheet = sheet.borrow();
    let a1 = sheet.getCellData(0, 0);
    assert_eq!(a1.formulaType, 1);
    assert_eq!(number(&sheet, 0, 0), 2.0);
    let b1 = sheet.getCellData(0, 1);
    assert_eq!(b1.formulaType, 1);
    let b1_cache_id = sheet.getCellValueId(0, 1);
    assert_eq!(
        sheet
            .getCellValueStore()
            .borrow_mut()
            .stringValue(b1_cache_id),
        "ab"
    );
    let c1 = sheet.getCellData(0, 2);
    assert_eq!(c1.formulaType, 1);
    assert!(!c1.isBlank());
    assert_eq!(sheet.getCellValueType__2(0, 2), 0);
}

#[test]
fn open3_formula_flag_and_calc_result_use_loaded_formula_record() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&fixture("formula-cache.xlsx"), None, true), 1);
    let sheet = api.book().sheetAt(0);
    let sheet = sheet.borrow();
    let mut cell = KmoCellData::default();
    let mut formula = KmoFormulaData::KmoFormulaData();
    EvaluationPresent::loadBuffer(&sheet, 0, 2, &mut cell, &mut formula);
    assert_eq!(cell.formulaType, 1);
    let tokens_id = formula.getTokensId();
    assert!(tokens_id > 0);
    assert_eq!(formula.getCacheValueId(), 0);
    assert_eq!(formula.getCalcFlags(), 0);

    let store = sheet.getCellValueStore();
    store.borrow_mut().setFormulaCalcFlag(cell.id, 0b101);
    assert!(EvaluationPresent::isNeedRecalculationFlagSet(&sheet, 0, 2));
    EvaluationPresent::setNeedRecalculationFlag(&sheet, 0, 2, false);
    assert_eq!(store.borrow().getFormulaCalcFlag(cell.id), 0b100);
    EvaluationPresent::setNeedRecalculationFlag(&sheet, 0, 2, true);
    assert_eq!(store.borrow().getFormulaCalcFlag(cell.id), 0b101);
    assert!(EvaluationPresent::isNeedRecalculationFlagSet(&sheet, 40, 40));

    let error_id = store.borrow_mut().newErrorValue(7);
    formula.setCacheValueId(error_id);
    EvaluationPresent::storeBuffer(&sheet, 0, 2, &mut cell, &formula);
    let mut reread_cell = KmoCellData::default();
    let mut reread_formula = KmoFormulaData::KmoFormulaData();
    EvaluationPresent::loadBuffer(&sheet, 0, 2, &mut reread_cell, &mut reread_formula);
    assert_eq!(reread_cell.id, cell.id);
    assert_eq!(reread_formula.getTokensId(), tokens_id);
    assert_eq!(reread_formula.getCacheValueId(), error_id);
    assert_eq!(sheet.getCellValueType__2(0, 2), 6);
}

#[test]
fn open3_formula_registration_iterator_reads_real_fixture_in_row_order() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&fixture("formula-cache.xlsx"), None, true), 1);
    let sheet = api.book().sheetAt(0);
    let sheet = sheet.borrow();
    let mut iter = sheet.getCellIterator(0, 1_048_575, 0, 16_383);
    let mut formulas = Vec::new();
    while iter.hasNext() {
        iter.next();
        let row = iter.row();
        let col = iter.col();
        let data = iter.cellData();
        if KmoCellValueStore::identifyTypeByID(data.id) == 7 {
            formulas.push((row, col, data.formulaType));
        }
    }
    assert_eq!(formulas, vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)]);
}

#[test]
fn open3_evaluation_cell_constructor_reads_java_formula_cache_tokens() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&fixture("formula-cache.xlsx"), None, true), 1);
    let sheet = api.book().sheetAt(0);
    let uid = sheet.borrow().getSheetUid();
    for (col, plain_len) in [(0, 7), (1, 17), (2, 7)] {
        let formula = EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 0, col);
        assert_eq!(formula.getFmlRow(), 0);
        assert_eq!(formula.getFmlCol(), col);
        assert_eq!(formula.getSheetUid(), uid);
        assert_eq!(formula.getSheetIndex(), 0);
        assert!(!formula.isSharedFormula());
        assert_eq!(formula.getEncodedTokensLen(), plain_len);
        assert_eq!(formula.getEncodedTokens().unwrap().len(), plain_len as usize);
        assert!(!formula.isNeedRecalculationFlagSet());
    }
}

#[test]
fn open3_ptg_read_tokens_matches_java_device_sequences() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let stage_path = fixture("../evidence/java-formula-cache-ptg-stages.jsonl");
    let stage = std::fs::read_to_string(stage_path).unwrap();
    let java: serde_json::Value = serde_json::from_str(stage.lines().find(|line| line.contains("after_open_return")).unwrap()).unwrap();
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&fixture("formula-cache.xlsx"), None, true), 1);
    let sheet = api.book().sheetAt(0);
    for cell in java["cells"].as_array().unwrap() {
        let col = cell["col"].as_i64().unwrap() as i32;
        let fml = EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 0, col);
        let bytes = fml.getEncodedTokens().unwrap();
        assert_eq!(hex_bytes(bytes), cell["tokenHex"].as_str().unwrap());
        let parsed = Ptg::readTokens(fml.getEncodedTokensLen(), bytes, sheet.borrow().getVersion());
        let expected = cell["parsedPtgs"].as_array().unwrap();
        assert_eq!(parsed.len(), expected.len(), "col={col}");
        for (ptg, gold) in parsed.iter().zip(expected) {
            assert_eq!(ptg.getSid() as i64, gold["sid"].as_i64().unwrap(), "col={col}");
            assert_eq!(ptg.getSize() as i64, gold["size"].as_i64().unwrap(), "col={col}");
            assert_eq!(ptg.class as i64, gold["ptgClass"].as_i64().unwrap(), "col={col}");
        }
        assert_eq!(serializePtgs(&parsed), bytes, "col={col}");
        assert_eq!(hex_bytes(&serializePtgs(&fml.getTokens())), cell["resolvedTokenHex"].as_str().unwrap(), "col={col}");
    }

    let sample = format!("{}/../../samples/ss-render/出入库管理明细表.xlsx", env!("CARGO_MANIFEST_DIR"));
    let gold_path = fixture("../evidence/java-sample-ptg-records.json");
    let gold: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(gold_path).unwrap()).unwrap();
    let session = SpreadsheetSession::open_path(&sample, SpreadsheetLoadConfig::default());
    assert_eq!(session.input_sha256(), gold["inputSha256"].as_str().unwrap());
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&sample, None, true), 1);
    let sheet = api.book().sheetAt(0);
    for cell in gold["records"].as_array().unwrap() {
        let row = cell["row"].as_i64().unwrap() as i32;
        let col = cell["col"].as_i64().unwrap() as i32;
        let data = sheet.borrow().getCellData(row, col);
        let mut formula = KmoFormulaData::KmoFormulaData();
        let store = sheet.borrow().getCellValueStore();
        store.borrow().formulaValue(data.id, &mut formula);
        let bytes = store.borrow().formulaTokens(formula.getTokensId());
        assert_eq!(hex_bytes(&bytes), cell["formulaTokenHex"].as_str().unwrap());
        let parsed = Ptg::readTokens(formula.getTokensPlainSize(), &bytes, sheet.borrow().getVersion());
        let expected = cell["formulaParsedPtgs"].as_array().unwrap();
        assert_eq!(parsed.len(), expected.len(), "row={row} col={col}");
        for (ptg, gold) in parsed.iter().zip(expected) {
            assert_eq!(ptg.getSid() as i64, gold["sid"].as_i64().unwrap(), "row={row} col={col}");
            assert_eq!(ptg.getSize() as i64, gold["size"].as_i64().unwrap(), "row={row} col={col}");
            assert_eq!(ptg.class as i64, gold["ptgClass"].as_i64().unwrap(), "row={row} col={col}");
        }
        assert_eq!(serializePtgs(&parsed), bytes, "row={row} col={col}");
        let evaluation = EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), row, col);
        assert_eq!(hex_bytes(&serializePtgs(&evaluation.getTokens())), cell["formulaResolvedTokenHex"].as_str().unwrap(), "row={row} col={col}");
    }
}

#[test]
fn open3_sample_evaluation_cell_constructor_preserves_formula_classes() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let sample = format!("{}/../../samples/ss-render/出入库管理明细表.xlsx", env!("CARGO_MANIFEST_DIR"));
    let gold_path = format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/evidence/java-sample-formula-records.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let gold: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(gold_path).unwrap()).unwrap();
    let session = SpreadsheetSession::open_path(&sample, SpreadsheetLoadConfig::default());
    assert_eq!(session.input_sha256(), gold["inputSha256"].as_str().unwrap());
    let java_records = gold["records"].as_array().unwrap();
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&sample, None, true), 1);
    let sheet = api.book().sheetAt(0);
    let mut iter = sheet.borrow().getCellIterator(0, 39, 0, 23);
    let mut positions = Vec::new();
    while iter.hasNext() {
        iter.next();
        let row = iter.row();
        let col = iter.col();
        let data = iter.cellData();
        if KmoCellValueStore::identifyTypeByID(data.id) == 7 {
            positions.push((row, col, data.formulaType));
        }
    }
    assert_eq!(positions.len(), java_records.len());
    let mut normal = 0;
    let mut shared = 0;
    let mut shared_encoded = 0;
    for ((row, col, formula_type), java) in positions.into_iter().zip(java_records) {
        assert_eq!(row, java["row"].as_i64().unwrap() as i32);
        assert_eq!(col, java["col"].as_i64().unwrap() as i32);
        assert_eq!(formula_type, java["formulaType"].as_i64().unwrap() as i32);
        let data = sheet.borrow().getCellData(row, col);
        let mut record = KmoFormulaData::KmoFormulaData();
        let store = sheet.borrow().getCellValueStore();
        store.borrow().formulaValue(data.id, &mut record);
        let raw_tokens = store.borrow().formulaTokens(record.getTokensId());
        assert_eq!(record.getTokensId(), java["formulaTokenId"].as_i64().unwrap() as i32);
        assert_eq!(record.getTokensPlainSize(), java["formulaTokenPlainSize"].as_i64().unwrap() as i32);
        assert_eq!(record.getCalcFlags(), java["formulaCalcFlags"].as_i64().unwrap() as i8);
        assert_eq!(hex_bytes(&raw_tokens), java["formulaTokenHex"].as_str().unwrap());
        let formula = EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), row, col);
        assert_eq!((formula.getFmlRow(), formula.getFmlCol()), (row, col));
        assert_eq!(formula.getSheetIndex(), 0);
        match formula_type {
            1 => {
                normal += 1;
                assert!(!formula.isSharedFormula());
                assert!(!formula.isMasterSharedFormula());
                assert!(formula.getSharedFormula().is_none());
                assert!(formula.getEncodedTokensLen() > 0);
                assert!(formula.getEncodedTokens().is_some());
                assert_eq!(formula.getEncodedTokensLen(), java["formulaTokenPlainSize"].as_i64().unwrap() as i32);
                assert_eq!(hex_bytes(formula.getEncodedTokens().unwrap()), java["formulaTokenHex"].as_str().unwrap());
            }
            3 => {
                shared += 1;
                if formula.isSharedFormula() {
                    shared_encoded += 1;
                    assert!(formula.getEncodedTokensLen() > 0);
                    assert!(formula.getEncodedTokens().is_some());
                    let master = ExpPtg::getRowCol(sheet.borrow().getVersion(), &raw_tokens);
                    assert_eq!(master[0], java["sharedMasterRow"].as_i64().unwrap() as i32);
                    assert_eq!(master[1], java["sharedMasterCol"].as_i64().unwrap() as i32);
                    assert_eq!(formula.isMasterSharedFormula(), row == master[0] && col == master[1]);
                    let shared_formula = formula.getSharedFormula().expect("shared formula descriptor");
                    let matched = sheet.borrow().getExpFmlManager().borrow().query(master[0], master[1], true).expect("shared range");
                    assert_eq!((shared_formula.getFmlRow(), shared_formula.getFmlCol()), (matched.firstRow, matched.firstCol));
                    assert_eq!((shared_formula.getFmlLastRow(), shared_formula.getFmlLastCol()), (matched.lastRow, matched.lastCol));
                    let master_formula = EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), master[0], master[1]);
                    assert_eq!(serializePtgs(&shared_formula.getTokens()), serializePtgs(&master_formula.getTokens()));
                    assert_eq!(formula.getEncodedTokensLen(), java["sharedEncodedTokenPlainSize"].as_i64().unwrap() as i32);
                    assert_eq!(hex_bytes(formula.getEncodedTokens().unwrap()), java["sharedEncodedTokenHex"].as_str().unwrap());
                }
            }
            _ => panic!("unexpected formula type"),
        }
    }
    assert_eq!((normal, shared), (28, 25));
    assert_eq!(shared_encoded, 25);
}

fn hex_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect::<Vec<_>>().join("")
}

#[test]
fn open3_sample_formula_iterator_matches_java_device_positions() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let sample = format!("{}/../../samples/ss-render/出入库管理明细表.xlsx", env!("CARGO_MANIFEST_DIR"));
    let gold_path = format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/evidence/java-sample-formula-positions.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let gold: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(gold_path).unwrap()).unwrap();
    assert_eq!(gold["inputSha256"], "611d9b930f6daa00f8fcf2fe0ee3465e09b779fd8a843d604dab53b3b2a336bc");
    let session = SpreadsheetSession::open_path(&sample, SpreadsheetLoadConfig::default());
    assert_eq!(session.input_sha256(), gold["inputSha256"].as_str().unwrap());
    let expected: Vec<(i32, i32, i32)> = gold["formulas"].as_array().unwrap().iter().map(|item| {
        let v = item.as_array().unwrap();
        (v[0].as_i64().unwrap() as i32, v[1].as_i64().unwrap() as i32, v[2].as_i64().unwrap() as i32)
    }).collect();

    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&sample, None, true), 1);
    let sheet = api.book().sheetAt(0);
    let sheet = sheet.borrow();
    let mut iter = sheet.getCellIterator(0, 39, 0, 23);
    let mut actual = Vec::new();
    while iter.hasNext() {
        iter.next();
        let row = iter.row();
        let col = iter.col();
        let data = iter.cellData();
        if KmoCellValueStore::identifyTypeByID(data.id) == 7 {
            actual.push((row, col, data.formulaType));
        }
    }
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 53);
}

#[test]
fn open4_missing_file_returns_throwable_code() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open("/tmp/m1-no-such-book.xlsx", None, true), 2);
    assert_eq!(api.book().sheetCount(), 0);
}

#[test]
fn open5_not_workbook_returns_throwable_code() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&fixture("not-workbook.zip"), None, true), 2);
}

#[test]
fn open6_broken_sheet_part_still_returns_success() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&fixture("broken-rel.xlsx"), None, true), 1);
    assert_eq!(api.book().sheetCount(), 1);
    assert_eq!(api.book().sheetAt(0).borrow().name(), "Sheet1");
}

#[test]
fn open7_empty_sheet_data_has_no_cell() {
    let _open_guard=OPEN_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let api = KmoBookApi::KmoBookApi();
    // Isolate reader/record tests from Java's asynchronous open recalculation.
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&fixture("empty.xlsx"), None, true), 1);
    assert!(api.book().sheetAt(0).borrow().getCellData(0, 0).isNull());
}
