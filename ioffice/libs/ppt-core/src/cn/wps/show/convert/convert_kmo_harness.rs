#![allow(non_snake_case)]
//! Convert-path test harness: real `KmoShowShape` + encoded spid + DummySlide.
//! DummySlide is boxed so its address stays stable after `ISlide` pointer capture.

use crate::cn::wps::drawing::app::DgStoreData::DgStoreData;
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use crate::cn::wps::drawing::data::shape::shapepr_hosts::Transform2D;
use crate::cn::wps::show::app::shape::kmo_show_shape_hosts;
use crate::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use crate::cn::wps::show::app::slide::ISlide::{
    Background, ColorMapping, ISlide, KmoPresentation, KmoSlideEventListeners, KmoTheme, KsoWMInfo,
};

pub struct DummySlide {
    pres: KmoPresentation,
    theme: KmoTheme,
    events: KmoSlideEventListeners,
    bg: Background,
    wm: KsoWMInfo,
    cm: ColorMapping,
}

impl DummySlide {
    fn new() -> Self {
        DummySlide {
            pres: KmoPresentation::default(),
            theme: KmoTheme,
            events: KmoSlideEventListeners::new(),
            // rust: ★W6c-4 —— 三个真身类型不再是单元结构体 ⇒ 必须构造实例。
            //   owner: ppt.show.convert.convert_kmo_harness    wave: M-PPT-KMOSLIDE-W6c-4
            bg: Background::new(),
            wm: KsoWMInfo::new(),
            cm: ColorMapping::create(),
        }
    }
}

impl ISlide for DummySlide {
    fn r#type(&self) -> i32 {
        0
    }
    fn id(&self) -> i64 {
        0
    }
    fn getPresentation(&self) -> &KmoPresentation {
        &self.pres
    }
    fn theme(&self) -> &KmoTheme {
        &self.theme
    }
    fn eventListeners(&self) -> &KmoSlideEventListeners {
        &self.events
    }
    fn spTree(&self) -> &KmoShapeTree {
        panic!("DummySlide.spTree unused by convert harness")
    }
    fn replaceSpid(&self, _var1: i32, _var2: i32) {}
    fn background(&self) -> &Background {
        &self.bg
    }
    fn ksoWMInfo(&self) -> &KsoWMInfo {
        &self.wm
    }
    fn colorMapping(&self) -> &ColorMapping {
        &self.cm
    }
    fn isIOReading(&self) -> bool {
        false
    }
}

pub struct ConvertKmo {
    _slide: Box<DummySlide>,
    pub shape: KmoShowShape,
}

impl ConvertKmo {
    pub fn encode_spid(shape_type: i32, index: i32) -> i32 {
        DgStoreData::DgStoreData_new().helper_storeIndex2Id(shape_type, index)
    }

    pub fn new(shape_type: i32, index: i32) -> Self {
        let slide = Box::new(DummySlide::new());
        let spid = Self::encode_spid(shape_type, index);
        let shape = KmoShowShape::KmoShowShape__islide_int_int(&*slide, shape_type, spid);
        ConvertKmo {
            _slide: slide,
            shape,
        }
    }

