#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]
// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.app.format.color.DgColor
// LLM: DgColor depends on KProto chain, using stub
// Java: import cn.wps.drawing.app.format.color.DgColorFormat
use crate::cn::wps::drawing::app::format::color::DgColorFormat::DgColorFormat;
// Java: import cn.wps.drawing.data.color.Color
// LLM: Color is a KProto type, using stub
// Java: import cn.wps.drawing.data.style.ShapeStyle
// LLM: ShapeStyle is a KProto type, using stub
// Java: import cn.wps.drawing.data.style.StyleMatrixReference
// LLM: StyleMatrixReference is a KProto type, using stub
// Java: import cn.wps.kso.kproto.KoBin
// LLM: KoBin stub

pub struct KoBin;
pub struct Color;
impl Color {
    pub fn open(_bin: KoBin) -> Self { Color }
    pub fn save(&self) -> KoBin { KoBin }
}
pub struct DgColor;
impl DgColor {
    pub fn scheme(_v: i32) -> Color { Color }
}
pub struct StyleMatrixReference;
impl StyleMatrixReference {
    pub fn idx(&self) -> i32 { 0 }
    pub fn putIdx(&mut self, _v: i32) {}
    pub fn color(&self) -> Color { Color }
    pub fn writeColor(&mut self, _c: Color) {}
}
pub struct ShapeStyle;
impl ShapeStyle {
    pub fn create() -> Self { ShapeStyle }
    pub fn save(&self) -> KoBin { KoBin }
    pub fn load(&mut self, _bin: KoBin) {}
    pub fn lnRef(&mut self) -> &mut StyleMatrixReference { todo!() }
    pub fn fillRef(&mut self) -> &mut StyleMatrixReference { todo!() }
    pub fn effectRef(&mut self) -> &mut StyleMatrixReference { todo!() }
    pub fn fontRef(&mut self) -> &mut StyleMatrixReference { todo!() }
    pub fn hasFontRef(&self) -> bool { false }
}

// Java: public class DgShapeStyle implements KmoProtocol
pub struct DgShapeStyle {
    // Java field: ShapeStyle shapeStyle
    pub(crate) shapeStyle: ShapeStyle,
    // Java field: DgColorFormat lnColor
    pub(crate) lnColor: DgColorFormat,
    // Java field: DgColorFormat fillColor
    pub(crate) fillColor: DgColorFormat,
    // Java field: DgColorFormat effectColor
    pub(crate) effectColor: DgColorFormat,
    // Java field: DgColorFormat fontColor
    pub(crate) fontColor: DgColorFormat,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_DK1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_DK1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_ACCENT1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_ACCENT1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_ACCENT2
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_ACCENT2: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_ACCENT3
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_ACCENT3: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_ACCENT4
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_ACCENT4: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_ACCENT5
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_ACCENT5: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_ACCENT6
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_ACCENT6: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_FILL_DK1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_FILL_DK1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_FILL_ACCENT1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_FILL_ACCENT1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_FILL_ACCENT2
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_FILL_ACCENT2: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_FILL_ACCENT3
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_FILL_ACCENT3: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_FILL_ACCENT4
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_FILL_ACCENT4: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_FILL_ACCENT5
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_FILL_ACCENT5: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_FILL_ACCENT6
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_FILL_ACCENT6: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_FILL_DK1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_FILL_DK1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_FILL_ACCENT1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_FILL_ACCENT1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_FILL_ACCENT2
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_FILL_ACCENT2: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_FILL_ACCENT3
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_FILL_ACCENT3: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_FILL_ACCENT4
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_FILL_ACCENT4: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_FILL_ACCENT5
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_FILL_ACCENT5: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_LN_FILL_ACCENT6
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_LN_FILL_ACCENT6: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_TINY_DK1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_TINY_DK1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_TINY_ACCENT1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_TINY_ACCENT1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_TINY_ACCENT2
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_TINY_ACCENT2: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_TINY_ACCENT3
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_TINY_ACCENT3: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_TINY_ACCENT4
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_TINY_ACCENT4: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_TINY_ACCENT5
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_TINY_ACCENT5: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_TINY_ACCENT6
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_TINY_ACCENT6: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_MEDIUM_DK1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_MEDIUM_DK1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_MEDIUM_ACCENT1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_MEDIUM_ACCENT1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_MEDIUM_ACCENT2
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_MEDIUM_ACCENT2: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_MEDIUM_ACCENT3
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_MEDIUM_ACCENT3: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_MEDIUM_ACCENT4
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_MEDIUM_ACCENT4: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_MEDIUM_ACCENT5
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_MEDIUM_ACCENT5: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_MEDIUM_ACCENT6
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_MEDIUM_ACCENT6: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_INTENSITY_DK1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_INTENSITY_DK1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_INTENSITY_ACCENT1
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_INTENSITY_ACCENT1: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_INTENSITY_ACCENT2
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_INTENSITY_ACCENT2: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_INTENSITY_ACCENT3
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_INTENSITY_ACCENT3: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_INTENSITY_ACCENT4
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_INTENSITY_ACCENT4: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_INTENSITY_ACCENT5
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_INTENSITY_ACCENT5: Option<Box<DgShapeStyle>>,
    // Java field: DgShapeStyle QUICK_SHAPE_STYLE_INTENSITY_ACCENT6
    // LLM: Java static field; lazy-initialized preset cache
    pub(crate) QUICK_SHAPE_STYLE_INTENSITY_ACCENT6: Option<Box<DgShapeStyle>>,
}

