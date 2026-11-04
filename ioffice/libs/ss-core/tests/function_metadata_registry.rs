use ss_core::cn::wps::et::ss::formula::parser::FormulaParser::parse;
use ss_core::cn::wps::et::ss::formula::parser::ParserOption::ParserOption;
use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{function_by_name, Ptg, PtgKind};
use ss_core::cn::wps::et::ss::formula::function::FunctionMetadataRegistry;
use ss_core::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[test]
fn function_data_holder_selects_java_fixed_and_variable_tokens() {
    let book = KmoBook::KmoBook__parent(false);
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let option = ParserOption::createOOXMLParserOption();

    for (formula, expected_index, fixed) in [
        ("PI()", 19, true),
        ("COUNT(1,2)", 0, false),
        ("YEN(1)", 204, false),
        ("VLOOKUP(1,2,3)", 102, false),
    ] {
        // java FormulaParser.getFunction -> FunctionMetadataRegistry -> FuncPtg/FuncVarPtg。
        let tokens = parse(formula, &workbook, 0, 0, SpreadsheetVersion::EXCEL2007, &option).unwrap();
        match &tokens.last().unwrap().kind {
            PtgKind::Func { meta } if fixed => assert_eq!(meta.getIndex(), expected_index),
            PtgKind::FuncVar { meta, .. } if !fixed => assert_eq!(meta.getIndex(), expected_index),
            other => panic!("{formula}: wrong token {other:?}"),
        }
        let bytes: Vec<u8> = tokens.iter().flat_map(|token| {
            let mut out = Vec::new(); token.write(&mut out); out
        }).collect();
        let decoded = Ptg::readTokens(bytes.len() as i32, &bytes, SpreadsheetVersion::EXCEL2007);
        assert_eq!(decoded.len(), tokens.len(), "{formula}");
    }

    assert_eq!(function_by_name("SUMIF").unwrap().getIndex(), 345);
    assert!(function_by_name("CONCAT").is_none());
    // java FunctionMetadataRegistry 是大小写敏感表；AbstractFunctionPtg 调用方负责大写。
    assert_eq!(FunctionMetadataRegistry::lookupIndexByName("SUMIF"), 345);
    assert_eq!(FunctionMetadataRegistry::lookupIndexByName("sumif"), -1);
    assert_eq!(FunctionMetadataRegistry::getFunctionByName("JIS").unwrap().getIndex(), 215);
    // java addFootnote 将同一个 FunctionMetadata 对象登记到两个名称下。
    assert!(std::ptr::eq(
        FunctionMetadataRegistry::getFunctionByName("JIS").unwrap(),
        FunctionMetadataRegistry::getFunctionByIndex(215).unwrap(),
    ));
}

#[test]
fn java_remaining_base_and_deleted_3d_tokens_keep_wire_bytes() {
    for (version, bytes) in [
        (SpreadsheetVersion::EXCEL97, vec![0, 2, 7, 0, 3, 0, 15, 16, 17, 22, 24, 29, 4, 3, 2, 1, 60, 1, 0, 4, 3, 2, 1, 61, 2, 0, 8, 7, 6, 5, 4, 3, 2, 1]),
        (SpreadsheetVersion::EXCEL2007, vec![2, 7, 0, 0, 0, 3, 0, 15, 16, 17, 22, 24, 29, 4, 3, 2, 1, 92, 1, 0, 4, 3, 2, 1, 125, 2, 0, 8, 7, 6, 5, 4, 3, 2, 1]),
    ] {
        // java Ptg.createBasePtg/createClassifiedPtg -> 各具体子类 write。
        let tokens = Ptg::readTokens(bytes.len() as i32, &bytes, version);
        assert!(!tokens.is_empty());
        let encoded: Vec<u8> = tokens.iter().flat_map(|token| {
            let mut out = Vec::new(); token.write(&mut out); out
        }).collect();
        assert_eq!(encoded, bytes);
        assert!(tokens.iter().any(|token| matches!(token.kind, PtgKind::DeletedRef3D { .. })));
        assert!(tokens.iter().any(|token| matches!(token.kind, PtgKind::DeletedArea3D { .. })));
    }
}

