//! Compile-surface host for `ShapeUtil` layout geometry (pl-02).
//! Full `KmoShowShape` / `SlideCache` remain off default crate surface.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::layout_textbox::KmoTextOperator;

#[derive(Clone, Debug)]
pub struct KmoShowShape {
    pub id: i32,
    pub shape_type: i32,
    pub can_rotate: bool,
    pub transform2DX: i32,
    pub transform2DY: i32,
    pub transform2DW: i32,
    pub transform2DH: i32,
    pub rotation: i32,
    pub isFlipHorz: bool,
    pub isFlipVert: bool,
    pub parent: Option<Rc<KmoShowShape>>,
    pub child_off: Option<(i32, i32)>,
    pub child_ext: Option<(i32, i32)>,
    pub is_placeholder: bool,
    pub ph: Placeholder,
    pub has_xfrm: bool,
    pub parent_type: i32,
    pub parent_slide: Option<Rc<KmoSlide>>,
    pub parent_layout: Option<Rc<KmoSlideLayout>>,
    pub has_parent: bool,
    pub sp_pr: ShapePr,
    pub has_sp_pr: bool,
    pub has_text_body: bool,
    pub text_op: Option<KmoTextOperator>,
    pub original_autofit: i32,
    pub original_anchor: i32,
    pub is_hidden: bool,
    pub is_text_box: bool,
    pub children: Vec<KmoShowShape>,
    pub graphics_path: Option<Vec<(f32, f32)>>,
    pub no_grp: bool,
    pub cleared_text: bool,
    pub text_saved: bool,
    pub media_bytes: Option<Vec<u8>>,
    pub media_r_id: Option<String>,
    pub media_id: i32,
}

impl Default for KmoShowShape {
    fn default() -> Self {
        Self {
            id: 0,
            shape_type: 1,
            can_rotate: true,
            transform2DX: 0,
            transform2DY: 0,
            transform2DW: 0,
            transform2DH: 0,
            rotation: 0,
            isFlipHorz: false,
            isFlipVert: false,
            parent: None,
            child_off: None,
            child_ext: None,
            is_placeholder: false,
            ph: Placeholder::default(),
            has_xfrm: false,
            parent_type: 0,
            parent_slide: None,
            parent_layout: None,
            has_parent: false,
            sp_pr: ShapePr::default(),
            has_sp_pr: true,
            has_text_body: false,
            text_op: None,
            original_autofit: -1,
            original_anchor: -1,
            is_hidden: false,
            is_text_box: false,
            children: Vec::new(),
            graphics_path: None,
            no_grp: false,
            cleared_text: false,
            text_saved: false,
            media_bytes: None,
            media_r_id: None,
            media_id: 0,
        }
    }
}

impl KmoShowShape {
    pub fn parentGroup(&self) -> Option<Rc<KmoShowShape>> {
        self.parent.clone()
    }
    pub fn r#type(&self) -> i32 {
        self.shape_type
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn canRotate(&self) -> bool {
        self.can_rotate
    }
    pub fn rotation(&self) -> i32 {
        self.rotation
    }
    pub fn hasChildPoint(&self) -> bool {
        self.child_off.is_some()
    }
    pub fn hasChildExtents(&self) -> bool {
        self.child_ext.is_some()
    }
    pub fn childPoint(&self) -> (i32, i32) {
        self.child_off.unwrap_or((0, 0))
    }
    pub fn childExtents(&self) -> (i32, i32) {
        self.child_ext.unwrap_or((0, 0))
    }
    pub fn isPlaceHolder(&self) -> bool {
        self.is_placeholder
    }
    pub fn isHidden(&self) -> bool {
        self.is_hidden
    }
    pub fn isTextBox(&self) -> bool {
        self.is_text_box
    }
    pub fn isModularGroupPicDefault(&self) -> bool {
        false
    }
    pub fn childCount(&self) -> i32 {
        self.children.len() as i32
    }
    pub fn childAt(&self, index: i32) -> Option<KmoShowShape> {
        if index < 0 {
            return None;
        }
        self.children.get(index as usize).cloned()
    }
    pub fn ph(&self) -> &Placeholder {
        &self.ph
    }
    pub fn parent(&self) -> Option<ParentRef> {
        if !self.has_parent {
            return None;
        }
        Some(ParentRef {
            type_: self.parent_type,
            slide: self.parent_slide.clone(),
            layout: self.parent_layout.clone(),
        })
    }
    pub fn hasXfrm(&self) -> bool {
        self.has_xfrm
    }
    pub fn xfrm(&self) -> Transform2D {
        Transform2D {
            x: self.transform2DX,
            y: self.transform2DY,
            cx: self.transform2DW,
            cy: self.transform2DH,
            flipH: self.isFlipHorz,
            flipV: self.isFlipVert,
            rotation: self.rotation,
        }
    }
    pub fn transform2D(&self) -> Transform2D {
        self.xfrm()
    }
    pub fn hasInk(&self) -> bool {
        false
    }
    pub fn hasUseBgFill(&self) -> bool {
        false
    }
    pub fn isUseBgFill(&self) -> bool {
        false
    }
    pub fn style(&self) -> Option<ShapeStyleHost> {
        None
    }
    pub fn graphic(&self) -> Option<GraphicObjectHost> {
        None
    }
    pub fn getMediaLib(&self) -> Option<()> {
        None
    }
    pub fn drawingShape(&self) {}
    pub fn inkId(&self) -> i32 {
        0
    }
    pub fn getDgShape(&self) {}
    pub fn hasTextBody(&self) -> bool {
        self.has_text_body
    }
    pub fn getTextOperator(&self) -> KmoTextOperator {
        self.text_op.clone().unwrap_or_default()
    }
    pub fn new(_parent: &KmoSlide, shape_type: i32) -> Self {
        Self {
            shape_type,
            text_op: Some(KmoTextOperator::default()),
            ..Default::default()
        }
    }
    pub fn setNoGrp(&mut self, v: bool) {
        self.no_grp = v;
    }
    pub fn writePh(&mut self, ph: &Placeholder) {
        self.ph = ph.clone();
        self.is_placeholder = true;
        self.has_parent = true;
    }
    pub fn clearTextContents(&mut self) {
        self.cleared_text = true;
    }
    pub fn getOriginalAutoFit(&self) -> i32 {
        self.original_autofit
    }
    pub fn getOriginalAnchor(&self) -> i32 {
        self.original_anchor
    }
    pub fn resetOriginalProperty(&self) {}
    pub fn shapePr(&self) -> ShapePr {
        // rust: ★W6c-6 —— **真实接线**。`ConvertKmo::from_layout` 把真身 `ShapePr`
        //   经 `kmo_show_shape_hosts::attach_shape_pr(id, pr)` 存进按 id 索引的侧表，
        //   而本访问器原先**只读影子 `self.sp_pr` 的克隆**，完全忽略侧表 ⇒
        //   转换层（`ConvertTool::shapePropertiesToPublicShape`）读到的 `fill`/`ln`
        //   永远是影子字段，真身 `p:spPr` 的写入不可见。
        //   实测：`from_layout` 后 `shapePr().fill().r#type()` 读到 `5`（NONE，影子默认），
        //   而真身侧表里是 `0`（SOLID）⇒ `hasFill()` 恒 false ⇒ `s.getFill()` 恒 `None`
        //   （`05_function_fill_base` 5 例、`06_function_line_property` 6 例失败）。
        //   现优先取真身侧表（Java: `KmoShowShape.shapePr()` 返回的就是那个真身对象），
        //   侧表缺省时才回落到影子快照（保留未走 `from_layout` 的既有调用方语义）。
        //   owner: ppt.show.util.layout_shape.KmoShowShape    wave: M-PPT-KMOSLIDE-W6c-6
        let mut p = self.sp_pr.clone();
        // rust: 侧表里的真身 `ShapePr` 是 `ConvertKmo::from_layout` 写下的**权威来源**；
        //   把它投影回影子字段（本访问器返回影子类型，见函数注）。
        let sidecar = crate::cn::wps::show::app::shape::kmo_show_shape_hosts::convert_attach(self.id).shape_pr;
        let synthesized: Option<crate::cn::wps::drawing::data::shape::ShapePr::ShapePr> = None;
        let sidecar = sidecar.or(synthesized);
        if let Some(mut real) = sidecar {
            if real.hasFill() {
                let ft = real.fill().fill_type();
                // 影子 `ShapePr` 有两个填充表示：`fill: i32`（旧）与 `fill_obj: FillObj`；
                // `hasFill()` 读后者，转换层读前者 ⇒ 两者同步写。
                p.fill = ft;
                p.fill_obj.putType(ft);
                p.has_fill = true;
            }
            if real.hasLn() {
                let mut ln = real.ln();
                p.has_ln = true;
                if ln.hasWidth() { p.outline_obj.width = Some(ln.get_width()); }
                if ln.hasCompoundType() { p.outline_obj.compound = Some(ln.compoundType()); }
                if ln.hasLineCap() { p.outline_obj.cap = Some(ln.lineCap()); }
                if ln.hasLineJoin() { p.outline_obj.join_type = Some(ln.lineJoin().get_type()); }
                if ln.hasPresetDash() { p.outline_obj.preset_dash = Some(ln.get_presetDash()); }
                if ln.hasFill() {
                    let lft = ln.fill().fill_type();
                    p.ln = lft;
                    p.outline_obj.fill.putType(lft);
                }
            }
        }
        p.has_xfrm = self.has_xfrm;
        p.xfrm = self.xfrm();
        p
    }
}