// Java: public class PresetStyleBuilder
pub struct PresetStyleBuilder;

impl DgShapeStyle {
    pub fn DgShapeStyle_new() -> Self {
        DgShapeStyle {
            // Java: private ShapeStyle shapeStyle = ShapeStyle.create();
            shapeStyle: ShapeStyle::create(),
            // Java: private DgColorFormat lnColor = new DgColorFormat();
            lnColor: DgColorFormat::DgColorFormat_new(),
            // Java: private DgColorFormat fillColor = new DgColorFormat();
            fillColor: DgColorFormat::DgColorFormat_new(),
            // Java: private DgColorFormat effectColor = new DgColorFormat();
            effectColor: DgColorFormat::DgColorFormat_new(),
            // Java: private DgColorFormat fontColor = new DgColorFormat();
            fontColor: DgColorFormat::DgColorFormat_new(),
            QUICK_SHAPE_STYLE_LN_DK1: None,
            QUICK_SHAPE_STYLE_LN_ACCENT1: None,
            QUICK_SHAPE_STYLE_LN_ACCENT2: None,
            QUICK_SHAPE_STYLE_LN_ACCENT3: None,
            QUICK_SHAPE_STYLE_LN_ACCENT4: None,
            QUICK_SHAPE_STYLE_LN_ACCENT5: None,
            QUICK_SHAPE_STYLE_LN_ACCENT6: None,
            QUICK_SHAPE_STYLE_FILL_DK1: None,
            QUICK_SHAPE_STYLE_FILL_ACCENT1: None,
            QUICK_SHAPE_STYLE_FILL_ACCENT2: None,
            QUICK_SHAPE_STYLE_FILL_ACCENT3: None,
            QUICK_SHAPE_STYLE_FILL_ACCENT4: None,
            QUICK_SHAPE_STYLE_FILL_ACCENT5: None,
            QUICK_SHAPE_STYLE_FILL_ACCENT6: None,
            QUICK_SHAPE_STYLE_LN_FILL_DK1: None,
            QUICK_SHAPE_STYLE_LN_FILL_ACCENT1: None,
            QUICK_SHAPE_STYLE_LN_FILL_ACCENT2: None,
            QUICK_SHAPE_STYLE_LN_FILL_ACCENT3: None,
            QUICK_SHAPE_STYLE_LN_FILL_ACCENT4: None,
            QUICK_SHAPE_STYLE_LN_FILL_ACCENT5: None,
            QUICK_SHAPE_STYLE_LN_FILL_ACCENT6: None,
            QUICK_SHAPE_STYLE_TINY_DK1: None,
            QUICK_SHAPE_STYLE_TINY_ACCENT1: None,
            QUICK_SHAPE_STYLE_TINY_ACCENT2: None,
            QUICK_SHAPE_STYLE_TINY_ACCENT3: None,
            QUICK_SHAPE_STYLE_TINY_ACCENT4: None,
            QUICK_SHAPE_STYLE_TINY_ACCENT5: None,
            QUICK_SHAPE_STYLE_TINY_ACCENT6: None,
            QUICK_SHAPE_STYLE_MEDIUM_DK1: None,
            QUICK_SHAPE_STYLE_MEDIUM_ACCENT1: None,
            QUICK_SHAPE_STYLE_MEDIUM_ACCENT2: None,
            QUICK_SHAPE_STYLE_MEDIUM_ACCENT3: None,
            QUICK_SHAPE_STYLE_MEDIUM_ACCENT4: None,
            QUICK_SHAPE_STYLE_MEDIUM_ACCENT5: None,
            QUICK_SHAPE_STYLE_MEDIUM_ACCENT6: None,
            QUICK_SHAPE_STYLE_INTENSITY_DK1: None,
            QUICK_SHAPE_STYLE_INTENSITY_ACCENT1: None,
            QUICK_SHAPE_STYLE_INTENSITY_ACCENT2: None,
            QUICK_SHAPE_STYLE_INTENSITY_ACCENT3: None,
            QUICK_SHAPE_STYLE_INTENSITY_ACCENT4: None,
            QUICK_SHAPE_STYLE_INTENSITY_ACCENT5: None,
            QUICK_SHAPE_STYLE_INTENSITY_ACCENT6: None,
        }
    }

