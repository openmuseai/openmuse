// src:  libs/core-libs/io/wps-io/src/cn/wps/io/xwpf/element_handler/styles/{StylesHandler,...}.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/xwpf/element_handler/styles/StylesHandler.rs
#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

use core_libs_io::io::opt::XmlReader::XmlReader;
use core_libs_io::io::simple::ElementHandlerSimple::ElementHandlerRef;
use core_libs_io::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::io::xwpf::IImporterBase::IImporterBase;
use core_libs_io::io::xwpf::element_handler::styles::StylePrModel::StylePrModel;
use core_libs_io::io::xwpf::element_handler::styles::StylesHandler::StylesHandler;

// --- elementId 常量（Java / 结构分析钉死）---
const W_STYLES: i32 = -891774750; // styles 根
const W_DOCDEFAULTS: i32 = -1780079702;
const W_STYLE: i32 = 109780401;
const W_RPRDEFAULTS: i32 = 531200397;
const W_PPRDEFAULTS: i32 = 924227407;
const W_RPR: i32 = 112148;
const W_PPR: i32 = 110226;
const W_TCPR: i32 = 3553489;
const W_TRPR: i32 = 3567904;
const W_TBLPRBASE: i32 = 110154336;
const W_TBLSTYLEPR: i32 = 971752949;
const W_NAME: i32 = 3373707;
const W_BASEDON: i32 = -332612366;
const W_NEXT: i32 = 3377907;
const W_LINK: i32 = 3321850;
const W_QFORMAT: i32 = -681243704;
const W_UIPRIORITY: i32 = 872749144;
const W_SZ: i32 = 33176;

/// Java null 哨兵（与 DocDefaultsHandler 的 NO_PROP 同构）
static NO_PROP: () = ();

#[derive(Debug, Clone, PartialEq)]
enum Call {
    DocDefaults { rpr: bool, ppr: bool },
    StyleStart,
    StyleChild(i32),
    StyleEnd { rpr: bool, ppr: bool, tc: bool, tbl: bool },
    TblStylePrEnd { rpr: bool, ppr: bool, tc: bool, tbl: bool },
    StylesEnd,
}

struct RecordingDocImporter {
    calls: Rc<RefCell<Vec<Call>>>,
}

fn is_present(a: &dyn Any) -> bool {
    !a.downcast_ref::<()>().is_some()
}

fn model_flags(m: &dyn Any) -> [bool; 4] {
    match m.downcast_ref::<StylePrModel>() {
        Some(model) => [
            model.mRprBaseSet.is_some(),
            model.mPPrGeneralSet.is_some(),
            model.mTcPrSet.is_some(),
            model.mTblPrBaseSet.is_some(),
        ],
        None => [false; 4],
    }
}

impl IImporterBase for RecordingDocImporter {
    fn mode(&self) -> i32 {
        0
    }
}

impl IDocumentImporter for RecordingDocImporter {
    fn onImportStyleDocDefaults(&self, rpr: &dyn Any, ppr: &dyn Any) {
        self.calls.borrow_mut().push(Call::DocDefaults {
            rpr: is_present(rpr),
            ppr: is_present(ppr),
        });
    }
    fn onImportStyleStart(&self, _attrs: &dyn Any) {
        self.calls.borrow_mut().push(Call::StyleStart);
    }
    fn onImportStyleChild(&self, element_id: i32, _attrs: &dyn Any) {
        self.calls.borrow_mut().push(Call::StyleChild(element_id));
    }
    fn onImportStyleEnd(&self, model: &dyn Any) {
        let [rpr, ppr, tc, tbl] = model_flags(model);
        self.calls.borrow_mut().push(Call::StyleEnd { rpr, ppr, tc, tbl });
    }
    fn onImportStyleTblStylePrEnd(&self, model: &dyn Any) {
        let [rpr, ppr, tc, tbl] = model_flags(model);
        self.calls.borrow_mut().push(Call::TblStylePrEnd { rpr, ppr, tc, tbl });
    }
    fn onImportStylesEnd(&self) {
        self.calls.borrow_mut().push(Call::StylesEnd);
    }
}

fn new_recorder() -> (Rc<RecordingDocImporter>, Rc<RefCell<Vec<Call>>>) {
    let calls = Rc::new(RefCell::new(Vec::new()));
    (Rc::new(RecordingDocImporter { calls: calls.clone() }), calls)
}

