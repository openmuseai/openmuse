#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code)]
//! Compile-surface hosts for ConvertTool parked signatures (3D/effect/dg types).
//! Not under libs/core-libs/stubs. Hot-path types stay layout_shape / drawing.

use std::sync::Arc;
use std::any::Any;
use std::collections::HashMap;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

#[derive(Default, Clone, Debug)]
pub struct GraphicObject;
#[derive(Default, Clone, Debug)]
pub struct DgPicture;
#[derive(Default, Clone, Debug)]
pub struct KmoTextBody;
impl KmoTextBody {
    pub fn hasTextBodyProperties(&self) -> bool { false }
    pub fn textBodyProperties(&self) -> TextBodyProperties { TextBodyProperties::default() }
}
#[derive(Default, Clone, Debug)]
pub struct GroupTransform2D;
impl GroupTransform2D {
    pub fn childPoint(&self) -> Point2D { Point2D::default() }
    pub fn childExtents(&self) -> PositiveSize2D { PositiveSize2D::default() }
}
#[derive(Default, Clone, Debug)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}
impl Point2D {
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
}
#[derive(Default, Clone, Debug)]
pub struct PositiveSize2D {
    pub cx: i32,
    pub cy: i32,
}
impl PositiveSize2D {
    pub fn cx(&self) -> i32 { self.cx }
    pub fn cy(&self) -> i32 { self.cy }
}
#[derive(Default, Clone, Debug)]
pub struct Color {
    /// rust: ★W6c-6 —— 颜色载体（`0xAARRGGBB`）。原为空单元壳，使
    ///   `FillConvert::convertToSolidFill` 拿不到任何颜色值。
    ///   owner: ppt.show.convert.j2r_hosts.Color    wave: M-PPT-KMOSLIDE-W6c-6
    pub argb: i32,
}
impl Color {
    pub fn new() -> Self { Self { argb: 0 } }
    pub fn new_with_argb(argb: i32) -> Self { Self { argb } }
}
#[derive(Default, Clone, Debug)]
pub struct ColorTransform;
#[derive(Default, Clone, Debug)]
pub struct EffectList;
impl EffectList {
    pub fn hasOuterShadow(&self) -> bool { false }
    pub fn hasInnerShadow(&self) -> bool { false }
    pub fn hasPresetShadowEffect(&self) -> bool { false }
    pub fn hasFillOverlay(&self) -> bool { false }
    pub fn innerShadow(&self) -> InnerShadowEffect { InnerShadowEffect::default() }
    pub fn presetShadowEffect(&self) -> PresetShadowEffect { PresetShadowEffect::default() }
    pub fn fillOverlay(&self) -> FillOverlayEffect { FillOverlayEffect::default() }
    pub fn hasReflection(&self) -> bool { false }
    pub fn hasGlow(&self) -> bool { false }
    pub fn hasSoftEdge(&self) -> bool { false }
    pub fn outerShadow(&self) -> OuterShadowEffect { OuterShadowEffect::default() }
    pub fn reflection(&self) -> ReflectionEffect { ReflectionEffect::default() }
    pub fn glow(&self) -> GlowEffect { GlowEffect::default() }
    pub fn softEdge(&self) -> SoftEdgeEffect { SoftEdgeEffect::default() }
}
#[derive(Default, Clone, Debug)]
pub struct FillOverlayEffect;
impl FillOverlayEffect {
    pub fn hasFill(&self) -> bool { false }
    pub fn fill(&self) -> Fill { Fill::default() }
}
#[derive(Default, Clone, Debug)]
pub struct GlowEffect;
#[derive(Default, Clone, Debug)]
pub struct InnerShadowEffect;
#[derive(Default, Clone, Debug)]
pub struct OuterShadowEffect;
#[derive(Default, Clone, Debug)]
pub struct PresetShadowEffect;
#[derive(Default, Clone, Debug)]
pub struct ReflectionEffect;
#[derive(Default, Clone, Debug)]
pub struct SoftEdgeEffect;
#[derive(Default, Clone, Debug)]
pub struct CustomGeometry;
impl CustomGeometry {
    pub fn hasAvLst(&self) -> bool { false }
}
#[derive(Default, Clone, Debug)]
pub struct Geometry2D {
    pub geom_type: i32,
    pub has_prst: bool,
    pub has_cust: bool,
    pub prst: i32,
}
impl Geometry2D {
    pub fn r#type(&self) -> i32 { self.geom_type }
    pub fn hasPrstGeom(&self) -> bool { self.has_prst }
    pub fn hasCustGeom(&self) -> bool { self.has_cust }
    pub fn prstGeom(&self) -> PresetGeometry {
        PresetGeometry { prst: self.prst, has_av: false }
    }
    pub fn custGeom(&self) -> CustomGeometry { CustomGeometry::default() }
}
#[derive(Default, Clone, Debug)]
pub struct PresetGeometry {
    pub prst: i32,
    pub has_av: bool,
}
impl PresetGeometry {
    pub fn prst(&self) -> i32 { self.prst }
    pub fn hasAvLst(&self) -> bool { self.has_av }
}
#[derive(Default, Clone, Debug)]
pub struct ShapeStyle;
impl ShapeStyle {
    pub fn hasLnRef(&self) -> bool { false }
    pub fn hasFillRef(&self) -> bool { false }
    pub fn lnRef(&self) -> StyleMatrixReference { StyleMatrixReference::default() }
    pub fn fillRef(&self) -> StyleMatrixReference { StyleMatrixReference::default() }
}
#[derive(Default, Clone, Debug)]
pub struct StyleMatrixReference;
impl StyleMatrixReference {
    pub fn hasIdx(&self) -> bool { true }
    pub fn idx(&self) -> i32 { 0 }
    pub fn color(&self) -> Color { Color::default() }
}
#[derive(Default, Clone, Debug)]
pub struct TextBodyProperties;
impl TextBodyProperties {
    pub fn hasTextAutofit(&self) -> bool { false }
    pub fn textAutofit(&self) -> TextAutofit { TextAutofit::default() }
}
#[derive(Default, Clone, Debug)]
pub struct TextAutofit {
    pub r#type: i32,
}
impl TextAutofit {
    pub fn r#type(&self) -> i32 { self.r#type }
}
#[derive(Default, Clone, Debug)]
pub struct Apply3DShapeProperties;
#[derive(Default, Clone, Debug)]
pub struct Camera;
#[derive(Default, Clone, Debug)]
pub struct LightRig;
#[derive(Default, Clone, Debug)]
pub struct Rotation;
#[derive(Default, Clone, Debug)]
pub struct KmoPresentation;
impl KmoPresentation {
    pub fn inkLib(&self) -> Option<KmoInkLib> { None }
    pub fn getFileManager(&self) -> Element { None }
    pub fn slideWidth(&self) -> i32 { 0 }
    pub fn slideHeight(&self) -> i32 { 0 }
    pub fn getSlideCache(&self) -> crate::cn::wps::show::convert::j2r_hosts::SlideCache {
        crate::cn::wps::show::convert::j2r_hosts::SlideCache::global()
    }
}
#[derive(Default, Clone, Debug)]
pub struct KmoInkLib;
impl KmoInkLib {
    pub fn getInk(&self, _id: i32) -> Option<Ink> { None }
}
#[derive(Default, Clone, Debug)]
pub struct KmoModel3D;
#[derive(Default, Clone, Debug)]
pub struct KmoOleObject;
#[derive(Default, Clone, Debug)]
pub struct IShape2Bitmap;
#[derive(Default, Clone, Debug)]
pub struct ShapeTool;
#[derive(Default, Clone, Debug)]
pub struct Background;
impl Background {
    pub fn hasFill(&self) -> bool { false }
    pub fn hasBgRef(&self) -> bool { false }
    pub fn fill(&self) -> Fill { Fill::default() }
    pub fn bgRef(&self) -> StyleMatrixReference { StyleMatrixReference::default() }
}
#[derive(Default, Clone, Debug)]
pub struct Object3DExt;
#[derive(Default, Clone, Debug)]
pub struct Shape3dConvert;
#[derive(Default, Clone, Debug)]
pub struct Vector3f;
#[derive(Default, Clone, Debug)]
pub struct ImagePool;
#[derive(Default, Clone, Debug)]
pub struct RenderLogDumper;
impl RenderLogDumper {
    pub fn increaseConvert(_start: i64, _end: i64) {}
}
#[derive(Default, Clone, Debug)]
pub struct Ink;
#[derive(Default, Clone, Debug)]
pub struct InkModel;
impl InkModel {
    pub fn new(_ink: Ink, _fm: Element) -> Self { Self }
}
#[derive(Default, Clone, Debug)]
pub struct ColorScheme;
impl ColorScheme {
    pub fn create() -> Self { Self }
}
#[derive(Default, Clone, Debug)]
pub struct ColorMapping;
#[derive(Default, Clone, Debug)]
pub struct KmoSysColor;
impl KmoSysColor {
    pub const DEFAULT_SYS_COLOR: KmoSysColor = KmoSysColor {};
}
#[derive(Default, Clone, Debug)]
pub struct KmoTheme;
impl KmoTheme {
    pub fn getColorScheme(&self) -> ColorScheme { ColorScheme::default() }
}
#[derive(Default, Clone, Debug)]
pub struct ColorInterpreter;
impl ColorInterpreter {
    // rust: ★W6c-6 —— **转写纠错：返回载体里的真实颜色**。原实现恒返回不透明黑，
    //   使任何实色填充都变成 `0xFF000000`（并被下游 `.filter(|c| *c != 0)` 放行，
    //   表现为"颜色全错"而非"无颜色"）。
    //   Java: `ColorInterpreter.getHexRGBAValue` 会做主题/相位色解析；
    //   此处仅搬运已解析的 argb，主题解析属 `ColorInterpretCtx` 既有职责。
    //   owner: ppt.show.convert.j2r_hosts.ColorInterpreter    wave: M-PPT-KMOSLIDE-W6c-6
    pub fn getHexRGBAValue(color: &Color, _ctx: Option<&ColorInterpretCtxHost>) -> i32 {
        color.argb
    }
}