    // Java: static void clear()
    pub fn clear(&mut self) {
        // Java: QUICK_SHAPE_STYLE_LN_DK1 = null;
        self.QUICK_SHAPE_STYLE_LN_DK1 = None;
        // Java: QUICK_SHAPE_STYLE_LN_ACCENT1 = null;
        self.QUICK_SHAPE_STYLE_LN_ACCENT1 = None;
        // Java: QUICK_SHAPE_STYLE_LN_ACCENT2 = null;
        self.QUICK_SHAPE_STYLE_LN_ACCENT2 = None;
        // Java: QUICK_SHAPE_STYLE_LN_ACCENT3 = null;
        self.QUICK_SHAPE_STYLE_LN_ACCENT3 = None;
        // Java: QUICK_SHAPE_STYLE_LN_ACCENT4 = null;
        self.QUICK_SHAPE_STYLE_LN_ACCENT4 = None;
        // Java: QUICK_SHAPE_STYLE_LN_ACCENT5 = null;
        self.QUICK_SHAPE_STYLE_LN_ACCENT5 = None;
        // Java: QUICK_SHAPE_STYLE_LN_ACCENT6 = null;
        self.QUICK_SHAPE_STYLE_LN_ACCENT6 = None;
        // Java: QUICK_SHAPE_STYLE_FILL_DK1 = null;
        self.QUICK_SHAPE_STYLE_FILL_DK1 = None;
        // Java: QUICK_SHAPE_STYLE_FILL_ACCENT1 = null;
        self.QUICK_SHAPE_STYLE_FILL_ACCENT1 = None;
        // Java: QUICK_SHAPE_STYLE_FILL_ACCENT2 = null;
        self.QUICK_SHAPE_STYLE_FILL_ACCENT2 = None;
        // Java: QUICK_SHAPE_STYLE_FILL_ACCENT3 = null;
        self.QUICK_SHAPE_STYLE_FILL_ACCENT3 = None;
        // Java: QUICK_SHAPE_STYLE_FILL_ACCENT4 = null;
        self.QUICK_SHAPE_STYLE_FILL_ACCENT4 = None;
        // Java: QUICK_SHAPE_STYLE_FILL_ACCENT5 = null;
        self.QUICK_SHAPE_STYLE_FILL_ACCENT5 = None;
        // Java: QUICK_SHAPE_STYLE_FILL_ACCENT6 = null;
        self.QUICK_SHAPE_STYLE_FILL_ACCENT6 = None;
        // Java: QUICK_SHAPE_STYLE_LN_FILL_DK1 = null;
        self.QUICK_SHAPE_STYLE_LN_FILL_DK1 = None;
        // Java: QUICK_SHAPE_STYLE_LN_FILL_ACCENT1 = null;
        self.QUICK_SHAPE_STYLE_LN_FILL_ACCENT1 = None;
        // Java: QUICK_SHAPE_STYLE_LN_FILL_ACCENT2 = null;
        self.QUICK_SHAPE_STYLE_LN_FILL_ACCENT2 = None;
        // Java: QUICK_SHAPE_STYLE_LN_FILL_ACCENT3 = null;
        self.QUICK_SHAPE_STYLE_LN_FILL_ACCENT3 = None;
        // Java: QUICK_SHAPE_STYLE_LN_FILL_ACCENT4 = null;
        self.QUICK_SHAPE_STYLE_LN_FILL_ACCENT4 = None;
        // Java: QUICK_SHAPE_STYLE_LN_FILL_ACCENT5 = null;
        self.QUICK_SHAPE_STYLE_LN_FILL_ACCENT5 = None;
        // Java: QUICK_SHAPE_STYLE_LN_FILL_ACCENT6 = null;
        self.QUICK_SHAPE_STYLE_LN_FILL_ACCENT6 = None;
        // Java: QUICK_SHAPE_STYLE_TINY_DK1 = null;
        self.QUICK_SHAPE_STYLE_TINY_DK1 = None;
        // Java: QUICK_SHAPE_STYLE_TINY_ACCENT1 = null;
        self.QUICK_SHAPE_STYLE_TINY_ACCENT1 = None;
        // Java: QUICK_SHAPE_STYLE_TINY_ACCENT2 = null;
        self.QUICK_SHAPE_STYLE_TINY_ACCENT2 = None;
        // Java: QUICK_SHAPE_STYLE_TINY_ACCENT3 = null;
        self.QUICK_SHAPE_STYLE_TINY_ACCENT3 = None;
        // Java: QUICK_SHAPE_STYLE_TINY_ACCENT4 = null;
        self.QUICK_SHAPE_STYLE_TINY_ACCENT4 = None;
        // Java: QUICK_SHAPE_STYLE_TINY_ACCENT5 = null;
        self.QUICK_SHAPE_STYLE_TINY_ACCENT5 = None;
        // Java: QUICK_SHAPE_STYLE_TINY_ACCENT6 = null;
        self.QUICK_SHAPE_STYLE_TINY_ACCENT6 = None;
        // Java: QUICK_SHAPE_STYLE_MEDIUM_DK1 = null;
        self.QUICK_SHAPE_STYLE_MEDIUM_DK1 = None;
        // Java: QUICK_SHAPE_STYLE_MEDIUM_ACCENT1 = null;
        self.QUICK_SHAPE_STYLE_MEDIUM_ACCENT1 = None;
        // Java: QUICK_SHAPE_STYLE_MEDIUM_ACCENT2 = null;
        self.QUICK_SHAPE_STYLE_MEDIUM_ACCENT2 = None;
        // Java: QUICK_SHAPE_STYLE_MEDIUM_ACCENT3 = null;
        self.QUICK_SHAPE_STYLE_MEDIUM_ACCENT3 = None;
        // Java: QUICK_SHAPE_STYLE_MEDIUM_ACCENT4 = null;
        self.QUICK_SHAPE_STYLE_MEDIUM_ACCENT4 = None;
        // Java: QUICK_SHAPE_STYLE_MEDIUM_ACCENT5 = null;
        self.QUICK_SHAPE_STYLE_MEDIUM_ACCENT5 = None;
        // Java: QUICK_SHAPE_STYLE_MEDIUM_ACCENT6 = null;
        self.QUICK_SHAPE_STYLE_MEDIUM_ACCENT6 = None;
        // Java: QUICK_SHAPE_STYLE_INTENSITY_DK1 = null;
        self.QUICK_SHAPE_STYLE_INTENSITY_DK1 = None;
        // Java: QUICK_SHAPE_STYLE_INTENSITY_ACCENT1 = null;
        self.QUICK_SHAPE_STYLE_INTENSITY_ACCENT1 = None;
        // Java: QUICK_SHAPE_STYLE_INTENSITY_ACCENT2 = null;
        self.QUICK_SHAPE_STYLE_INTENSITY_ACCENT2 = None;
        // Java: QUICK_SHAPE_STYLE_INTENSITY_ACCENT3 = null;
        self.QUICK_SHAPE_STYLE_INTENSITY_ACCENT3 = None;
        // Java: QUICK_SHAPE_STYLE_INTENSITY_ACCENT4 = null;
        self.QUICK_SHAPE_STYLE_INTENSITY_ACCENT4 = None;
        // Java: QUICK_SHAPE_STYLE_INTENSITY_ACCENT5 = null;
        self.QUICK_SHAPE_STYLE_INTENSITY_ACCENT5 = None;
        // Java: QUICK_SHAPE_STYLE_INTENSITY_ACCENT6 = null;
        self.QUICK_SHAPE_STYLE_INTENSITY_ACCENT6 = None;
    }

