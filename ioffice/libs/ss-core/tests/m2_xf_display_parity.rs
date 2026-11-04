use kfc::cn::wps::{
    kfc::numfmt::{
        format::NumFmtResult::NumFmtResult, i18n::KmoLang::KmoLang,
        usermodel::NumberFormatter::NumberFormatter,
    },
    memor4j::KmoAtomStore::{KmoAtomStore, KmoAtomStoreTrait},
};
use ss_core::{
    api::SpreadsheetSession::{SpreadsheetLoadConfig, SpreadsheetSession},
    cn::wps::moss::{
        app::{xf::KmoXFManager::KmoXFManager, KmoBookApi::KmoBookApi, KmoLocale::KmoLocale},
        engine::grid::KmoCellData::KmoCellData,
    },
    java_runtime::Rc,
};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};
static SERIAL: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
fn locale() {
    KmoLocale::setLocale("zh_CN".into());
    KmoLang::setLocale("zh_CN".into());
    KmoXFManager::loadBuiltinFormats();
}
fn sample() -> String {
    format!(
        "{}/../../samples/ss-render/出入库管理明细表.xlsx",
        env!("CARGO_MANIFEST_DIR")
    )
}
fn open_fixture() -> Rc<KmoBookApi> {
    let api = KmoBookApi::KmoBookApi();
    api.book().setAutoCalcOn(false);
    assert_eq!(
        api.open(
            &format!(
                "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/basic-types.xlsx",
                env!("CARGO_MANIFEST_DIR")
            ),
            None,
            true
        ),
        1
    );
    api
}
fn hex(s: &str) -> String {
    s.encode_utf16().map(|u| format!("{u:04x}")).collect()
}
fn unhex(s: &str) -> String {
    String::from_utf16(
        &(0..s.len())
            .step_by(4)
            .map(|i| u16::from_str_radix(&s[i..i + 4], 16).unwrap())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}
#[test]
fn original_java_numfmt_4088_results_and_metadata_are_identical() {
    let _guard = SERIAL.lock().unwrap();
    let evidence =
        include_str!("../rewrite-review/milestones/m2-xf-display/evidence/java-numfmt-r0b22.tsv");
    let mut count = 0;
    for line in evidence.lines().chain(
        include_str!(
            "../rewrite-review/milestones/m2-xf-display/evidence/java-numfmt-extra-r0b22.tsv"
        )
        .lines(),
    ) {
        let a: Vec<_> = line.split('\t').collect();
        let mut formatter = NumberFormatter::createInstance(a[0].parse().unwrap());
        let mut result = NumFmtResult::new();
        if a[1] == "v" {
            result.mNumFmtStr = format!(
                "{:016x}",
                formatter
                    .formatValue(
                        f64::from_bits(u64::from_str_radix(a[2], 16).unwrap()),
                        a[5] == "true",
                        &unhex(a[3])
                    )
                    .to_bits()
            );
        } else if a[1] == "n" {
            formatter.format_num(
                f64::from_bits(u64::from_str_radix(a[2], 16).unwrap()),
                &unhex(a[3]),
                a[4].parse().unwrap(),
                a[5] == "true",
                &mut result,
            );
        } else {
            formatter.format_str(
                &unhex(a[2]),
                &unhex(a[3]),
                a[4].parse().unwrap(),
                &mut result,
            );
        }
        assert_eq!(
            vec![
                hex(&result.mNumFmtStr),
                hex(&result.mNumRealValue),
                result
                    .mNumFmtColorIdx
                    .map_or_else(|| "null".into(), |c| c.to_string()),
                result.richFormatInvalid.to_string(),
                result.mNumericAsText.to_string(),
                result.mFillPos.to_string(),
                (result.mFillChar as u32).to_string()
            ],
            a[6..].iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            "LCID={} value={} format={} width={}",
            a[0],
            a[2],
            unhex(a[3]),
            a[4]
        );
        count += 1;
    }
    assert_eq!(count, 4088);
}
#[test]
fn java_sample_all_110_xfs_and_296_display_texts_match() {
    let _guard = SERIAL.lock().unwrap();
    locale();
    let session = SpreadsheetSession::open_path(&sample(), SpreadsheetLoadConfig::default());
    let snapshot = session.snapshot(0, 0, 40, 24);
    assert_eq!(
        snapshot.input_sha256,
        "611d9b930f6daa00f8fcf2fe0ee3465e09b779fd8a843d604dab53b3b2a336bc"
    );
    assert_eq!(snapshot.xf_count, 110);
    assert_eq!(snapshot.cell_count, 296);
    assert_eq!(snapshot.sheets[0].merged_ranges.len(), 9);
    let java: serde_json::Value = serde_json::from_str(include_str!(
        "../../../samples/ss-render/sample-java-dump.json"
    ))
    .unwrap();
    let cells: HashMap<_, _> = snapshot.sheets[0]
        .cells
        .iter()
        .map(|c| ((c.row, c.col), c))
        .collect();
    for expected in java["sheets"][0]["cells"].as_array().unwrap() {
        let row = expected["row"].as_i64().unwrap() as i32;
        let col = expected["col"].as_i64().unwrap() as i32;
        let actual = cells[&(row, col)];
        assert_eq!(
            actual.display_text,
            expected["displayText"].as_str().unwrap(),
            "display at {row},{col}"
        );
        assert_eq!(actual.ixf, expected["ixf"].as_i64().unwrap() as i32);
    }
    assert_eq!(cells[&(7, 1)].display_text, "5月17日");
    // A separate book observer compares every field collected by the Java XF probe.
    let api = KmoBookApi::KmoBookApi();
    api.book().setAutoCalcOn(false);
    assert_eq!(api.open(&sample(), None, true), 1);
    let book = api.book();
    let manager = book.getXFManager().borrow();
    let java: serde_json::Value = serde_json::from_str(include_str!(
        "../rewrite-review/milestones/m2-xf-display/evidence/java-sample-xf-r0b16.json"
    ))
    .unwrap();
    for expected in java["xfRecords"].as_array().unwrap() {
        let index = expected["index"].as_i64().unwrap() as i32;
        let x = manager.getXF(index).unwrap();
        let font = x.getFont().unwrap();
        let actual = serde_json::json!({"index":index,"present":true,"class":if x.getXFType()==1{"KmoStyleXF"}else{"KmoCellXF"},"fontIndex":x.getFontIndex(),"formatIndex":x.getFormatIndex(),"numFormat":x.getNumFormat(),"alignment":x.getAlignment(),"verticalAlignment":x.getVerticalAlignment(),"wrapText":x.getWrapText(),"locked":x.isLocked(),"hidden":x.isHidden(),"borderLeft":x.getBorderLeft(),"borderRight":x.getBorderRight(),"borderTop":x.getBorderTop(),"borderBottom":x.getBorderBottom(),"fontName":font.getFontName(),"fontHeight":font.getFontHeight()});
        assert_eq!(&actual, expected, "XF {index}");
    }
}
#[test]
fn live_xf_and_font_atoms_keep_commit_undo_redo() {
    let _guard = SERIAL.lock().unwrap();
    locale();
    KmoAtomStore::enable();
    KmoAtomStore::clearUp();
    let api = open_fixture();
    let book = api.book();
    let xf = book.getXFManager().borrow().getXF(15).unwrap();
    let font = book.getXFManager().borrow().getFont(0);
    let oldHeight = font.getFontHeight();
    let oldFormat = xf.getFormatIndex();
    let store = |f: &mut dyn FnMut(&mut dyn KmoAtomStoreTrait)| {
        let mut store = KmoAtomStore::sharedStore();
        f(store.as_mut().unwrap().as_mut());
    };
    store(&mut |s| s.start());
    xf.setFormatIndex(58);
    font.setFontHeight(480);
    store(&mut |s| assert!(s.commit()));
    assert_eq!(xf.getFormatIndex(), 58);
    assert_eq!(font.getFontHeight(), 480);
    store(&mut |s| {
        s.undo();
    });
    assert_eq!(xf.getFormatIndex(), oldFormat);
    assert_eq!(font.getFontHeight(), oldHeight);
    store(&mut |s| {
        s.redo();
    });
    assert_eq!(xf.getFormatIndex(), 58);
    assert_eq!(font.getFontHeight(), 480);
    assert!(Rc::ptr_eq(
        &xf,
        &book.getXFManager().borrow().getXF(15).unwrap()
    ));
    KmoAtomStore::clearUp();
}
#[test]
fn sheet_format_cache_conditional_override_zeros_and_precision_follow_java() {
    let _guard = SERIAL.lock().unwrap();
    locale();
    let api = open_fixture();
    let book = api.book();
    let sheet = book.sheetAt(0);
    let first = sheet.borrow().getNumFormatResult(0, 0, i32::MAX);
    assert_eq!(first.borrow().numfmtStr(), "1.5");
    let second = sheet.borrow().getNumFormatResult(0, 1, i32::MAX);
    assert!(Rc::ptr_eq(&first, &second));
    assert_eq!(first.borrow().numfmtStr(), "TRUE");
    use ss_core::cn::wps::moss::{
        app::condfmt::{
            rule::Rule::Rule,
            KmoCfRule::KmoCfRule,
            KmoConditionFormatter::{CfCacheItem, CfRuleResult},
        },
        filefmt::biff8::record::cf::NumberFormatFormatting::NumberFormatFormatting,
        util::AppLogicHelper::AppLogicHelper,
    };
    let rule = Rc::new(Rule::new(book.getSpreadsheetVersion()));
    rule.setNumFmt(Some(Rc::new(NumberFormatFormatting::new(
        2,
        Some("0.00".into()),
    ))));
    let cached = Rc::new(CfCacheItem {
        rules:ss_core::java_runtime::RefCell::new(CfRuleResult {
            generals: Some(vec![Rc::new(KmoCfRule::fromRule(rule))]),..Default::default()
        }),
        tip:ss_core::java_runtime::RefCell::new(None),
    });
    sheet
        .borrow()
        .getCFormatter()
        .getResultCache()
        .put(AppLogicHelper::rc2long(0, 0), Some(cached));
    assert_eq!(sheet.borrow().getFormattedCellValue(0, 0), "1.50");
    sheet.borrow().getCFormatter().getResultCache().clear();
    assert_eq!(sheet.borrow().getFormattedCellValue(0, 0), "1.5");
    let id = book.getCellValueStore().borrow_mut().newDoubleValue(0.0);
    let mut data = sheet.borrow().getCellData(0, 0);
    data.id = id;
    sheet.borrow().setCellData(0, 0, &data);
    sheet.borrow().getWindow2().setIsShowZeros(false);
    assert_eq!(
        sheet
            .borrow()
            .getNumFormatResult(0, 0, 8)
            .borrow()
            .numfmtStr(),
        ""
    );
    assert_eq!(sheet.borrow().getFormattedCellValue(0, 0), "0");
    use ss_core::cn::wps::{
        et::ss::{formula::eval::NumberEval::NumberEval, interfaces::IEvalWorkbook::IEvalWorkbook},
        moss::{
            engine::eval::EvaluationWorkbook::EvaluationWorkbook,
            filefmt::biff8::record::ExtendedFormatRecord::ExtendedFormatRecord,
        },
    };
    let mut rec = ExtendedFormatRecord::ExtendedFormatRecord();
    rec.setFormatIndex(2);
    let ixf = book.getXFManager().borrow().createXF(&rec);
    data.ixf = ixf;
    sheet.borrow().setCellData(0, 0, &data);
    let ew = EvaluationWorkbook::EvaluationWorkbook(&book);
    let result = ew
        .getPrecisionValue(Rc::new(NumberEval::NumberEval(1.005)), 0, 0, 0)
        .unwrap_or_else(|_| panic!("precision formatting failed"));
    assert_eq!(
        result
            .as_any()
            .downcast_ref::<NumberEval>()
            .unwrap()
            .getNumberValue(),
        1.01
    );
}

#[test]
fn java_style_references_gradient_copy_modes_and_option_cache_are_preserved() {
    let _guard = SERIAL.lock().unwrap();
    locale();
    use ss_core::{
        cn::wps::moss::{
            app::xf::{KmoFont::KmoFont, KmoXF::KmoXF},
            filefmt::biff8::record::{
                cf::PatternFormatting::PatternFormatting,
                common::XFGradient::{ColorStop, XFGradient},
                ExtendedFormatRecord::ExtendedFormatRecord,
                FontRecord::FontRecord,
                FormatRecord::FormatRecord,
            },
            xlsxr::model::StylesTable::StylesTable,
        },
        java_runtime::RefCell,
    };
    let stop = Rc::new(RefCell::new(ColorStop {
        position: 0.0,
        color: 0x112233,
    }));
    let gradient = Rc::new(RefCell::new(XFGradient {
        isLinear: true,
        degree: 90,
        colorStops: vec![stop.clone()],
        ..Default::default()
    }));
    let mut rec = ExtendedFormatRecord::ExtendedFormatRecord();
    rec.setXFGradient(Some(gradient.clone()));
    let xf = KmoXF::fromRecord(&rec);
    assert!(Rc::ptr_eq(&xf.getXFGradient().unwrap(), &gradient));
    let copy = KmoXF::createXF(Some(&xf));
    let copied = copy.getXFGradient().unwrap();
    assert!(!Rc::ptr_eq(&copied, &gradient));
    assert!(Rc::ptr_eq(&copied.borrow().colorStops[0], &stop));
    let mut pattern = PatternFormatting::PatternFormatting();
    pattern.setXfGradient(Some(gradient.clone()));
    let cloned = pattern.clone();
    let deep = cloned.getXFGradient().unwrap();
    assert!(!Rc::ptr_eq(&deep, &gradient));
    assert!(!Rc::ptr_eq(&deep.borrow().colorStops[0], &stop));
    stop.borrow_mut().color = 123;
    assert_eq!(copied.borrow().colorStops[0].borrow().color, 123);
    assert_eq!(deep.borrow().colorStops[0].borrow().color, 0x112233);
    let table = StylesTable::StylesTable();
    let pattern = Rc::new(RefCell::new(pattern));
    table.addPatternFill(Some(pattern.clone()));
    assert!(Rc::ptr_eq(&table.getPatternFillAt(0).unwrap(), &pattern));
    pattern.borrow_mut().setFillPattern(3);
    assert_eq!(
        table.getPatternFillAt(0).unwrap().borrow().getFillPattern(),
        3
    );
    let mut font = FontRecord::FontRecord();
    font.setAttributes(192);
    let font = KmoFont::fromRecord(&font);
    assert!(font.isCondense() && font.isExtend());
    font.initOption();
    assert!(font.isCondense() && font.isExtend());
    let copy = KmoFont::createFont(&font);
    assert!(!copy.isCondense() && !copy.isExtend());
    let manager = KmoXFManager::KmoXFManager();
    let first = manager.checkFormat(None);
    let second = manager.checkFormat(None);
    assert_ne!(first, second);
    assert!(manager
        .getFormat(first)
        .unwrap()
        .getFormatString()
        .is_none());
    let record = FormatRecord::FormatRecord(200, Some("0.0000".into()));
    manager.createFormat(&record);
    assert_eq!(manager.checkFormat(Some("0.0000")), 200);
    assert_eq!(manager.checkFormat(Some("0.00000")), 201);
    let api = open_fixture();
    let x = api.book().getXFManager().borrow().getXF(15).unwrap();
    x.setRotation(45);
    x.setAlignment(3);
    let before = (x.getRotation(), x.getAlignment(), x.getAdtlFillPattern());
    api.book().getXFManager().borrow().iniXFOption();
    assert_eq!(
        (x.getRotation(), x.getAlignment(), x.getAdtlFillPattern()),
        before
    );
}

#[test]
fn original_java_theme_colors_and_tints_match_the_real_theme_reader() {
    let _guard = SERIAL.lock().unwrap();
    locale();
    use ss_core::cn::wps::moss::core::theme::ColorScheme::ColorScheme;
    ColorScheme::clean();
    let rows = include_str!(
        "../rewrite-review/milestones/m2-xf-display/evidence/java-theme-color-r0b22.tsv"
    );
    let mut api = None;
    let mut count = 0;
    for row in rows.lines() {
        let a: Vec<_> = row.split('\t').collect();
        if a[0] == "1" && api.is_none() {
            let book = KmoBookApi::KmoBookApi();
            book.book().setAutoCalcOn(false);
            assert_eq!(book.open(&sample(), None, true), 1);
            api = Some(book);
        }
        let tint = f32::from_bits(u32::from_str_radix(a[2], 16).unwrap());
        assert_eq!(
            ColorScheme::getColor(a[1].parse().unwrap(), tint),
            a[3].parse::<i32>().unwrap(),
            "loaded={} theme={} tint={tint}",
            a[0],
            a[1]
        );
        count += 1;
    }
    assert_eq!(count, 288);
}

#[test]
fn dxf_records_keep_java_indices_gradients_and_font_default_height() {
    let _guard = SERIAL.lock().unwrap();
    locale();
    use ss_core::{
        cn::wps::moss::{
            app::{KmoBook::KmoBook, KmoBooks::KmoBooks},
            xlsxr::{
                handler::style::StylesHandler::StylesHandler, model::StylesTable::StylesTable,
                util::UnifiedXmlReader::UnifiedXmlReader,
            },
        },
        java_runtime::RefCell,
    };
    use std::io::Read;
    let mut zip = zip::ZipArchive::new(std::fs::File::open(sample()).unwrap()).unwrap();
    let mut xml = Vec::new();
    zip.by_name("xl/styles.xml")
        .unwrap()
        .read_to_end(&mut xml)
        .unwrap();
    let book = KmoBook::KmoBook();
    KmoBooks::loadTemplates(&book);
    let table = Rc::new(RefCell::new(StylesTable::StylesTable()));
    UnifiedXmlReader::read(
        &mut xml.as_slice(),
        Box::new(StylesHandler::StylesHandler(book.clone(), table.clone())),
    )
    .unwrap();
    let table_ref = table.borrow();
    assert_eq!(table_ref.getDxfListSize(), 1);
    assert_eq!(table_ref.getpatternFillListsize(), 36);
    let dxf = table_ref.getdxf(0).unwrap();
    assert_eq!(
        (
            dxf.getPatid(),
            dxf.getBordid(),
            dxf.getFontid(),
            dxf.getNumFmtId()
        ),
        (35, -1, -1, -1)
    );
    let pattern = table_ref.getPatternFillAt(35).unwrap();
    let gradient = pattern.borrow().getXFGradient().unwrap();
    let gradient = gradient.borrow();
    assert!(gradient.isLinear);
    assert_eq!(gradient.degree, 90);
    assert_eq!(gradient.colorStops.len(), 2);
    assert_eq!(gradient.colorStops[0].borrow().color, 0xff80d5c6u32 as i32);
    assert_eq!(gradient.colorStops[1].borrow().color, 0xff4ea49au32 as i32);
    drop(gradient);
    drop(table_ref);
    let xml=br#"<styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><dxfs count="2"><dxf><font><b/><i/><sz val="20"/></font><numFmt numFmtId="200" formatCode="0.00000"/><fill><patternFill patternType="solid"><fgColor rgb="FF112233"/></patternFill></fill><border><left style="thin"><color rgb="FF445566"/></left></border></dxf><dxf><font><color rgb="FF123456"/></font></dxf></dxfs></styleSheet>"#;
    let table = Rc::new(RefCell::new(StylesTable::StylesTable()));
    UnifiedXmlReader::read(
        &mut xml.as_slice(),
        Box::new(StylesHandler::StylesHandler(book.clone(), table.clone())),
    )
    .unwrap();
    let table = table.borrow();
    assert_eq!(table.getDxfListSize(), 2);
    let first = table.getdxf(0).unwrap();
    assert_eq!(
        (
            first.getFontid(),
            first.getBordid(),
            first.getPatid(),
            first.getNumFmtId()
        ),
        (0, 0, 0, 164)
    );
    assert_eq!(table.getFontRecordAt(0).borrow().getFontHeight(), 400);
    assert_eq!(table.getFontRecordAt(0).borrow().getBoldWeight(), 700);
    assert!(table.getFontRecordAt(0).borrow().isItalic());
    let second = table.getdxf(1).unwrap();
    assert_eq!(
        (
            second.getFontid(),
            second.getBordid(),
            second.getPatid(),
            second.getNumFmtId()
        ),
        (1, -1, -1, -1)
    );
    assert_eq!(table.getFontRecordAt(1).borrow().getFontHeight(), -1);
    assert_eq!(
        table.getFontRecordAt(1).borrow().getColorPaletteIndex(),
        0xff123456u32 as i32
    );
    assert!(Rc::ptr_eq(
        &table.getFontRecordAt(1),
        &table.getFontRecordAt(1)
    ));
}
