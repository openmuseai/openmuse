#![allow(non_snake_case)]

// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::{Document, SoftTextDocument};
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
// Java: import cn.wps.moffice.writer.core.WdCompatibilityMode
use crate::cn::wps::moffice::writer::core::WdCompatibilityMode::WdCompatibilityMode;
// Java: import cn.wps.moffice.writer.data.PLCSection
// Java: import cn.wps.moffice.writer.data.value.DxaColumn
use crate::cn::wps::moffice::writer::data::value::DxaColumn::DxaColumn;
// Java: import cn.wps.moffice.writer.data.value.PageSetup
use crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
// Java: import cn.wps.moffice.drawing.GRF
use drawing::cn::wps::moffice::drawing::GRF::GRF;
// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::IPos;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.ShapePos
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::RectF;

// Java: public class Tools
pub struct Tools;

impl Tools {
    // Java: public static int getMainCp(Document document, int cp)
    pub fn getMainCp(document: &dyn Document, cp: i32) -> i32 {
        match document.getType__on_IDocument() {
            // java case 0: return cp;
            0 => cp,
            // java case 1: footnote txt findFloor -> ref CP
            1 => {
                let Some(txt) = document.getFootnoteTxt() else {
                    return 0;
                };
                let txt = unsafe { &*txt };
                let Some(node) = txt.findFootnoteEndnoteTxtContainsCP(cp) else {
                    return 0;
                };
                let Some(reference) = node.getFootnoteEndnoteRef() else {
                    return 0;
                };
                reference.node.getCP()
            }
            // java case 2: header section -> main section start
            2 => {
                let Some(header) = document.getPLCHeader() else {
                    return 0;
                };
                let header = unsafe { &*header };
                let Some(section) = header.getSection(cp) else {
                    return 0;
                };
                let Some(main) = document.getMainDocument() else {
                    return 0;
                };
                let Some(sections) = main.getPLCSection() else {
                    return 0;
                };
                let sections = unsafe { &*sections };
                sections.getSectionStart(&section)
            }
            // java case 4: endnote txt findFloor -> ref CP
            4 => {
                let Some(txt) = document.getEndnoteTxt() else {
                    return 0;
                };
                let txt = unsafe { &*txt };
                let Some(node) = txt.findFootnoteEndnoteTxtContainsCP(cp) else {
                    return 0;
                };
                let Some(reference) = node.getFootnoteEndnoteRef() else {
                    return 0;
                };
                reference.node.getCP()
            }
            // java case 5: textbox shape anchor position
            5 => Self::textbox_anchor_cp(document, cp, true),
            // java case 6: header textbox, then getMainCp(headerDoc, anchor.position())
            6 => Self::textbox_anchor_cp(document, cp, false),
            _ => 0,
        }
    }

    fn textbox_anchor_cp(document: &dyn Document, cp: i32, main_anchor: bool) -> i32 {
        let Some(textbox) = document.getTextboxText() else {
            return 0;
        };
        let textbox = unsafe { &*textbox };
        let Some(node) = textbox.findTextboxByCP(cp) else {
            return 0;
        };
        let anchor_document: &dyn Document = if main_anchor {
            let Some(main) = document.getMainDocument() else {
                return 0;
            };
            main
        } else {
            let Some(text_document) = document.getTextDocument() else {
                return 0;
            };
            let Some(header) = text_document.getSubdocument(2) else {
                return 0;
            };
            unsafe { &*header }
        };
        let Some(anchors) = anchor_document.getShapeAnchors() else {
            return 0;
        };
        let anchors = unsafe { &*anchors };
        let Some(anchor) = anchors.findById(node.getShapeId()) else {
            return 0;
        };
        if main_anchor {
            anchor.getCP()
        } else {
            Self::getMainCp(anchor_document, anchor.getCP())
        }
    }