    // Java: static DgShapeStyle getPresetStyle(int type)
    pub fn getPresetStyle(r#type: i32) -> DgShapeStyle {
        // Java: return DgShapeStyle.PresetStyleBuilder.build(type);
        // LLM: PresetStyleBuilder.build depends on static lazy-init cache; entire method aggregated
        todo!()
    }

    // Java: static int presetType(DgShapeStyle shapeStyle)
    pub fn presetType_static(shapeStyle: &DgShapeStyle) -> i32 {
        // Java: if (getPresetStyle(7).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 7;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(8).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 8;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(9).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 9;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(10).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 10;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(11).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 11;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(12).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 12;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(13).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 13;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(0).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 0;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(1).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(2).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 2;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(3).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 3;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(4).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 4;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(5).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 5;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(6).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 6;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(35).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 35;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(36).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 36;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(37).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 37;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(38).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 38;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(39).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 39;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(40).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 40;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(41).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 41;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(14).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 14;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(15).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 15;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(16).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 16;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(17).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 17;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(18).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 18;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(19).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 19;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(20).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 20;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(28).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 28;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(29).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 29;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(30).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 30;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(31).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 31;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(32).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 32;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(33).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 33;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(34).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 34;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(21).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 21;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(22).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 22;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(23).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 23;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(24).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 24;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(25).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 25;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else if (getPresetStyle(26).isEqual(shapeStyle))
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return 26;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return getPresetStyle(27).isEqual(shapeStyle) ? 27 : -1;
        // LLM: getPresetStyle depends on static lazy-init cache; entire if-else chain aggregated
        todo!()
    }

