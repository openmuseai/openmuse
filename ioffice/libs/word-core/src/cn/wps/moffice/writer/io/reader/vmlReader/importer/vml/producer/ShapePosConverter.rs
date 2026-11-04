// Java: cn.wps.moffice.writer.io.reader.vmlReader.importer.vml.producer.ShapePosConverter
#![allow(non_snake_case)]

use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
use util::cn::wps::graphics::RectF::RectF;

use crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::helper::VmlStyle::{
    position_css_unit_to_pt, CssUnit, VmlStyle,
};

// Java: public class ShapePosConverter
pub struct ShapePosConverter;

impl ShapePosConverter {
    // Java: public static void setWrapDistances(CssStyle, IPos)
    pub fn setWrapDistances(style: Option<&VmlStyle>, shape_pos: &mut ShapePos) {
        let Some(style) = style else { return; };
        if let Some(unit) = style.wrap_left() { shape_pos.setWrapDistLeft(position_css_unit_to_pt(false, unit)); }
        if let Some(unit) = style.wrap_right() { shape_pos.setWrapDistRight(position_css_unit_to_pt(false, unit)); }
        if let Some(unit) = style.wrap_top() { shape_pos.setWrapDistTop(position_css_unit_to_pt(false, unit)); }
        if let Some(unit) = style.wrap_bottom() { shape_pos.setWrapDistBottom(position_css_unit_to_pt(false, unit)); }
    }

    // Java: public static void setRotation — CoreHelper.vmlRotation2DmlRot
    pub fn setRotation(style: Option<&VmlStyle>, shape_pos: &mut ShapePos, flip_h: bool, flip_v: bool) {
        let Some(value) = style.and_then(|s| s.rotation_degrees()) else { return; };
        // Java: if (flipH && !flipV || !flipH && flipV) rotation = -rot; if (rotation < 0) rotation += 360;
        let mut rotation = value;
        if flip_h != flip_v { rotation = -rotation; }
        if rotation < 0.0 { rotation += 360.0; }
        shape_pos.setRotation(rotation);
    }

    // Java: public static void setShapePosAnchor — default relative page (2) when style omits mso-position-*.
    pub fn setShapePosAnchor(shape_pos: &mut ShapePos) {
        // Java: convertPositionHRel/VRel default 2.
        shape_pos.setAlignmentH(0);
        shape_pos.setAlignmentV(0);
        shape_pos.setRelativeH(2);
        shape_pos.setRelativeV(2);
    }

    // Java: public static RectF getShapePosRect(boolean isInGroup, CssStyle, boolean bInline)
    pub fn getShapePosRect(is_in_group: bool, style: Option<&VmlStyle>, inline: bool) -> RectF {
        if inline {
            Self::inline_rect(is_in_group, style)
        } else {
            Self::anchor_rect(is_in_group, style)
        }
    }

    fn inline_rect(is_in_group: bool, style: Option<&VmlStyle>) -> RectF {
        let width = style.and_then(|s| s.width());
        let height = style.and_then(|s| s.height());
        let right = convert_right(is_in_group, 0.0, width);
        let bottom = convert_bottom(is_in_group, 0.0, height);
        RectF::new_with(0.0, 0.0, right, bottom)
    }

    fn anchor_rect(is_in_group: bool, style: Option<&VmlStyle>) -> RectF {
        let width = style.and_then(|s| s.width());
        let height = style.and_then(|s| s.height());
        let top = style.and_then(|s| s.top());
        let left = style.and_then(|s| s.left());
        let margin_left = style.and_then(|s| s.margin_left());
        let margin_top = style.and_then(|s| s.margin_top());
        let rect_left = convert_left(is_in_group, left, margin_left);
        let rect_top = convert_top(is_in_group, top, margin_top);
        let rect_right = convert_right(is_in_group, rect_left, width);
        let rect_bottom = convert_bottom(is_in_group, rect_top, height);
        RectF::new_with(rect_left, rect_top, rect_right, rect_bottom)
    }
}

fn convert_left(is_in_group: bool, left: Option<CssUnit>, margin_left: Option<CssUnit>) -> f32 {
    let rect_left = left.map(|u| position_css_unit_to_pt(is_in_group, u)).unwrap_or(0.0);
    match margin_left {
        None => rect_left,
        Some(margin) => rect_left + position_css_unit_to_pt(is_in_group, margin),
    }
}

fn convert_top(is_in_group: bool, top: Option<CssUnit>, margin_top: Option<CssUnit>) -> f32 {
    let rect_top = top.map(|u| position_css_unit_to_pt(is_in_group, u)).unwrap_or(0.0);
    match margin_top {
        None => rect_top,
        Some(margin) => rect_top + position_css_unit_to_pt(is_in_group, margin),
    }
}

fn convert_right(is_in_group: bool, rect_left: f32, width: Option<CssUnit>) -> f32 {
    match width {
        None => rect_left + 50.0,
        Some(width) => rect_left + position_css_unit_to_pt(is_in_group, width),
    }
}

fn convert_bottom(is_in_group: bool, rect_top: f32, height: Option<CssUnit>) -> f32 {
    match height {
        None => rect_top + 50.0,
        Some(height) => rect_top + position_css_unit_to_pt(is_in_group, height),
    }
}
