use serde_json::{json, Value};
use ss_core::{
    cn::wps::{
        et::ss::constant::{NumfmtCategory::NumfmtCategory, RecogniseResult::RecogniseResult},
        moss::{
            app::{
                event::KmoSheetChangeListenerGroup::KmoSheetChangeEventsListener, KmoBook::KmoBook,
                KmoBookApi::KmoBookApi, KmoSheet::KmoSheet,
            },
            engine::{
                areaevent::AreaWatcher::{IAreaChangeListener, INameChangeListener},
                grid::{KmoCellData::KmoCellData, KmoCellValueStore::KmoCellValueStore},
            },
            filefmt::biff8::record::common::{FormatRun::FormatRun, UnicodeString::UnicodeString},
            util::KmoRect::KmoRect,
        },
    },
    java_runtime::{Rc, RefCell},
};

struct Events(Rc<RefCell<String>>);
impl KmoSheetChangeEventsListener for Events {
    fn sheetContentsDidFinishChanging(&self) {
        self.0.borrow_mut().push('S');
    }
}
impl INameChangeListener for Events {
    fn onSheetNameChanged(&mut self, _: &KmoSheet) {}
}
impl IAreaChangeListener for Events {
    fn onContentChanged(&mut self) {
        self.0.borrow_mut().push('A');
    }
    fn onAreaMoved(&mut self, _: &KmoSheet, _: &KmoRect) {}
    fn onAreaDeleted(&mut self) {}
}

fn typed(book: &Rc<KmoBook>) -> Value {
    let sheet = book.appendSheet__2(Some("M4Typed".into()));
    let sheet = sheet.borrow();
    let events = Rc::new(RefCell::new(String::new()));
    sheet.subscribeChangeEvents(Rc::new(Events(events.clone())));
    book.areaWatcher().borrow_mut().registerListener(
        &sheet,
        &KmoRect::KmoRect__bounds(70, 31, 70, 31),
        Rc::new(RefCell::new(Events(events.clone()))),
    );
    let mut rows = Vec::new();
    for ixf in [0, 15, 4095] {
        for formula in 0..4 {
            for flags in 0..4 {
                for op in 0..10 {
                    let seed = KmoCellData {
                        ixf,
                        formulaType: formula,
                        hasComment: flags & 1 != 0,
                        isPicInCell: flags & 2 != 0,
                        id: 0,
                    };
                    sheet.setCellData(70, 31, &seed);
                    book.setDirty(false);
                    events.borrow_mut().clear();
                    match op {
                        0 => sheet.setCellBlank(70, 31),
                        1 => sheet.setCellBoolValue(70, 31, false),
                        2 => sheet.setCellBoolValue(70, 31, true),
                        3 => sheet.setCellNumericValue(70, 31, -0.0),
                        4 => sheet.setCellNumericValue(70, 31, 123.25),
                        5 => sheet.setCellErrorValue(70, 31, 7),
                        6 => sheet.setCellStringValue(70, 31, Some("中文😀\n&<>")),
                        7 => sheet.setCellStringValue(70, 31, Some("")),
                        8 => {
                            let mut s = UnicodeString::new("A😀中".into());
                            s.resetFormatRuns(vec![FormatRun::new(0, 0), FormatRun::new(3, 1)]);
                            sheet.setCellUnicodeStringValue(70, 31, Some(Rc::new(s)));
                        }
                        9 => sheet.setCellUnicodeStringValue(
                            70,
                            31,
                            Some(Rc::new(UnicodeString::new("no runs".into()))),
                        ),
                        _ => unreachable!(),
                    }
                    let data = sheet.getCellData(70, 31);
                    let mut bytes = [0; 7];
                    data.write(&mut bytes);
                    let type_ = KmoCellValueStore::identifyTypeByID(data.id);
                    let mut row = json!({"ixf":ixf,"formula":formula,"flags":flags,"op":op,
                        "bytes":bytes.iter().map(|b|format!("{b:02x}")).collect::<String>(),
                        "dirty":book.isDirty(),"events":events.borrow().clone(),"type":type_});
                    if type_ == 1 {
                        let v = book.getCellValueStore().borrow_mut().doubleValue(data.id);
                        row["numberBits"] = json!(format!("{:x}", v.to_bits()));
                    }
                    if type_ == 2 {
                        row["text"] =
                            json!(book.getCellValueStore().borrow_mut().stringValue(data.id));
                    }
                    rows.push(row);
                }
            }
        }
    }
    Value::Array(rows)
}

