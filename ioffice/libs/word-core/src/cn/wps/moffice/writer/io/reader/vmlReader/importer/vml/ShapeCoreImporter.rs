// Java: cn.wps.moffice.writer.io.reader.vmlReader.importer.vml.ShapeCoreImporter
#![allow(non_snake_case)]

use std::any::Any;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::GRF::GRF;
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
use drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
use drawing::cn::wps::moffice::drawing::diagram::Diagram::Diagram;
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use io_wps_io::cn::wps::io::xwpf::element_handler::vml::ConvertHelper::ConvertHelper;
use util::cn::wps::graphics::RectF::RectF;

use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;
use crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::helper::VmlStyle::{is_anchored, VmlStyle};
use crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::producer::ImageDataConverter::ImageDataConverter;
use crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::producer::ShapeAttrConverter::ShapeAttrConverter;
use crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::producer::ShapePosConverter::ShapePosConverter;

// Java: private static int sZIndexForShapeInGroup
static S_Z_INDEX_FOR_SHAPE_IN_GROUP: AtomicI32 = AtomicI32::new(0);

/// Java subclass chosen by `VmlImporter.createCurrentImporter`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VmlShapeKind {
    ShapeType,
    RoundRect,
    Line,
    Normal,
    Group,
    Shape,
}

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public abstract class ShapeCoreImporter
pub struct ShapeCoreImporter {
    pub kind: VmlShapeKind,
    pub element_id: i32,
    pub mShape: Option<Arc<Mutex<Shape>>>,
    pub mDocument: *mut SubDocument,
    pub mPart: *const POIXMLDocumentPart,
    pub mCss: Option<VmlStyle>,
    pub mIsAnchored: bool,
    pub mIsGroupShape: bool,
    pub mParent: Option<Arc<Mutex<Shape>>>,
    pub mHasTextbox: bool,
    pub mode: i32,
    // Java: ShapeBaseImporter.mStrokeConverter + VMLContext stroke attributes.
    stroke_child_present: bool,
    stroke_child_on: Option<bool>,
    stroke_child_weight: Option<String>,
    shape_stroked: Option<bool>,
    shape_stroke_weight: Option<String>,
}

impl ShapeCoreImporter {
    // Java: ShapeCoreImporter(part, document, parent, mode, ...)
    pub fn new(kind: VmlShapeKind, element_id: i32, part: *const POIXMLDocumentPart, document: *mut SubDocument, parent: Option<Arc<Mutex<Shape>>>, mode: i32) -> Self {
        Self {
            kind,
            element_id,
            mShape: None,
            mDocument: document,
            mPart: part,
            mCss: None,
            mIsAnchored: false,
            mIsGroupShape: kind == VmlShapeKind::Group,
            mParent: parent,
            mHasTextbox: false,
            mode,
            stroke_child_present: false,
            stroke_child_on: None,
            stroke_child_weight: None,
            shape_stroked: None,
            shape_stroke_weight: None,
        }
    }

    pub fn shape(&self) -> Arc<Mutex<Shape>> {
        self.mShape.clone().expect("mShape")
    }

    // Java: protected Shape importShapeStart(int, Attributes)
    pub fn importShapeStart(&mut self, element_id: i32, shape_attr: &dyn Any) {
        self.element_id = element_id;
        self.newShapeInstance();
        self.initCssStyle(shape_attr);
        // Java: ShapeAttrConverter.convertAGAllShapeAttributes(shapeAttr, mVmlContext).
        self.shape_stroked = ConvertHelper::toBoolean4STTrueFalse(shape_attr, "stroked");
        self.shape_stroke_weight = ConvertHelper::getString4EmptyNamespace(shape_attr, "strokeweight");
        self.initVMLContext();
        self.setShapeId(shape_attr);
        self.setGRF();
        self.setVisibility();
        self.setShapePos(shape_attr);
        self.setShapeType(shape_attr);
        self.setZIndex();
        {
            let shape = self.shape();
            let mut shape = shape.lock().expect("shape lock");
            ShapeAttrConverter::setAlt(shape_attr, &mut shape);
            ShapeAttrConverter::setInDggInfo(self.mIsAnchored, &mut shape);
        }
        if self.kind == VmlShapeKind::Group {
            self.setCoordBound(shape_attr);
            self.setEditAsType(shape_attr);
        }
        if self.kind == VmlShapeKind::Shape {
            self.import_shape_coord(shape_attr);
        }
    }