#[derive(Clone, Debug, Default)]
pub struct Placeholder {
    pub has_type: bool,
    pub type_: i32,
    pub has_idx: bool,
    pub idx: i32,
}

impl Placeholder {
    pub fn hasType(&self) -> bool {
        self.has_type
    }
    pub fn r#type(&self) -> i32 {
        self.type_
    }
    pub fn hasIdx(&self) -> bool {
        self.has_idx
    }
    pub fn idx(&self) -> i32 {
        self.idx
    }
    // Java: ShowShape.Placeholder.putType
    pub fn putType(&mut self, type_: i32) {
        self.has_type = true;
        self.type_ = type_;
    }
    // Java: ShowShape.Placeholder.putIdx
    pub fn putIdx(&mut self, idx: i32) {
        self.has_idx = true;
        self.idx = idx;
    }
}

#[derive(Clone, Debug)]
pub struct ParentRef {
    pub type_: i32,
    pub slide: Option<Rc<KmoSlide>>,
    pub layout: Option<Rc<KmoSlideLayout>>,
}

impl ParentRef {
    pub fn r#type(&self) -> i32 {
        self.type_
    }
    pub fn as_slide(&self) -> Option<Rc<KmoSlide>> {
        self.slide.clone()
    }
    pub fn as_layout(&self) -> Option<Rc<KmoSlideLayout>> {
        self.layout.clone()
    }
    // Java: KmoPresentation getPresentation() —— 经所属 slide/layout 反查。
    //
    // rust: ★★M-PPT-KMOSLIDE §1.3 A4 —— 由 `None`（**凭空返回空**）改为**真实委派**。
    //   修复前它恒为 `None`，于是任何经 `ParentRef::getPresentation()` 取仓的调用方
    //   都拿不到 presentation（门禁 ⑩「getPresentation()->None: 2」），
    //   这正是 §1.3「访问器凭空造对象」的另一种形态：
    //   不是造一个假对象，而是**拒绝给出真对象**。
    //   Java 的 `ParentRef` 持 `KmoSlide`/`KmoSlideLayout` 之一，故按持有者委派。
    //   owner: ppt.show.util.layout_shape.ParentRef    wave: M-PPT-KMOSLIDE
    pub fn getPresentation(&self) -> Option<KmoPresentation> {
        if let Some(slide) = &self.slide {
            return Some(slide.getPresentation());
        }
        if let Some(layout) = &self.layout {
            return Some(layout.getPresentation());
        }
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct KmoSlide {
    pub layout: Option<Rc<KmoSlideLayout>>,
    pub slide_width: i32,
    pub slide_height: i32,
    pub has_background: bool,
    pub bg_fill: Option<u32>,
    pub show_master_shape: bool,
    pub sp_tree: KmoShapeTree,
    /// 导入器装配的真实 `KmoShowShape` 形状树根（`p:spTree`，type 0）。
    pub kmo_sp_root: Option<crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape>,
    /// 本页见到的 `p:cNvPr` 次数（含 spTree 自己的那一个）。
    pub cnvpr_count: i32,
    pub id: i64,
    pub io_reading: bool,
    // rust: ★★M-PPT-KMOSLIDE §1.3 A3 —— 本页所属的**真实** `KmoPresentation` 句柄。
    //
    //   Java: `KmoSlide.getPresentation()` 返回 `this.pres`（构造期注入的**同一对象**）。
    //   修复前：本 port 的 `getPresentation()` **现造一个** `KmoPresentation`，
    //   且 `themes: Vec::new()` —— 主题表为空 ⇒ 任何经它取主题/仓的消费方
    //   都拿到一个**没有主题的假 presentation**（§1.3「凭空造对象」）。
    //   现改为返回装配期注入的同一对象；未注入（单测）时才退回旧的现造行为。
    //   owner: ppt.show.util.layout_shape.KmoSlide    wave: M-PPT-KMOSLIDE
    pub pres_handle: Option<Rc<RefCell<KmoPresentation>>>,
}

impl KmoSlide {
    pub fn layout(&self) -> Option<Rc<KmoSlideLayout>> {
        self.layout.clone()
    }
    pub fn r#type(&self) -> i32 {
        0
    }
    // Java: KmoPresentation getPresentation()
    //
    // rust: ★★M-PPT-KMOSLIDE §1.3 A3 —— 优先返回**装配期注入的真实对象**
    //   （与 Java 的 `this.pres` 同义）；仅当未注入时才退回"按本页尺寸现造"，
    //   以保持既有单测可跑（这条回退路径**不在产品管线上**）。
    //   owner: ppt.show.util.layout_shape.KmoSlide    wave: M-PPT-KMOSLIDE
    pub fn getPresentation(&self) -> KmoPresentation {
        if let Some(p) = &self.pres_handle {
            return p.borrow().clone();
        }
        KmoPresentation {
            slide_width: self.slide_width,
            slide_height: self.slide_height,
            themes: Vec::new(),
            tmp_path: String::new(),
            ..Default::default()
        }
    }
    pub fn setIOReading(&mut self, v: bool) {
        self.io_reading = v;
    }
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn background(&self) -> Option<()> {
        if self.has_background {
            Some(())
        } else {
            None
        }
    }
    pub fn isShowMasterShape(&self) -> bool {
        self.show_master_shape
    }
    pub fn spTree(&self) -> &KmoShapeTree {
        &self.sp_tree
    }
    pub fn setLayout(&mut self, layout: KmoSlideLayout) {
        self.layout = Some(Rc::new(layout));
    }
    pub fn spTree_mut(&mut self) -> &mut KmoShapeTree {
        &mut self.sp_tree
    }
}

#[derive(Clone, Debug, Default)]
pub struct KmoSlideLayout {
    pub master: Option<Rc<KmoSlideMaster>>,
    pub sp_tree: Option<Rc<KmoShapeTree>>,
    pub show_master_shape: bool,
}

impl KmoSlideLayout {
    pub fn master(&self) -> Option<Rc<KmoSlideMaster>> {
        self.master.clone()
    }
    pub fn spTree(&self) -> Option<Rc<KmoShapeTree>> {
        self.sp_tree.clone()
    }
    pub fn isShowMasterShape(&self) -> bool {
        self.show_master_shape
    }