fn with_flags(f: i32) -> RecogniseResult {
    let mut rs = RecogniseResult::RecogniseResult__2(Some(1.0));
    if f & 1 != 0 {
        rs.setGroupedFlag();
    }
    if f & 2 != 0 {
        rs.setDateFlag();
    }
    if f & 4 != 0 {
        rs.setTimeFlag();
    }
    if f & 8 != 0 {
        rs.setCurrencyFlag();
    }
    if f & 16 != 0 {
        rs.setErrorFlag();
    }
    if f & 32 != 0 {
        rs.setSqleadingFlag();
    }
    if f & 64 != 0 {
        rs.setPercentageFlage();
    }
    if f & 128 != 0 {
        rs.setFractionFlag();
    }
    if f & 256 != 0 {
        rs.setHyperlinkFlag();
    }
    if f & 512 != 0 {
        rs.setSenticFlag();
    }
    rs
}

fn decisions() -> Value {
    use NumfmtCategory::*;
    let mut rows = Vec::new();
    for cat in [
        NONE, GENERAL, NUMBER, ACCOUNTING, CURRENCY, DATE, TIME, DATE_TIME, PERCENTAGE, FRACTION,
        SCIENTIC, TEXT,
    ] {
        for old in [-1i16, 0, 9, 164] {
            for builtin in [-1, 0, 9, 164, 65536] {
                for code in [false, true] {
                    for cover in [false, true] {
                        let mut bits = String::new();
                        for f in (0..1024).step_by(4) {
                            let mut nibble = 0;
                            for k in 0..4 {
                                let mut rs = with_flags(f + k);
                                rs.setSuggestedNumfmtI(builtin);
                                if code {
                                    rs.setSuggestedNumfmt(Some("0.0000".into()));
                                }
                                if KmoSheet::_needReplaceNumfmt(old, cat, &rs, cover) {
                                    nibble |= 1 << k;
                                }
                            }
                            bits.push(char::from_digit(nibble, 16).unwrap());
                        }
                        rows.push(json!({"cat":format!("{cat:?}"),"old":old,"builtin":builtin,
                            "code":code,"cover":cover,"bits":bits}));
                    }
                }
            }
        }
    }
    Value::Array(rows)
}

fn new_xf(book: &Rc<KmoBook>) -> Value {
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    let manager = book.getXFManager().borrow();
    let mut rows = Vec::new();
    for i in 0..8 {
        let old = manager.cloneCellXF(if i % 2 == 0 { 0 } else { 15 });
        let xf = manager.getXF(old).unwrap();
        let mut rs = RecogniseResult::RecogniseResult(Some("abc".encode_utf16().collect()));
        if i & 2 != 0 {
            rs.setSqleadingFlag();
        }
        if i >= 4 {
            rs.setSuggestedNumfmt(Some(if i == 7 { "0.0000" } else { "0.000" }.into()));
        }
        let result = sheet._newXF(
            old,
            &xf,
            xf.getFormatIndex(),
            NumfmtCategory::GENERAL,
            if i & 1 == 0 { None } else { Some("a\nb") },
            &rs,
            i >= 4,
        );
        let changed = if result == 4095 {
            xf.clone()
        } else {
            manager.getXF(result).unwrap()
        };
        let mut row = json!({"case":i,"old":old,"result":result,"count":manager.getNumExFormats(),
            "wrap":changed.getWrapText(),"prefix":changed.get123Prefix(),"format":changed.getFormatIndex(),
            "oldWrap":xf.getWrapText(),"oldPrefix":xf.get123Prefix()});
        if let Some(code) = changed.getNumFormat() {
            row["code"] = json!(code);
        }
        rows.push(row);
    }
    Value::Array(rows)
}

fn cell_bytes(sheet: &KmoSheet) -> Value {
    cell_bytes_at(sheet, 70, 31)
}

fn cell_bytes_at(sheet: &KmoSheet, row: i32, col: i32) -> Value {
    let mut bytes = [0; 7];
    sheet.getCellData(row, col).write(&mut bytes);
    json!(bytes.iter().map(|b| format!("{b:02x}")).collect::<String>())
}

