//! Page-level inheritance + absolute geometry orchestrator (pp-13:03).
//! Not a Java class: walks each page and calls `PlaceHolderMerger` / `ShapeUtil`.

use crate::cn::wps::show::layout::PlaceHolderMerger::PlaceHolderMerger;
use crate::cn::wps::show::util::layout_shape::{KmoShowShape, KmoSlide, Transform2D};
use crate::cn::wps::show::util::layout_textbox::KmoTextOperator;
use crate::cn::wps::show::util::ShapeUtil::ShapeUtil;
use util::cn::wps::graphics::RectF::RectF;

#[derive(Clone)]
pub struct ResolvedShape {
    pub xfrm: Transform2D,
    pub abs: RectF,
}

fn apply_xfrm(shape: &mut KmoShowShape, t: &Transform2D) {
    shape.has_xfrm = true;
    shape.transform2DX = t.x;
    shape.transform2DY = t.y;
    shape.transform2DW = t.cx;
    shape.transform2DH = t.cy;
}

fn text_op_empty(op: &Option<KmoTextOperator>) -> bool {
    match op {
        None => true,
        Some(op) => op
            .paras
            .iter()
            .all(|p| p.runs.iter().all(|r| r.text.trim().is_empty())),
    }
}

fn inherit_ph_text(shape: &mut KmoShowShape) {
    if !shape.is_placeholder || !text_op_empty(&shape.text_op) {
        return;
    }
    if let Some(layout_ph) = PlaceHolderMerger::findLayoutPhShape(Some(shape)) {
        if !text_op_empty(&layout_ph.text_op) {
            shape.text_op = layout_ph.text_op.clone();
            shape.has_text_body = true;
            shape.is_text_box = true;
        }
    }
}

/// Java `calcChildRelativeRect` stays group-local (no group origin).
/// Product `RenderCommand` has no canvas translate, so writeback offsets
/// that local rect by the parent group's slide-absolute origin.
fn writeback_slide_abs(shape: &mut KmoShowShape, abs: &RectF) {
    // Groups keep XML off/ext; only non-group children get slide-absolute xfrm.
    if shape.r#type() == 0 {
        return;
    }
    shape.has_xfrm = true;
    shape.transform2DX = abs.left as i32;
    shape.transform2DY = abs.top as i32;
    shape.transform2DW = abs.width() as i32;
    shape.transform2DH = abs.height() as i32;
}

fn resolve_shape(
    shape: &mut KmoShowShape,
    out: &mut Vec<ResolvedShape>,
    parent_slide_origin: (f32, f32),
) {
    let t = PlaceHolderMerger::mergeTransform(shape);
    apply_xfrm(shape, &t);
    // Java: PlaceHolderMerger.mergeShapePr — write back; do not drop.
    shape.sp_pr = PlaceHolderMerger::mergeShapePr(shape);
    inherit_ph_text(shape);
    let mut abs = RectF::new();
    ShapeUtil::calcRelativeRect__kmoshowshape_rectf(shape, &mut abs);
    if let Some(parent) = shape.parentGroup() {
        if parent.hasChildPoint() && parent.hasChildExtents() {
            abs.offset(parent_slide_origin.0, parent_slide_origin.1);
        }
    }
    writeback_slide_abs(shape, &abs);
    let child_origin = if shape.r#type() == 0 {
        (abs.left, abs.top)
    } else {
        parent_slide_origin
    };
    out.push(ResolvedShape {
        xfrm: t,
        abs,
    });
    let parent = std::rc::Rc::new(shape.clone());
    for child in shape.children.iter_mut() {
        child.parent = Some(parent.clone());
        resolve_shape(child, out, child_origin);
    }
}

/// Bind layout/master on each slide, inherit ph xfrm, then compute absolute rects.
/// Shape XML order is preserved (depth-first, original child order).
pub fn resolve_pages(slides: &mut [KmoSlide]) -> Vec<Vec<ResolvedShape>> {
    let mut pages = Vec::with_capacity(slides.len());
    for slide in slides.iter_mut() {
        pages.push(resolve_page(slide));
    }
    pages
}

pub fn resolve_page(slide: &mut KmoSlide) -> Vec<ResolvedShape> {
    let mut out = Vec::new();
    for shape in slide.sp_tree.children.iter_mut() {
        resolve_shape(shape, &mut out, (0.0, 0.0));
    }
    out
}