    // Java: protected void newShapeInstance
    fn newShapeInstance(&mut self) {
        let doc = unsafe { &mut *self.mDocument };
        let drawing = doc.getDrawingContainer().expect("drawingContainer");
        let shape = IOHelper::createShape(drawing, self.mIsGroupShape);
        let shape = shape.into_shared();
        shape.lock().expect("shape lock").setIsFromDml(false);
        self.mShape = Some(shape);
    }

    // Java: private void initCssStyle
    fn initCssStyle(&mut self, shape_attr: &dyn Any) {
        if let Some(style) = ConvertHelper::getString4EmptyNamespace(shape_attr, "style") {
            self.mCss = Some(VmlStyle::new(&style));
        }
    }

    // Java: private void initVMLContext
    fn initVMLContext(&mut self) {
        self.mIsAnchored = is_anchored(self.mCss.as_ref());
    }

    fn setShapeId(&mut self, shape_attr: &dyn Any) {
        let doc = unsafe { &mut *self.mDocument };
        let doc_type = doc.getType();
        let drawing = doc.getDrawingContainer().expect("drawingContainer");
        let shape = self.shape();
        let mut shape = shape.lock().expect("shape lock");
        ShapeAttrConverter::setShapeId(shape_attr, doc_type, self.mParent.is_some(), drawing, &mut shape);
    }

    // Java: protected void setGRF
    fn setGRF(&mut self) {
        let mut grf = GRF::new();
        let is_group = self.mIsGroupShape;
        grf.setGroup(is_group);
        grf.setChild(self.mParent.is_some());
        grf.setPatriarch(false);
        grf.setDeleted(false);
        grf.setHaveMaster(false);
        grf.setConnector(false);
        grf.setHaveAnchor(true);
        grf.setBackground(false);
        grf.setHaveSpt(!is_group);
        self.setFlip(&mut grf);
        grf.setOleShape(false);
        let shape = self.shape();
        shape.lock().expect("shape lock").setGRF(Some(Arc::new(grf)));
    }

    // Java: VmlCssStyleConverter.setFlip
    fn setFlip(&self, grf: &mut GRF) {
        let Some(flip) = self.mCss.as_ref().and_then(|s| s.flip()) else { return; };
        match flip.to_ascii_lowercase().as_str() {
            "x" => grf.setFlipH(true),
            "y" => grf.setFlipV(true),
            "xy" | "yx" => { grf.setFlipH(true); grf.setFlipV(true); }
            _ => {}
        }
    }

    // Java: protected void setVisibility
    fn setVisibility(&mut self) {
        let hidden = self.mCss.as_ref().is_some_and(|s| s.is_hidden());
        self.shape().lock().expect("shape lock").setHidden(hidden);
    }

    // Java: protected void setShapePos
    fn setShapePos(&mut self, shape_attr: &dyn Any) {
        let mut shape_pos = ShapePos::new();
        ShapePosConverter::setWrapDistances(self.mCss.as_ref(), &mut shape_pos);
        let shape = self.shape();
        let (flip_h, flip_v) = {
            let mut shape = shape.lock().expect("shape lock");
            (shape.isFlipH(), shape.isFlipV())
        };
        ShapePosConverter::setRotation(self.mCss.as_ref(), &mut shape_pos, flip_h, flip_v);
        ShapeAttrConverter::setLayoutInCell(shape_attr, &mut shape_pos);
        ShapeAttrConverter::setAllowOverlap(shape_attr, &mut shape_pos);
        let rect = self.getShapePosRect(shape_attr, !self.mIsAnchored);
        shape_pos.setShapeRect(Some(Arc::new(rect)));
        shape_pos.setScaleH(1.0);
        shape_pos.setScaleV(1.0);
        if self.mIsAnchored {
            ShapePosConverter::setShapePosAnchor(&mut shape_pos);
        } else {
            shape.lock().expect("shape lock").setInlineShape(true);
        }
        shape.lock().expect("shape lock").setShapePos(Some(Arc::new(shape_pos)));
        let shape = self.shape();
        let mut locked = shape.lock().expect("shape lock");
        ShapeAttrConverter::setWrapEdited(self.mCss.as_ref(), &mut locked);
    }