    // Java: static void intensityColor(DgShapeStyle style, int color)
    fn intensityColor(style: &mut DgShapeStyle, color: i32) {
        // Java: initQuickStyleLn(style, 0, color, (double)-1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFill(style, 3, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleEffect(style, 3, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFont(style, 1, 1);
        // LLM: initQuickStyleLn/Fill/Effect/Font depend on KProto chain; entire method aggregated
        todo!()
    }

    // Java: static void mediumColor(DgShapeStyle style, int color)
    fn mediumColor(style: &mut DgShapeStyle, color: i32) {
        // Java: initQuickStyleLn(style, 1, color, (double)-1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFill(style, 3, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleEffect(style, 2, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFont(style, 1, 1);
        // LLM: initQuickStyleLn/Fill/Effect/Font depend on KProto chain; entire method aggregated
        todo!()
    }

    // Java: static void tinyColor(DgShapeStyle style, int color)
    fn tinyColor(style: &mut DgShapeStyle, color: i32) {
        // Java: initQuickStyleLn(style, 1, color, (double)-1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFill(style, 2, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleEffect(style, 1, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFont(style, 1, 0);
        // LLM: initQuickStyleLn/Fill/Effect/Font depend on KProto chain; entire method aggregated
        todo!()
    }

    // Java: static void lnFillColor(DgShapeStyle style, int color)
    fn lnFillColor(style: &mut DgShapeStyle, color: i32) {
        // Java: initQuickStyleLn(style, 3, 1, (double)-1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFill(style, 1, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleEffect(style, 1, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFont(style, 1, 1);
        // LLM: initQuickStyleLn/Fill/Effect/Font depend on KProto chain; entire method aggregated
        todo!()
    }

    // Java: static void fillColor(DgShapeStyle style, int color)
    fn fillColor_static(style: &mut DgShapeStyle, color: i32) {
        // Java: initQuickStyleLn(style, 2, color, (double)0.5F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFill(style, 1, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleEffect(style, 0, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFont(style, 1, 1);
        // LLM: initQuickStyleLn/Fill/Effect/Font depend on KProto chain; entire method aggregated
        todo!()
    }

    // Java: static void lnColor(DgShapeStyle style, int color)
    fn lnColor_static(style: &mut DgShapeStyle, color: i32) {
        // Java: initQuickStyleLn(style, 2, color, (double)-1.0F);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFill(style, 1, 1);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleEffect(style, 0, color);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: initQuickStyleFont(style, 1, 0);
        // LLM: initQuickStyleLn/Fill/Effect/Font depend on KProto chain; entire method aggregated
        todo!()
    }