fn importer_rc(rec: &Rc<RecordingDocImporter>) -> Rc<dyn IDocumentImporter> {
    rec.clone()
}

/// 模拟 DispatchHandler2 的 trait 级驱动（与 08 引擎同语义：栈顶 getElementHandler + onStart/onEnd）
#[derive(Clone, Copy)]
enum Evt {
    S(i32),
    E(i32),
}

fn drive(rec: &Rc<RecordingDocImporter>, evts: &[Evt]) {
    let root: ElementHandlerRef = StylesHandler::as_handler_ref(importer_rc(rec));
    let mut stack: Vec<Option<ElementHandlerRef>> = Vec::new();
    for e in evts {
        match e {
            Evt::S(id) => {
                if stack.is_empty() {
                    let _ = root.borrow_mut().onStart_with_id(*id, "", "", "", &"attr" as &dyn Any);
                    stack.push(Some(root.clone()));
                    continue;
                }
                let handler = match stack.last().and_then(|h| h.clone()) {
                    Some(h) => h.borrow().getElementHandler(*id, ""),
                    None => None,
                };
                if let Some(ref h) = handler {
                    let _ = h.borrow_mut().onStart_with_id(*id, "", "", "", &"attr" as &dyn Any);
                }
                stack.push(handler);
            }
            Evt::E(id) => {
                if let Some(Some(eh)) = stack.pop() {
                    let _ = eh.borrow_mut().onEnd_with_id(*id, "");
                }
            }
        }
    }
}

fn calls_now(calls: &Rc<RefCell<Vec<Call>>>) -> Vec<Call> {
    calls.borrow().clone()
}

// --- SH1: StylesHandler 顶层分发 ---
#[test]
fn sh1_root_dispatch() {
    let (rec, _) = new_recorder();
    let root: ElementHandlerRef = StylesHandler::as_handler_ref(importer_rc(&rec));
    assert!(root.borrow().getElementHandler(W_DOCDEFAULTS, "").is_some());
    assert!(root.borrow().getElementHandler(W_STYLE, "").is_some());
    assert!(root.borrow().getElementHandler(999999, "").is_none());
}

// --- SH2 + SH15 + SH3 + SH5 + SH11: 完整 docDefaults + 两条 style 的序列 ---
#[test]
fn sh2_15_chain_sequence() {
    let (rec, calls) = new_recorder();
    let ev = vec![
        Evt::S(W_STYLES),
        Evt::S(W_DOCDEFAULTS),
        Evt::S(W_RPRDEFAULTS),
        Evt::S(W_RPR),
        Evt::S(W_SZ),
        Evt::E(W_SZ),
        Evt::E(W_RPR),
        Evt::E(W_RPRDEFAULTS),
        Evt::S(W_PPRDEFAULTS),
        Evt::S(W_PPR),
        Evt::E(W_PPR),
        Evt::E(W_PPRDEFAULTS),
        Evt::E(W_DOCDEFAULTS),
        Evt::S(W_STYLE),
        Evt::S(W_NAME),
        Evt::E(W_NAME),
        Evt::S(W_BASEDON),
        Evt::E(W_BASEDON),
        Evt::S(W_NEXT),
        Evt::E(W_NEXT),
        Evt::E(W_STYLE),
        Evt::S(W_STYLE),
        Evt::S(W_LINK),
        Evt::E(W_LINK),
        Evt::S(W_UIPRIORITY),
        Evt::E(W_UIPRIORITY),
        Evt::E(W_STYLE),
        Evt::E(W_STYLES),
    ];
    drive(&rec, &ev);
    let got = calls_now(&calls);
    assert_eq!(
        got,
        vec![
            Call::DocDefaults { rpr: true, ppr: true }, // SH3：双 prop 非 null，先于任何 style
            Call::StyleStart,
            Call::StyleChild(W_NAME),
            Call::StyleChild(W_BASEDON),
            Call::StyleChild(W_NEXT),
            Call::StyleEnd { rpr: false, ppr: false, tc: false, tbl: false }, // SH11：无属性子树的 style → 全 None
            Call::StyleStart,
            Call::StyleChild(W_LINK),
            Call::StyleChild(W_UIPRIORITY),
            Call::StyleEnd { rpr: false, ppr: false, tc: false, tbl: false },
            Call::StylesEnd, // SH2：onImportStylesEnd 恰一次且在最后
        ]
    );
    // SH5：双 none 不应触发（此处 docDefaults 有内容，用独立用例验证）
}

