#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! Compile-surface hosts for ShapePr sibling data types (pp-19 function 05).
//! Real Geometry2D/Outline/Transform2D/Fill templates stay off-surface until those
//! leaves are ungated. Not under libs/core-libs/stubs.

use crate::cn::wps::kso::kproto::KoBin::{Encode, KoBin, Null};
use crate::cn::wps::kso::kproto::KoZone::KoZone;

fn empty_bin(_zone: &KoZone) -> Box<dyn KoBin> {
    Null::value1()
}

#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
pub struct Transform2D {
    parent: bool,
    dirty: bool,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    rotation: i32,
    flip_h: bool,
    flip_v: bool,
}

impl Transform2D {
    pub fn Transform2D_from(_bin: &dyn KoBin) -> Self {
        Self::default()
    }
    pub fn hasParent(&self) -> bool {
        self.parent
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn follow(&mut self, _parent: &Transform2D) {}
    pub fn unFollow(&mut self) {}
    pub fn hashCode(&self) -> i32 {
        0
    }
    pub fn putX(&mut self, value: i32) {
        self.x = value;
        self.dirty = true;
    }
    pub fn putY(&mut self, value: i32) {
        self.y = value;
        self.dirty = true;
    }
    pub fn putCx(&mut self, value: i32) {
        self.cx = value;
        self.dirty = true;
    }
    pub fn putCy(&mut self, value: i32) {
        self.cy = value;
        self.dirty = true;
    }
    pub fn getX(&self) -> i32 {
        self.x
    }
    pub fn getY(&self) -> i32 {
        self.y
    }
    pub fn getCx(&self) -> i32 {
        self.cx
    }
    pub fn getCy(&self) -> i32 {
        self.cy
    }
    pub fn putRotation(&mut self, value: i32) {
        self.rotation = value;
        self.dirty = true;
    }
    pub fn putFlipH(&mut self, value: bool) {
        self.flip_h = value;
        self.dirty = true;
    }
    pub fn putFlipV(&mut self, value: bool) {
        self.flip_v = value;
        self.dirty = true;
    }
    pub fn rotation(&self) -> i32 {
        self.rotation
    }
    pub fn isFlipHorizontal(&self) -> bool {
        self.flip_h
    }
    pub fn isFlipVertical(&self) -> bool {
        self.flip_v
    }
    pub fn point(&self) -> Point2DHost {
        Point2DHost {
            x: self.x,
            y: self.y,
        }
    }
    pub fn extents(&self) -> PositiveSize2DHost {
        PositiveSize2DHost {
            cx: self.cx,
            cy: self.cy,
        }
    }
}

#[derive(Default, Clone)]
pub struct Geometry2D {
    parent: bool,
    dirty: bool,
    type_: i32,
    prst: Option<i32>,
}

impl Geometry2D {
    pub fn Geometry2D_from(_bin: &dyn KoBin) -> Self {
        Self::default()
    }
    pub fn hasParent(&self) -> bool {
        self.parent
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn follow(&mut self, _parent: &Geometry2D) {}
    pub fn unFollow(&mut self) {}
    pub fn hashCode(&self) -> i32 {
        0
    }
    pub fn putType(&mut self, value: i32) {
        self.type_ = value;
        self.dirty = true;
    }
    pub fn get_type(&self) -> i32 {
        self.type_
    }
    pub fn r#type(&self) -> i32 {
        self.type_
    }
    pub fn putPrst(&mut self, value: i32) {
        self.prst = Some(value);
        self.type_ = 0;
        self.dirty = true;
    }
    pub fn hasPrstGeom(&self) -> bool {
        self.prst.is_some()
    }
    pub fn hasCustGeom(&self) -> bool {
        self.type_ == 1
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
}

#[derive(Default, Clone)]
pub struct Fill {
    parent: bool,
    dirty: bool,
    type_: i32,
}

impl Fill {
    pub fn Fill_from(_bin: &dyn KoBin) -> Self {
        Self::default()
    }
    pub fn hasParent(&self) -> bool {
        self.parent
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn follow(&mut self, _parent: &Fill) {}
    pub fn unFollow(&mut self) {}
    pub fn hashCode(&self) -> i32 {
        0
    }
    pub fn putType(&mut self, value: i32) {
        self.type_ = value;
        self.dirty = true;
    }
    pub fn r#type(&self) -> i32 {
        self.type_
    }
}

#[derive(Default, Clone)]
pub struct LineJoinHost {
    pub join_type: i32,
}
impl LineJoinHost {
    pub fn r#type(&self) -> i32 {
        self.join_type
    }
}

#[derive(Default, Clone)]
pub struct LineEndHost {
    pub r#type: i32,
}
impl LineEndHost {
    pub fn r#type(&self) -> i32 {
        self.r#type
    }
    pub fn len(&self) -> i32 {
        0
    }
    pub fn width(&self) -> i32 {
        0
    }
}

#[derive(Default, Clone)]
pub struct Outline {
    parent: bool,
    dirty: bool,
    width: i32,
    has_width: bool,
    compound: Option<i32>,
    cap: Option<i32>,
    join_type: Option<i32>,
    preset_dash: Option<i32>,
    fill: Fill,
    has_fill: bool,
    head_end: Option<LineEndHost>,
    tail_end: Option<LineEndHost>,
}