    // Java: static void initQuickStyleLn(DgShapeStyle style, int lnRefId, int lnColorValue, double shade)
    fn initQuickStyleLn(style: &mut DgShapeStyle, lnRefId: i32, lnColorValue: i32, shade: f64) {
        // Java: style.setOutlineRefIndex(lnRefId);
        style.setOutlineRefIndex(lnRefId);
        // Java: style.outlineColor().putColor(DgColor.scheme(lnColorValue));
        // LLM: DgColor.scheme depends on KProto chain; putColor aggregated
        // Java: if (shade != (double)-1.0F)
        if shade != -1.0f64 {
            // Java: style.outlineColor().putShade(shade);
            // LLM: putShade depends on KProto chain
            todo!()
        }
    }

    // Java: static void initQuickStyleFill(DgShapeStyle style, int fillRefId, int fillColor)
    fn initQuickStyleFill(style: &mut DgShapeStyle, fillRefId: i32, fillColor: i32) {
        // Java: style.setFillRefIndex(fillRefId);
        style.setFillRefIndex(fillRefId);
        // Java: style.fillColor().putColor(DgColor.scheme(fillColor));
        // LLM: DgColor.scheme depends on KProto chain
        todo!()
    }

    // Java: static void initQuickStyleEffect(DgShapeStyle style, int effectRefId, int effectColor)
    fn initQuickStyleEffect(style: &mut DgShapeStyle, effectRefId: i32, effectColor: i32) {
        // Java: style.setEffectRefIndex(effectRefId);
        style.setEffectRefIndex(effectRefId);
        // Java: style.effectColor().putColor(DgColor.scheme(effectColor));
        // LLM: DgColor.scheme depends on KProto chain
        todo!()
    }

    // Java: static void initQuickStyleFont(DgShapeStyle style, int fontRefId, int fontColor)
    fn initQuickStyleFont(style: &mut DgShapeStyle, fontRefId: i32, fontColor: i32) {
        // Java: style.setFontRefIndex(fontRefId);
        style.setFontRefIndex(fontRefId);
        // Java: style.fontColor().putColor(DgColor.scheme(fontColor));
        // LLM: DgColor.scheme depends on KProto chain
        todo!()
    }

    // Java: KoBin protocol()
    pub fn protocol_local(&mut self) -> KoBin {
        // Java: this.iniColor2Protocol();
        self.iniColor2Protocol();
        // Java: return this.shapeStyle.save();
        self.shapeStyle.save()
    }

    // Java: void updateProtocol(KoBin data)
    pub fn updateProtocol_local(&mut self, data: KoBin) {
        // Java: this.shapeStyle.load(data);
        self.shapeStyle.load(data);
        // Java: this.iniProtocol2Color();
        self.iniProtocol2Color();
    }

    // Java: int outlineRefIndex()
    pub fn outlineRefIndex(&mut self) -> i32 {
        // Java: return this.shapeStyle.lnRef().idx();
        self.shapeStyle.lnRef().idx()
    }

    // Java: void setOutlineRefIndex(int index)
    pub fn setOutlineRefIndex(&mut self, index: i32) {
        // Java: this.shapeStyle.lnRef().putIdx(index);
        self.shapeStyle.lnRef().putIdx(index);
    }

    // Java: DgColorFormat outlineColor()
    pub fn outlineColor(&mut self) -> &mut DgColorFormat {
        // Java: return this.lnColor;
        &mut self.lnColor
    }

    // Java: int fillRefIndex()
    pub fn fillRefIndex(&mut self) -> i32 {
        // Java: return this.shapeStyle.fillRef().idx();
        self.shapeStyle.fillRef().idx()
    }

    // Java: void setFillRefIndex(int index)
    pub fn setFillRefIndex(&mut self, index: i32) {
        // Java: this.shapeStyle.fillRef().putIdx(index);
        self.shapeStyle.fillRef().putIdx(index);
    }

    // Java: DgColorFormat fillColor()
    pub fn fillColor(&mut self) -> &mut DgColorFormat {
        // Java: return this.fillColor;
        &mut self.fillColor
    }

    // Java: int effectRefIndex()
    pub fn effectRefIndex(&mut self) -> i32 {
        // Java: return this.shapeStyle.effectRef().idx();
        self.shapeStyle.effectRef().idx()
    }

    // Java: void setEffectRefIndex(int index)
    pub fn setEffectRefIndex(&mut self, index: i32) {
        // Java: this.shapeStyle.effectRef().putIdx(index);
        self.shapeStyle.effectRef().putIdx(index);
    }