    // Java: protected RectF getShapePosRect — LineImporter overrides with from/to.
    fn getShapePosRect(&self, shape_attr: &dyn Any, inline: bool) -> RectF {
        if self.kind == VmlShapeKind::Line {
            return line_rect(shape_attr, inline);
        }
        ShapePosConverter::getShapePosRect(self.mParent.is_some(), self.mCss.as_ref(), inline)
    }

    // Java: protected abstract void setShapeType
    fn setShapeType(&mut self, shape_attr: &dyn Any) {
        let shape = self.shape();
        let mut shape = shape.lock().expect("shape lock");
        match self.kind {
            VmlShapeKind::Group => shape.setShapeType(0),
            VmlShapeKind::Line => {
                if shape.getShapeTypeObj().is_none() { shape.setShapeType(20); }
            }
            VmlShapeKind::RoundRect => {
                if shape.getShapeTypeObj().is_none() { shape.setShapeType(2); }
            }
            VmlShapeKind::Normal => {
                if shape.getShapeTypeObj().is_none() {
                    let typ = match self.element_id {
                        3423314 => 3,
                        3496420 => 1,
                        100313435 => 75,
                        561938880 => 0,
                        _ => 0,
                    };
                    shape.setShapeType(typ);
                }
            }
            VmlShapeKind::ShapeType | VmlShapeKind::Shape => {
                // Java: CustomShapeBaseImporter.setShapeType — spt, 75 becomes 204.
                if shape.getShapeTypeObj().is_none() {
                    let mut spt = ConvertHelper::getFloat(shape_attr, "spt", "urn:schemas-microsoft-com:office:office")
                        .map(|v| v as i32)
                        .unwrap_or(0);
                    if spt == 75 { spt = 204; }
                    shape.setShapeType(spt);
                }
                if self.kind == VmlShapeKind::Shape {
                    // Java: ShapeImporter.setShapeType — type="#_x0000_tNNN" when type is null/0/75.
                    let current = shape.getShapeTypeObj();
                    if current.is_none() || current == Some(0) || current == Some(75) {
                        shape.setShapeType(shape_type_from_attr(shape_attr));
                    }
                }
            }
        }
    }

    // Java: protected void setZIndex
    fn setZIndex(&mut self) {
        let mut z_index = self.z_index();
        let shape = self.shape();
        let mut shape = shape.lock().expect("shape lock");
        if z_index < 0 {
            z_index -= i32::MAX;
            shape.setBehindDocument(true);
        }
        shape.setZIndex(z_index);
    }

    fn z_index(&self) -> i32 {
        if self.mParent.is_some() {
            S_Z_INDEX_FOR_SHAPE_IN_GROUP.fetch_add(1, Ordering::Relaxed) + 1
        } else {
            self.mCss.as_ref().and_then(|s| s.z_index()).unwrap_or(0)
        }
    }

    // Java: GroupShapeImporter.setCoordBound — default origin 0 and size 1000.
    fn setCoordBound(&mut self, shape_attr: &dyn Any) {
        let (ox, oy) = pair_attr(shape_attr, "coordorigin").unwrap_or((0.0, 0.0));
        let (w, h) = pair_attr(shape_attr, "coordsize").unwrap_or((1000.0, 1000.0));
        let rect = RectF::new_with(ox, oy, ox + w, oy + h);
        self.shape().lock().expect("shape lock").setPropertyValue(733, Some(Arc::new(rect)));
    }

    // Java: GroupShapeImporter.setEditAsType
    fn setEditAsType(&mut self, shape_attr: &dyn Any) {
        let edit_as = ConvertHelper::getString4EmptyNamespace(shape_attr, "editas");
        let typ = match edit_as.as_deref() {
            None => 4095,
            Some("canvas") => 0,
            Some("orgchart") => 1,
            Some("radial") => 2,
            Some("cycle") => 3,
            Some("stacked") => 4,
            Some("venn") => 5,
            Some("bullseye") => 6,
            Some(_) => 4095,
        };
        if typ != 4095 {
            let mut diagram = Diagram::new();
            diagram.setDiagramType(typ);
            self.shape().lock().expect("shape lock").setDiagram(Some(Arc::new(diagram)));
        }
    }

