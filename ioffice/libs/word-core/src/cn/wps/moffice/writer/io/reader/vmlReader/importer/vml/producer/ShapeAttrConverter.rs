// Java: cn.wps.moffice.writer.io.reader.vmlReader.importer.vml.producer.ShapeAttrConverter
#![allow(non_snake_case)]

use std::any::Any;
use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::IDrawingContainer::IDrawingContainer;
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
use io_wps_io::cn::wps::io::xwpf::element_handler::vml::ConvertHelper::ConvertHelper;

use crate::cn::wps::moffice::writer::io::reader::docxReader::helper::ShapeIDTool::ShapeIDTool;
use crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::helper::VmlHelper::VmlHelper;
use crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::helper::VmlStyle::VmlStyle;

const OFFICE_NS: &str = "urn:schemas-microsoft-com:office:office";

// Java: public class ShapeAttrConverter
pub struct ShapeAttrConverter;

impl ShapeAttrConverter {
    // Java: public static void setShapeId(Attributes, int, VMLContext, Shape)
    pub fn setShapeId(shape_attr: &dyn Any, doc_type: i32, in_group: bool, drawing: &mut dyn IDrawingContainer, shape: &mut Shape) {
        // Java: Integer id = getId(...)
        let id = Self::get_id(shape_attr, doc_type, in_group, drawing).unwrap_or(0);
        // Java: shape.setShapeID(id);
        shape.setShapeID(id);
    }

    // Java: private static Integer getId(...)
    fn get_id(shape_attr: &dyn Any, doc_type: i32, in_group: bool, drawing: &mut dyn IDrawingContainer) -> Option<i32> {
        // Java: if (vmlContext.mIsInGroupShape) return ShapeIDTool.getNewID(drawingContainer);
        if in_group {
            return ShapeIDTool::getNewID__idrawingcontainer(Some(drawing));
        }
        // Java: String vmlShapeId = getVmlShapeId(shapeAttr);
        let vml_shape_id = Self::getVmlShapeId(shape_attr);
        // Java: if (!VmlHelper.stringHasValue(vmlShapeId)) return ShapeIDTool.getNewID(drawingContainer);
        if !VmlHelper::stringHasValue(vml_shape_id.as_deref()) {
            return ShapeIDTool::getNewID__idrawingcontainer(Some(drawing));
        }
        let vml_shape_id = vml_shape_id.unwrap_or_default();
        // Java: Integer shapeId = ShapeIDTool.getID(vmlShapeId, docType);
        match ShapeIDTool::getID(&vml_shape_id, doc_type) {
            None => ShapeIDTool::getNewID__string_idrawingcontainer(&vml_shape_id, drawing),
            Some(shape_id) => {
                // Java: boolean isExistence = null != drawingContainer.findShapeByID(shapeId);
                if drawing.findShapeByID(shape_id).is_some() {
                    ShapeIDTool::getNewID__idrawingcontainer(Some(drawing))
                } else {
                    Some(shape_id)
                }
            }
        }
    }

    // Java: public static String getVmlShapeId(Attributes shapeAttr)
    pub fn getVmlShapeId(shape_attr: &dyn Any) -> Option<String> {
        // Java: return ConvertHelper.getString4EmptyNamespace(shapeAttr, "id");
        ConvertHelper::getString4EmptyNamespace(shape_attr, "id")
    }

    // Java: public static void setAllowOverlap(Attributes, IPos)
    pub fn setAllowOverlap(shape_attr: &dyn Any, shape_pos: &mut ShapePos) {
        // Java: Boolean isAllowOverlap = ConvertHelper.toBoolean4STTrueFalse(..., "allowoverlap", office);
        if let Some(allow) = ConvertHelper::toBoolean4STTrueFalse__Attributes_String_String(shape_attr, "allowoverlap", OFFICE_NS) {
            shape_pos.setAllowOverlap(allow);
        }
    }

    // Java: public static void setLayoutInCell(Attributes, IPos)
    pub fn setLayoutInCell(shape_attr: &dyn Any, shape_pos: &mut ShapePos) {
        if let Some(allow) = ConvertHelper::toBoolean4STTrueFalse__Attributes_String_String(shape_attr, "allowincell", OFFICE_NS) {
            shape_pos.setLayoutInCell(allow);
        }
    }

    // Java: public static void setWrapEdited(CssStyle, Shape)
    pub fn setWrapEdited(style: Option<&VmlStyle>, shape: &mut Shape) {
        // Java: boolean isWrapEdited = null == wrapEdited ? false : wrapEdited;
        let edited = style.and_then(|s| s.wrap_edited()).unwrap_or(false);
        shape.setEditedWrap(edited);
    }

    // Java: public static void setAlt(Attributes, Shape)
    pub fn setAlt(shape_attr: &dyn Any, shape: &mut Shape) {
        if let Some(alt) = ConvertHelper::getString4EmptyNamespace(shape_attr, "alt") {
            shape.setDecription(alt);
        }
    }

    // Java: public static void setInDggInfo(boolean isAnchored, Shape)
    pub fn setInDggInfo(is_anchored: bool, shape: &mut Shape) {
        shape.setInDggInfo(is_anchored);
    }

    // Java: public static void convertAGAllShapeAttributes — stored for later fill/stroke.
    pub fn filled(shape_attr: &dyn Any) -> Option<bool> {
        ConvertHelper::toBoolean4STTrueFalse(shape_attr, "filled")
    }
}

pub fn lock_shape(shape: &Arc<Mutex<Shape>>) -> std::sync::MutexGuard<'_, Shape> {
    shape.lock().expect("shape lock")
}
