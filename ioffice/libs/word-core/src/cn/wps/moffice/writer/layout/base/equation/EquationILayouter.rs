#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationILayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
use crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::cache::TypoTextsOpIterator::TypoTextsOpIterator;
use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool::{calcRectForPage, CoordRect};
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationContentLayouter::EquationContentLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLayouter::{EquationException, EquationLayouter};
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;
use crate::cn::wps::moffice::writer::layout::base::equation::IEquationLayouter::IEquationLayouter;

struct CoreRect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl CoreRect {
    fn empty() -> Self { Self { left: 0, top: 0, right: 0, bottom: 0 } }
    fn center_y(&self) -> i32 { (self.top + self.bottom) >> 1 }
    fn right(&self) -> i32 { self.right }
}

// java public class EquationILayouter extends IEquationLayouter
pub struct EquationILayouter;

impl EquationILayouter {
    // java public int flow(Document document, EquationResult eqResult, int end)
    pub fn flow(
        main_state: &mut MainState,
        document_env: &mut DocumentEnv,
        view_env: &mut ViewEnv,
        document: *mut dyn Document,
        eq_result: &EquationResult,
        _end: i32,
        enable_revision: bool,
        revision_final: bool,
    ) -> Result<i32, EquationException> {
        if eq_result.getValuesSize() != 3 {
            return Err(EquationException);
        }
        let mut nary_char: char = char::from_u32(8747).unwrap_or('\u{222B}');
        let mut need_scale_nary = true;
        let mut flow_down_and_up = false;
        let op_size = eq_result.getOPSize();
        let mut i = 1;
        while i < op_size {
            let op = eq_result.getOP(i);
            if op == "su" {
                nary_char = char::from_u32(8721).unwrap_or('\u{2211}');
            } else if op == "pr" {
                nary_char = char::from_u32(8719).unwrap_or('\u{220F}');
            } else if op == "in" {
                flow_down_and_up = true;
            } else if op == "fc" || op == "vc" {
                if op == "fc" {
                    need_scale_nary = false;
                }
                i += 1;
                if i < op_size {
                    nary_char = eq_result.getOP(i).chars().next().unwrap_or(nary_char);
                }
            }
            i += 1;
        }
        let mut drawing = obtain_drawing(main_state);
        let down = child(main_state, document_env, view_env, document, eq_result, 0, enable_revision, revision_final)?;
        if down != 0 {
            drawing.addChildDrawing(down);
        }
        let up = child(main_state, document_env, view_env, document, eq_result, 1, enable_revision, revision_final)?;
        if up != 0 {
            drawing.addChildDrawing(up);
        }
        let main = child(main_state, document_env, view_env, document, eq_result, 2, enable_revision, revision_final)?;
        if main != 0 {
            drawing.addChildDrawing(main);
        }
        let color = IEquationLayouter::finalRunColor(unsafe { &*document }, revision_final, eq_result.getStartCP() + 1);
        let nary = EquationContentLayouter::get_char_drawing(main_state, document_env, view_env, document, nary_char, color, false, false);
        if nary == 0 {
            main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
            return Err(EquationException);
        }
        drawing.addChildDrawing(nary);
        let mut nary_drawing = main_state.snapshot.getTypoTools().obtainTypoDrawing__int(nary);
        if need_scale_nary {
            let mut nary_height = if main != 0 { TypoDrawing::height__int(main, &main_state.snapshot) } else { 0 };
            if flow_down_and_up {
                let mut count = 0;
                if up != 0 {
                    count += TypoDrawing::height__int(up, &main_state.snapshot);
                }
                if down != 0 {
                    count += TypoDrawing::height__int(down, &main_state.snapshot);
                }
                nary_height = nary_height.max(count);
            }
            if nary_height != 0 {
                let height = nary_drawing.height();
                if height != 0 {
                    scale_math(&mut nary_drawing, &mut main_state.snapshot, nary_height as f32 / height as f32);
                }
            }
        }
        let (drawing_width, drawing_height) = if flow_down_and_up {
            place_in_nary(&mut nary_drawing, up, down, main, &mut main_state.snapshot)
        } else {
            place_stacked(&mut nary_drawing, up, down, main, &mut main_state.snapshot)
        };
        if main != 0 {
            drawing.setAlignLine(TypoObjBase::getTop__int_typosnapshot(main, &main_state.snapshot) + TypoDrawing::getAlignLine__int_typosnapshot(main, &main_state.snapshot));
            drawing.setAlignType(TypoDrawing::getAlignType__int_typosnapshot(main, &main_state.snapshot));
        } else {
            IEquationLayouter::calcBaseline(&mut drawing, unsafe { &*document }, view_env, (nary_drawing.getTop() + nary_drawing.getBottom()) / 2, eq_result.getStartCP() + 1);
        }
        drawing.setHeight(drawing_height);
        drawing.setWidth(drawing_width);
        let pointer = drawing.getPointer();
        main_state.snapshot.getTypoTools().recycle_drawing(&mut nary_drawing);
        main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
        Ok(pointer)
    }
}

