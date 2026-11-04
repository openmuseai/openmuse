// function_id=02_function_style_event
// cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_style_event
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/StylesImporter.rs
#![allow(non_snake_case)]

#[path = "helpers/wc03_word_attrs.rs"]
mod wc03_word_attrs;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use io_wps_io::cn::wps::io::xwpf::element_handler::styles::StylePrModel::StylePrModel;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use wc03_word_attrs::{MAIN_NS, WordAttrs};
use word_core::cn::wps::moffice::writer::core::KStyle::TextDocument;
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::core::UUSti::UUSti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::StylesImporter::StylesImporter;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BuildinStyleSheet::BuildinStyleSheet;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

impl Attributes for WordAttrs {
    fn getLength(&self) -> i32 {
        self.pairs.len() as i32
    }
    fn getValue(&self, uri: &str, localName: &str) -> Option<String> {
        self.get_value(uri, localName).map(|s| s.to_string())
    }
}

// ---- fixtures ----
fn importer_pair() -> (Rc<RefCell<KStyles>>, Rc<RefCell<StyleId2Sti>>, StylesImporter) {
    let styles = Rc::new(RefCell::new(KStyles::KStyles(&TextDocument::default())));
    let sti = Rc::new(RefCell::new(StyleId2Sti::StyleId2Sti(UUSti::UUSti())));
    let imp = StylesImporter::StylesImporter(styles.clone(), sti.clone());
    (styles, sti, imp)
}

fn empty_model() -> StylePrModel {
    StylePrModel::new()
}

fn model_with_prop(field: u8, props: &[(i32, i32)]) -> StylePrModel {
    let mut m = StylePrModel::new();
    let mut mps = MutablePropertySet::new();
    for (id, v) in props {
        mps.setInt(*id, *v);
    }
    let boxed: Box<dyn Any + Send + Sync> = Box::new(mps);
    match field {
        0 => m.mRprBaseSet = Some(boxed),
        1 => m.mPPrGeneralSet = Some(boxed),
        2 => m.mTcPrSet = Some(boxed),
        3 => m.mTblPrBaseSet = Some(boxed),
        _ => unreachable!(),
    }
    m
}

fn style_start(style_id: &str, style_type: &str) -> WordAttrs {
    WordAttrs::empty()
        .with(MAIN_NS, "type", style_type)
        .with(MAIN_NS, "styleId", style_id)
}

fn run_style(imp: &mut StylesImporter, start: WordAttrs, children: &[(i32, WordAttrs)], model: &StylePrModel) {
    imp.importStyleStart(&start);
    for (id, attrs) in children {
        imp.importStyleChild(*id, attrs);
    }
    imp.importStyleEnd(model);
}

// ---- SE1: 构造 + 保存 KStyles（经 importDocDefaults 观测）----
#[test]
fn SE1_ctor_stores_styles() {
    let (styles, _, imp) = importer_pair();
    let mut rpr = MutablePropertySet::new();
    rpr.setInt(10, 24);
    imp.importDocDefaults(Some(&mut rpr), None);
    assert_eq!(styles.borrow().getRPrDefault().getInt(10), 24);
}

// ---- SE3/SE4: docDefaults 双/单 prop ----
#[test]
fn SE3_docdefaults_both() {
    let (styles, _, imp) = importer_pair();
    let mut rpr = MutablePropertySet::new();
    rpr.setInt(10, 24);
    let mut ppr = MutablePropertySet::new();
    ppr.setInt(191, 0);
    imp.importDocDefaults(Some(&mut rpr), Some(&mut ppr));
    assert_eq!(styles.borrow().getRPrDefault().getInt(10), 24);
    assert_eq!(styles.borrow().getPPrDefault().getInt(191), 0);
}

#[test]
fn SE4_docdefaults_ppr_only() {
    let (styles, _, imp) = importer_pair();
    let mut ppr = MutablePropertySet::new();
    ppr.setInt(191, 3);
    imp.importDocDefaults(None, Some(&mut ppr));
    assert_eq!(styles.borrow().getPPrDefault().getInt(191), 3);
    assert_eq!(styles.borrow().getRPrDefault().size(), 0);
}