    // Java: ShapeImporter records coordorigin/coordsize. Missing values stay at the VML default 0 / 1000.
    fn import_shape_coord(&mut self, shape_attr: &dyn Any) {
        if ConvertHelper::getString4EmptyNamespace(shape_attr, "coordorigin").is_none()
            && ConvertHelper::getString4EmptyNamespace(shape_attr, "coordsize").is_none()
        {
            return;
        }
        self.setCoordBound(shape_attr);
    }

    // Java: public void importEGShapeElementsStart
    pub fn importEGShapeElementsStart(&mut self, element_id: i32, namespace_uri: &str, attributes: &dyn Any) {
        match element_id {
            -1417835138 => self.importTextbox(),
            -1296358048 => { self.shape().lock().expect("shape lock").setAnchorLock(true); }
            -877694075 => self.importImageData(attributes),
            // Java: ShapeBaseImporter.importEGShapeElementsStart creates StrokeProducer.
            -891980232 => {
                self.stroke_child_present = true;
                self.stroke_child_on = ConvertHelper::toBoolean4STTrueFalse(attributes, "on");
                self.stroke_child_weight = ConvertHelper::getString4EmptyNamespace(attributes, "weight");
            }
            3143043 if namespace_uri == "urn:schemas-microsoft-com:vml" => {}
            3327275 | 3657802 | 548646960 => {}
            _ => {}
        }
    }

    // Java: private void importTextbox — node + separator bookkeeping. Property fill is TextboxVmlImporter.
    fn importTextbox(&mut self) {
        let doc = unsafe { &*self.mDocument };
        let textbox = IOHelper::getTextboxDocument(doc);
        if !self.mHasTextbox {
            let shape = self.shape();
            let shape = shape.lock().expect("shape lock");
            let _node = IOHelper::addTextboxTextNode(textbox, &shape);
        }
        self.mHasTextbox = true;
    }

    // Java: protected void importImageData
    fn importImageData(&mut self, image_attr: &dyn Any) {
        let doc = unsafe { &mut *self.mDocument };
        let text_document = doc.getTextDocument();
        if text_document.is_null() { return; }
        let text_document = unsafe { &mut *text_document };
        let part = if self.mPart.is_null() { None } else { Some(unsafe { &*self.mPart }) };
        let shape = self.shape();
        let old_type = shape.lock().expect("shape lock").getShapeTypeObj();
        let picture = ImageDataConverter::getPicture(part, text_document, &shape, image_attr, self.mode);
        if let Some(picture) = picture {
            // Java: this.mShape.setPicture(picture) keeps the same mutable
            // Picture that deferred BlipDocxInfo later updates with mediaID.
            shape.lock().expect("shape lock").setPicture(Some(Arc::new(Mutex::new(picture))));
            self.correctShapeType(old_type);
        }
    }

    // Java: ShapeImporter.correctShapeType
    fn correctShapeType(&mut self, old_type: Option<i32>) {
        if self.kind != VmlShapeKind::Shape { return; }
        if old_type.is_none() || old_type == Some(0) || old_type == Some(75) {
            let shape = self.shape();
            let mut shape = shape.lock().expect("shape lock");
            if shape.getPicture().is_some() {
                shape.setShapeType(204);
            } else {
                shape.setShapeType(75);
            }
        }
    }

    // Java: public void importShapeEnd
    pub fn importShapeEnd(&mut self) {
        self.importStrokeData();
        if self.mHasTextbox {
            self.mHasTextbox = false;
            let doc = unsafe { &*self.mDocument };
            let textbox = IOHelper::getTextboxDocument(doc);
            IOHelper::addTextboxSeparator(textbox);
        }
        self.addChildShape();
    }