fn child(
    main_state: &mut MainState,
    document_env: &mut DocumentEnv,
    view_env: &mut ViewEnv,
    document: *mut dyn Document,
    eq_result: &EquationResult,
    index: i32,
    enable_revision: bool,
    revision_final: bool,
) -> Result<i32, EquationException> {
    EquationLayouter::getEquation(
        main_state,
        document_env,
        view_env,
        Some(document),
        eq_result.getStart(index),
        eq_result.getEnd(index),
        enable_revision,
        revision_final,
        false,
    )
}

fn obtain_drawing(main_state: &mut MainState) -> TypoDrawing {
    let snapshot = &mut main_state.snapshot;
    let mut drawing = snapshot.getTypoTools().obtainTypoDrawing();
    drawing.alloc(snapshot);
    drawing.setDrawingType(15);
    drawing.setIsRotateWithPage(true);
    drawing.setEmbed(true);
    drawing
}

fn place_in_nary(nary: &mut TypoDrawing, up: i32, down: i32, main: i32, snapshot: &mut TypoSnapshot) -> (i32, i32) {
    let mut drawing_height = TypoDrawing::layoutHeight__int_typosnapshot(nary.getPointer(), snapshot);
    if main != 0 {
        drawing_height = drawing_height.max(TypoDrawing::height__int(main, snapshot));
    }
    offset_rotate(nary, 0, (drawing_height - TypoDrawing::layoutHeight__int_typosnapshot(nary.getPointer(), snapshot)) / 2, snapshot);
    let tmp = layout_rect_for_page_core(nary, snapshot);
    let mut drawing_width = tmp.right();
    if up != 0 {
        TypoObjBase::offsetTo__int_int_int_typosnapshot(tmp.right, tmp.top, up, snapshot);
        drawing_width = drawing_width.max(TypoObjBase::getRight__int_typosnapshot(up, snapshot));
    }
    if down != 0 {
        TypoObjBase::offsetTo__int_int_int_typosnapshot(tmp.right, tmp.bottom - TypoDrawing::height__int(down, snapshot), down, snapshot);
        drawing_width = drawing_width.max(TypoObjBase::getRight__int_typosnapshot(down, snapshot));
    }
    if main != 0 {
        TypoObjBase::offsetTo__int_int_int_typosnapshot(drawing_width, tmp.center_y() - TypoDrawing::height__int(main, snapshot) / 2, main, snapshot);
        drawing_width = TypoObjBase::getRight__int_typosnapshot(main, snapshot);
    }
    (drawing_width, drawing_height)
}

fn place_stacked(nary: &mut TypoDrawing, up: i32, down: i32, main: i32, snapshot: &mut TypoSnapshot) -> (i32, i32) {
    let mut max_width = TypoDrawing::layoutWidth__int_typosnapshot(nary.getPointer(), snapshot);
    let mut drawing_height = 0;
    if up != 0 {
        max_width = max_width.max(TypoDrawing::layoutWidth__int_typosnapshot(up, snapshot));
        drawing_height = TypoDrawing::height__int(up, snapshot);
    }
    if down != 0 {
        max_width = max_width.max(TypoDrawing::layoutWidth__int_typosnapshot(down, snapshot));
    }
    offset_rotate(nary, (max_width - TypoDrawing::layoutWidth__int_typosnapshot(nary.getPointer(), snapshot)) / 2, drawing_height, snapshot);
    drawing_height += TypoDrawing::layoutHeight__int_typosnapshot(nary.getPointer(), snapshot);
    if up != 0 {
        TypoObjBase::offsetTo__int_int_int_typosnapshot((max_width - TypoDrawing::layoutWidth__int_typosnapshot(up, snapshot)) / 2, 0, up, snapshot);
    }
    if down != 0 {
        TypoObjBase::offsetTo__int_int_int_typosnapshot((max_width - TypoDrawing::layoutWidth__int_typosnapshot(down, snapshot)) / 2, drawing_height, down, snapshot);
        drawing_height += TypoDrawing::height__int(down, snapshot);
    }
    let tmp = layout_rect_for_page_core(nary, snapshot);
    let mut drawing_width = tmp.right();
    if main != 0 {
        TypoObjBase::offsetTo__int_int_int_typosnapshot(drawing_width, tmp.center_y() - TypoDrawing::height__int(main, snapshot) / 2, main, snapshot);
        drawing_width += TypoDrawing::width__int(main, snapshot);
    }
    (drawing_width, drawing_height)
}