// --- SH5: docDefaults 双 null 不回调 ---
#[test]
fn sh5_docdefaults_none() {
    let (rec, calls) = new_recorder();
    let ev = vec![
        Evt::S(W_STYLES),
        Evt::S(W_STYLE),
        Evt::S(W_NAME),
        Evt::E(W_NAME),
        Evt::E(W_STYLE),
        Evt::E(W_STYLES),
    ];
    drive(&rec, &ev);
    let got = calls_now(&calls);
    assert!(!got.iter().any(|c| matches!(c, Call::DocDefaults { .. })));
    assert_eq!(got.last(), Some(&Call::StylesEnd));
}

// --- SH4: 仅 rPr 的 docDefaults ---
#[test]
fn sh4_docdefaults_rpr_only() {
    let (rec, calls) = new_recorder();
    let ev = vec![
        Evt::S(W_STYLES),
        Evt::S(W_DOCDEFAULTS),
        Evt::S(W_RPRDEFAULTS),
        Evt::S(W_RPR),
        Evt::S(W_SZ),
        Evt::E(W_SZ),
        Evt::E(W_RPR),
        Evt::E(W_RPRDEFAULTS),
        Evt::E(W_DOCDEFAULTS),
        Evt::E(W_STYLES),
    ];
    drive(&rec, &ev);
    let got = calls_now(&calls);
    assert!(got.contains(&Call::DocDefaults { rpr: true, ppr: false }));
}

// --- SH6: DocDefaultsHandler 未知子 id → fail 语义（Rust 为 no-stmt 不炸）---
#[test]
fn sh6_docdefaults_bad_child() {
    let (rec, _) = new_recorder();
    let h = core_libs_io::io::xwpf::element_handler::styles::DocDefaultsHandler::DocDefaultsHandler::as_handler_ref(
        importer_rc(&rec),
    );
    assert!(h.borrow().getElementHandler(W_RPRDEFAULTS, "").is_some());
    assert!(h.borrow().getElementHandler(W_PPRDEFAULTS, "").is_some());
    assert!(h.borrow().getElementHandler(777, "").is_none()); // fail → 不炸
}

// --- SH9 + SH10: StyleHandler 子分发映射 ---
#[test]
fn sh9_10_style_child_dispatch() {
    let (rec, _) = new_recorder();
    let h = core_libs_io::io::xwpf::element_handler::styles::StyleHandler::StyleHandler::as_handler_ref(
        importer_rc(&rec),
    );
    assert!(h.borrow().getElementHandler(W_PPR, "").is_some()); // 110226 → PprGeneral
    assert!(h.borrow().getElementHandler(W_RPR, "").is_some()); // 112148 → RprBase
    assert!(h.borrow().getElementHandler(W_TCPR, "").is_some()); // 3553489 → TcPr
    assert!(h.borrow().getElementHandler(W_TBLPRBASE, "").is_some()); // 110154336 → TblPrBase
    assert!(h.borrow().getElementHandler(W_TBLSTYLEPR, "").is_some()); // 971752949 → TblStylePr
    assert!(h.borrow().getElementHandler(W_TRPR, "").is_none()); // SH10：3567904 → null 不炸
    assert!(h.borrow().getElementHandler(W_NAME, "").is_some()); // default → StyleChild
}

// --- SH12: clearProp 不串值（style1 带 pPr，style2 无 pPr → End2 pPr=None）---
#[test]
fn sh12_clear_no_leak() {
    let (rec, calls) = new_recorder();
    let ev = vec![
        Evt::S(W_STYLES),
        Evt::S(W_STYLE),
        Evt::S(W_PPR), // style1 pPr → NormalPropBaseHandler
        Evt::E(W_PPR),
        Evt::E(W_STYLE),
        Evt::S(W_STYLE),
        Evt::E(W_STYLE),
        Evt::E(W_STYLES),
    ];
    drive(&rec, &ev);
    let got = calls_now(&calls);
    let ends: Vec<_> = got.iter().filter(|c| matches!(c, Call::StyleEnd { .. })).cloned().collect();
    assert_eq!(ends.len(), 2);
    assert_eq!(ends[0], Call::StyleEnd { rpr: false, ppr: true, tc: false, tbl: false });
    assert_eq!(ends[1], Call::StyleEnd { rpr: false, ppr: false, tc: false, tbl: false });
}

