// Test 02_function_textbox_props — TextBodyProperties → TextBoxData 框属性（pl-04）
// 冻结矩阵 FUNCTION.md §3.2 TP1–TP8。
use std::sync::{Arc, Once};

use ppt_core::cn::wps::base::converter::UnitsConverter::UnitsConverter;
use ppt_core::cn::wps::show::layout::dataaction::DefineConverter::DefineConverter;
use ppt_core::cn::wps::show::layout::dataaction::ShapeDataTransform::ShapeDataTransform;
use ppt_core::cn::wps::show::util::layout_shape::KmoShowShape;
use ppt_core::cn::wps::show::util::layout_textbox::{
    Autofit, KmoTextOperator, PresetTextWarp, TextBodyProperties,
};
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util_adapter::cn::wps::core::runtime::Platform::Platform;
use wpslayout::cn::wps::text::layout::data::TextBoxData::TextBoxData;

static INIT: Once = Once::new();

fn setup_metrics() {
    INIT.call_once(|| {
        Platform::setDisplayMetrics(Arc::new(KDisplayMetrics::new_with_params(
            0, 0, 1.0, 160, 1.0, 96.0, 96.0,
        )));
    });
}

fn apply(shape: &KmoShowShape, body_pr: TextBodyProperties) -> TextBoxData {
    let op = KmoTextOperator {
        body_pr,
        ..Default::default()
    };
    let mut xf = ShapeDataTransform::default();
    let mut boxd = TextBoxData::new();
    xf.covertTextBodyProperties(shape, &op, &mut boxd);
    boxd
}

#[test]
fn tp1_default_insets_and_wrap() {
    setup_metrics();
    let boxd = apply(&KmoShowShape::default(), TextBodyProperties::default());
    let u = UnitsConverter::layout();
    assert_eq!(boxd.getLeftInset(), u.fromEmuY(91440.0));
    assert_eq!(boxd.getRightInset(), u.fromEmuY(91440.0));
    assert_eq!(boxd.getTopInset(), u.fromEmuY(45720.0));
    assert_eq!(boxd.getBottomInset(), u.fromEmuY(45720.0));
    assert!(boxd.isWrapTextInShape());
    // Java: hasAnchor() false on empty map → convert does not write anchor(); TextBoxData stays 0.
    assert_eq!(boxd.getAnchor(), 0);
}

#[test]
fn tp2_wrap_off_when_wrapping_type_zero() {
    setup_metrics();
    let off = apply(
        &KmoShowShape::default(),
        TextBodyProperties {
            has_text_wrapping_type: true,
            text_wrapping_type: 0,
            ..Default::default()
        },
    );
    assert!(!off.isWrapTextInShape());
    let on = apply(
        &KmoShowShape::default(),
        TextBodyProperties {
            has_text_wrapping_type: true,
            text_wrapping_type: 1,
            ..Default::default()
        },
    );
    assert!(on.isWrapTextInShape());
}

#[test]
fn tp3_convert_txt_anchor() {
    for t in [0, 1, 2, 3, 4] {
        assert_eq!(DefineConverter::convertTxtAnchor(t), t);
    }
    assert_eq!(DefineConverter::convertTxtAnchor(99), 0);
}

#[test]
fn tp4_explicit_insets_override_defaults() {
    setup_metrics();
    let boxd = apply(
        &KmoShowShape::default(),
        TextBodyProperties {
            has_left_inset: true,
            left_inset: 1000,
            has_right_inset: true,
            right_inset: 2000,
            has_top_inset: true,
            top_inset: 3000,
            has_bottom_inset: true,
            bottom_inset: 4000,
            ..Default::default()
        },
    );
    let u = UnitsConverter::layout();
    assert_eq!(boxd.getLeftInset(), u.fromEmuY(1000.0));
    assert_eq!(boxd.getRightInset(), u.fromEmuY(2000.0));
    assert_eq!(boxd.getTopInset(), u.fromEmuY(3000.0));
    assert_eq!(boxd.getBottomInset(), u.fromEmuY(4000.0));
}

#[test]
fn tp5_autofit_and_warp_no_scale_side_effect() {
    setup_metrics();
    let boxd = apply(
        &KmoShowShape::default(),
        TextBodyProperties {
            has_text_autofit: true,
            text_autofit: Autofit {
                type_: 1,
                has_font_scale: true,
                font_scale: 0.5,
                has_ln_spc_reduction: true,
                ln_spc_reduction: 0.1,
            },
            has_preset_text_warp: true,
            preset_text_warp: PresetTextWarp { prst: 7 },
            ..Default::default()
        },
    );
    assert_eq!(boxd.textAutofit(), 1);
    assert_eq!(boxd.getFontScale(), 0.5);
    assert!((boxd.getLnSpcReduction() - 0.1).abs() < 1e-6);
    assert_eq!(boxd.getPresetTextWarp(), 7);
}

#[test]
fn tp6_vertical_text() {
    setup_metrics();
    let boxd = apply(
        &KmoShowShape::default(),
        TextBodyProperties {
            has_vertical_text: true,
            vertical_text: 3,
            ..Default::default()
        },
    );
    assert_eq!(boxd.getTextVerticalType(), 3);
}

#[test]
fn tp7_original_autofit_anchor_snapshot() {
    setup_metrics();
    let skip = apply(&KmoShowShape::default(), TextBodyProperties::default());
    assert_eq!(skip.getOldTextAutoFit(), -1);
    assert_eq!(skip.getOldAnchor(), -1);
    let shape = KmoShowShape {
        original_autofit: 5,
        original_anchor: 4,
        ..KmoShowShape::default()
    };
    let boxd = apply(&shape, TextBodyProperties::default());
    assert_eq!(boxd.getOldTextAutoFit(), 5);
    assert_eq!(boxd.getOldAnchor(), 4);
}

#[test]
fn tp8_emu_to_layout_units() {
    setup_metrics();
    let xf = ShapeDataTransform::default();
    assert_eq!(
        xf.emuToLayoutUnits(91440.0),
        UnitsConverter::layout().fromEmuY(91440.0)
    );
    assert_eq!(
        xf.emuToLayoutUnits(45720.0),
        UnitsConverter::layout().fromEmuY(45720.0)
    );
}