// java DrawingPositionTool.offsetRotateDrawing
fn offset_rotate(drawing: &mut TypoDrawing, left: i32, top: i32, snapshot: &mut TypoSnapshot) {
    let rect = layout_rect_for_page_core(drawing, snapshot);
    TypoObjBase::offset__int_int_int_typosnapshot(left - rect.left, top - rect.top, drawing.getPointer(), snapshot);
}

// java IEquationLayouter.scaleMath
fn scale_math(drawing: &mut TypoDrawing, snapshot: &mut TypoSnapshot, scale: f32) {
    let zoom = TypoDrawing::getZoom__int_typosnapshot(drawing.getPointer(), snapshot);
    drawing.setZoom(zoom * scale);
    let expanded = expand_drawing(drawing, snapshot, true);
    let rect_left = (expanded.left as f32 * scale) as i32;
    let rect_top = (expanded.top as f32 * scale) as i32;
    let rect_right = (expanded.right as f32 * scale) as i32;
    let rect_bottom = (expanded.bottom as f32 * scale) as i32;
    write_layout_padding(drawing, snapshot, rect_left, rect_top, rect_right, rect_bottom);
    let align = TypoDrawing::getAlignLine__int_typosnapshot(drawing.getPointer(), snapshot);
    drawing.setAlignLine((align as f32 * scale) as i32);
}

// java TypoDrawing.layoutRectForPageCore while the drawing is still inside the equation.
// Page attachment is 0 here, so the page-margin conversion is not applied.
fn layout_rect_for_page_core(drawing: &TypoDrawing, snapshot: &mut TypoSnapshot) -> CoreRect {
    // java doGetRectForPageCore: an embed whose parent is a text line walks the text ops.
    if is_embed_with_text_line_anchor(drawing, snapshot) {
        return embed_rect_for_page_core(drawing, snapshot, true);
    }
    rect_for_page(drawing, snapshot, true, false)
}

fn is_embed_with_text_line_anchor(drawing: &TypoDrawing, snapshot: &mut TypoSnapshot) -> bool {
    // java return this.isEmbed() && TypoMemoryObj.getType(this.getParent(), this.mSnapshot) == 3;
    let parent = TypoDrawing::getParent__int_typosnapshot(drawing.getPointer(), snapshot);
    drawing.isEmbed() && TypoMemoryObj::getType__int_typosnapshot(parent, snapshot) == 3
}

