#![allow(dead_code, non_snake_case)]

use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
use drawing::cn::wps::moffice::drawing::effects::Glow::Glow;
use drawing::cn::wps::moffice::drawing::effects::Reflection::Reflection;
use drawing::cn::wps::moffice::drawing::effects::Shadow::Shadow;
use drawing::cn::wps::moffice::drawing::line::Arrow::Arrow;
use drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
use drawing::cn::wps::moffice::drawing::textbox::Text::Text;

use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;

type Element = Option<Arc<dyn std::any::Any + Send + Sync>>;

#[derive(Clone, Copy)]
struct FRect {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl FRect {
    fn width(self) -> f32 { self.right - self.left }
    fn height(self) -> f32 { self.bottom - self.top }
    fn union(&mut self, other: FRect) {
        self.left = self.left.min(other.left);
        self.top = self.top.min(other.top);
        self.right = self.right.max(other.right);
        self.bottom = self.bottom.max(other.bottom);
    }
}

// java ShapeMetrics.round
fn shape_round(value: f32) -> i32 {
    if value >= 0.0 { (value + 0.5) as i32 } else { (value - 0.5) as i32 }
}

// java Math.round(float)
fn math_round(value: f32) -> i32 {
    (value + 0.5).floor() as i32
}

fn downcast_ref<'a, T: 'static>(element: &'a Element) -> Option<&'a T> {
    element.as_ref().and_then(|arc| arc.downcast_ref::<T>())
}

fn lock_ref<T: 'static>(element: &Element) -> Option<std::sync::MutexGuard<'_, T>> {
    let arc = element.as_ref()?;
    let mutex = arc.downcast_ref::<Mutex<T>>()?;
    mutex.lock().ok()
}

// java public final class DrawingShapeRectTool
pub struct DrawingShapeRectTool;

impl DrawingShapeRectTool {
    // java TypoDrawing content size plus layout padding used by setDrawing.expandRect.
    pub fn embed_box(shape: &mut Shape) -> (i32, i32, i32, i32, i32, i32) {
        use crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl;
        use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
        let mut snapshot = TypoSnapshot::new();
        let mut drawing = snapshot.getTypoTools().obtainTypoDrawing();
        drawing.alloc(&mut snapshot);
        drawing.setEmbed(true);
        Self::set_shape_rect(&mut drawing, shape, true, true, 0, 0, 0);
        Self::set_render_and_layout_padding(&mut drawing, shape);
        let pad = drawing.getLayoutPadding();
        (
            drawing.width(),
            drawing.height(),
            TypoRectMemoryImpl::getLeft__int_typosnapshot(pad, &snapshot),
            TypoRectMemoryImpl::getTop__int_typosnapshot(pad, &snapshot),
            TypoRectMemoryImpl::getRight__int_typosnapshot(pad, &snapshot),
            TypoRectMemoryImpl::getBottom__int_typosnapshot(pad, &snapshot),
        )
    }

    // java LayoutState.setDrawing width/height after expandRect(layout padding).
    pub fn embed_outer_size(shape: &mut Shape) -> (i32, i32) {
        use crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl;
        use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
        use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool;
        let mut snapshot = TypoSnapshot::new();
        let mut drawing = snapshot.getTypoTools().obtainTypoDrawing();
        drawing.alloc(&mut snapshot);
        drawing.setEmbed(true);
        Self::set_shape_rect(&mut drawing, shape, true, true, 0, 0, 0);
        Self::set_render_and_layout_padding(&mut drawing, shape);
        let pad = drawing.getLayoutPadding();
        let pl = TypoRectMemoryImpl::getLeft__int_typosnapshot(pad, &snapshot);
        let pt = TypoRectMemoryImpl::getTop__int_typosnapshot(pad, &snapshot);
        let pr = TypoRectMemoryImpl::getRight__int_typosnapshot(pad, &snapshot);
        let pb = TypoRectMemoryImpl::getBottom__int_typosnapshot(pad, &snapshot);
        (
            DrawingResizeTool::expanded_width(drawing.width(), pl, pr),
            DrawingResizeTool::expanded_height(drawing.height(), pt, pb),
        )
    }

    // java DrawingLayouter.doPreFlowEmbedDrawing for a shape that has no vertical page rotation.
    pub fn preflow_embed(drawing: &mut TypoDrawing, shape: &mut Shape, global_page_idx: i32) {
        // java doPreFlowDrawing(drawing, shape, 0, 0, true)
        Self::set_shape_rect(drawing, shape, true, true, global_page_idx, 0, 0);
        // java textbox flow is skipped when getTextObj() is null or hasText() is false
        let _ = shape.getTextObj();
        Self::set_render_and_layout_padding(drawing, shape);
    }

    // java DrawingLayouter.getEmbed horz-rule tail
    pub fn apply_horz_rule(drawing: &mut TypoDrawing, shape: &mut Shape, core_width: i32, left_indent: i32, right_indent: i32) {
        if !shape.getHorzRule() { return; }
        let max_width = (core_width - left_indent - right_indent).max(0);
        let hr_scale = shape.getHorzRuleScale();
        if hr_scale > 0.0 {
            drawing.setWidth((hr_scale * core_width as f32) as i32);
        }
        if drawing.width() > max_width {
            drawing.setWidth(max_width);
        }
        if drawing.height() <= 0 {
            drawing.setHeight(20);
        }
        let spare = max_width - drawing.width();
        match shape.getHorzRuleAlignment() {
            1 => {
                let half = spare / 2;
                if half > 0 {
                    drawing.setLayoutPaddingEdge(0, half);
                    drawing.setLayoutPaddingEdge(2, half);
                }
            }
            2 => drawing.setLayoutPaddingEdge(0, spare),
            _ => drawing.setLayoutPaddingEdge(2, spare),
        }
    }

    // java DrawingShapeRectTool.calcDrawingShapeRect
    pub fn calc_shape_rect(drawing: &mut TypoDrawing, shape: &mut Shape, need_calc_size: bool, is_embed: bool, global_page_idx: i32, align_width: i32, align_height: i32) {
        Self::set_shape_rect(drawing, shape, need_calc_size, is_embed, global_page_idx, align_width, align_height);
    }

    // java setDrawingRenderRect effect padding, without the embed layout copy.
    pub fn apply_render_padding(drawing: &mut TypoDrawing, shape: &mut Shape) {
        let effect = effect_rect_twip(drawing, shape);
        let pl = drawing.getLeft() - effect.0;
        let pt = drawing.getTop() - effect.1;
        let pr = effect.2 - drawing.getRight();
        let pb = effect.3 - drawing.getBottom();
        drawing.setRenderPadding(pl, pt, pr, pb);
    }

    // java setDrawingShapeRect + DrawingShapeRectTool.calcDrawingShapeRect
    fn set_shape_rect(drawing: &mut TypoDrawing, shape: &mut Shape, need_calc_size: bool, is_embed: bool, global_page_idx: i32, align_width: i32, align_height: i32) {
        let pos_element = shape.getShapePos();
        let pos_guard;
        let pos: Option<&ShapePos> = if let Some(p) = downcast_ref::<ShapePos>(&pos_element) {
            Some(p)
        } else if let Some(g) = lock_ref::<ShapePos>(&pos_element) {
            pos_guard = g;
            Some(&*pos_guard)
        } else {
            None
        };
        let Some(pos) = pos else { return; };
        let rect_element = pos.getShapeRect();
        if let Some(rect) = rect_element.as_ref().and_then(|arc| arc.downcast_ref::<util::cn::wps::graphics::RectF::RectF>()) {
            if !need_calc_size {
                drawing.offsetTo(shape_round(rect.left * 20.0), shape_round(rect.top * 20.0));
            } else {
                drawing.set(
                    shape_round(rect.left * 20.0),
                    shape_round(rect.top * 20.0),
                    shape_round(rect.right * 20.0),
                    shape_round(rect.bottom * 20.0),
                );
            }
        } else if !need_calc_size {
            drawing.offsetTo(0, 0);
        } else {
            drawing.setEmpty();
        }
        if need_calc_size {
            let hori = pos.getSizeHPercent();
            if is_relative_width(shape, hori) {
                drawing.setWidth(size_by_percent(normalize_rel_h(pos.getSizeRelH(), global_page_idx), hori, true));
            }
            let vert = pos.getSizeVPercent();
            if vert > 0.0 {
                drawing.setHeight(size_by_percent(normalize_rel_v(pos.getSizeRelV(), global_page_idx), vert, false));
            }
        }
        let hori_pos = pos.getPosHPercent();
        if !is_embed && hori_pos != 0.0 {
            let left = (align_width as f32 * hori_pos / 100.0) as i32;
            drawing.offsetTo(left, drawing.getTop());
        }
        let vert_pos = pos.getPosVPercent();
        if !is_embed && vert_pos != 0.0 {
            let top = (align_height as f32 * vert_pos / 100.0) as i32;
            drawing.offsetTo(drawing.getLeft(), top);
        }
        if is_embed {
            drawing.offsetTo(0, 0);
        }
        if need_calc_size {
            let scale_h = pos.getScaleH();
            if scale_h != 1.0 {
                drawing.setWidth((drawing.width() as f32 * scale_h) as i32);
            }
            let scale_v = pos.getScaleV();
            if scale_v != 1.0 {
                drawing.setHeight((drawing.height() as f32 * scale_v) as i32);
            }
        }
    }

    // java setDrawingRenderRect + setDrawingLayoutRect embed branch
    fn set_render_and_layout_padding(drawing: &mut TypoDrawing, shape: &mut Shape) {
        Self::apply_render_padding(drawing, shape);
        if drawing.isEmbed() {
            drawing.copyRenderPaddingToLayout();
        }
    }
}

fn is_relative_width(shape: &mut Shape, hori: f32) -> bool {
    if hori <= 0.0 { return false; }
    let text_element = shape.getTextObj();
    let flow = if let Some(text) = downcast_ref::<Text>(&text_element) {
        if text.hasText() { Some(text.getTextFlow()) } else { None }
    } else if let Some(text) = lock_ref::<Text>(&text_element) {
        if text.hasText() { Some(text.getTextFlow()) } else { None }
    } else {
        None
    };
    match flow {
        Some(1) | Some(2) | Some(3) | Some(5) => false,
        _ => true,
    }
}

// java getShapeRectWidthByPercentage / Height when PageSetup is null
fn size_by_percent(rel: i32, percentage: f32, horizontal: bool) -> i32 {
    let base = if horizontal {
        match rel {
            0 => 8306,
            1 => 11906,
            2 | 3 => 1800,
            _ => 0,
        }
    } else {
        match rel {
            0 => 13958,
            1 => 16838,
            2 | 3 => 1440,
            _ => 0,
        }
    };
    let sized = (base as f32 * (percentage / 100.0)) as i32;
    sized.max(20)
}

fn normalize_rel_h(rel_h: i32, global_page_idx: i32) -> i32 {
    let even = global_page_idx % 2 == 0;
    if rel_h == 4 { if even { 3 } else { 2 } } else if rel_h == 5 { if even { 2 } else { 3 } } else { rel_h }
}

fn normalize_rel_v(rel_v: i32, global_page_idx: i32) -> i32 {
    let even = global_page_idx % 2 == 0;
    if rel_v == 4 || rel_v == 5 { if even { 2 } else { 3 } } else { rel_v }
}

// java DrawingShapeRectTool.calcDrawingOutRect → EffectTool.getEffectRect → adjustToLayout → Math.round
fn effect_rect_twip(drawing: &TypoDrawing, shape: &mut Shape) -> (i32, i32, i32, i32) {
    let mut rect = FRect {
        left: drawing.getLeft() as f32 * 0.05,
        top: drawing.getTop() as f32 * 0.05,
        right: drawing.getRight() as f32 * 0.05,
        bottom: drawing.getBottom() as f32 * 0.05,
    };
    let rot = shape.getRotation();
    // java shape.getOutRect(rect, rot). Rust getOutRect_with_rot currently returns the input rect.
    let _ = shape.getOutRect_with_rot(&util::cn::wps::graphics::RectF::RectF { left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom }, rot);
    rect = line_rect(shape, rect);
    if let Some(size) = glow_size(shape) {
        let size = (size / 2 + 1) as f32;
        rect.union(FRect { left: rect.left - size, top: rect.top - size, right: rect.right + size, bottom: rect.bottom + size });
    }
    if let Some(shadow_bounds) = shadow_bounds(shape, rect, rot) {
        rect.union(shadow_bounds);
    }
    if let Some((dist, end_pos)) = reflection_metrics(shape) {
        let mut invalid = FRect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.top + rect.height() * end_pos,
        };
        invalid.top += rect.height() + dist;
        invalid.bottom += rect.height() + dist;
        rect.union(invalid);
    }
    (
        math_round(rect.left * 20.0),
        math_round(rect.top * 20.0),
        math_round(rect.right * 20.0),
        math_round(rect.bottom * 20.0),
    )
}

fn line_rect(shape: &mut Shape, rect: FRect) -> FRect {
    let element = shape.getLineProperty();
    let expand = if let Some(line) = downcast_ref::<LineProperty>(&element) {
        Some(line_expand(line))
    } else if let Some(line) = lock_ref::<LineProperty>(&element) {
        Some(line_expand(&line))
    } else {
        None
    };
    match expand {
        Some(width) => FRect { left: rect.left - width, top: rect.top - width, right: rect.right + width, bottom: rect.bottom + width },
        None => rect,
    }
}

fn line_expand(line: &LineProperty) -> f32 {
    let mut width = line.getWidth() / 2.0;
    let mut arrow_size = 0.0;
    if let Some(size) = arrow_size_of(&line.getStartArrow(), width) {
        arrow_size = size;
    }
    if let Some(size) = arrow_size_of(&line.getEndArrow(), width) {
        if size > arrow_size { arrow_size = size; }
    }
    width + arrow_size
}

fn arrow_size_of(element: &Element, stroke: f32) -> Option<f32> {
    let arrow = downcast_ref::<Arrow>(element).map(|arrow| arrow.getHead() != 0).and_then(|has| {
        if has { downcast_ref::<Arrow>(element).map(|arrow| arrow.getLineWidth(stroke)) } else { None }
    });
    if arrow.is_some() { return arrow; }
    let guard = lock_ref::<Arrow>(element)?;
    if guard.getHead() == 0 { None } else { Some(guard.getLineWidth(stroke)) }
}

fn glow_size(shape: &mut Shape) -> Option<i32> {
    let element = shape.getGlow();
    if let Some(glow) = downcast_ref::<Glow>(&element) { return Some(glow.getGlowSize()); }
    lock_ref::<Glow>(&element).map(|glow| glow.getGlowSize())
}

fn reflection_metrics(shape: &mut Shape) -> Option<(f32, f32)> {
    let element = shape.getReflection();
    if let Some(reflection) = downcast_ref::<Reflection>(&element) {
        return Some((reflection.getReflectionDist(), reflection.getReflectionEndPos()));
    }
    lock_ref::<Reflection>(&element).map(|reflection| (reflection.getReflectionDist(), reflection.getReflectionEndPos()))
}

fn shadow_bounds(shape: &mut Shape, rect: FRect, rot: f32) -> Option<FRect> {
    let element = shape.getShadow();
    if let Some(shadow) = downcast_ref::<Shadow>(&element) { return shadow_rect(shadow, rect, rot); }
    lock_ref::<Shadow>(&element).and_then(|shadow| shadow_rect(&shadow, rect, rot))
}

// java EffectTool.getShadowRect for non-DML types 0/1/5. DML perspective stays the input rect.
fn shadow_rect(shadow: &Shadow, rect: FRect, rot: f32) -> Option<FRect> {
    if shadow.isDml() {
        return Some(rect);
    }
    let type_id = shadow.getShadowType();
    if type_id != 0 && type_id != 1 && type_id != 5 && type_id != 2 && type_id != 3 && type_id != 4 {
        return Some(rect);
    }
    if type_id == 2 || type_id == 3 || type_id == 4 {
        return Some(rect);
    }
    let mut shadow_rect = offset_shadow(shadow, rect, rot, false);
    if type_id == 1 || type_id == 5 {
        shadow_rect.union(offset_shadow(shadow, rect, rot, true));
    }
    Some(shadow_rect)
}

fn offset_shadow(shadow: &Shadow, rect: FRect, rot: f32, second: bool) -> FRect {
    let mut x = shadow.getShadowOffsetXObj().unwrap_or_else(|| shadow.getShadowOffsetXFrac() * rect.width());
    let mut y = shadow.getShadowOffsetYObj().unwrap_or_else(|| shadow.getShadowOffsetYFrac() * rect.height());
    if second {
        if shadow.getShadowType() == 1 {
            x = shadow.getShadowSecondOffsetX();
            y = shadow.getShadowSecondOffsetY();
        } else {
            x = -x;
            y = -y;
        }
    }
    if rot > 0.0 && !shadow.getRotWithShape() {
        let rad = (-rot).to_radians();
        let (sin, cos) = rad.sin_cos();
        let nx = x * cos - y * sin;
        let ny = x * sin + y * cos;
        x = nx;
        y = ny;
    }
    FRect { left: rect.left + x, top: rect.top + y, right: rect.right + x, bottom: rect.bottom + y }
}