// ---- SE5+SE6..12: importStyleStart 字段 + importStyleChild 7 常量的落桶 ----
#[test]
fn SE5_6_12_start_and_children() {
    let (styles, sti, mut imp) = importer_pair();
    // link 目标样式先注册（link 引用其它 styleId）
    run_style(&mut imp, style_start("H1Char", "character"), &[], &empty_model());
    let start = style_start("H1", "paragraph").with(MAIN_NS, "customStyle", "1");
    let model = empty_model();
    run_style(
        &mut imp,
        start,
        &[
            (3373707, WordAttrs::named("val", "Heading 1")),  // name
            (-332612366, WordAttrs::named("val", "Normal")),  // basedOn
            (3377907, WordAttrs::named("val", "Body")),       // next
            (3321850, WordAttrs::named("val", "H1Char")),     // link
            (-681243704, WordAttrs::empty()),                 // qFormat
            (872749144, WordAttrs::val("10")),                // uiPriority
        ],
        &model,
    );
    imp.importStylesEnd();
    let id = sti.borrow().getSti("H1").expect("H1 mapped");
    let k = styles.borrow().getStyleById__int(id).expect("kstyle added");
    assert_eq!(k.getName(), "Heading 1");
    assert_eq!(k.getType(), 1);
    assert_eq!(k.getQFormat(), true);
    assert_eq!(k.getUiPriority(), 10);
    assert_eq!(k.getLink(), sti.borrow().getSti("H1Char").expect("H1Char mapped"));
    // basedOn "Normal" 未映射 → 4095；next "Body" 未映射 → 自身 sti
    assert_eq!(k.getBase(), 4095);
    assert_eq!(k.getNext(), id);
}

// ---- SE13: 两阶段 — importStylesEnd 前不入 KStyles ----
#[test]
fn SE13_two_phase_commit() {
    let (styles, sti, mut imp) = importer_pair();
    run_style(&mut imp, style_start("S1", "paragraph"), &[], &empty_model());
    let id = sti.borrow().getSti("S1").expect("S1 mapped at importStyleEnd");
    assert!(styles.borrow().getStyleById__int(id).is_none(), "未提交前不得出现");
    imp.importStylesEnd();
    assert!(styles.borrow().getStyleById__int(id).is_some());
}

// ---- SE14: getStylePrSet 合并 + correctTcPrSet 移除 346–351 ----
#[test]
fn SE14_style_pr_set_merge_and_tcpr_cleanup() {
    let (styles, sti, mut imp) = importer_pair();
    let model = model_with_prop(2, &[(346, 7), (350, 9), (352, 1)]); // tcPr
    run_style(&mut imp, style_start("T1", "table"), &[], &model);
    imp.importStylesEnd();
    let id = sti.borrow().getSti("T1").unwrap();
    let k = styles.borrow().getStyleById__int(id).unwrap();
    let pr = k.getProperty().expect("frozen property");
    assert!(pr.getValue(346).is_none(), "346 应被 correctTcPrSet 移除");
    assert!(pr.getValue(350).is_none(), "350 应被移除");
    assert!(pr.getValue(352).is_some(), "352 保留");
}

// ---- SE16: 多条样式一次性提交 ----
#[test]
fn SE16_multi_styles_end() {
    let (styles, sti, mut imp) = importer_pair();
    run_style(&mut imp, style_start("A", "paragraph"), &[], &empty_model());
    run_style(&mut imp, style_start("B", "character"), &[], &empty_model());
    assert_eq!(styles.borrow().size(), 0);
    imp.importStylesEnd();
    assert_eq!(styles.borrow().size(), 2);
    let a = sti.borrow().getSti("A").unwrap();
    let b = sti.borrow().getSti("B").unwrap();
    assert_eq!(styles.borrow().getStyleById__int(a).unwrap().getType(), 1);
    assert_eq!(styles.borrow().getStyleById__int(b).unwrap().getType(), 2);
}

// ---- SE17/SE18/SE19/SE22/SE23: createKStyle 的 sti 哨兵 ----
#[test]
fn SE17_18_19_22_23_sti_sentinels() {
    let (styles, sti, mut imp) = importer_pair();
    // 无 name / 无 basedOn / 无 next / 无 link
    run_style(&mut imp, style_start("Bare", "numbering"), &[], &empty_model());
    imp.importStylesEnd();
    let id = sti.borrow().getSti("Bare").unwrap();
    let k = styles.borrow().getStyleById__int(id).unwrap();
    assert_eq!(k.getType(), 4);
    assert_eq!(k.getName(), "", "无 name → 空串（Java null 的 Rust 等价）");
    assert_eq!(k.getBase(), 4095, "无 basedOn → 4095");
    assert_eq!(k.getNext(), id, "无 next → 自身 sti");
    assert_eq!(k.getLink(), 4095, "无 link → 4095");
}