    pub fn from_layout(layout: &crate::cn::wps::show::util::layout_shape::KmoShowShape) -> Self {
        let this = Self::new(layout.r#type(), layout.id());
        let mut pr = ShapePr::create();
        if let Some(p) = layout.sp_pr.geom_obj.prst {
            pr.geom().putPrst(p);
        }
        if layout.sp_pr.has_fill {
            // rust: ★W6c-6 —— **转写纠错：`fill_obj` 未设时回落到标量 `fill`**。
            //   影子 `ShapePr` 有**两个**填充表示：`fill: i32`（旧）与 `fill_obj: FillObj`。
            //   构造方（如 `mvp_pipeline::hello_slide`）习惯写标量 `fill`，而本函数原先
            //   只读 `fill_obj` ⇒ 取到默认值 `-1`（NONE），实色形状变成无填充
            //   （实测 `03_function_mvp_hello_slide::t01` 期望 `0xFF4472C4` 实得白）。
            //   判定顺序与 `ShapePr::fill()` 的 Java 语义一致：**显式设置者优先**，
            //   两者都未设时保持 `FillObj::default()` 的 `-1`。
            //   owner: ppt.show.convert.convert_kmo_harness.ConvertKmo    wave: M-PPT-KMOSLIDE-W6c-6
            let ft = match layout.sp_pr.fill_obj.fill_type {
                Some(t) => t,
                None => layout.sp_pr.fill,
            };
            pr.fill().putType(ft);
            // rust: ★W6c-6 —— **实色颜色搬运**。影子 `FillObj` 的颜色在 `solid`
            //   （`0xAARRGGBB`）或 `color`（scheme/srgb 槽位）；`from_layout` 原先只搬
            //   填充**类型**、不搬颜色 ⇒ 侧表里的真身填充满是"有型无色"，下游
            //   `plain_solid_argb()` 读回 0，实色形状渲染成白。
            //   owner: ppt.show.convert.convert_kmo_harness.ConvertKmo    wave: M-PPT-KMOSLIDE-W6c-6
            if ft == 0 {
                if let Some(argb) = layout.sp_pr.fill_obj.solid {
                    let mut rgb = pr.fill().solidFill_mut().srgb();
                    rgb.putR((argb >> 16) & 0xFF);
                    rgb.putG((argb >> 8) & 0xFF);
                    rgb.putB(argb & 0xFF);
                }
            }
        }
        if layout.sp_pr.has_ln {
            let o = &layout.sp_pr.outline_obj;
            if o.hasWidth() {
                pr.ln().putWidth(o.width());
            }
            if o.hasFill() {
                pr.ln().putFillType(o.fill().r#type());
            }
            if o.hasCompoundType() {
                pr.ln().putCompoundType(o.compoundType());
            }
            if o.hasLineCap() {
                pr.ln().putLineCap(o.lineCap());
            }
            if o.hasLineJoin() {
                pr.ln().putJoinType(o.lineJoin().r#type());
            }
            if o.hasPresetDash() {
                pr.ln().putPresetDash(o.presetDash());
            }
        }
        kmo_show_shape_hosts::attach_shape_pr(this.shape.id(), pr);
        let mut xfrm = Transform2D::default();
        xfrm.putX(layout.transform2DX);
        xfrm.putY(layout.transform2DY);
        xfrm.putCx(layout.transform2DW);
        xfrm.putCy(layout.transform2DH);
        xfrm.putRotation(layout.rotation);
        xfrm.putFlipH(layout.isFlipHorz);
        xfrm.putFlipV(layout.isFlipVert);
        kmo_show_shape_hosts::attach_xfrm(this.shape.id(), xfrm);
        kmo_show_shape_hosts::set_placeholder(this.shape.id(), layout.is_placeholder);
        kmo_show_shape_hosts::set_text_box(this.shape.id(), layout.is_text_box);
        kmo_show_shape_hosts::set_hidden(this.shape.id(), layout.is_hidden);
        kmo_show_shape_hosts::set_placeholder_type(this.shape.id(), layout.ph.type_);
        let mut side = kmo_show_shape_hosts::ConvertAttach::default();
        side.placeholder_type = layout.ph.type_;
        side.has_text_body = layout.has_text_body;
        side.prst = layout.sp_pr.geom_obj.prst;
        side.fill = if layout.sp_pr.has_fill {
            Some(layout.sp_pr.fill as u32)
        } else {
            None
        };
        side.graphics_path = layout.graphics_path.clone();
        side.media_bytes = layout.media_bytes.clone();
        side.media_r_id = layout.media_r_id.clone();
        side.media_id = if layout.media_id != 0 {
            Some(layout.media_id)
        } else {
            None
        };
        side.text_op = layout.text_op.clone();
        kmo_show_shape_hosts::merge_render_sidecar(this.shape.id(), side);
        this
    }

    pub fn from_layout_tree(layout: &crate::cn::wps::show::util::layout_shape::KmoShowShape) -> Self {
        let mut this = Self::from_layout(layout);
        if layout.r#type() == 0 {
            for child in &layout.children {
                this.shape.addChild(Self::from_layout_tree(child).into_shape());
            }
        }
        this
    }

    pub fn into_shape(self) -> KmoShowShape {
        let ConvertKmo { _slide, shape } = self;
        std::mem::forget(_slide);
        shape
    }

    pub fn attach_shape_pr(&self, pr: ShapePr) {
        kmo_show_shape_hosts::attach_shape_pr(self.shape.id(), pr);
    }

    pub fn attach_xfrm(&self, xfrm: Transform2D) {
        kmo_show_shape_hosts::attach_xfrm(self.shape.id(), xfrm);
    }

    pub fn set_placeholder(&self, v: bool) {
        kmo_show_shape_hosts::set_placeholder(self.shape.id(), v);
    }

    pub fn set_text_box(&self, v: bool) {
        kmo_show_shape_hosts::set_text_box(self.shape.id(), v);
    }

    pub fn set_hidden(&self, v: bool) {
        kmo_show_shape_hosts::set_hidden(self.shape.id(), v);
    }
}
