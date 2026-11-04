use kfc::cn::wps::memor4j::KmoAtomStore::{KmoAtomStore, KmoAtomStoreTrait};
use serde_json::{json, Value};
use ss_core::{
    cn::wps::{
        et::ss::constant::RecogniseResult::RecogniseResult,
        moss::{
            app::{
                KmoBook::KmoBook,
                KmoBookApi::KmoBookApi,
                KmoSheet::KmoSheet,
                KmoTransaction::{KmoSheetTransactionListener, KmoTransactionListener},
            },
            engine::grid::{KmoCellData::KmoCellData, KmoCellValueStore::KmoCellValueStore},
            util::KmoRect::KmoRect,
        },
    },
    java_runtime::{Rc, RefCell},
};

fn store<R>(f: impl FnOnce(&mut dyn KmoAtomStoreTrait) -> R) -> R {
    let mut s = KmoAtomStore::sharedStore();
    f(s.as_mut().unwrap().as_mut())
}
fn bytes(sheet: &KmoSheet, row: i32, col: i32) -> String {
    let mut b = [0; 7];
    sheet.getCellData(row, col).write(&mut b);
    b.iter().map(|b| format!("{b:02x}")).collect()
}
fn result(op: i32) -> RecogniseResult {
    let mut rs = match op {
        1 => RecogniseResult::RecogniseResult__2(Some(0.012345)),
        2 => RecogniseResult::RecogniseResult__2(Some(46100.123)),
        3 => RecogniseResult::RecogniseResult(Some("'123".encode_utf16().collect())),
        4 => RecogniseResult::RecogniseResult(Some("a\nb".encode_utf16().collect())),
        5 => RecogniseResult::RecogniseResult__3(Some(true)),
        6 => RecogniseResult::RecogniseResult__2(Some(7.0)),
        7 => RecogniseResult::RecogniseResult(Some(vec![])),
        8 => RecogniseResult::RecogniseResult__2(Some(1.25)),
        _ => RecogniseResult::RecogniseResult__2(Some(1.255)),
    };
    match op {
        1 => {
            rs.setSuggestedNumfmt__2(9);
            rs.setPercentageFlage();
        }
        2 => {
            rs.setSuggestedNumfmt__2(14);
            rs.setDateFlag();
        }
        3 => {
            rs.setSuggestedNumfmt__2(0);
            rs.setSqleadingFlag();
        }
        6 => {
            rs.setSuggestedNumfmt__2(0);
            rs.setErrorFlag();
        }
        7 => {}
        9 => rs.setSuggestedNumfmt(Some("0.000".into())),
        _ => rs.setSuggestedNumfmt__2(0),
    };
    rs
}
fn raw_results(book: &Rc<KmoBook>) -> Value {
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    let mut rows = vec![];
    for format in [0, 2, 9, 14, 164] {
        for blank in [false, true] {
            for precision in [false, true] {
                for cover in [false, true] {
                    for op in 0..10 {
                        let manager = book.getXFManager().borrow();
                        let xf = manager.cloneCellXF(15);
                        let old = manager.getXF(xf).unwrap();
                        let effective = if format == 164 {
                            manager.checkFormat(Some("0.00"))
                        } else {
                            format
                        };
                        old.setFormatIndex(effective as i16);
                        old.setNumFormat(None);
                        let seed = KmoCellData {
                            ixf: xf,
                            formulaType: 3,
                            hasComment: true,
                            isPicInCell: true,
                            id: if blank {
                                0
                            } else {
                                book.getCellValueStore().borrow_mut().newDoubleValue(99.99)
                            },
                        };
                        sheet.setCellData(70, 31, &seed);
                        sheet.setFullPrecision(precision);
                        book.setDirty(false);
                        let raw = [
                            "1.255",
                            "1.2345%",
                            "2026/3/19",
                            "'123",
                            "a\nb",
                            "true",
                            "#DIV/0!",
                            "",
                            "0x1.8p2D",
                            "1.255",
                        ][op as usize];
                        sheet._setCellRgnResult(70, 31, Some(raw), &result(op), cover);
                        let cell = sheet.getCellData(70, 31);
                        let t = KmoCellValueStore::identifyTypeByID(cell.id);
                        let now = manager.getXF(cell.ixf).unwrap();
                        let mut r = json!({"format":format,"blank":blank,"precision":precision,"cover":cover,"op":op,"bytes":bytes(&sheet,70,31),"type":t,"dirty":book.isDirty(),"xfCount":manager.getNumExFormats(),"formatIndex":now.getFormatIndex(),"formatCode":now.getNumFormat(),"wrap":now.getWrapText(),"prefix":now.get123Prefix(),"display":sheet.getFormattedCellValue(70,31)});
                        if t == 1 {
                            let v = book.getCellValueStore().borrow_mut().doubleValue(cell.id);
                            r["bits"] = json!(format!(
                                "{:x}",
                                if v.is_nan() {
                                    0x7ff8000000000000
                                } else {
                                    v.to_bits()
                                }
                            ));
                        }
                        if t == 2 {
                            r["text"] =
                                json!(book.getCellValueStore().borrow_mut().stringValue(cell.id));
                        }
                        rows.push(r);
                    }
                }
            }
        }
    }
    sheet.setFullPrecision(true);
    Value::Array(rows)
}
struct Trace(Rc<RefCell<String>>);
impl ss_core::cn::wps::moss::app::event::KmoBookChangeEventsListener::KmoBookChangeEventsListener for Trace {
    fn bookDidFinishInitializing(&self) {}
    fn bookSettingsDidFinishChanging(&self) { self.0.borrow_mut().push('B'); }
    fn bookContentsDidFinishChanging(&self) {}
    fn sheetDidFinishActivating(&self) { self.0.borrow_mut().push('V'); }
}
impl ss_core::cn::wps::moss::app::event::KmoBookTransListener::KmoBookTransListener for Trace {
    fn onUndo(&self) { self.0.borrow_mut().push('U'); }
    fn onRedo(&self) { self.0.borrow_mut().push('R'); }
}
impl KmoTransactionListener for Trace {
    fn onStarted(&self) {
        self.0.borrow_mut().push('T');
    }
    fn onCommited(&self) {
        self.0.borrow_mut().push('C');
    }
}
impl KmoSheetTransactionListener for Trace {
    fn onCommit(&self) {
        self.0.borrow_mut().push('S');
    }
}
fn record(out: &mut Vec<Value>, book: &Rc<KmoBook>, trace: &Rc<RefCell<String>>) {
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    let r = sheet.getSelectionRect();
    out.push(json!({"trace":trace.borrow().clone(),"started":book.transaction().isTransStarted(),"canUndo":book.canUndo(),"canRedo":book.canRedo(),"bytes":bytes(&sheet,70,31),"selection":[r.tl.row,r.tl.col,r.br.row,r.br.col]}));
}
fn transactions(book: &Rc<KmoBook>) -> Value {
    KmoAtomStore::enable();
    KmoAtomStore::clearUp();
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    let tx = book.transaction();
    tx.reset();
    let trace = Rc::new(RefCell::new(String::new()));
    let listener: Rc<dyn KmoTransactionListener> = Rc::new(Trace(trace.clone()));
    tx.addTransListener(Some(listener.clone()));
    tx.addTransListener(Some(listener.clone()));
    tx.addTransListener(None);
    let sl: Rc<dyn KmoSheetTransactionListener> = Rc::new(Trace(trace.clone()));
    tx.addTransactionListener(sheet.getSheetUid(), Some(sl.clone()));
    tx.addTransactionListener(sheet.getSheetUid(), Some(sl));
    let mut out = vec![];
    tx.commit();
    record(&mut out, book, &trace);
    tx.start();
    tx.commit();
    record(&mut out, book, &trace);
    tx.start();
    sheet.setCellNumericValue(70, 31, 12.5);
    sheet.setSelection__2(&KmoRect::KmoRect__bounds(4, 5, 6, 7));
    tx.commit();
    record(&mut out, book, &trace);
    out.push(json!(tx.back()));
    store(|s| s.undo());
    record(&mut out, book, &trace);
    out.push(json!(tx.forward()));
    store(|s| s.redo());
    record(&mut out, book, &trace);
    tx.start();
    sheet.setCellBoolValue(70, 31, true);
    tx.setEnable(false);
    tx.discard();
    tx.commit();
    record(&mut out, book, &trace);
    tx.setEnable(true);
    tx.commit();
    record(&mut out, book, &trace);
    tx.start();
    sheet.setCellBlank(70, 31);
    tx.discard();
    record(&mut out, book, &trace);
    tx.removeTransListener(Some(&listener));
    tx.start();
    sheet.setCellNumericValue(70, 31, 9.5);
    tx.commit();
    record(&mut out, book, &trace);
    Value::Array(out)
}