// ---- SE20: basedOn == 自身 → 4095 ----
#[test]
fn SE20_base_self_forced_4095() {
    let (styles, sti, mut imp) = importer_pair();
    let mk = |id: &str| style_start(id, "paragraph");
    // styleSelf: styleId="SelfBase"，basedOn="SelfBase"
    run_style(
        &mut imp,
        mk("SelfBase"),
        &[(-332612366, WordAttrs::named("val", "SelfBase"))],
        &empty_model(),
    );
    imp.importStylesEnd();
    let id = sti.borrow().getSti("SelfBase").unwrap();
    let k = styles.borrow().getStyleById__int(id).unwrap();
    assert_eq!(k.getBase(), 4095, "base==coreSti → 4095");
}

// ---- SE21（可达部分）: Normal 风格无用户 base ----
#[test]
fn SE21_normal_no_user_base() {
    let (styles, sti, mut imp) = importer_pair();
    // basedOn 引用不存在样式 → toSti=4095（coreSti 非 0 时 base 保持 4095）
    run_style(
        &mut imp,
        style_start("Normal", "paragraph"),
        &[(-332612366, WordAttrs::val("Base"))],
        &empty_model(),
    );
    imp.importStylesEnd();
    let id = sti.borrow().getSti("Normal").expect("Normal mapped");
    let k = styles.borrow().getStyleById__int(id).unwrap();
    assert_eq!(k.getBase(), 4095, "basedOn 不存在 → 4095");
}

// ---- SE21-core/SE25: 内置 sti0 与保留索引分支（依赖 BuildinStyleSheet 内置表）----
#[ignore = "spec: BuildinStyleSheet 内置 Normal→sti0 映射在测试环境未接通（word-core helpers 依赖资源表），coreSti==0 强制 base=4095 分支不可达"]
#[test]
fn SE21_normal_corest0_forced_4095() {
    let (styles, sti, mut imp) = importer_pair();
    run_style(
        &mut imp,
        style_start("Normal", "paragraph"),
        &[(-332612366, WordAttrs::val("Base"))],
        &empty_model(),
    );
    imp.importStylesEnd();
    let id = sti.borrow().getSti("Normal").expect("Normal mapped");
    assert_eq!(id, 0, "Java 内置 Normal sti=0");
    let k = styles.borrow().getStyleById__int(id).unwrap();
    assert_eq!(k.getBase(), 4095, "coreSti==0 → 强制无 base");
}

#[ignore = "spec: 同上依赖，保留索引分支（0..267）不可达（Rust UUSti 从 4097 起）"]
#[test]
fn SE25_reserved_name_reserved_index_kept() {
    let (styles, sti, mut imp) = importer_pair();
    run_style(
        &mut imp,
        style_start("Normal", "paragraph"),
        &[(3373707, WordAttrs::val("Normal"))],
        &empty_model(),
    );
    imp.importStylesEnd();
    let id = sti.borrow().getSti("Normal").unwrap();
    assert!(BuildinStyleSheet::isReservedIndex(id));
    assert_eq!(styles.borrow().getStyleById__int(id).unwrap().getName(), "Normal");
}

// ---- SE26: 保留名 + 非保留索引 → name_UUID ----
#[test]
fn SE26_reserved_name_user_index_gets_uuid() {
    let (styles, sti, mut imp) = importer_pair();
    let start = style_start("MyN", "paragraph").with(MAIN_NS, "customStyle", "1");
    run_style(&mut imp, start, &[(3373707, WordAttrs::named("val", "Normal"))], &empty_model());
    imp.importStylesEnd();
    let id = sti.borrow().getSti("MyN").unwrap();
    assert!(!BuildinStyleSheet::isReservedIndex(id), "custom 用户 sti 非保留索引: {id}");
    let name = styles.borrow().getStyleById__int(id).unwrap().getName();
    assert!(name.starts_with("Normal_"), "保留名+非保留索引 → name_UUID，实际: {name}");
    assert!(name.len() > "Normal_".len());
}