// java TypoDrawing.doGetEmbedRectForPageCore
fn embed_rect_for_page_core(drawing: &TypoDrawing, snapshot: &mut TypoSnapshot, use_padding: bool) -> CoreRect {
    let parent = TypoDrawing::getParent__int_typosnapshot(drawing.getPointer(), snapshot);
    let p_para = TypoTextLine::getPara__int_typosnapshot(parent, snapshot);
    let line_rel_top = snapshot.get__int(parent + crate::cn::wps::moffice::writer::cache::TypoTextLine::OFFSET_TEXT_RECT_TOP)
        + TypoObjBase::getTop__int_typosnapshot(parent, snapshot);
    let mut drawing_rect = CoreRect::empty();
    let snap = snapshot as *mut TypoSnapshot;
    let mut texts = TypoTextsOpIterator::newInstance();
    texts.init(parent, snap);
    while let Some(typo_text) = texts.next() {
        if typo_text.runType == 6 && TypoPara::getEmbed__int_int_typosnapshot(typo_text.embedIndex, p_para, snapshot) == drawing.getPointer() {
            let p_padding = drawing.getLayoutPadding();
            let begin = if typo_text.hasBeginBdr() { typo_text.getBdrWidth() } else { 0 };
            let left = typo_text.textLeft + TypoRectMemoryImpl::getLeft__int_typosnapshot(p_padding, snapshot) + begin;
            let max_ascent = typo_text.typoFontStyle.as_ref().map(|style| style.maxAscent).unwrap_or(0);
            let top = typo_text.baseline - max_ascent + TypoRectMemoryImpl::getTop__int_typosnapshot(p_padding, snapshot) + typo_text.getBdrWidth();
            drawing_rect.left = left;
            drawing_rect.top = top;
            drawing_rect.right = left + drawing.width();
            drawing_rect.bottom = top + drawing.height();
            break;
        }
    }
    // java DrawingResizeTool.expandRect(drawingRect, paddings, outRect)
    let mut out = if use_padding {
        let pad = drawing.getLayoutPadding();
        CoreRect {
            left: drawing_rect.left - TypoRectMemoryImpl::getLeft__int_typosnapshot(pad, snapshot),
            top: drawing_rect.top - TypoRectMemoryImpl::getTop__int_typosnapshot(pad, snapshot),
            right: drawing_rect.right + TypoRectMemoryImpl::getRight__int_typosnapshot(pad, snapshot),
            bottom: drawing_rect.bottom + TypoRectMemoryImpl::getBottom__int_typosnapshot(pad, snapshot),
        }
    } else {
        drawing_rect
    };
    // java outRect.offset(0, lineRelTop);
    out.top += line_rel_top;
    out.bottom += line_rel_top;
    out
}

fn rect_for_page(drawing: &TypoDrawing, snapshot: &mut TypoSnapshot, use_padding: bool, need_check_embed: bool) -> CoreRect {
    if need_check_embed && is_embed_with_text_line_anchor(drawing, snapshot) {
        let mut out = embed_rect_for_page_core(drawing, snapshot, use_padding);
        let page = TypoObjBase::getPage__int_typosnapshot(drawing.getPointer(), snapshot);
        if page != 0 {
            let mut coord = CoordRect { left: out.left, top: out.top, right: out.right, bottom: out.bottom };
            calcRectForPage(page, page, &mut coord, snapshot);
            out.left = coord.left;
            out.top = coord.top;
            out.right = coord.right;
            out.bottom = coord.bottom;
        }
        return out;
    }
    let parent = TypoDrawing::getParent__int_typosnapshot(drawing.getPointer(), snapshot);
    if parent == 0 {
        return CoreRect::empty();
    }
    let mut out = expand_drawing(drawing, snapshot, use_padding);
    if TypoMemoryObj::getType__int_typosnapshot(parent, snapshot) == 7 {
        let parent_drawing = snapshot.getTypoTools().obtainTypoDrawing__int(parent);
        let parent_rect = rect_for_page(&parent_drawing, snapshot, false, true);
        out.left += parent_rect.left;
        out.right += parent_rect.left;
        out.top += parent_rect.top;
        out.bottom += parent_rect.top;
    }
    out
}

fn expand_drawing(drawing: &TypoDrawing, snapshot: &TypoSnapshot, use_padding: bool) -> CoreRect {
    let mut left = drawing.getLeft();
    let mut top = drawing.getTop();
    let mut right = drawing.getRight();
    let mut bottom = drawing.getBottom();
    if use_padding {
        let pad = drawing.getLayoutPadding();
        left -= TypoRectMemoryImpl::getLeft__int_typosnapshot(pad, snapshot);
        top -= TypoRectMemoryImpl::getTop__int_typosnapshot(pad, snapshot);
        right += TypoRectMemoryImpl::getRight__int_typosnapshot(pad, snapshot);
        bottom += TypoRectMemoryImpl::getBottom__int_typosnapshot(pad, snapshot);
    }
    CoreRect { left, top, right, bottom }
}

fn write_layout_padding(drawing: &TypoDrawing, snapshot: &mut TypoSnapshot, rect_left: i32, rect_top: i32, rect_right: i32, rect_bottom: i32) {
    let pad = drawing.getLayoutPadding();
    TypoRectMemoryImpl::setLeft__int_int_typosnapshot(drawing.getLeft() - rect_left, pad, snapshot);
    TypoRectMemoryImpl::setTop__int_int_typosnapshot(drawing.getTop() - rect_top, pad, snapshot);
    TypoRectMemoryImpl::setRight__int_int_typosnapshot(rect_right - drawing.getRight(), pad, snapshot);
    TypoRectMemoryImpl::setBottom__int_int_typosnapshot(rect_bottom - drawing.getBottom(), pad, snapshot);
}