fn snapshots(book: &Rc<KmoBook>) -> Value {
    use kfc::cn::wps::memor4j::KmoAtomStore::{KmoAtomStore, KmoAtomStoreTrait};
    fn store<R>(f: impl FnOnce(&mut dyn KmoAtomStoreTrait) -> R) -> R {
        let mut s = KmoAtomStore::sharedStore();
        f(s.as_mut().unwrap().as_mut())
    }
    KmoAtomStore::enable();
    KmoAtomStore::clearUp();
    store(|_| ());
    let sheet = book.sheetAt(book.sheetCount() - 1);
    let sheet = sheet.borrow();
    let mut rows = Vec::new();
    sheet.setCellStringValue(70, 31, Some("baseline"));
    rows.push(cell_bytes(&sheet));
    store(|s| s.start());
    sheet.setCellNumericValue(70, 31, 44.5);
    sheet.setCellBoolValue(70, 31, true);
    rows.push(json!(store(|s| s.commit())));
    rows.push(cell_bytes(&sheet));
    store(|s| s.undo());
    rows.push(cell_bytes(&sheet));
    store(|s| s.redo());
    rows.push(cell_bytes(&sheet));
    store(|s| s.start());
    sheet.setCellBlank(70, 31);
    rows.push(json!(store(|s| s.commit())));
    rows.push(cell_bytes(&sheet));
    store(|s| s.undo());
    rows.push(cell_bytes(&sheet));
    store(|s| s.undo());
    rows.push(cell_bytes(&sheet));
    store(|s| s.redo());
    rows.push(cell_bytes(&sheet));
    store(|s| s.redo());
    rows.push(cell_bytes(&sheet));
    store(|s| s.undo());
    store(|s| s.undo());
    rows.push(json!(sheet.getCellStringValue(70, 31)));
    rows.push(cell_bytes_at(&sheet, 150, 40));
    store(|s| s.start());
    sheet.setCellNumericValue(150, 40, 7.25);
    rows.push(json!(store(|s| s.commit())));
    rows.push(cell_bytes_at(&sheet, 150, 40));
    store(|s| s.undo());
    rows.push(cell_bytes_at(&sheet, 150, 40));
    store(|s| s.redo());
    rows.push(cell_bytes_at(&sheet, 150, 40));
    KmoAtomStore::clearUp();
    Value::Array(rows)
}

fn formulas(book: &Rc<KmoBook>) -> Value {
    use ss_core::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
    let sheet = book.sheetAt(book.sheetCount() - 1);
    let sheet = sheet.borrow();
    let events = Rc::new(RefCell::new(String::new()));
    sheet.subscribeChangeEvents(Rc::new(Events(events.clone())));
    let store = book.getCellValueStore();
    let mut rows = Vec::new();
    for i in 0..5 {
        book.setDirty(false);
        events.borrow_mut().clear();
        let mut f = KmoFormulaData::KmoFormulaData();
        if i < 3 {
            let cache = if i == 0 {
                -1
            } else {
                store.borrow_mut().newDoubleValue(9.5)
            };
            sheet
                .initCellFormula(70, 31, ["1+2", "ABS(-3)", "SUM(A1:B2)"][i], cache, 15)
                .unwrap();
        } else {
            store
                .borrow()
                .formulaValue(sheet.getCellData(70, 31).id, &mut f);
            if i == 3 {
                sheet.setCellFormulaValue(70, 31, &f);
            } else {
                sheet.setCellFormulaValue__2(70, 31, &f, 3);
            }
        }
        store
            .borrow()
            .formulaValue(sheet.getCellData(70, 31).id, &mut f);
        let hex = store
            .borrow()
            .formulaTokens(f.getTokensId())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        rows.push(json!({"bytes":cell_bytes(&sheet),"events":events.borrow().clone(),"dirty":book.isDirty(),
            "tokenHex":hex,"tokensId":f.getTokensId(),"plainSize":f.getTokensPlainSize(),
            "cacheId":f.getCacheValueId(),"calcFlags":f.getCalcFlags()}));
    }
    Value::Array(rows)
}