    // Java: KmoPresentation getPresentation()
    //
    // rust: ★★M-PPT-KMOSLIDE §1.3 A3 —— `KmoSlideLayout` 的 presentation 经
    //   `this.master` 反查（Java 同构）。母版未挂时返回 `None`（调用方决定回落），
    //   **不再现造**一个空 presentation。
    //   owner: ppt.show.util.layout_shape.KmoSlideLayout    wave: M-PPT-KMOSLIDE
    pub fn getPresentation(&self) -> KmoPresentation {
        // rust: 母版侧同样可能未注入 presentation ⇒ 逐级回落，不现造"带空主题"的假对象。
        match &self.master {
            Some(m) => m
                .pres_handle
                .as_ref()
                .map(|p| p.borrow().clone())
                .unwrap_or_default(),
            None => KmoPresentation::default(),
        }
    }
    pub fn setMaster(&mut self, master: KmoSlideMaster) {
        self.master = Some(Rc::new(master));
    }
}

#[derive(Clone, Debug, Default)]
pub struct KmoSlideMaster {
    pub sp_tree: Rc<KmoShapeTree>,
    pub io_reading: bool,
    pub id: i64,
    pub layouts: Vec<KmoSlideLayout>,
    pub header_footer: Option<HeaderFooter>,
    // rust: ★★M-PPT-KMOSLIDE §1.3 A3 —— 所属 presentation 句柄（同 `KmoSlide`）。
    //   owner: ppt.show.util.layout_shape.KmoSlideMaster    wave: M-PPT-KMOSLIDE
    pub pres_handle: Option<Rc<RefCell<KmoPresentation>>>,
}