    // Java: DgColorFormat effectColor()
    pub fn effectColor(&mut self) -> &mut DgColorFormat {
        // Java: return this.effectColor;
        &mut self.effectColor
    }

    // Java: int fontRefIndex()
    pub fn fontRefIndex(&mut self) -> i32 {
        // Java: return this.shapeStyle.fontRef().idx();
        self.shapeStyle.fontRef().idx()
    }

    // Java: void setFontRefIndex(int index)
    pub fn setFontRefIndex(&mut self, index: i32) {
        // Java: this.shapeStyle.fontRef().putIdx(index);
        self.shapeStyle.fontRef().putIdx(index);
    }

    // Java: DgColorFormat fontColor()
    pub fn fontColor(&mut self) -> &mut DgColorFormat {
        // Java: return this.fontColor;
        &mut self.fontColor
    }

    // Java: boolean hasFontRef()
    pub fn hasFontRef(&self) -> bool {
        // Java: return this.shapeStyle.hasFontRef();
        self.shapeStyle.hasFontRef()
    }

    // Java: StyleMatrixReference fontRef()
    pub fn fontRef(&mut self) -> &mut StyleMatrixReference {
        // Java: return this.shapeStyle.fontRef();
        self.shapeStyle.fontRef()
    }

    // Java: void iniProtocol2Color()
    fn iniProtocol2Color(&mut self) {
        // Java: Color fillColor = this.shapeStyle.fillRef().color();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.fillColor.update(fillColor.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: Color effectColor = this.shapeStyle.effectRef().color();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.effectColor.update(effectColor.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: Color lnColor = this.shapeStyle.lnRef().color();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.lnColor.update(lnColor.save());
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: Color fontColor = this.shapeStyle.fontRef().color();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.fontColor.update(fontColor.save());
        // LLM: cross-module KoBin types differ; Color.save() returns KProto KoBin; entire method aggregated
        todo!()
    }

    // Java: void iniColor2Protocol()
    fn iniColor2Protocol(&mut self) {
        // Java: this.shapeStyle.fillRef().writeColor(Color.open(this.fillColor.data()));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.shapeStyle.effectRef().writeColor(Color.open(this.effectColor.data()));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.shapeStyle.lnRef().writeColor(Color.open(this.lnColor.data()));
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.shapeStyle.fontRef().writeColor(Color.open(this.fontColor.data()));
        // LLM: cross-module KoBin types differ; Color.open depends on KProto chain; entire method aggregated
        todo!()
    }

    // Java: void copy(DgShapeStyle style)
    pub fn copy(&mut self, style: &mut DgShapeStyle) {
        // Java: this.updateProtocol(style.protocol());
        // LLM: self.updateProtocol mapped to updateProtocol_local; style.protocol mapped to protocol_local; cross-module KoBin types differ
        todo!()
    }

    // Java: boolean isEqual(DgShapeStyle shapeStyle1)
    pub fn isEqual(&mut self, shapeStyle1: &mut DgShapeStyle) -> bool {
        // Java: if (shapeStyle1 == null)
        // LLM No Rust equivalent: Rust has no null; shapeStyle1 is a reference
        // Java: return false;
        // LLM No Rust equivalent: null check not needed in Rust
        // Java: else
        // LLM No Rust equivalent: null else branch collapsed in Rust
        // Java: boolean isSameLn = shapeStyle1.outlineRefIndex() == this.outlineRefIndex() && DgColorFormat.isEqual(shapeStyle1.lnColor, this.lnColor);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (!isSameLn)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return false;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM No Rust equivalent: nested else collapsed in Rust
        // Java: boolean isSameFill = shapeStyle1.fillRefIndex() == this.fillRefIndex() && DgColorFormat.isEqual(shapeStyle1.fillColor, this.fillColor);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (!isSameFill)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return false;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM No Rust equivalent: nested else collapsed in Rust
        // Java: boolean isSameEffect = shapeStyle1.effectRefIndex() == this.effectRefIndex() && DgColorFormat.isEqual(shapeStyle1.effectColor, this.effectColor);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (!isSameEffect)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return false;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: else
        // LLM No Rust equivalent: nested else collapsed in Rust
        // Java: boolean isSameFont = shapeStyle1.fontRefIndex() == this.fontRefIndex() && DgColorFormat.isEqual(shapeStyle1.fontColor, this.fontColor);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return isSameFont;
        // LLM: DgColorFormat.isEqual depends on KProto chain; entire method aggregated
        todo!()
    }
}