#[test]
fn original_android_edit_methods_match_bytes_notifications_and_format_branches() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let evidence = root.join("rewrite-review/milestones/m4-edit-save/evidence");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(evidence.join("java-edit-r0b25.json")).unwrap())
            .unwrap();
    let api = KmoBookApi::KmoBookApi();
    assert_eq!(
        api.open(
            root.join("rewrite-review/milestones/m1-xlsx-grid-read/fixtures/basic-types.xlsx")
                .to_str()
                .unwrap(),
            None,
            true
        ),
        1
    );
    let book = api.book();
    for _ in 0..200 {
        if book.isCalcChainInitialized() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert!(book.isCalcChainInitialized());
    // Match the Java observer's ui_plus_1000ms phase. The initialized flag is set
    // before openCalc's final per-sheet notification, which can otherwise join our edit trace.
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let mut actual = json!({"source":"original ET KmoSheet / RecogniseResult methods",
        "typed":typed(&book),"formatDecisions":decisions(),"newXf":new_xf(&book)});
    actual["snapshots"] = snapshots(&book);
    actual["formulaSetters"] = formulas(&book);
    let mut rs = RecogniseResult::RecogniseResult(Some("text".encode_utf16().collect()));
    rs.setSuggestedNumfmt(Some("0.0000".into()));
    rs.setSuggestedNumfmtI(9);
    actual["dualSuggestedFormat"] =
        json!({"builtin":rs.getBuiltInNumfmtIdx(),"code":rs.getSuggestedNumfmt()});
    rs.clear();
    assert!(rs.getValue().is_none());
    assert!(!rs.hasGroupedFlag() && !rs.hasDateFlag() && !rs.hasTimeFlag());
    actual["cleared"] =
        json!({"builtin":rs.getBuiltInNumfmtIdx(),"code":rs.getSuggestedNumfmt(),"value":null});
    std::fs::write(
        evidence.join("rust-edit-r0b25.json"),
        serde_json::to_vec_pretty(&actual).unwrap(),
    )
    .unwrap();
    for key in [
        "typed",
        "formatDecisions",
        "newXf",
        "snapshots",
        "formulaSetters",
    ] {
        let a = actual[key].as_array().unwrap();
        let e = expected[key].as_array().unwrap();
        assert_eq!(a.len(), e.len(), "{key} vector count");
        for (i, (a, e)) in a.iter().zip(e).enumerate() {
            assert_eq!(a, e, "{key}[{i}]");
        }
    }
    assert_eq!(actual, expected);
}

#[test]
fn recognise_result_preserves_nullable_boxes_utf16_and_independent_format_fields() {
    let units = vec![0xd800, 0, 0xdc00];
    let mut rs = RecogniseResult::RecogniseResult(Some(units.clone()));
    assert_eq!(rs.getStrValue(), Some(units));
    rs.setSuggestedNumfmt(Some("0.000".into()));
    rs.setSuggestedNumfmt__2(9);
    assert_eq!(rs.getSuggestedNumfmt().as_deref(), Some("0.000"));
    assert_eq!(rs.getBuiltInNumfmtIdx(), 9);
    rs.setValue__2(Some(-0.0));
    assert_eq!(rs.getDblValue().unwrap().to_bits(), (-0.0f64).to_bits());
    rs.setValue__3(Some(true));
    assert_eq!(rs.getBoolValue(), Some(true));
    rs.setValue(None);
    assert!(rs.getStrValue().is_none());
    assert!(RecogniseResult::RecogniseResult__2(None)
        .getDblValue()
        .is_none());
    assert!(RecogniseResult::RecogniseResult__3(None)
        .getBoolValue()
        .is_none());
    let mut rs = with_flags(1023);
    rs.clear();
    assert!(rs.getValue().is_none());
    assert!(rs.getSuggestedNumfmt().is_none());
    assert_eq!(rs.getBuiltInNumfmtIdx(), -1);
    assert!(
        !rs.hasGroupedFlag()
            && !rs.hasDateFlag()
            && !rs.hasTimeFlag()
            && !rs.hasCurrencyFlag()
            && !rs.hasErrorFlag()
            && !rs.hasSqleadingFlag()
            && !rs.hasPercentageFlag()
            && !rs.hasFractionFlag()
            && !rs.hasHyperlinkFlag()
            && !rs.hasSenticFlag()
    );
}