    // Java: public static PageSetup getPageSetup(Document document, int cp)
    pub fn getPageSetup(document: &dyn Document, cp: i32) -> PageSetup {
        // java return document.getTextDocument().getSectionProperty().getPageSetup(getMainCp(document, cp));
        let cp = Self::getMainCp(document, cp);
        match document.getTextDocument() {
            Some(text_document) => text_document.getPageSetup(cp),
            None => PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440),
        }
    }

    // Java: public static int getMainDocumentCoreWidth(Document document, int cp)
    pub fn getMainDocumentCoreWidth(document: &dyn Document, cp: i32) -> i32 {
        // java cp = getMainCp(document, cp);
        let cp = Self::getMainCp(document, cp);
        // java PageSetup pageSetup = document.getTextDocument().getSectionProperty().getPageSetup(cp);
        let page_setup = Self::getPageSetup(document, cp);
        // java int coreWidth = pageSetup.getWidth() - pageSetup.getMarginLeft() - pageSetup.getMarginRight();
        let mut core_width = page_setup.getWidth() - page_setup.getMarginLeft() - page_setup.getMarginRight();
        // java if (0 == document.getType())
        if document.getType__on_IDocument() == 0 {
            if let Some(sections) = document.getPLCSection() {
                let sections = unsafe { &*sections };
                if let Some(section_node) = sections.getSection(cp) {
                    // java PropertySet secPropSet = sectionNode.sepx;
                    let mut sec_prop_set = section_node.getPropertySet();
                    // java Revision revision = (Revision)secPropSet.getValue(727);
                    if let Some(revision) = sec_prop_set
                        .getValue(727)
                        .and_then(|value| value.as_any().downcast_ref::<Revision>())
                    {
                        if let Some(revision_prop) = revision.getProperty() {
                            // java secPropSet = PropertySet.merge(secPropSet, revision.getProperty());
                            sec_prop_set = PropertySet::merge(&sec_prop_set, &revision_prop);
                        }
                    }
                    // java if (secPropSet.getInt(680, 0) > 0)
                    if sec_prop_set.getInt__int_int(680, 0) > 0 {
                        core_width = Self::getMinColumnWidth(core_width, &sec_prop_set);
                    }
                }
            }
        }
        core_width
    }

    // Java: private static int getMinColumnWidth(int coreWidth, PropertySet secPropSet)
    pub fn getMinColumnWidth(mut core_width: i32, sec_prop_set: &PropertySet) -> i32 {
        // java DxaColumn[] dxaColumns = (DxaColumn[])secPropSet.getValue(721);
        let dxa_columns = sec_prop_set.getValue(721);
        // java int columns = secPropSet.getInt(680, 0) + 1;
        let columns = sec_prop_set.getInt__int_int(680, 0) + 1;
        if let Some(dxa_columns) = dxa_columns
            .as_ref()
            .and_then(|value| value.as_any().downcast_ref::<Vec<DxaColumn>>())
        {
            for column in dxa_columns {
                // java coreWidth = Math.min(coreWidth, dxaColumns[i].getWidth());
                core_width = core_width.min(column.getWidth());
            }
        } else if columns != 0 {
            // java int spaceBeforeColumns = secPropSet.getInt(692, 720);
            let space_before_columns = sec_prop_set.getInt__int_int(692, 720);
            // java coreWidth = (coreWidth - spaceBeforeColumns * (columns - 1)) / columns;
            core_width = (core_width - space_before_columns * (columns - 1)) / columns;
        }
        core_width
    }

    // Java: public static int getTextBoxCoreWidth(Document document, int cp)
    pub fn getTextBoxCoreWidth(document: &dyn Document, cp: i32) -> i32 {
        let Some(textbox) = document.getTextboxText() else {
            return 0;
        };
        let textbox = unsafe { &*textbox };
        // java if (textboxTextNode == null) return 0;
        let Some(node) = textbox.findTextboxByCP(cp) else {
            return 0;
        };
        let shape_id = node.getShapeId();
        let anchor_document: &dyn Document = if document.getType__on_IDocument() == 5 {
            let Some(main) = document.getMainDocument() else {
                return 0;
            };
            main
        } else {
            let Some(text_document) = document.getTextDocument() else {
                return 0;
            };
            let Some(header) = text_document.getSubdocument(2) else {
                return 0;
            };
            unsafe { &*header }
        };
        let Some(subdocument) = anchor_document.asAny().downcast_ref::<SubDocument>() else {
            return 0;
        };
        let Some(container) = subdocument.drawingContainer() else {
            return 0;
        };
        // java Shape shape = anchorDocument.getDrawingContainer().findShapeByID(shapeID);
        let found = container.findShapeByID(shape_id);
        let Some(shape) = found
            .as_ref()
            .and_then(|shape| shape.as_ref().downcast_ref::<Shape>())
        else {
            return 0;
        };
        // java float textMargin = textbox != null ? textbox.getMarginLeft() + textbox.getMarginRight() : 0;
        let text_margin = 0.0f32;
        // java return ShapeMetrics.transToLayout(getShapeWidth(shape) - textMargin);
        let width = Self::getShapeWidth(shape) - text_margin;
        Self::round_half_away(width * 20.0)
    }

    // Java: public static float getShapeWidth(Shape shape)
    fn getShapeWidth(shape: &Shape) -> f32 {
        let rect_width = Self::shape_rect_width(shape);
        // java shape.isChildShape() ? width / ShapeTool.getShapeScale(parent) : width
        if Self::is_child_shape(shape) {
            let scale = Self::shape_scale(shape);
            if scale == 0.0 {
                rect_width
            } else {
                rect_width / scale
            }
        } else {
            rect_width
        }
    }

    fn is_child_shape(shape: &Shape) -> bool {
        let Some(map) = shape.base.mProperty.as_ref() else {
            return false;
        };
        map.base
            .getValue(803)
            .as_ref()
            .and_then(|value| value.as_ref().downcast_ref::<GRF>())
            .is_some_and(|grf| grf.isChild())
    }

    fn shape_rect_width(shape: &Shape) -> f32 {
        let Some(map) = shape.base.mProperty.as_ref() else {
            return 0.0;
        };
        let pos_value = map.base.getValue(22);
        let Some(pos) = pos_value
            .as_ref()
            .and_then(|value| value.as_ref().downcast_ref::<ShapePos>())
        else {
            return 0.0;
        };
        IPos::getShapeRect(pos).width()
    }

    // Java: ShapeTool.getShapeScale walks parent GroupShape coord/layout widths.
    fn shape_scale(shape: &Shape) -> f32 {
        let Some(parent) = shape.getParent().as_ref().and_then(|parent| parent.downcast_ref::<Shape>()) else {
            return 1.0;
        };
        let coord = Self::rect_prop(parent, 733);
        let layout = Self::shape_rect(parent);
        match (coord, layout) {
            (Some(coord), Some(layout)) if layout.width() != 0.0 => {
                let scale_x = coord.width() / layout.width();
                if parent.getParent().is_some() {
                    scale_x * Self::shape_scale(parent)
                } else {
                    scale_x
                }
            }
            _ => 1.0,
        }
    }

    fn shape_rect(shape: &Shape) -> Option<RectF> {
        let map = shape.base.mProperty.as_ref()?;
        let pos = map.base.getValue(22)?;
        let pos = pos.as_ref().downcast_ref::<ShapePos>()?;
        Some(IPos::getShapeRect(pos))
    }

    fn rect_prop(shape: &Shape, id: i32) -> Option<RectF> {
        let map = shape.base.mProperty.as_ref()?;
        map.base.getValue(id)?.as_ref().downcast_ref::<RectF>().cloned()
    }

    fn round_half_away(value: f32) -> i32 {
        if value >= 0.0 {
            (value + 0.5) as i32
        } else {
            (value - 0.5) as i32
        }
    }

    pub fn compatibility_mode(document: &dyn Document) -> WdCompatibilityMode {
        document
            .getTextDocument()
            .map(|text_document| text_document.getCompatibilityMode())
            .unwrap_or(WdCompatibilityMode::wdWord2010)
    }

    pub fn document_property_boolean(document: &dyn Document, id: i32, default_value: bool) -> bool {
        document
            .getTextDocument()
            .map(|text_document| text_document.getProperty().getBoolean__int_boolean(id, default_value))
            .unwrap_or(default_value)
    }
}
