#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.IEquationLayouter
use crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use thirdparty_font::cn::wps::font::FontHost::FontHost;
use thirdparty_font::cn::wps::font::IFontFamily::IFontFamily;
use thirdparty_font::cn::wps::font::IFontTable::IFontTable;
use thirdparty_font::cn::wps::font::ITypeface::{FontMetrics, ITypeface};
use util::moffice::util::UnitUtil::UnitUtil;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::IChpxTable::{Entry as ChpxEntry, IChpxTable};
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
use crate::cn::wps::moffice::writer::layout::base::env::PropertyTool::PropertyTool;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;

// java public abstract class IEquationLayouter
pub struct IEquationLayouter;

impl IEquationLayouter {
    // java protected float getFloat(String op, int start)
    pub fn getFloat(op: &str, start: i32) -> f32 {
        let rest: String = op
            .chars()
            .skip(start.max(0) as usize)
            .skip_while(|ch| !(*ch >= '0' && *ch <= '9'))
            .collect();
        if rest.is_empty() {
            0.0
        } else {
            rest.parse::<f32>().unwrap_or(0.0)
        }
    }

    // java protected int getInt(String op, int start)
    pub fn getInt(op: &str, start: i32) -> i32 {
        let chars: Vec<char> = op.chars().collect();
        let max = chars.len();
        let mut index = start.max(0) as usize;
        while index < max && !(chars[index] >= '0' && chars[index] <= '9') {
            index += 1;
        }
        if index >= max {
            return 0;
        }
        let mut end_index = index + 1;
        while end_index < max && chars[end_index] >= '0' && chars[end_index] <= '9' {
            end_index += 1;
        }
        chars[index..end_index].iter().collect::<String>().parse::<i32>().unwrap_or(0)
    }

    // java protected void calcBaseline(TypoDrawing drawing, Document document, int alignCenter, int mainOpCP)
    pub fn calcBaseline(drawing: &mut TypoDrawing, document: &dyn Document, view_env: &ViewEnv, align_center: i32, main_op_cp: i32) {
        let Some(papx) = document.getPapxTable().filter(|p| !p.is_null()) else { return };
        let Some(chpx) = document.getChpxTable().filter(|p| !p.is_null()) else { return };
        let Some(para) = (unsafe { IPapxTable::seek(&*papx, main_op_cp) }) else { return };
        let Some(run) = (unsafe { IChpxTable::seek(&*chpx, main_op_cp) }) else { return };
        let doc_type = document.getType__on_IDocument();
        let (para_cache, run_cache) = if doc_type == 0 { (64, 128) } else { (4, 4) };
        let mut tool = PropertyTool::PropertyTool(document, para_cache, run_cache);
        let revision_final = view_env.isRevisionFinal();
        let para_prop = tool.getFinalParaProp(para.as_ref() as &dyn PapxEntry, revision_final).clone();
        let run_prop = tool.getFinalRunPropert(run.as_ref() as &dyn ChpxEntry, &para_prop, revision_final).clone();
        let font_name = run_prop.getString__int_string(3, "Times New Roman");
        let font_pt = run_prop.getFloat__int_float(10, 10.0);
        let size = UnitUtil::PT2TWIPS(font_pt);
        let table = FontHost::getFontTable();
        let Some(family) = IFontTable::get_by_name_bool(table.as_ref(), &font_name, true) else { return };
        let Some(face) = IFontFamily::getTypeface(family.as_ref(), 0) else { return };
        let mut metrics = FontMetrics::new();
        ITypeface::getFontMetrics(face.as_ref(), size as f32, &mut metrics);
        let max_ascent = metrics.mMaxAscent as i32;
        let max_descent = metrics.mMaxDescent as i32;
        let half_of_height = (max_ascent + max_descent) / 2;
        let base_line = align_center - half_of_height + max_ascent;
        drawing.setAlignType(1);
        drawing.setAlignLine(base_line);
    }

    // java PropertySet.getInt(11, -16777216) after getFinalRunPropert
    pub fn finalRunColor(document: &dyn Document, revision_final: bool, cp: i32) -> i32 {
        let Some(papx) = document.getPapxTable().filter(|p| !p.is_null()) else { return -16777216; };
        let Some(chpx) = document.getChpxTable().filter(|p| !p.is_null()) else { return -16777216; };
        let Some(para) = (unsafe { IPapxTable::seek(&*papx, cp) }) else { return -16777216; };
        let Some(run) = (unsafe { IChpxTable::seek(&*chpx, cp) }) else { return -16777216; };
        let doc_type = document.getType__on_IDocument();
        let (para_cache, run_cache) = if doc_type == 0 { (64, 128) } else { (4, 4) };
        let mut tool = PropertyTool::PropertyTool(document, para_cache, run_cache);
        let para_prop = tool.getFinalParaProp(para.as_ref() as &dyn PapxEntry, revision_final).clone();
        let run_prop = tool.getFinalRunPropert(run.as_ref() as &dyn ChpxEntry, &para_prop, revision_final);
        run_prop.getInt__int_int(11, -16777216)
    }

    // java protected void setMathScale(TypoDrawing drawing, float scale, int type)
    pub fn setMathScale(drawing: &mut TypoDrawing, snapshot: &mut TypoSnapshot, scale: f32, zoom_type: i32) {
        drawing.setMathZoomType(zoom_type);
        drawing.setZoom(scale);
        let (x_scale, y_scale) = match zoom_type {
            0 => (scale, scale),
            1 => (scale, 1.0),
            _ => (1.0, scale),
        };
        let left = drawing.getLeft();
        let top = drawing.getTop();
        let right = drawing.getRight();
        let bottom = drawing.getBottom();
        let rect_left = (left as f32 * x_scale) as i32;
        let rect_top = (top as f32 * y_scale) as i32;
        let rect_right = (right as f32 * x_scale) as i32;
        let rect_bottom = (bottom as f32 * y_scale) as i32;
        let pad = drawing.getLayoutPadding();
        TypoRectMemoryImpl::setLeft__int_int_typosnapshot(left - rect_left, pad, snapshot);
        TypoRectMemoryImpl::setTop__int_int_typosnapshot(top - rect_top, pad, snapshot);
        TypoRectMemoryImpl::setRight__int_int_typosnapshot(rect_right - right, pad, snapshot);
        TypoRectMemoryImpl::setBottom__int_int_typosnapshot(rect_bottom - bottom, pad, snapshot);
        if zoom_type != 1 {
            let align = TypoDrawing::getAlignLine__int_typosnapshot(drawing.getPointer(), snapshot);
            drawing.setAlignLine((align as f32 * scale) as i32);
        }
    }
}