    // Java: ShapeBaseImporter.importStrokeData → StrokeProducer.convertLineStroke.
    fn importStrokeData(&mut self) {
        let shape = self.shape();
        let mut shape = shape.lock().expect("shape lock");
        let existing = shape.getLineProperty();
        // Java creates a default stroke for non-picture shapes, while a picture
        // (type 204) receives one only when a stroke child or parent line exists.
        if existing.is_none() && !self.stroke_child_present && shape.getShapeType() == 204 {
            return;
        }
        let mut line = existing
            .as_ref()
            .and_then(|value| value.downcast_ref::<LineProperty>())
            .map(|line| LineProperty { base: line.base.clone() })
            .unwrap_or_else(LineProperty::new);
        // Java: StrokeProducer.setLineOn delegates child `on` then shape `stroked`.
        if let Some(on) = self.stroke_child_on.or(self.shape_stroked) {
            line.setfLine(on);
        }
        // Java: StrokeBaseProducer.setLineWeight; LineProperty defaults to 0.75 pt.
        if let Some(width) = stroke_weight_pt(self.stroke_child_weight.as_deref().or(self.shape_stroke_weight.as_deref())) {
            line.setWidth(width);
        }
        shape.setLineProperty(Some(Arc::new(line)));
    }

    // Java: private void addChildShape
    fn addChildShape(&mut self) {
        let Some(parent) = self.mParent.clone() else { return; };
        let shape = self.shape();
        if self.mode == 3 && self.mIsGroupShape && shape.lock().expect("shape lock").getChildCount() == 0 {
            return;
        }
        parent.lock().expect("parent lock").addChildShape(shape);
    }

    // Java: public void importTextboxEnd
    pub fn importTextboxEnd(&mut self, _has_txbx_content: bool) {}

    pub fn is_anchored(&self) -> bool { self.mIsAnchored }
}

fn shape_type_from_attr(shape_attr: &dyn Any) -> i32 {
    // Java: ShapeImporter.toShapeType — "#_x0000_t" + id, else 0.
    let Some(typ) = ConvertHelper::getString4EmptyNamespace(shape_attr, "type") else { return 0; };
    let typ = typ.trim();
    let Some(id) = typ.strip_prefix("#_x0000_t") else { return 0; };
    if id.is_empty() { return 0; }
    id.parse::<i32>().unwrap_or(0)
}

// Java: StrokeWeight.toPt. Absent weight leaves LineProperty's 0.75 pt default.
fn stroke_weight_pt(raw: Option<&str>) -> Option<f32> {
    let raw = raw?.trim();
    let split = raw.find(|ch: char| ch.is_ascii_alphabetic()).unwrap_or(raw.len());
    let (magnitude, unit) = raw.split_at(split);
    let magnitude: f32 = magnitude.trim().parse().ok()?;
    match unit.trim().to_ascii_lowercase().as_str() {
        "pt" => Some(magnitude),
        "mm" => Some(magnitude * 72.0 / 25.4),
        "cm" => Some(magnitude * 72.0 / 2.54),
        "in" => Some(magnitude * 72.0),
        "pc" | "pi" => Some(magnitude * 12.0),
        "" => Some(magnitude / 12700.0),
        _ => None,
    }
}

fn pair_attr(shape_attr: &dyn Any, name: &str) -> Option<(f32, f32)> {
    let raw = ConvertHelper::getString4EmptyNamespace(shape_attr, name)?;
    let mut parts = raw.split(',');
    let x = parts.next()?.trim().parse::<f32>().ok()?;
    let y = parts.next()?.trim().parse::<f32>().ok()?;
    Some((x, y))
}

fn line_rect(shape_attr: &dyn Any, inline: bool) -> RectF {
    // Java: LineImporter.getShapePosRect from "from"/"to".
    let from = pair_attr(shape_attr, "from");
    let to = pair_attr(shape_attr, "to");
    let left = from.map(|p| p.0).unwrap_or(0.0);
    let top = from.map(|p| p.1).unwrap_or(0.0);
    let mut right = to.map(|p| p.0).unwrap_or(0.0);
    let mut bottom = to.map(|p| p.1).unwrap_or(0.0);
    if inline {
        right += left.abs();
        bottom += top.abs();
        RectF::new_with(0.0, 0.0, right, bottom)
    } else {
        RectF::new_with(left, top, right, bottom)
    }
}