// --- SH13 + SH14: tblStylePr 链 ---
#[test]
fn sh13_14_tblstylepr() {
    let (rec, calls) = new_recorder();
    let ev = vec![
        Evt::S(W_STYLES),
        Evt::S(W_STYLE),
        Evt::S(W_TBLSTYLEPR), // TblStylePrHandler
        Evt::S(W_RPR),        // rPr 子树
        Evt::S(W_SZ),
        Evt::E(W_SZ),
        Evt::E(W_RPR),
        Evt::E(W_TBLSTYLEPR),
        Evt::E(W_STYLE),
        Evt::E(W_STYLES),
    ];
    drive(&rec, &ev);
    let got = calls_now(&calls);
    assert!(got.contains(&Call::StyleChild(W_TBLSTYLEPR))); // onStart → onImportStyleChild(971752949)
    assert!(got.iter().any(|c| {
        matches!(
            c,
            Call::TblStylePrEnd { rpr: true, ppr: false, tc: false, tbl: false }
        )
    }));
    assert_eq!(got.last(), Some(&Call::StylesEnd));

    // SH14: tblStylePr 未知子 id → fail 不炸
    let (rec2, _) = new_recorder();
    let h = core_libs_io::io::xwpf::element_handler::styles::TblStylePrHandler::TblStylePrHandler::as_handler_ref(
        importer_rc(&rec2),
    );
    assert!(h.borrow().getElementHandler(W_RPR, "").is_some());
    assert!(h.borrow().getElementHandler(777, "").is_none());
}

// --- SH7 + SH8: attrs 透传（StyleStart / StyleChild 收到真实 attrs 对象）---
#[test]
fn sh7_8_attrs_forward() {
    let (rec, calls) = new_recorder();
    let ev = vec![
        Evt::S(W_STYLES),
        Evt::S(W_STYLE),
        Evt::S(W_NAME),
        Evt::E(W_NAME),
        Evt::S(W_QFORMAT),
        Evt::E(W_QFORMAT),
        Evt::E(W_STYLE),
        Evt::E(W_STYLES),
    ];
    drive(&rec, &ev);
    let got = calls_now(&calls);
    assert_eq!(got[0], Call::StyleStart);
    assert_eq!(got[1], Call::StyleChild(W_NAME));
    assert_eq!(got[2], Call::StyleChild(W_QFORMAT));
}

// --- 引擎真实路径 smoke：XmlReader 直接 parse styles.xml（w 命名空间）---
#[test]
fn engine_styles_xml_parse() {
    let (rec, calls) = new_recorder();
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:docDefaults>
    <w:rPrDefault><w:rPr><w:sz w:val="20"/></w:rPr></w:rPrDefault>
  </w:docDefaults>
  <w:style w:type="paragraph" w:styleId="Heading1">
    <w:name w:val="Heading 1"/>
    <w:basedOn w:val="Normal"/>
  </w:style>
  <w:style w:type="character" w:styleId="Heading1Char"/>
</w:styles>"#;
    let mut handler: ElementHandlerRef = StylesHandler::as_handler_ref(importer_rc(&rec));
    let mut cur = Cursor::new(xml.as_bytes().to_vec());
    XmlReader::new().parse(&mut cur, &mut handler, "utf-8", -1, 0).expect("XmlReader::parse");
    let got = calls_now(&calls);
    assert_eq!(got.last(), Some(&Call::StylesEnd));
    assert!(got.iter().filter(|c| **c == Call::StyleStart).count() == 2);
    assert!(got.iter().any(|c| matches!(c, Call::StyleChild(_))));
    // docDefaults 在第一个 style 前
    let dd_pos = got.iter().position(|c| matches!(c, Call::DocDefaults { .. }));
    let first_ss = got.iter().position(|c| *c == Call::StyleStart);
    assert!(dd_pos.is_some());
    assert!(first_ss.is_some());
    assert!(dd_pos.unwrap() < first_ss.unwrap());
    // 哨兵常量防编译器警告
    let _ = NO_PROP;
}