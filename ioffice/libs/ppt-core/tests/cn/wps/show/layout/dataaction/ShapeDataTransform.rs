// Test 01_function_shape_to_textbox — 段/run → TextBoxData（pl-04）
// 冻结矩阵 FUNCTION.md §3.2 TB1–TB7。
use std::sync::{Arc, Once};

use ppt_core::cn::wps::base::converter::UnitsConverter::UnitsConverter;
use ppt_core::cn::wps::show::layout::dataaction::DefineConverter::DefineConverter;
use ppt_core::cn::wps::show::layout::dataaction::ShapeDataTransform::ShapeDataTransform;
use ppt_core::cn::wps::show::util::layout_shape::{KmoShowShape, Placeholder};
use ppt_core::cn::wps::show::util::layout_textbox::{
    KmoTextOperator, TextCharOptions, TextCharacterProperties, TextPara, TextParagraphProperties,
    TextParaOptions, TextRun,
};
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

static INIT: Once = Once::new();

fn setup_metrics() {
    INIT.call_once(|| {
        Platform::setDisplayMetrics(Arc::new(KDisplayMetrics::new_with_params(
            0, 0, 1.0, 160, 1.0, 96.0, 96.0,
        )));
    });
}

fn run_prop(size_pt: f32, color: i32, bold: bool, name: &str) -> TextCharacterProperties {
    TextCharacterProperties {
        font_name: name.to_string(),
        font_size_pt: size_pt,
        font_color: color,
        options: TextCharOptions {
            has_bold: true,
            bold,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn para_bu_none() -> TextParagraphProperties {
    TextParagraphProperties {
        options: TextParaOptions {
            has_bu_none: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn shape_with_op(op: KmoTextOperator) -> KmoShowShape {
    KmoShowShape {
        has_text_body: true,
        text_op: Some(op),
        ..KmoShowShape::default()
    }
}

fn tb1_shape() -> KmoShowShape {
    let ppr = para_bu_none();
    let op = KmoTextOperator {
        paras: vec![
            TextPara {
                para_prop: ppr.clone(),
                runs: vec![
                    TextRun {
                        text: "Hello".into(),
                        start: 0,
                        end: 5,
                        ch_prop: run_prop(18.0, 0xFF0000, true, "Arial"),
                    },
                    TextRun {
                        text: "World".into(),
                        start: 5,
                        end: 10,
                        ch_prop: run_prop(12.0, 0x00FF00, false, "Arial"),
                    },
                ],
            },
            TextPara {
                para_prop: ppr,
                runs: vec![
                    TextRun {
                        text: "Foo".into(),
                        start: 0,
                        end: 3,
                        ch_prop: run_prop(14.0, 0x0000FF, false, "Calibri"),
                    },
                    TextRun {
                        text: "Bar".into(),
                        start: 3,
                        end: 6,
                        ch_prop: run_prop(14.0, 0x0000FF, false, "Calibri"),
                    },
                ],
            },
        ],
        has_paragraph_list: true,
        ph_type: 1,
        ..Default::default()
    };
    shape_with_op(op)
}

#[test]
fn tb1_multi_para_runs() {
    setup_metrics();
    let mut xf = ShapeDataTransform::default();
    let boxd = xf.exToInternal(Some(&tb1_shape()), None);
    assert_eq!(boxd.getTextDatas().len(), 2);
    let t0 = &boxd.getTextDatas()[0];
    assert_eq!(t0.mSourceText.as_deref(), Some("HelloWorld"));
    assert_eq!(t0.mFirstSpan.mStartPos, 0);
    assert_eq!(t0.mFirstSpan.mEndPos, 5);
    assert_eq!(t0.mOtherSpans.len(), 1);
    assert_eq!(t0.mOtherSpans[0].mStartPos, 5);
    assert_eq!(t0.mOtherSpans[0].mEndPos, 10);
    let t1 = &boxd.getTextDatas()[1];
    assert_eq!(t1.mSourceText.as_deref(), Some("FooBar"));
}

#[test]
fn tb2_no_body_empty() {
    setup_metrics();
    let mut xf = ShapeDataTransform::default();
    let shape = KmoShowShape::default();
    let boxd = xf.exToInternal(Some(&shape), None);
    assert!(boxd.isEmpty());
}

#[test]
fn tb3_empty_or_cr_invisible() {
    setup_metrics();
    for src in ["", "\r"] {
        let op = KmoTextOperator {
            paras: vec![TextPara {
                para_prop: para_bu_none(),
                runs: vec![TextRun {
                    text: src.into(),
                    start: 0,
                    end: src.chars().count() as i32,
                    ch_prop: run_prop(12.0, 0, false, "Arial"),
                }],
            }],
            has_paragraph_list: true,
            ph_type: 1,
            ..Default::default()
        };
        let mut xf = ShapeDataTransform::default();
        let boxd = xf.exToInternal(Some(&shape_with_op(op)), None);
        assert!(!boxd.hasVisibleText(), "src={src:?}");
        if src.is_empty() {
            assert!(boxd.isEmptyText());
        }
    }
}

#[test]
fn tb4_note_ph_15() {
    setup_metrics();
    let op = KmoTextOperator {
        paras: vec![TextPara {
            para_prop: para_bu_none(),
            runs: vec![TextRun {
                text: "note".into(),
                start: 0,
                end: 4,
                ch_prop: run_prop(12.0, 0, false, "Arial"),
            }],
        }],
        has_paragraph_list: true,
        ph_type: 15,
        ..Default::default()
    };
    let shape = KmoShowShape {
        has_text_body: false,
        text_op: Some(op),
        ph: Placeholder {
            has_type: true,
            type_: 15,
            ..Default::default()
        },
        ..KmoShowShape::default()
    };
    let mut xf = ShapeDataTransform::default();
    let boxd = xf.exToInternal(Some(&shape), None);
    assert_eq!(boxd.getTextDatas().len(), 1);
    assert_eq!(boxd.getTextDatas()[0].mSourceText.as_deref(), Some("note"));
    assert_eq!(boxd.getPHolderType(), 15);
}

#[test]
fn tb5_convert_ph_holder() {
    for t in [0, 1, 2, 3, 4, 5, 10, 15] {
        assert_eq!(DefineConverter::convertPhHolder(t), t);
    }
    assert_eq!(DefineConverter::convertPhHolder(99), 1);
}

#[test]
fn tb6_span_style() {
    setup_metrics();
    let mut xf = ShapeDataTransform::default();
    let boxd = xf.exToInternal(Some(&tb1_shape()), None);
    let t0 = &boxd.getTextDatas()[0];
    assert_eq!(t0.mFirstSpan.mFontColor, 0xFF0000);
    assert!(t0.mFirstSpan.mIsBold);
    assert_eq!(t0.mOtherSpans[0].mFontColor, 0x00FF00);
    assert!(!t0.mOtherSpans[0].mIsBold);
    let expected_18 = UnitsConverter::layout().fromPointY(18.0);
    assert!((t0.mFirstSpan.mFontSize - expected_18).abs() < 0.01);
}

#[test]
fn tb7_emu_to_layout_units() {
    setup_metrics();
    let xf = ShapeDataTransform::default();
    let a = xf.emuToLayoutUnits(91440.0);
    let b = xf.emuToLayoutUnits(45720.0);
    assert_eq!(a, UnitsConverter::layout().fromEmuY(91440.0));
    assert_eq!(b, UnitsConverter::layout().fromEmuY(45720.0));
}