#[test]
fn external_name_3d_tokens_keep_both_sheet_indices() {
    for (version, bytes) in [
        (SpreadsheetVersion::EXCEL97, vec![58, 1, 0, 3, 0, 7, 0, 5, 0, 91, 2, 0, 4, 0, 1, 0, 9, 0, 3, 0, 8, 0]),
        (SpreadsheetVersion::EXCEL2007, vec![58, 1, 0, 3, 0, 7, 0, 0, 0, 5, 0, 91, 2, 0, 4, 0, 1, 0, 0, 0, 9, 0, 0, 0, 3, 0, 8, 0]),
    ] {
        // java Ptg.createClassifiedPtg(externalName=true) -> Ref3DExtPtg/Area3DExtPtg。
        let tokens = Ptg::readTokens__stream(bytes.len() as i32, &bytes, true, version);
        assert!(matches!(tokens[0].kind, PtgKind::Ref3DExt { sheet_index1: 1, sheet_index2: 3, .. }));
        assert!(matches!(tokens[1].kind, PtgKind::Area3DExt { sheet_index1: 2, sheet_index2: 4, .. }));
        let encoded: Vec<u8> = tokens.iter().flat_map(|token| {
            let mut out = Vec::new(); token.write(&mut out); out
        }).collect();
        assert_eq!(encoded, bytes);
    }
}

#[test]
fn java_calc_chain_volatile_function_indices_match_the_registry() {
    // java FormulaHelper.checkVolatileFunc 使用固定十项有序索引，不直接读 metadata.isVolatile。
    for index in [63, 74, 78, 125, 148, 221, 244, 255, 257, 344] {
        assert!(FormulaHelper::checkVolatileFunc(index), "{index}");
    }
    for index in [0, 1, 62, 64, 343, 345, 367] {
        assert!(!FormulaHelper::checkVolatileFunc(index), "{index}");
    }
}

#[test]
fn java_has_subtotal_checks_function_tokens_only() {
    let subtotal = function_by_name("SUBTOTAL").unwrap();
    let sum = function_by_name("SUM").unwrap();
    assert!(!FormulaHelper::hasSubtotal(&[Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::FuncVar { meta: sum, nargs: 1 }, class: 32 }]));
    assert!(FormulaHelper::hasSubtotal(&[
        Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Name { index: 344 }, class: 0 },
        Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::FuncVar { meta: subtotal, nargs: 2 }, class: 32 },
    ]));
}

#[test]
fn java_check_volatile_follows_token_sid_branches() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let formula = EvaluationCellFormula::EvaluationCellFormula(sheet, 0, 0);
    // java case 17 RangePtg is immediately volatile; case 25 uses the low option bit.
    assert!(FormulaHelper::checkVolatile(&[Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Range, class: 0 }], &formula, &workbook));
    assert!(FormulaHelper::checkVolatile(&[Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Attr { options: 1, data: 0, jump_table:None, choose_offset:-1 }, class: 0 }], &formula, &workbook));
    assert!(!FormulaHelper::checkVolatile(&[Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Attr { options: 16, data: 0, jump_table:None, choose_offset:-1 }, class: 0 }], &formula, &workbook));
    // java case 33/34 checks the fixed index list, not FunctionMetadata.isVolatile.
    let rand = function_by_name("RAND").unwrap();
    let sum = function_by_name("SUM").unwrap();
    assert!(FormulaHelper::checkVolatile(&[Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Func { meta: rand }, class: 32 }], &formula, &workbook));
    assert!(!FormulaHelper::checkVolatile(&[Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::FuncVar { meta: sum, nargs: 1 }, class: 32 }], &formula, &workbook));
}