fn undo_record(out: &mut Vec<Value>, book: &Rc<KmoBook>, trace: &Rc<RefCell<String>>) {
    let s = book.sheetAt(0); let s = s.borrow();
    out.push(json!({"trace":trace.borrow().clone(),"source":s.getCellNumericValue(70,31),
        "formula":s.getCellNumericValue(72,31),"formulaBytes":bytes(&s,72,31),"dirty":book.isDirty(),
        "autoSaveInterrupted":book.isInterruptAutoSave(),"canUndo":book.canUndo(),"canRedo":book.canRedo(),
        "pendingRanges":s.getUTransactionList().len()}));
}
fn book_undo(book: &Rc<KmoBook>) -> Value {
    let sheet=book.sheetAt(0);let sheet=sheet.borrow();book.transaction().reset();
    sheet.setCellNumericValue(70,31,5.0);sheet.initCellFormula(72,31,"AF71+1",-1,15).unwrap();
    let cc=book.getCalculationChain().unwrap();cc.registerFormula(0,72,31);cc.markRecalculation__area(0,70,72,31,31,true);cc.synCalculate();
    let trace=Rc::new(RefCell::new(String::new()));book.subscribeChangeEvents(Some(Rc::new(Trace(trace.clone()))));
    book.transListenerGroup().addListener(Some(Rc::new(Trace(trace.clone()))));let mut out=vec![];
    book.setDirty(false);trace.borrow_mut().clear();undo_record(&mut out,book,&trace);
    book.transaction().start();sheet.setCellNumericValue(70,31,9.0);book.transaction().commit();
    cc.markRecalculation__area(0,70,70,31,31,true);cc.synCalculate();undo_record(&mut out,book,&trace);
    trace.borrow_mut().clear();book.undo();std::thread::sleep(std::time::Duration::from_millis(200));undo_record(&mut out,book,&trace);
    trace.borrow_mut().clear();book.redo();std::thread::sleep(std::time::Duration::from_millis(200));undo_record(&mut out,book,&trace);
    Value::Array(out)
}
#[test]
fn original_android_raw_results_precision_and_transaction_manager() {
    ss_core::cn::wps::moss::app::KmoLocale::KmoLocale::setLocale("zh_CN".into());
    kfc::cn::wps::kfc::numfmt::i18n::KmoLang::KmoLang::setLocale("zh_CN".into());
    ss_core::cn::wps::moss::app::xf::KmoXFManager::KmoXFManager::loadBuiltinFormats();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let evidence = root.join("rewrite-review/milestones/m4-edit-save/evidence/raw-edit-r0b26");
    let mut expected: Value =
        serde_json::from_slice(&std::fs::read(evidence.join("java.json")).unwrap()).unwrap();
    // org.json serializes whole-valued doubles as integers; restore only the
    // two fields whose Java getters return double before comparing JSON types.
    for row in expected["bookUndo"].as_array_mut().unwrap() {
        for key in ["source", "formula"] { row[key] = json!(row[key].as_f64().unwrap()); }
    }
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
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let actual = json!({"rawResults":raw_results(&book),"transactions":transactions(&book),"bookUndo":book_undo(&book)});
    std::fs::write(
        evidence.join("rust.json"),
        serde_json::to_vec_pretty(&actual).unwrap(),
    )
    .unwrap();
    for key in ["rawResults", "transactions", "bookUndo"] {
        let a = actual[key].as_array().unwrap();
        let e = expected[key].as_array().unwrap();
        assert_eq!(a.len(), e.len(), "{key}");
        for (i, (a, e)) in a.iter().zip(e).enumerate() {
            assert_eq!(a, e, "{key}[{i}]");
        }
    }
}