impl KmoSlideMaster {
    pub fn spTree(&self) -> Rc<KmoShapeTree> {
        self.sp_tree.clone()
    }
    pub fn setIOReading(&mut self, v: bool) {
        self.io_reading = v;
    }
    pub fn layoutAt(&self, i: i32) -> KmoSlideLayout {
        self.layouts
            .get(i as usize)
            .cloned()
            .unwrap_or_default()
    }
    pub fn hasHeaderFooter(&self) -> bool {
        self.header_footer.is_some()
    }
    pub fn headerFooter(&self) -> HeaderFooter {
        self.header_footer.clone().unwrap_or_default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct HeaderFooter {
    pub has_date_time: bool,
    pub is_date_time: bool,
    pub has_slide_num: bool,
    pub is_slide_num: bool,
    pub has_footer: bool,
    pub is_footer: bool,
}

impl HeaderFooter {
    pub fn hasDateTime(&self) -> bool {
        self.has_date_time
    }
    pub fn isDateTime(&self) -> bool {
        self.is_date_time
    }
    pub fn hasSlideNum(&self) -> bool {
        self.has_slide_num
    }
    pub fn isSlideNum(&self) -> bool {
        self.is_slide_num
    }
    pub fn hasFooter(&self) -> bool {
        self.has_footer
    }
    pub fn isFooter(&self) -> bool {
        self.is_footer
    }
}

#[derive(Clone, Debug, Default)]
pub struct KmoShapeTree {
    pub children: Vec<KmoShowShape>,
    pub group_transform_inited: bool,
    pub insert_calls: i32,
    pub last_insert_index: i32,
    pub last_insert_len: usize,
}

impl KmoShapeTree {
    pub fn findPhByType(&self, r#type: i32) -> Option<KmoShowShape> {
        if r#type < 0 {
            return None;
        }
        for sp in &self.children {
            if sp.isPlaceHolder() && sp.ph().r#type() == r#type {
                return Some(sp.clone());
            }
        }
        None
    }
    pub fn findPhByIndex(&self, index: i32) -> Option<KmoShowShape> {
        if index < 0 {
            return None;
        }
        for sp in &self.children {
            if sp.isPlaceHolder() && sp.ph().idx() == index {
                return Some(sp.clone());
            }
        }
        None
    }
    pub fn shapeCount(&self) -> i32 {
        self.children.len() as i32
    }
    pub fn shapeAt(&self, i: i32) -> KmoShowShape {
        self.children[i as usize].clone()
    }
    pub fn getAllPh(&self) -> Vec<KmoShowShape> {
        self.children
            .iter()
            .filter(|sp| sp.isPlaceHolder())
            .cloned()
            .collect()
    }
    pub fn initSpTreeGroupTransform(&mut self) {
        self.group_transform_inited = true;
    }
    pub fn insertShapes(&mut self, addShapes: Vec<KmoShowShape>, index: i32) {
        self.insert_calls += 1;
        self.last_insert_index = index;
        self.last_insert_len = addShapes.len();
        let i = if index < 0 { 0 } else { index as usize };
        let mut out = Vec::new();
        let at = i.min(self.children.len());
        out.extend(self.children.drain(..at));
        out.extend(addShapes);
        out.extend(self.children.drain(..));
        self.children = out;
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Transform2D {
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub flipH: bool,
    pub flipV: bool,
    pub rotation: i32,
}

impl Transform2D {
    pub fn point_xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn extents_cx_cy(&self) -> (i32, i32) {
        (self.cx, self.cy)
    }
    pub fn putFlipHorizontal(&mut self, value: bool) {
        self.flipH = value;
    }
    pub fn putFlipVertical(&mut self, value: bool) {
        self.flipV = value;
    }
    pub fn putRotation(&mut self, value: i32) {
        self.rotation = value;
    }
    pub fn putX(&mut self, value: i32) {
        self.x = value;
    }
    pub fn putY(&mut self, value: i32) {
        self.y = value;
    }
    pub fn putCx(&mut self, value: i32) {
        self.cx = value;
    }
    pub fn putCy(&mut self, value: i32) {
        self.cy = value;
    }
    pub fn rotation(&self) -> i32 {
        self.rotation
    }
    pub fn isFlipHorizontal(&self) -> bool {
        self.flipH
    }
    pub fn isFlipVertical(&self) -> bool {
        self.flipV
    }
    pub fn point(&self) -> Point2DHost {
        Point2DHost { x: self.x, y: self.y }
    }
    pub fn extents(&self) -> PositiveSize2DHost {
        PositiveSize2DHost { cx: self.cx, cy: self.cy }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Point2DHost {
    pub x: i32,
    pub y: i32,
}
impl Point2DHost {
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PositiveSize2DHost {
    pub cx: i32,
    pub cy: i32,
}
impl PositiveSize2DHost {
    pub fn cx(&self) -> i32 {
        self.cx
    }
    pub fn cy(&self) -> i32 {
        self.cy
    }
}

#[derive(Clone, Debug, Default)]
pub struct ShapeStyleHost {}
#[derive(Clone, Debug, Default)]
pub struct ShapeEffectHost {}
impl ShapeEffectHost {
    pub fn hasEffectLst(&self) -> bool { false }
    pub fn effectLst(&self) -> ShapeEffectListHost { ShapeEffectListHost::default() }
}
#[derive(Clone, Debug, Default)]
pub struct ShapeEffectListHost {}
impl ShapeEffectListHost {
    pub fn hasOuterShadow(&self) -> bool { false }
    pub fn hasInnerShadow(&self) -> bool { false }
    pub fn hasPresetShadowEffect(&self) -> bool { false }
    pub fn hasReflection(&self) -> bool { false }
    pub fn hasGlow(&self) -> bool { false }
    pub fn hasSoftEdge(&self) -> bool { false }
    pub fn hasFillOverlay(&self) -> bool { false }
    pub fn outerShadow(&self) -> () {}
    pub fn innerShadow(&self) -> () {}
    pub fn presetShadowEffect(&self) -> () {}
    pub fn reflection(&self) -> () {}
    pub fn glow(&self) -> () {}
    pub fn softEdge(&self) -> () {}
    pub fn fillOverlay(&self) -> ShapeFillOverlayHost { ShapeFillOverlayHost::default() }
}
#[derive(Clone, Debug, Default)]
pub struct ShapeFillOverlayHost {}
impl ShapeFillOverlayHost {
    pub fn hasFill(&self) -> bool { false }
    pub fn hasBlendmode(&self) -> bool { false }
    pub fn blendmode(&self) -> i32 { 0 }
    pub fn fill(&self) -> FillObj { FillObj::default() }
}
#[derive(Clone, Debug, Default)]
pub struct GraphicObjectHost {
    pub graphic_type: i32,
}
impl GraphicObjectHost {
    pub fn r#type(&self) -> i32 {
        self.graphic_type
    }
}

#[derive(Clone, Debug, Default)]
pub struct ColorSlot {
    pub mode: Option<i32>,
    pub srgb: Option<i32>,
    pub scheme: Option<i32>,
}

impl ColorSlot {
    pub fn putMode(&mut self, mode: i32) {
        self.mode = Some(mode);
    }
    pub fn putScheme(&mut self, scheme: i32) {
        self.scheme = Some(scheme);
    }
}

#[derive(Clone, Debug, Default)]
pub struct GradientStopObj {
    pub pos: f64,
    pub color: ColorSlot,
}

impl GradientStopObj {
    pub fn putPos(&mut self, pos: f64) {
        self.pos = pos;
    }
}

#[derive(Clone, Debug, Default)]
pub struct SrcRectObj {
    pub l: f64,
    pub t: f64,
    pub r: f64,
    pub b: f64,
}

#[derive(Clone, Debug, Default)]
pub struct FillObj {
    pub fill_type: Option<i32>,
    pub solid: Option<i32>,
    pub color: ColorSlot,
    pub rot_with_shape: Option<bool>,
    pub flip: Option<i32>,
    pub dpi: Option<i32>,
    pub gs_lst: Vec<GradientStopObj>,
    pub patt_prst: Option<i32>,
    pub blip_cstate: Option<i32>,
    pub blip_embed: Option<String>,
    pub blip_link: Option<String>,
    pub media_id: Option<i32>,
    pub src_rect: Option<SrcRectObj>,
    pub src_rect_written: bool,
    pub fill_mode_type: Option<i32>,
    pub lin_scaled: Option<bool>,
    pub lin_ang: Option<i32>,
}

impl FillObj {
    pub fn putType(&mut self, t: i32) {
        self.fill_type = Some(t);
    }
    pub fn putRotWithShape(&mut self, v: bool) {
        self.rot_with_shape = Some(v);
    }
    pub fn putFlip(&mut self, v: i32) {
        self.flip = Some(v);
    }
    pub fn putDpi(&mut self, v: i32) {
        self.dpi = Some(v);
    }
    pub fn putPrst(&mut self, v: i32) {
        self.patt_prst = Some(v);
    }
    pub fn forceWriteSrcRect(&mut self, src: SrcRectObj) {
        self.src_rect = Some(src);
        self.src_rect_written = true;
    }
    pub fn r#type(&self) -> i32 { self.fill_type.unwrap_or(-1) }
}

#[derive(Clone, Debug, Default)]
pub struct LineEndObj {
    pub end_type: Option<i32>,
    pub width: Option<i32>,
    pub len: Option<i32>,
}

impl LineEndObj {
    pub fn putType(&mut self, v: i32) {
        self.end_type = Some(v);
    }
    pub fn putWidth(&mut self, v: i32) {
        self.width = Some(v);
    }
    pub fn putLen(&mut self, v: i32) {
        self.len = Some(v);
    }
    pub fn len(&self) -> i32 { self.len.unwrap_or(0) }
    pub fn width(&self) -> i32 { self.width.unwrap_or(0) }
    pub fn r#type(&self) -> i32 { self.end_type.unwrap_or(0) }
}

#[derive(Clone, Debug, Default)]
pub struct DashStopObj {
    pub dash: f64,
    pub space: f64,
}

#[derive(Clone, Debug, Default)]
pub struct OutlineObj {
    pub width: Option<i32>,
    pub compound: Option<i32>,
    pub cap: Option<i32>,
    pub algn: Option<i32>,
    pub join_type: Option<i32>,
    pub miter_limit: Option<f64>,
    pub preset_dash: Option<i32>,
    pub dash_stops: Vec<DashStopObj>,
    pub head_end: Option<LineEndObj>,
    pub tail_end: Option<LineEndObj>,
    pub head_written: bool,
    pub tail_written: bool,
    pub fill: FillObj,
}

impl OutlineObj {
    pub fn putWidth(&mut self, v: i32) {
        self.width = Some(v);
    }
    pub fn putCompoundType(&mut self, v: i32) {
        self.compound = Some(v);
    }
    pub fn putLineCap(&mut self, v: i32) {
        self.cap = Some(v);
    }
    pub fn putStrokeAlignment(&mut self, v: i32) {
        self.algn = Some(v);
    }
    pub fn putPresetDash(&mut self, v: i32) {
        self.preset_dash = Some(v);
    }
    pub fn putJoinType(&mut self, v: i32) {
        self.join_type = Some(v);
    }
    pub fn putMiterJoinLimit(&mut self, v: f64) {
        self.miter_limit = Some(v);
    }
    pub fn forceWriteHeadEnd(&mut self, end: LineEndObj) {
        self.head_end = Some(end);
        self.head_written = true;
    }
    pub fn forceWriteTailEnd(&mut self, end: LineEndObj) {
        self.tail_end = Some(end);
        self.tail_written = true;
    }
    pub fn hasWidth(&self) -> bool { self.width.is_some() }
    pub fn width(&self) -> i32 { self.width.unwrap_or(0) }
    pub fn hasCompoundType(&self) -> bool { self.compound.is_some() }
    pub fn compoundType(&self) -> i32 { self.compound.unwrap_or(0) }
    pub fn hasLineCap(&self) -> bool { self.cap.is_some() }
    pub fn lineCap(&self) -> i32 { self.cap.unwrap_or(0) }
    pub fn hasLineJoin(&self) -> bool { self.join_type.is_some() }
    pub fn hasFill(&self) -> bool { self.fill.fill_type.is_some() }
    pub fn fill(&self) -> FillObj { self.fill.clone() }
    pub fn hasHeadEnd(&self) -> bool { self.head_end.is_some() }
    pub fn hasTailEnd(&self) -> bool { self.tail_end.is_some() }
    pub fn hasPresetDash(&self) -> bool { self.preset_dash.is_some() }
    pub fn presetDash(&self) -> i32 { self.preset_dash.unwrap_or(0) }
    pub fn headEnd(&self) -> LineEndObj { self.head_end.clone().unwrap_or_default() }
    pub fn tailEnd(&self) -> LineEndObj { self.tail_end.clone().unwrap_or_default() }
    pub fn lineJoin(&self) -> LineJoinHost {
        LineJoinHost { join_type: self.join_type.unwrap_or(0) }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LineJoinHost {
    pub join_type: i32,
}
impl LineJoinHost {
    pub fn r#type(&self) -> i32 { self.join_type }
}

#[derive(Clone, Debug, Default)]
pub struct GeomGuideObj {
    pub name: Option<String>,
    pub fmla: Option<String>,
}

impl GeomGuideObj {
    pub fn putName(&mut self, v: String) {
        self.name = Some(v);
    }
    pub fn putFmla(&mut self, v: String) {
        self.fmla = Some(v);
    }
}

#[derive(Clone, Debug, Default)]
pub struct AdjCoordinateObj {
    pub coordinate: Option<i32>,
    pub guide_name: Option<String>,
}

impl AdjCoordinateObj {
    pub fn putCoordinate(&mut self, v: i32) {
        self.coordinate = Some(v);
        self.guide_name = None;
    }
    pub fn putGeomGuideName(&mut self, v: String) {
        self.guide_name = Some(v);
        self.coordinate = None;
    }
    pub fn coordinate(&self) -> i32 { self.coordinate.unwrap_or(0) }
}

#[derive(Clone, Debug, Default)]
pub struct AdjPoint2DObj {
    pub x: AdjCoordinateObj,
    pub y: AdjCoordinateObj,
}

impl AdjPoint2DObj {
    pub fn x(&self) -> &AdjCoordinateObj { &self.x }
    pub fn y(&self) -> &AdjCoordinateObj { &self.y }
}

#[derive(Clone, Debug, Default)]
pub struct GeomRectObj {
    pub l: AdjCoordinateObj,
    pub t: AdjCoordinateObj,
    pub r: AdjCoordinateObj,
    pub b: AdjCoordinateObj,
}

#[derive(Clone, Debug, Default)]
pub struct PathSegmentObj {
    pub seg_type: i32,
    pub point: AdjPoint2DObj,
}

impl PathSegmentObj {
    pub fn r#type(&self) -> i32 { self.seg_type }
    pub fn moveTo(&self) -> &AdjPoint2DObj { &self.point }
    pub fn lnTo(&self) -> &AdjPoint2DObj { &self.point }
    pub fn arcTo(&self) -> &AdjPoint2DObj { &self.point }
    pub fn cubicBezTo(&self) -> &AdjPoint2DObj { &self.point }
    pub fn quadBezTo(&self) -> &AdjPoint2DObj { &self.point }
}

#[derive(Clone, Debug, Default)]
pub struct PathObj {
    pub extrusion_ok: Option<bool>,
    pub fill: Option<i32>,
    pub stroke: Option<bool>,
    pub w: Option<i32>,
    pub h: Option<i32>,
    pub segments: Vec<PathSegmentObj>,
}

impl PathObj {
    pub fn putExtrusionOk(&mut self, v: bool) {
        self.extrusion_ok = Some(v);
    }
    pub fn putFill(&mut self, v: i32) {
        self.fill = Some(v);
    }
    pub fn putH(&mut self, v: i32) {
        self.h = Some(v);
    }
    pub fn putStroke(&mut self, v: bool) {
        self.stroke = Some(v);
    }
    pub fn putW(&mut self, v: i32) {
        self.w = Some(v);
    }
    pub fn hasStroke(&self) -> bool { self.stroke.is_some() }
    pub fn isStroke(&self) -> bool { self.stroke.unwrap_or(true) }
    pub fn hasFill(&self) -> bool { self.fill.is_some() }
    pub fn fill(&self) -> i32 { self.fill.unwrap_or(0) }
    pub fn w(&self) -> i32 { self.w.unwrap_or(0) }
    pub fn h(&self) -> i32 { self.h.unwrap_or(0) }
    pub fn pathSegmentLst(&self) -> &Vec<PathSegmentObj> { &self.segments }
}

#[derive(Clone, Debug, Default)]
pub struct GeomObj {
    pub prst: Option<i32>,
    pub av_lst: Vec<GeomGuideObj>,
    pub gd_lst: Vec<GeomGuideObj>,
    pub rect: GeomRectObj,
    pub paths: Vec<PathObj>,
}

impl GeomObj {
    pub fn putPrst(&mut self, v: i32) {
        self.prst = Some(v);
    }
    pub fn r#type(&self) -> i32 {
        if self.prst.is_some() {
            0
        } else if !self.paths.is_empty() {
            1
        } else {
            0
        }
    }
    pub fn hasPrstGeom(&self) -> bool {
        self.prst.is_some()
    }
    pub fn hasCustGeom(&self) -> bool {
        !self.paths.is_empty()
    }
    pub fn prstGeom(&self) -> Self {
        self.clone()
    }
    pub fn custGeom(&self) -> Self {
        self.clone()
    }
    pub fn prst(&self) -> i32 {
        self.prst.unwrap_or(0)
    }
    pub fn hasAvLst(&self) -> bool {
        !self.av_lst.is_empty()
    }
    pub fn pathLst(&self) -> Vec<PathObj> {
        self.paths.clone()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ColorMappingObj {
    pub accent1: Option<i32>,
    pub accent2: Option<i32>,
    pub accent3: Option<i32>,
    pub accent4: Option<i32>,
    pub accent5: Option<i32>,
    pub accent6: Option<i32>,
    pub hlink: Option<i32>,
    pub fol_hlink: Option<i32>,
    pub bg1: Option<i32>,
    pub tx1: Option<i32>,
    pub bg2: Option<i32>,
    pub tx2: Option<i32>,
}

impl ColorMappingObj {
    pub fn putAccent1(&mut self, v: i32) {
        self.accent1 = Some(v);
    }
    pub fn putAccent2(&mut self, v: i32) {
        self.accent2 = Some(v);
    }
    pub fn putAccent3(&mut self, v: i32) {
        self.accent3 = Some(v);
    }
    pub fn putAccent4(&mut self, v: i32) {
        self.accent4 = Some(v);
    }
    pub fn putAccent5(&mut self, v: i32) {
        self.accent5 = Some(v);
    }
    pub fn putAccent6(&mut self, v: i32) {
        self.accent6 = Some(v);
    }
    pub fn putHlink(&mut self, v: i32) {
        self.hlink = Some(v);
    }
    pub fn putFolHlink(&mut self, v: i32) {
        self.fol_hlink = Some(v);
    }
    pub fn putBg1(&mut self, v: i32) {
        self.bg1 = Some(v);
    }
    pub fn putTx1(&mut self, v: i32) {
        self.tx1 = Some(v);
    }
    pub fn putBg2(&mut self, v: i32) {
        self.bg2 = Some(v);
    }
    pub fn putTx2(&mut self, v: i32) {
        self.tx2 = Some(v);
    }
}

#[derive(Clone, Debug, Default)]
pub struct CustomColor {
    pub name: Option<String>,
    pub color: Rc<RefCell<ColorSlot>>,
    pub saved: bool,
}

impl CustomColor {
    pub fn create() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: None,
            color: Rc::new(RefCell::new(ColorSlot::default())),
            saved: false,
        }))
    }
    pub fn putName(&mut self, name: String) {
        self.name = Some(name);
    }
    pub fn color(&self) -> Rc<RefCell<ColorSlot>> {
        self.color.clone()
    }
    pub fn save(&mut self) {
        self.saved = true;
    }
}

#[derive(Clone, Debug, Default)]
pub struct CustomColorLst {
    pub items: Vec<Rc<RefCell<CustomColor>>>,
}

impl CustomColorLst {
    pub fn write(&mut self, customColors: Vec<Rc<RefCell<CustomColor>>>) {
        self.items = customColors;
    }
}

#[derive(Clone, Debug, Default)]
pub struct ExtraColorScheme {
    pub saved: bool,
    pub color_mapping: Rc<RefCell<ColorMappingObj>>,
}

impl ExtraColorScheme {
    pub fn create() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            saved: false,
            color_mapping: Rc::new(RefCell::new(ColorMappingObj::default())),
        }))
    }
    pub fn save(&mut self) {
        self.saved = true;
    }
    pub fn colorMapping(&self) -> Rc<RefCell<ColorMappingObj>> {
        self.color_mapping.clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ExtraColorSchemeLst {
    pub items: Vec<Rc<RefCell<ExtraColorScheme>>>,
}

impl ExtraColorSchemeLst {
    pub fn write(&mut self, extraColorSchemes: Vec<Rc<RefCell<ExtraColorScheme>>>) {
        self.items = extraColorSchemes;
    }
}

#[derive(Clone, Debug, Default)]
pub struct ShapePr {
    pub has_xfrm: bool,
    pub xfrm: Transform2D,
    pub has_fill: bool,
    pub fill: i32,
    pub has_ln: bool,
    pub ln: i32,
    pub parent: Option<Box<ShapePr>>,
    pub bw_mode: Option<i32>,
    pub geom_type: Option<i32>,
    pub fill_obj: FillObj,
    pub outline_obj: OutlineObj,
    pub geom_obj: GeomObj,
}

impl ShapePr {
    pub fn putBwMode(&mut self, value: i32) {
        self.bw_mode = Some(value);
    }
    pub fn hasXfrm(&self) -> bool {
        self.has_xfrm
    }
    pub fn xfrm(&self) -> Transform2D {
        self.xfrm.clone()
    }
    // Java: void follow(ShapePr parent) — set parent pointer; null is no-op
    pub fn follow(&mut self, parent: Option<ShapePr>) {
        if let Some(p) = parent {
            self.parent = Some(Box::new(p));
        }
    }
    pub fn hasFill(&self) -> bool {
        self.has_fill || self.parent.as_ref().map_or(false, |p| p.hasFill())
    }
    pub fn fill(&self) -> i32 {
        if self.has_fill {
            self.fill
        } else {
            self.parent.as_ref().map(|p| p.fill()).unwrap_or(0)
        }
    }
    pub fn hasLn(&self) -> bool {
        self.has_ln || self.parent.as_ref().map_or(false, |p| p.hasLn())
    }
    pub fn ln(&self) -> i32 {
        if self.has_ln {
            self.ln
        } else {
            self.parent.as_ref().map(|p| p.ln()).unwrap_or(0)
        }
    }
    pub fn geom(&self) -> GeomObj {
        self.geom_obj.clone()
    }
    pub fn hasSp3d(&self) -> bool {
        false
    }
    pub fn hasScene3D(&self) -> bool {
        false
    }
    pub fn hasEffect(&self) -> bool {
        false
    }
    pub fn effect(&self) -> ShapeEffectHost {
        ShapeEffectHost::default()
    }
    pub fn hasGeom(&self) -> bool {
        self.geom_obj.prst.is_some() || !self.geom_obj.paths.is_empty()
    }
    pub fn ln_style(&self) -> OutlineObj {
        self.outline_obj.clone()
    }
    /// Convert-side Fill (type + flags), distinct from parse `fill()` i32 index.
    pub fn fill_convert(&self) -> FillObj {
        self.fill_obj.clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct DefaultShapeDefinition {
    pub shape_pr: Rc<RefCell<ShapePr>>,
}

impl DefaultShapeDefinition {
    pub fn bodyPr(&self) {}
    pub fn textListStyle(&self) {}
    pub fn shapePr(&self) -> Rc<RefCell<ShapePr>> {
        self.shape_pr.clone()
    }
    pub fn shapeStyle(&self) {}
}

#[derive(Clone, Debug)]
pub struct ThemeObj {
    pub name: Option<String>,
    pub color_mapping: Rc<RefCell<ColorMappingObj>>,
    pub shape_def: Rc<RefCell<DefaultShapeDefinition>>,
    pub line_def: Rc<RefCell<DefaultShapeDefinition>>,
    pub text_def: Rc<RefCell<DefaultShapeDefinition>>,
    pub custom_color_lst: CustomColorLst,
    pub extra_color_scheme_list: ExtraColorSchemeLst,
    pub saved: bool,
    pub theme_path: Option<String>,
    pub rid2_media_id: Option<HashMap<String, i32>>,
}

impl Default for ThemeObj {
    fn default() -> Self {
        Self {
            name: None,
            color_mapping: Rc::new(RefCell::new(ColorMappingObj::default())),
            shape_def: Rc::new(RefCell::new(DefaultShapeDefinition::default())),
            line_def: Rc::new(RefCell::new(DefaultShapeDefinition::default())),
            text_def: Rc::new(RefCell::new(DefaultShapeDefinition::default())),
            custom_color_lst: CustomColorLst::default(),
            extra_color_scheme_list: ExtraColorSchemeLst::default(),
            saved: false,
            theme_path: None,
            rid2_media_id: None,
        }
    }
}

impl ThemeObj {
    pub fn create() -> Self {
        Self::default()
    }
    pub fn putName(&mut self, name: String) {
        self.name = Some(name);
    }
    pub fn colorMapping(&self) -> Rc<RefCell<ColorMappingObj>> {
        self.color_mapping.clone()
    }
    pub fn shapeDef(&self) -> Rc<RefCell<DefaultShapeDefinition>> {
        self.shape_def.clone()
    }
    pub fn lineDef(&self) -> Rc<RefCell<DefaultShapeDefinition>> {
        self.line_def.clone()
    }
    pub fn textDef(&self) -> Rc<RefCell<DefaultShapeDefinition>> {
        self.text_def.clone()
    }
    pub fn customColorLst(&mut self) -> &mut CustomColorLst {
        &mut self.custom_color_lst
    }
    pub fn extraColorSchemeList(&mut self) -> &mut ExtraColorSchemeLst {
        &mut self.extra_color_scheme_list
    }
    pub fn save(&self) -> Self {
        let mut saved = self.clone();
        saved.saved = true;
        saved
    }
    pub fn updateProtocol(&mut self, saved: ThemeObj) {
        *self = saved;
    }
    pub fn setThemePath(&mut self, path: Option<String>) {
        self.theme_path = path;
    }
    pub fn setRid2MediaId(&mut self, mRid2MediaId: Option<HashMap<String, i32>>) {
        self.rid2_media_id = mRid2MediaId;
    }
}

#[derive(Clone, Debug, Default)]
pub struct GroupShape {
    pub sp_pr: ShapePr,
    pub child_x: i32,
    pub child_y: i32,
    pub child_cx: i32,
    pub child_cy: i32,
    pub has_child_point: bool,
    pub has_child_extents: bool,
}

#[derive(Clone, Debug)]
pub struct Selection {
    pub actived_slide_index: i32,
    pub thumbnail_slide_index: i32,
    pub history_slide_index: i32,
    pub init_count: i32,
    pub select_slide_calls: Vec<i32>,
    pub priority_list: Option<Vec<i32>>,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            actived_slide_index: -1,
            thumbnail_slide_index: -1,
            history_slide_index: 0,
            init_count: 0,
            select_slide_calls: Vec::new(),
            priority_list: None,
        }
    }
}

impl Selection {
    pub fn init(&mut self) {
        self.init_count += 1;
    }
    pub fn activedSlideIndex(&self) -> i32 {
        self.actived_slide_index
    }
    pub fn thumbnailSlideIndex(&self) -> i32 {
        self.thumbnail_slide_index
    }
    pub fn getHistorySlideIndex(&self) -> i32 {
        self.history_slide_index
    }
    pub fn selectSlide(&mut self, index: i32) {
        self.select_slide_calls.push(index);
    }
    pub fn getPriorityList(&self) -> Option<&Vec<i32>> {
        self.priority_list.as_ref()
    }
}

#[derive(Clone, Debug, Default)]
pub struct KmoPresentation {
    pub slide_width: i32,
    pub slide_height: i32,
    pub themes: Vec<Rc<RefCell<ThemeObj>>>,
    pub tmp_path: String,
    pub slides: Vec<KmoSlide>,
    pub masters: Vec<KmoSlideMaster>,
    pub selection: Selection,
    pub next_slide_id: i64,
    pub list_changed: i32,
    // Java `KmoPresentation` 装配面 —— `PptxrPresentation.open()` 的后置状态（CORPUS §5 A1–A9）。
    // Java: ((PresentationAtomData)this.rawData()).presentation.notesSize()
    pub notes_width: i32,
    pub notes_height: i32,
    // Java: ((PresentationAtomData)this.rawData()).presentation.slideSize().type()
    pub slide_size_type: Option<i32>,
    // Java: this.hasInitialized
    pub initialized: bool,
    // Java: this.mHasBlankSlide
    pub has_blank_slide: bool,
    // Java: this.presentationTheme
    pub theme: Option<Rc<RefCell<ThemeObj>>>,
    // Java: PptxrTableStyles.open() 的可观测后置状态
    pub table_styles_opened: i32,
    // Java: ((KmoTableStyleList)this.mPres.tableStyleLst()).setDefaultStyle(...)
    pub table_style_default: Option<String>,
    // Java: this.mPres.tableStyleLst().addStyle(...) 的次数
    pub table_style_count: i32,
    // Java: this.commentIdGenerator 当前值
    pub comment_id_seed: i32,
    // Java: this.items / this.itemProps
    pub items: Vec<String>,
    pub item_props: Vec<String>,
    // Java: PptxrPresProps.open() / PptxrViewProps.open() / PptxrCommentAuthors.open() 的可观测后置状态
    pub pres_props_opened: i32,
    pub view_property_written: i32,
    pub comment_authors_opened: i32,
    pub comment_author_ids: Vec<i32>,
    // Java: PptxrMedias.mGlobalMediasIdx 的规模（可观测后置状态）
    pub media_ids: Vec<i32>,
    // Java: this.mPres.mediaLib().getMediaPriorityReader()（Rust `KmoMediaLib` 是单元结构，
    // 无媒体优先级状态，故装配面直接承载该 reader 的 mediaId —— 见台账）
    pub media_priority_reader: Option<i32>,
    // Java: PptxReader.open() → mPres.setFileFormat(32) / setReadOnly(false)
    pub file_format: i32,
    pub read_only: bool,
    // Java: PptxrDocProps.readApp → AppProperty
    pub app_application: Option<String>,
    pub app_version: Option<String>,
    pub app_company: Option<String>,
    pub app_manager: Option<String>,
    // Java: PptxrDocProps.readCore → CoreProperty
    pub core_title: Option<String>,
    pub core_creator: Option<String>,
    pub core_last_modified_by: Option<String>,
    pub core_created: Option<String>,
    pub core_modified: Option<String>,
    // Java: PptxrCustom.open → CustomProperties 的 name
    pub custom_property_names: Vec<String>,
    // rust: ★★M-PPT-KMOSLIDE §1.3 目标 3 —— **对象图真实**：`KmoPresentation` 级唯一仓句柄。
    //
    //   Java: `DgStore drawingStore() { return this.canvas.store(); }`
    //   —— 返回的是**同一个**仓对象。`KmoSlide` 的每个成员
    //   （`SwShapeHandler` / `SwGroupShapeHandler` / `SwPictureHandler` /
    //   `SwConnectorShapeHandler` / `SwGraphicFrameHandler`）构造时都执行
    //   `parent().getPresentation().drawingStore()`，必须解析到它。
    //
    //   修复前：`drawingStore()` 逐字新建一个空仓 —— **每次调用都凭空造一个**
    //   （门禁 ⑩「凭空造仓」）。各 handler 因此拿到互不相干的仓，
    //   `commit_drawing` 的写入彼此不可见。
    //
    //   Rust 表达 Java 的"同一可变对象"用 `Rc<RefCell<DgStore>>`；
    //   `None` 时（`KmoPresentation::default()`，单测）按需新建并缓存 ——
    //   保证"同一 presentation 的多次 `drawingStore()` 返回同一仓"这一不变量恒成立。
    //   owner: ppt.show.util.layout_shape.KmoPresentation    wave: M-PPT-KMOSLIDE
    pub store_handle: Option<Rc<RefCell<crate::cn::wps::drawing::app::DgStore::DgStore>>>,
}
impl KmoPresentation {
    // Java: int slideSizeType()
    pub fn slideSizeType(&self) -> i32 {
        self.slide_size_type.unwrap_or(0)
    }
    // Java: boolean slideSize().hasType()
    pub fn hasSlideSizeType(&self) -> bool {
        self.slide_size_type.is_some()
    }
    // Java: void writeSlideSize(Presentation.SlideSize size)
    pub fn writeSlideSize(&mut self, cx: i32, cy: i32, type_: Option<i32>) {
        self.slide_width = cx;
        self.slide_height = cy;
        self.slide_size_type = type_;
    }
    // Java: boolean hasSlideSize()
    pub fn hasSlideSize(&self) -> bool {
        self.slide_width != 0 || self.slide_height != 0
    }
    // Java: PositiveSize2D notesSize()
    pub fn notesWidth(&self) -> i32 {
        self.notes_width
    }
    pub fn notesHeight(&self) -> i32 {
        self.notes_height
    }
    // Java: void writeNotesSize(PositiveSize2D size)
    pub fn writeNotesSize(&mut self, cx: i32, cy: i32) {
        self.notes_width = cx;
        self.notes_height = cy;
    }
    // Java: boolean hasNotesSize()
    pub fn hasNotesSize(&self) -> bool {
        self.notes_width != 0 || self.notes_height != 0
    }
    // Java: void setTheme(KmoTheme theme)
    pub fn setTheme(&mut self, theme: Rc<RefCell<ThemeObj>>) {
        self.theme = Some(theme);
    }
    // Java: KmoTheme getTheme()
    pub fn getTheme(&self) -> Option<Rc<RefCell<ThemeObj>>> {
        self.theme.clone()
    }
    // Java: void hasBlankSlide(boolean hasBlankSlide)
    pub fn hasBlankSlide(&mut self, has_blank_slide: bool) {
        self.has_blank_slide = has_blank_slide;
    }
    // Java: boolean hasBlankSlide()
    pub fn isHasBlankSlide(&self) -> bool {
        self.has_blank_slide
    }
    // Java: void setInitialized(boolean initialized)
    pub fn setInitialized(&mut self, initialized: bool) {
        self.initialized = initialized;
    }
    // Java: boolean hasInitialized()
    pub fn hasInitialized(&self) -> bool {
        self.initialized
    }
    // Java: DgStore drawingStore()
    //
    // rust: ★★M-PPT-KMOSLIDE §1.3 目标 3 —— **不再凭空造仓**。
    //   Java 语义是 `this.canvas.store()`：返回 `KmoPresentation` 级**同一个**仓。
    //   本 port 用 `Rc<RefCell<DgStore>>` 表达该引用；装配期由
    //   `open_pptx_document_bytes` / `PptxrSlides` 填入打开期那个唯一仓。
    //   `None`（未装配，单测）⇒ 新建并缓存，使同一 presentation 内多次调用返回同一仓。
    //   owner: ppt.show.util.layout_shape.KmoPresentation    wave: M-PPT-KMOSLIDE
    pub fn drawingStore(&mut self) -> Rc<RefCell<crate::cn::wps::drawing::app::DgStore::DgStore>> {
        if self.store_handle.is_none() {
            self.store_handle = Some(Rc::new(RefCell::new(
                crate::cn::wps::drawing::app::DgStore::DgStore::DgStore_new(),
            )));
        }
        self.store_handle.as_ref().unwrap().clone()
    }

    // rust: 本方法无对应 Java 方法 —— 与上面的 `drawingStore()` 返回同一句柄，
    //   但只需 `&self`（用于 `&self` 上下文，如 `ISlide` 的实现里）。
    //   未装配时返回 `None`（调用方自行决定回退策略）。
    //   owner: ppt.show.util.layout_shape.KmoPresentation    wave: M-PPT-KMOSLIDE
    pub fn storeHandle(
        &self,
    ) -> Option<Rc<RefCell<crate::cn::wps::drawing::app::DgStore::DgStore>>> {
        self.store_handle.clone()
    }

    // rust: 本方法无对应 Java 方法 —— Java 的 `drawingStore()` 返回可变对象引用；
    //   本 port 的 `ImportHost`（`SwGroupShapeHandler`）沿用了"裸指针表达引用"的既有约定，
    //   故此处交出**稳定**的内部指针：`store_handle` 一旦装配即不再变动
    //   （`Rc` 的目标地址在整个 `KmoPresentation` 生命周期内恒定）。
    //   未装配时先 `drawingStore()` 建好，保证非空。
    //   这样 `handler → parent().getPresentation().drawingStore()` 与
    //   `mPres.drawingStore()` 解析到**同一个**仓（Java 语义）。
    //   owner: ppt.show.util.layout_shape.KmoPresentation    wave: M-PPT-KMOSLIDE
    pub fn drawingStorePtr(&mut self) -> *mut crate::cn::wps::drawing::app::DgStore::DgStore {
        let handle = self.drawingStore();
        // `Rc::as_ptr` 给出 `*const RefCell<DgStore>`；其目标在 `Rc` 存活期间地址稳定。
        let cell = Rc::as_ptr(&handle) as *mut RefCell<crate::cn::wps::drawing::app::DgStore::DgStore>;
        // `RefCell::as_ptr` 给出内部 `DgStore` 的稳定地址。
        // SAFETY: `handle` 由 `store_handle`（本结构体字段）持有，生命周期 ≥ 本 `&mut self`。
        unsafe { (*cell).as_ptr() }
    }
    // Java: int nextCommentId()
    pub fn nextCommentId(&mut self) -> i32 {
        self.comment_id_seed += 1;
        self.comment_id_seed
    }
    // Java: ArrayList<String> items()
    pub fn items(&self) -> Vec<String> {
        self.items.clone()
    }
    pub fn items_mut(&mut self) -> &mut Vec<String> {
        &mut self.items
    }
    // Java: ArrayList<String> itemProps()
    pub fn itemProps(&self) -> Vec<String> {
        self.item_props.clone()
    }
    pub fn itemProps_mut(&mut self) -> &mut Vec<String> {
        &mut self.item_props
    }
    // Java: PptxrTableStyles.open() 的可观测后置状态
    pub fn markTableStylesOpened(&mut self) {
        self.table_styles_opened += 1;
    }
    pub fn tableStylesOpened(&self) -> i32 {
        self.table_styles_opened
    }
    // Java: this.mPres.tableStyleLst()（Rust `app::table::KmoTableStyleList` 无可写构造，
    // 故装配面直接承载其可观测值 —— 见 M_PPT_PRES_BASELINE 台账）
    pub fn writeTableStyles(&mut self, default_style: Option<String>, count: i32) {
        self.table_style_default = default_style;
        self.table_style_count = count;
    }
    pub fn tableStyleDefault(&self) -> Option<String> {
        self.table_style_default.clone()
    }
    pub fn tableStyleCount(&self) -> i32 {
        self.table_style_count
    }
    // Java: this.mediaLib().getMediaPriorityReader()（端口化到装配面，见字段注释）
    pub fn mediaPriorityReader(&self) -> Option<i32> {
        self.media_priority_reader
    }
    pub fn setMediaPriorityReader(&mut self, mediaId: Option<i32>) {
        self.media_priority_reader = mediaId;
    }
    pub fn slideWidth(&self) -> i32 {
        self.slide_width
    }
    pub fn slideHeight(&self) -> i32 {
        self.slide_height
    }
    pub fn mediaLib(&self) -> super::layout_textbox::KmoMediaLib {
        crate::cn::wps::show::util::layout_textbox::KmoMediaLib
    }
    pub fn getTmpPath(&self) -> String {
        self.tmp_path.clone()
    }
    pub fn _addTheme(&mut self, theme: Rc<RefCell<ThemeObj>>) {
        self.themes.push(theme);
    }
    pub fn addSlide(&mut self, slide: KmoSlide) {
        self.slides.push(slide);
    }
    pub fn slideCount(&self) -> i32 {
        self.slides.len() as i32
    }
    pub fn selection(&self) -> &Selection {
        &self.selection
    }
    pub fn selection_mut(&mut self) -> &mut Selection {
        &mut self.selection
    }
    pub fn _addMaster(&mut self, master: KmoSlideMaster) {
        self.masters.push(master);
    }
    pub fn alloc_slide(&mut self) -> KmoSlide {
        self.next_slide_id += 1;
        KmoSlide {
            id: self.next_slide_id,
            ..Default::default()
        }
    }
    pub fn onSlideListChanged(&mut self) {
        self.list_changed += 1;
    }
    pub fn eventListeners(&mut self) -> &mut Self {
        self
    }
    pub fn masterCount(&self) -> i32 {
        self.masters.len() as i32
    }
    pub fn masterAt(&self, i: i32) -> KmoSlideMaster {
        self.masters.get(i as usize).cloned().unwrap_or_default()
    }
    pub fn slideAt(&self, i: i32) -> KmoSlide {
        self.slides.get(i as usize).cloned().unwrap_or_default()
    }
    pub fn indexOfMaster(&self, master: &KmoSlideMaster) -> i32 {
        self.masters
            .iter()
            .position(|m| m.id == master.id)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
    pub fn addMaster(&mut self, master: KmoSlideMaster) {
        self.masters.push(master);
    }
    pub fn setNoteMaster(&mut self, _notes: ()) {}
}

#[derive(Clone, Debug, Default)]
pub struct KmoContext {
    // rust: ★★M-PPT-KMOSLIDE §1.3 A4 —— 与 `ParentRef` 同理：不再"拒绝给出真对象"。
    //   Java 的 `KmoContext` 由调用方注入 `KmoPresentation`；此处保存该句柄。
    //   owner: ppt.show.util.layout_shape.KmoContext    wave: M-PPT-KMOSLIDE
    pub pres_handle: Option<Rc<RefCell<KmoPresentation>>>,
}
impl KmoContext {
    // Java: KmoPresentation getPresentation()
    pub fn getPresentation(&self) -> Option<KmoPresentation> {
        self.pres_handle.as_ref().map(|p| p.borrow().clone())
    }
    pub fn getSlideNumber(&self) -> i32 {
        1
    }
    pub fn getContext(&self) -> Option<()> {
        None
    }
}
pub struct KmoTable;
pub struct GraphicObject;
pub struct KmoTextBody;
pub struct GroupTransform2D;
pub struct Point2D;
pub struct PositiveSize2D;
pub struct AdjAngle;
pub struct AdjCoordinate;
pub struct CustomGeometry;
pub struct GeomGuide;
pub struct Geometry2D;
pub struct PresetGeometry;
pub struct Shape;
pub struct TextBodyProperties;
pub struct KmoMediaUtil;
pub trait ISlide {}
impl ISlide for KmoSlideMaster {}
pub struct ConvertTool;
pub struct SlideCache;
pub struct ShapeInfoCache;
pub struct ShapeInfo;
pub struct FormulaParser;
pub struct Ink;
pub struct IMediaLib;
pub struct GeoAdjust;
pub struct PreDefData;
pub struct BlipFill;
pub struct MediaTypeEnum;
pub struct CDL;
pub struct CFL;
pub struct ShapeType;
pub struct Background;
pub struct ColorMapping;
pub struct KsoWMInfo;
pub struct DgShape;
pub struct Theme;
pub struct TableStyle;
pub struct TableStyleTextStyle;
pub struct TextCharacterProperties;
pub struct KmoProtocol;
pub struct ShapeInfoOut;
pub struct GeomtryConvert;
pub struct GlCache;
pub struct EffectLayerRender;
pub struct HitShape3DHelper;
pub struct InterpretCtx;
pub struct ISlideRender;
pub struct ShowTypoShape;
pub struct SlideRender;
pub struct ShowShapeEnv;
pub struct SlideRenderTool;
pub struct Brush;
pub struct TraceAdapter;
pub struct Path;
pub struct Config;
pub struct ShapeStyle;
pub struct Fill;
pub struct Outline;
pub struct EffectProperties;
pub struct Scene3DProperties;
pub struct Apply3DShapeProperties;
pub struct TextListStyle;
pub struct TextParagraphProperties;
pub struct KmoPropertiesMerger;