impl Outline {
    pub fn Outline_from(_bin: &dyn KoBin) -> Self {
        Self::default()
    }
    pub fn hasParent(&self) -> bool {
        self.parent
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn follow(&mut self, _parent: &Outline) {}
    pub fn unFollow(&mut self) {}
    pub fn hashCode(&self) -> i32 {
        0
    }
    pub fn putWidth(&mut self, value: i32) {
        self.width = value;
        self.has_width = true;
        self.dirty = true;
    }
    pub fn get_width(&self) -> i32 {
        self.width
    }
    pub fn hasWidth(&self) -> bool {
        self.has_width
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn putCompoundType(&mut self, value: i32) {
        self.compound = Some(value);
        self.dirty = true;
    }
    pub fn hasCompoundType(&self) -> bool {
        self.compound.is_some()
    }
    pub fn compoundType(&self) -> i32 {
        self.compound.unwrap_or(0)
    }
    pub fn putLineCap(&mut self, value: i32) {
        self.cap = Some(value);
        self.dirty = true;
    }
    pub fn hasLineCap(&self) -> bool {
        self.cap.is_some()
    }
    pub fn lineCap(&self) -> i32 {
        self.cap.unwrap_or(0)
    }
    pub fn putJoinType(&mut self, value: i32) {
        self.join_type = Some(value);
        self.dirty = true;
    }
    pub fn hasLineJoin(&self) -> bool {
        self.join_type.is_some()
    }
    pub fn lineJoin(&self) -> LineJoinHost {
        LineJoinHost {
            join_type: self.join_type.unwrap_or(0),
        }
    }
    pub fn putPresetDash(&mut self, value: i32) {
        self.preset_dash = Some(value);
        self.dirty = true;
    }
    pub fn hasPresetDash(&self) -> bool {
        self.preset_dash.is_some()
    }
    pub fn presetDash(&self) -> i32 {
        self.preset_dash.unwrap_or(0)
    }
    pub fn putFillType(&mut self, value: i32) {
        self.fill.putType(value);
        self.has_fill = true;
        self.dirty = true;
    }
    pub fn hasFill(&self) -> bool {
        self.has_fill
    }
    pub fn fill(&self) -> Fill {
        self.fill.clone()
    }
    pub fn hasHeadEnd(&self) -> bool {
        self.head_end.is_some()
    }
    pub fn hasTailEnd(&self) -> bool {
        self.tail_end.is_some()
    }
    pub fn headEnd(&self) -> LineEndHost {
        self.head_end.clone().unwrap_or_default()
    }
    pub fn tailEnd(&self) -> LineEndHost {
        self.tail_end.clone().unwrap_or_default()
    }
}

#[derive(Default, Clone)]
pub struct EffectProperties {
    parent: bool,
    dirty: bool,
}

impl EffectProperties {
    pub fn EffectProperties_from(_bin: &dyn KoBin) -> Self {
        Self::default()
    }
    pub fn hasParent(&self) -> bool {
        self.parent
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn follow(&mut self, _parent: &EffectProperties) {}
    pub fn unFollow(&mut self) {}
    pub fn hashCode(&self) -> i32 {
        0
    }
    pub fn hasEffectLst(&self) -> bool {
        false
    }
    pub fn effectLst(&self) -> EffectLstHost {
        EffectLstHost::default()
    }
}

#[derive(Default, Clone)]
pub struct EffectLstHost;
impl EffectLstHost {
    pub fn hasFillOverlay(&self) -> bool {
        false
    }
    pub fn fillOverlay(&self) -> FillOverlayHost {
        FillOverlayHost::default()
    }
}

#[derive(Default, Clone)]
pub struct FillOverlayHost;
impl FillOverlayHost {
    pub fn hasBlendmode(&self) -> bool {
        false
    }
    pub fn blendmode(&self) -> i32 {
        0
    }
    pub fn hasFill(&self) -> bool {
        false
    }
    pub fn fill(&self) -> Fill {
        Fill::default()
    }
}

#[derive(Default, Clone)]
pub struct Scene3DProperties {
    parent: bool,
    dirty: bool,
}

impl Scene3DProperties {
    pub fn Scene3DProperties_from(_bin: &dyn KoBin) -> Self {
        Self::default()
    }
    pub fn create() -> Self {
        Self::default()
    }
    pub fn hasParent(&self) -> bool {
        self.parent
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn follow(&mut self, _parent: &Scene3DProperties) {}
    pub fn unFollow(&mut self) {}
    pub fn hashCode(&self) -> i32 {
        0
    }
}

#[derive(Default)]
pub struct Apply3DShapeProperties {
    parent: bool,
    dirty: bool,
}

impl Apply3DShapeProperties {
    pub fn Apply3DShapeProperties_from(_bin: &dyn KoBin) -> Self {
        Self::default()
    }
    pub fn create() -> Self {
        Self::default()
    }
    pub fn hasParent(&self) -> bool {
        self.parent
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBin> {
        empty_bin(zone)
    }
    pub fn follow(&mut self, _parent: &Apply3DShapeProperties) {}
    pub fn unFollow(&mut self) {}
    pub fn hashCode(&self) -> i32 {
        0
    }
}

// silence unused Encode in this host file until ShapePr put* paths encode through KpMap
#[allow(dead_code)]
fn _encode_int(v: i32, zone: &KoZone) -> Box<dyn KoBin> {
    Encode::from_int(v, zone)
}