impl PresetStyleBuilder {
    // Java: static DgShapeStyle build(int type)
    pub fn build(r#type: i32) -> DgShapeStyle {
        // Java: switch (type)
        // LLM: Java static lazy-init cache; entire switch aggregated with todo!()
        // Java: case 0:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_DK1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_DK1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_DK1, 0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_DK1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 1:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT1, 4);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 2:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT2 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT2 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT2, 5);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT2;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 3:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT3 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT3 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT3, 6);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT3;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 4:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT4 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT4 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT4, 7);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT4;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 5:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT5 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT5 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT5, 8);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT5;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 6:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT6 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT6 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT6, 9);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_ACCENT6;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 7:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_FILL_DK1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_FILL_DK1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.fillColor(DgShapeStyle.QUICK_SHAPE_STYLE_FILL_DK1, 0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_FILL_DK1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 8:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.fillColor(DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT1, 4);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 9:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT2 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT2 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.fillColor(DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT2, 5);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT2;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 10:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT3 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT3 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.fillColor(DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT3, 6);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT3;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 11:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT4 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT4 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.fillColor(DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT4, 7);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT4;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 12:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT5 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT5 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.fillColor(DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT5, 8);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT5;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 13:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT6 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT6 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.fillColor(DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT6, 9);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_FILL_ACCENT6;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 14:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_DK1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_DK1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnFillColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_DK1, 0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_DK1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 15:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnFillColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT1, 4);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 16:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT2 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT2 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnFillColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT2, 5);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT2;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 17:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT3 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT3 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnFillColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT3, 6);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT3;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 18:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT4 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT4 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnFillColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT4, 7);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT4;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 19:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT5 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT5 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnFillColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT5, 8);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT5;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 20:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT6 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT6 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.lnFillColor(DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT6, 9);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_LN_FILL_ACCENT6;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 21:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_TINY_DK1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_TINY_DK1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.tinyColor(DgShapeStyle.QUICK_SHAPE_STYLE_TINY_DK1, 0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_TINY_DK1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 22:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.tinyColor(DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT1, 4);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 23:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT2 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT2 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.tinyColor(DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT2, 5);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT2;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 24:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT3 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT3 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.tinyColor(DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT3, 6);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT3;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 25:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT4 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT4 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.tinyColor(DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT4, 7);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT4;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 26:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT5 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT5 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.tinyColor(DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT5, 8);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT5;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 27:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT6 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT6 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.tinyColor(DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT6, 9);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_TINY_ACCENT6;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 28:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_DK1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_DK1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.mediumColor(DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_DK1, 0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_DK1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 29:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.mediumColor(DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT1, 4);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 30:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT2 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT2 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.mediumColor(DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT2, 5);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT2;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 31:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT3 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT3 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.mediumColor(DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT3, 6);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT3;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 32:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT4 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT4 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.mediumColor(DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT4, 7);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT4;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 33:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT5 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT5 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.mediumColor(DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT5, 8);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT5;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 34:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT6 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT6 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.mediumColor(DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT6, 9);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_MEDIUM_ACCENT6;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 35:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_DK1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_DK1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.intensityColor(DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_DK1, 0);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_DK1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 36:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT1 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT1 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.intensityColor(DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT1, 4);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT1;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 37:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT2 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT2 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.intensityColor(DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT2, 5);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT2;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 38:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT3 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT3 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.intensityColor(DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT3, 6);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT3;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 39:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT4 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT4 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.intensityColor(DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT4, 7);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT4;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 40:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT5 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT5 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.intensityColor(DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT5, 8);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT5;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: case 41:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT6 == null)
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT6 = new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: DgShapeStyle.intensityColor(DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT6, 9);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return DgShapeStyle.QUICK_SHAPE_STYLE_INTENSITY_ACCENT6;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: default:
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: assert false;
        // LLM No Rust equivalent: Java assert false
        // Java: return new DgShapeStyle();
        // LLM JavaLine Aggregation: merged into the next Rust statement
        todo!()
    }
}

impl KmoProtocol for DgShapeStyle {
    fn protocol(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin> { todo!() }
    fn updateProtocol(&mut self, _var1: &dyn crate::cn::wps::kso::kproto::KoBin::KoBin) { todo!() }
}