/// Fill host used by convert until drawing.data.Fill is on the default surface.
#[derive(Default, Clone, Debug)]
pub struct Fill {
    pub fill_type: i32,
    pub solid: i32,
}
impl Fill {
    pub fn r#type(&self) -> i32 { self.fill_type }
    // rust: ★W6c-6 —— 搬运 `solid`（见 `from_data_fill` 注）。
    pub fn solidFill(&self) -> Color { Color::new_with_argb(self.solid) }
    pub fn hasSolidFill(&self) -> bool { self.fill_type == 0 }
    pub fn hasGradFill(&self) -> bool { self.fill_type == 4 }
    pub fn hasBlipFill(&self) -> bool { self.fill_type == 2 }
    pub fn hasPattFill(&self) -> bool { self.fill_type == 1 }
    pub fn blipFill(&self) -> BlipFillData { BlipFillData::default() }
    pub fn from_fill_obj(obj: &crate::cn::wps::show::util::layout_shape::FillObj) -> Self {
        Self { fill_type: obj.r#type(), solid: 0 }
    }
    // rust: ★③-0c —— 形参由 `shapepr_hosts::Fill`（空壳）改为**真身** `data::fill::Fill`。
    // rust: ★W6c-6 —— **转写纠错：补上 `solid` 颜色搬运**。原实现恒 `solid: 0`，
    //   即把实色填充的颜色整个丢掉 ⇒ `ConvertTool` 侧 `fillBase.getColor()` 恒 0
    //   ⇒ `mvp_pipeline::render_slide` 的 `.filter(|c| *c != 0)` 判定失败、
    //   该形状**不产生 DrawRect 填充**（实测 `03_function_mvp_hello_slide::t01`
    //   期望 `0xFF4472C4` 而实得 `0xFFFFFFFF`）。
    //   Java 侧 `kmoFillBase = shapeProperties.fill()` 拿到的是**同一个对象**，
    //   颜色随对象一起流动；本 port 的 `FillData` 是值拷贝载体，故需显式搬运。
    //   owner: ppt.show.convert.j2r_hosts.FillData    wave: M-PPT-KMOSLIDE-W6c-6
    pub fn from_data_fill(obj: &crate::cn::wps::drawing::data::fill::Fill::Fill) -> Self {
        Self { fill_type: obj.r#type(), solid: obj.plain_solid_argb() }
    }
}
#[derive(Default, Clone, Debug)]
pub struct BlipFillData;

/// Re-export data ShapePr / layout Transform2D under the Java names used by convert.
pub use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
pub use crate::cn::wps::drawing::data::shape::shapepr_hosts::Transform2D;

#[derive(Default, Clone, Debug)]
pub struct EffectHolder;
impl EffectHolder {
    pub fn hasEffectLst(&self) -> bool { false }
    pub fn effectLst(&self) -> EffectList { EffectList::default() }
}

#[derive(Default, Clone, Debug)]
pub struct System;
impl System {
    pub fn nanoTime() -> i64 { 0 }
}

#[derive(Default, Clone, Debug)]
pub struct Math;
impl Math {
    pub fn round(v: f32) -> f32 { v.round() }
    pub fn max(a: f32, b: f32) -> f32 { a.max(b) }
}

#[derive(Default, Clone, Debug)]
pub struct FilterInterpretCtx {
    pub mInterpretFlag: i32,
}
impl FilterInterpretCtx {
    pub fn hasFilter(&self) -> bool { self.mInterpretFlag != 0 }
    pub fn isFilterBlipFill(&self) -> bool { (self.mInterpretFlag & 4097) != 0 }
    pub fn isFilterPatternFill(&self) -> bool { (self.mInterpretFlag & 4098) != 0 }
    pub fn reset(&mut self) { self.mInterpretFlag = 0; }
}

#[derive(Default, Clone, Debug)]
pub struct ContentInterpretCtx {
    pub mPageNumber: i32,
}
impl ContentInterpretCtx {
    pub fn slideNumber(&self) -> i32 { self.mPageNumber }
    pub fn setPageNumber(&mut self, n: i32) { self.mPageNumber = n; }
}

#[derive(Default, Clone, Debug)]
pub struct ThemeInterpretCtx;
impl ThemeInterpretCtx {
    pub fn fillAt(&self, _idx: i32) -> Option<Fill> { None }
    pub fn outlineAt(&self, _idx: i32) -> OutlineObj {
        OutlineObj::default()
    }
}

#[derive(Default, Clone, Debug)]
pub struct ColorInterpretCtxHost;
impl ColorInterpretCtxHost {
    pub fn new() -> Self { Self }
    pub fn reset(&mut self) {}
    pub fn setSlide(&mut self, _slide: Option<&()>) {}
    pub fn withPhColor(&self, _c: Color) -> &Self { self }
}

#[derive(Default, Clone, Debug)]
pub struct InterpretCtx {
    pub mFilterInterpretCtx: FilterInterpretCtx,
    pub mContentInterpretCtr: ContentInterpretCtx,
    pub mColorInterpretCtx: ColorInterpretCtxHost,
    pub mThemeInterpretCtx: ThemeInterpretCtx,
}
impl InterpretCtx {
    pub fn new() -> Self { Self::default() }
    pub fn getFilterInterpretCtx(&self) -> &FilterInterpretCtx { &self.mFilterInterpretCtx }
    pub fn getContentInterpretCtx(&self) -> &ContentInterpretCtx { &self.mContentInterpretCtr }
    pub fn getColorInterpretCtx(&self) -> &ColorInterpretCtxHost { &self.mColorInterpretCtx }
    pub fn getThemeInterpretCtx(&self) -> &ThemeInterpretCtx { &self.mThemeInterpretCtx }
}

#[derive(Default, Clone, Debug)]
pub struct TextContext;
impl TextContext {
    pub fn setPageNumber(&mut self, _n: i32) {}
}

#[derive(Default, Clone, Debug)]
pub struct ShapeText {
    pub text_ctx: TextContext,
}
impl ShapeText {
    pub fn new(_shape: &drawing::cn::wps::moffice::drawing::Shape::Shape, _body: KmoTextBody) -> Self {
        Self::default()
    }
    pub fn getTextContext(&mut self) -> &mut TextContext { &mut self.text_ctx }
    pub fn setFitShape2Text(&mut self, _v: bool) {}
}

use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::graphics::FillBase::FillBase;
use drawing::cn::wps::moffice::drawing::graphics::SolidFill::SolidFill;
use drawing::cn::wps::moffice::drawing::graphics::GradFill::GradFill;
use drawing::cn::wps::moffice::drawing::graphics::BlipFill::BlipFill;
use drawing::cn::wps::moffice::drawing::graphics::PatternFill::PatternFill;
use drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use crate::cn::wps::show::util::layout_shape::{OutlineObj, GeomObj};
use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;
use crate::cn::wps::show::render::cache::ShapeInfo::ShapeInfo;
use crate::cn::wps::show::render::cache::ShapeInfoCache::ShapeInfoCache;
use std::cell::RefCell;

pub struct DgShape {
    pub sp_pr: ShapePr,
    pub hidden: bool,
}
impl Default for DgShape {
    fn default() -> Self {
        Self { sp_pr: ShapePr::create(), hidden: false }
    }
}
impl Clone for DgShape {
    fn clone(&self) -> Self {
        Self { sp_pr: self.sp_pr.convert_clone(), hidden: self.hidden }
    }
}
impl DgShape {
    pub fn hasSpPr(&self) -> bool { true }
    pub fn load(&mut self, _dg: impl Sized) {}
    pub fn close(&mut self) {}
    pub fn hasBlipFill(&self) -> bool { false }
    pub fn blipFill(&self) -> Fill { Fill::default() }
    pub fn spPr(&self) -> ShapePr { self.sp_pr.convert_clone() }
    pub fn groupXfrm(&self) -> GroupTransform2D { GroupTransform2D::default() }
    pub fn cNvPr(&self) -> CNvPr { CNvPr { hidden: self.hidden } }
    pub fn drawingShape(&self) {}
}
#[derive(Default, Clone, Debug)]
pub struct CNvPr { pub hidden: bool }
impl CNvPr {
    pub fn isHidden(&self) -> bool { self.hidden }
}

thread_local! {
    static SLIDE_CACHE: RefCell<SlideCache> = RefCell::new(SlideCache::default());
}

#[derive(Default, Clone)]
pub struct SlideCache {
    pub info_cache: ShapeInfoCache,
}
thread_local! {
    static PUBLIC_SHAPES: RefCell<HashMap<i32, Shape>> = RefCell::new(HashMap::new());
}

impl SlideCache {
    pub fn global() -> Self {
        SLIDE_CACHE.with(|c| c.borrow().clone())
    }
    pub fn shapeInfoCache(&self) -> ShapeInfoCache {
        self.info_cache.clone()
    }
    pub fn getShape(&self, showShape: &KmoShowShape) -> Option<Shape> {
        PUBLIC_SHAPES.with(|m| {
            m.borrow_mut()
                .get_mut(&showShape.id())
                .and_then(|s| s.clone().ok())
        })
    }
    pub fn addShape(&mut self, showShape: &KmoShowShape, shape: Shape) -> bool {
        PUBLIC_SHAPES.with(|m| {
            m.borrow_mut().insert(showShape.id(), shape);
        });
        true
    }
    pub fn getDShape() -> DgShape {
        DgShape::default()
    }
}

pub struct IMediaLib;
