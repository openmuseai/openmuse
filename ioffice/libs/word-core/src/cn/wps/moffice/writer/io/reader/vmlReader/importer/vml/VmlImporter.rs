// Java: cn.wps.moffice.writer.io.reader.vmlReader.importer.vml.VmlImporter
#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::IDrawingContainer::IDrawingContainer;
use drawing::cn::wps::moffice::drawing::OLE::OLE;
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
use drawing::cn::wps::moffice::drawing::media::IMediaLib::IMediaLib;
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::MediaTypeEnum;
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use io_wps_io::cn::wps::io::xwpf::IVmlImporter::IVmlImporter;
use io_wps_io::cn::wps::io::xwpf::element_handler::XmlConvertHelper::BorrowedDocumentPart;
use io_wps_io::cn::wps::io::xwpf::element_handler::vml::ConvertHelper::ConvertHelper;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use crate::cn::wps::moffice::writer::data::field::KFieldType::KFieldType;
use crate::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;
use crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::helper::VmlHelper::VmlHelper;
use crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::producer::ShapeAttrConverter::ShapeAttrConverter;
use super::{
    GroupShapeImporter::GroupShapeImporter,
    LineImporter::LineImporter,
    NormalShapeImporter::NormalShapeImporter,
    RoundRectImporter::RoundRectImporter,
    ShapeCoreImporter::ShapeCoreImporter,
    ShapeImporter::ShapeImporter,
    ShapeTypeImporter::ShapeTypeImporter,
};

const REL_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

// Java: Shapes2Core.OLEObjectInfo
#[derive(Clone)]
pub struct OLEObjectInfo {
    pub mType: String,
    pub mProgID: Option<String>,
    pub mRid: Option<String>,
    pub mMediaId: i32,
    pub mPartName: Option<String>,
    pub mIsIcon: bool,
    pub mIsVba: bool,
    pub mVbaClsid: Option<String>,
    pub mPersistence: Option<String>,
}

// Java: public class VmlImporter extends ImporterBase implements IVmlImporter
pub struct VmlImporter {
    mDocument: *mut SubDocument,
    mPart: *const POIXMLDocumentPart,
    mRPrSet: MutablePropertySet,
    mCur: Option<Rc<RefCell<ShapeCoreImporter>>>,
    mGroups: Vec<Rc<RefCell<ShapeCoreImporter>>>,
    mShapes: Vec<Arc<Mutex<Shape>>>,
    mShapeID2OLEObjectInfo: HashMap<String, OLEObjectInfo>,
    mShape2ShapeID: HashMap<i32, String>,
    mode: i32,
    token: (),
}

impl VmlImporter {
    // Java: VmlImporter(documentImporter, shapes2Core, document, mode, xml07, xml03)
    pub fn create(document: *mut SubDocument, mode: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            mDocument: document,
            mPart: std::ptr::null(),
            mRPrSet: MutablePropertySet::new(),
            mCur: None,
            mGroups: Vec::new(),
            mShapes: Vec::new(),
            mShapeID2OLEObjectInfo: HashMap::new(),
            mShape2ShapeID: HashMap::new(),
            mode,
            token: (),
        }))
    }

    fn document(&mut self) -> &mut SubDocument {
        unsafe { &mut *self.mDocument }
    }

    fn part(&self) -> Option<&POIXMLDocumentPart> {
        if self.mPart.is_null() { None } else { Some(unsafe { &*self.mPart }) }
    }

    // Java: private void createCurrentImporter
    fn createCurrentImporter(&mut self, element_id: i32) {
        let parent = self.mGroups.last().map(|group| group.borrow().shape());
        let part = self.mPart;
        let document = self.mDocument;
        let mode = self.mode;
        let importer = match element_id {
            -1638324645 => ShapeTypeImporter::create(part, document, mode),
            -4156302 => RoundRectImporter::create(part, document, parent, mode),
            3321844 => LineImporter::create(part, document, parent, mode),
            3423314 | 3496420 | 100313435 => NormalShapeImporter::create(element_id, part, document, parent, mode),
            98629247 => GroupShapeImporter::create(part, document, parent, mode),
            109399969 => ShapeImporter::create(part, document, parent, mode),
            _ => return,
        };
        let importer = Rc::new(RefCell::new(importer));
        if element_id == 98629247 {
            self.mGroups.push(importer.clone());
        }
        self.mCur = Some(importer);
    }

    // Java: Shapes2Core.append → appendSingleShape, docx OLE branch.
    fn append_shapes(&mut self) {
        let has_ole = !self.mShapeID2OLEObjectInfo.is_empty();
        let shapes = std::mem::take(&mut self.mShapes);
        let ids = self.mShape2ShapeID.clone();
        let oles = self.mShapeID2OLEObjectInfo.clone();
        let rpr = self.mRPrSet.clone_mps();
        for shape in shapes {
            shape.lock().expect("shape lock").setIsVml(true);
            let key = shape.lock().expect("shape lock").getShapeID();
            let info = ids.get(&key).and_then(|id| oles.get(id)).cloned();
            let doc = unsafe { &mut *self.mDocument };
            if with_field(&shape, has_ole) {
                if let Some(info) = info.as_ref().filter(|info| info.mProgID.is_some()) {
                    append_inline_ole(doc, &shape, info, &rpr);
                    continue;
                }
            }
            append_without_field(doc, &shape, &rpr);
        }
        self.mShapes.clear();
        self.mShapeID2OLEObjectInfo.clear();
        self.mShape2ShapeID.clear();
    }
}

impl IImporterBase for VmlImporter {
    fn mode(&self) -> i32 { self.mode }
}

impl IVmlImporter for VmlImporter {
    fn getDocumentImporter(&self) -> &dyn Any { &self.token }

    // Java: public void onImportVmlStart(POIXMLDocumentPart part, MutablePropertySet rPrSet)
    fn onImportVmlStart(&mut self, part: &dyn Any, r_pr: &dyn Any) {
        self.mPart = part_ptr(part);
        if let Some(props) = r_pr.downcast_ref::<MutablePropertySet>() {
            self.mRPrSet = props.clone_mps();
        }
    }

    // Java: public void onImportVmlEnd()
    fn onImportVmlEnd(&mut self) {
        self.append_shapes();
    }

    // Java: public void onImportPictChildStart(int, Attributes)
    fn onImportPictChildStart(&mut self, element_id: i32, shape_attr: &dyn Any) {
        let top_level = self.mGroups.is_empty();
        self.createCurrentImporter(element_id);
        let Some(current) = self.mCur.clone() else { return; };
        current.borrow_mut().importShapeStart(element_id, shape_attr);
        let shape = current.borrow().shape();
        if let Some(vml_id) = ShapeAttrConverter::getVmlShapeId(shape_attr) {
            let key = shape.lock().expect("shape lock").getShapeID();
            self.mShape2ShapeID.insert(key, vml_id);
        }
        // Java: shapetype is not appended; grouped shapes are children.
        if element_id != -1638324645 && top_level {
            self.mShapes.push(shape);
        }
    }

    // Java: public void onImportPictChildEnd(int)
    fn onImportPictChildEnd(&mut self, element_id: i32) {
        if let Some(current) = self.mCur.clone() {
            current.borrow_mut().importShapeEnd();
        }
        if element_id == 98629247 {
            self.mGroups.pop();
        }
        self.mCur = self.mGroups.last().cloned();
    }

    // Java: public void onImportEGShapeElementsStart
    fn onImportEGShapeElementsStart(&mut self, element_id: i32, namespace_uri: &str, attributes: &dyn Any) {
        if let Some(current) = self.mCur.clone() {
            current.borrow_mut().importEGShapeElementsStart(element_id, namespace_uri, attributes);
        }
    }

    fn onImportTextboxEnd(&mut self, has_txbx_content: bool) {
        if let Some(current) = self.mCur.clone() {
            current.borrow_mut().importTextboxEnd(has_txbx_content);
        }
    }

    fn onImportWBackgroundStart(&mut self, _part: &dyn Any, _element_id: i32, _attr: &dyn Any) {}
    fn onImportVBackgroundStart(&mut self, _element_id: i32, _namespace_uri: &str, _attr: &dyn Any) {}
    fn onImportBackgroundEnd(&mut self, _element_id: i32, _namespace_uri: &str) {}
    fn onImportInk(&mut self, _ink_attr: &dyn Any) {}

    // Java: public void onImportOLEObject(Attributes)
    fn onImportOLEObject(&mut self, ole_attr: &dyn Any) {
        let Some(type_name) = ConvertHelper::getString4EmptyNamespace(ole_attr, "Type") else { return; };
        let Some(prog_id) = ConvertHelper::getString4EmptyNamespace(ole_attr, "ProgID") else { return; };
        let Some(shape_id) = ConvertHelper::getString4EmptyNamespace(ole_attr, "ShapeID") else { return; };
        let r_id = ConvertHelper::getString__Attributes_String_String(ole_attr, "id", REL_NS);
        let object_id = ConvertHelper::getString4EmptyNamespace(ole_attr, "ObjectID");
        let xml03 = self.mode == 3;
        if (xml03 || r_id.is_some()) && (!xml03 || object_id.is_some()) {
            let aspect = ConvertHelper::getString__Attributes_String_String(ole_attr, "DrawAspect", REL_NS);
            let is_icon = aspect.as_deref() == Some("Icon");
            let xml07 = self.mode == 1;
            let (media_id, part_name) = if xml03 {
                (0, None)
            } else if !xml07 {
                let Some(part) = self.part() else { return; };
                let Some(ole_part) = VmlHelper::getOleObject(r_id.as_deref(), part) else { return; };
                let part_name = ole_part.borrow().getPackagePart().as_ref().map(|package| package.getPartName().getName());
                let path = Platform::createTempFile("OLE_", ".bin").unwrap_or_default();
                let doc = self.document();
                let text = doc.getTextDocument();
                if text.is_null() { return; }
                let media_id = unsafe { IMediaLib::addMedia((*text).getMediaLib(), &path, &path, MediaTypeEnum::OLE) };
                (media_id, part_name)
            } else {
                (0, None)
            };
            self.mShapeID2OLEObjectInfo.insert(shape_id, OLEObjectInfo {
                mType: type_name,
                mProgID: Some(prog_id),
                mRid: r_id,
                mMediaId: media_id,
                mPartName: part_name,
                mIsIcon: is_icon,
                mIsVba: false,
                mVbaClsid: None,
                mPersistence: None,
            });
        }
    }

    // Java: public void onImportControlObject — ActiveX path records an OLE media id.
    fn onImportControlObject(&mut self, attr: &dyn Any) {
        if self.mode == 1 { return; }
        let Some(shape_id) = ConvertHelper::getString4EmptyNamespace(attr, "shapeid") else { return; };
        let Some(r_id) = ConvertHelper::getString__Attributes_String_String(attr, "id", REL_NS) else { return; };
        let Some(name) = ConvertHelper::getString(attr, "name") else { return; };
        let Some(part) = self.part() else { return; };
        let Some(control) = VmlHelper::getOleObject(Some(&r_id), part) else { return; };
        let part_name = control.borrow().getRelations().first().and_then(|part| {
            part.borrow().getPackagePart().as_ref().map(|package| package.getPartName().getName())
        });
        let path = Platform::createTempFile("OLE_", ".bin").unwrap_or_default();
        let doc = self.document();
        let text = doc.getTextDocument();
        if text.is_null() { return; }
        let media_id = unsafe { IMediaLib::addMedia((*text).getMediaLib(), &path, &path, MediaTypeEnum::OLE) };
        if media_id >= 0 {
            self.mShapeID2OLEObjectInfo.insert(shape_id, OLEObjectInfo {
                mType: String::new(),
                mProgID: Some(name),
                mRid: Some(r_id),
                mMediaId: media_id,
                mPartName: part_name,
                mIsIcon: false,
                mIsVba: true,
                mVbaClsid: None,
                mPersistence: None,
            });
        }
    }
}

fn part_ptr(part: &dyn Any) -> *const POIXMLDocumentPart {
    if let Some(borrowed) = part.downcast_ref::<BorrowedDocumentPart>() {
        return unsafe { borrowed.get() as *const POIXMLDocumentPart };
    }
    if let Some(part) = part.downcast_ref::<POIXMLDocumentPart>() {
        return part as *const POIXMLDocumentPart;
    }
    std::ptr::null()
}

// Java: Shapes2Core.appendSingleShape predicate.
fn with_field(shape: &Arc<Mutex<Shape>>, has_ole: bool) -> bool {
    let mut shape = shape.lock().expect("shape lock");
    let has_no_obj = !has_ole;
    let in_dgg = shape.isInDggInfo();
    let art = shape.isArtText();
    let picture = shape.getPicture().is_some();
    let horz = shape.getHorzRule();
    let typ = shape.getShapeType();
    let text = shape.getTextObj().is_some();
    let diagram = shape.getDiagramDrawingML().is_some();
    let ink = shape.getInkModel().is_some();
    let picture_in_field = shape.getShapePos().as_ref()
        .and_then(|pos| pos.downcast_ref::<ShapePos>())
        .is_some_and(|pos| pos.getRotation() % 90.0 != 0.0);
    !diagram && !ink && (!in_dgg || !has_no_obj) && !art
        && (!picture || picture_in_field || !has_no_obj) && !horz
        && (typ != 75 || text)
}

// Java: appendShape2CoreWithoutField — control char then addTopLevelShape.
fn append_without_field(doc: &mut SubDocument, shape: &Arc<Mutex<Shape>>, rpr: &MutablePropertySet) {
    let anchored = shape.lock().expect("shape lock").isInDggInfo();
    let control = if anchored { '\u{0008}' } else { '\u{0001}' };
    let mut props = rpr.clone_mps();
    let cp = IOHelper::appendControlChar(doc, control, &mut props);
    IOHelper::addTopLevelShape(doc, cp, shape.clone());
}

// Java: Shapes2Core4Docx.appendInlineOLE2CoreWithField
fn append_inline_ole(doc: &mut SubDocument, pic: &Arc<Mutex<Shape>>, info: &OLEObjectInfo, rpr: &MutablePropertySet) {
    append_field_char(doc, '\u{0013}', rpr, info.mIsIcon);
    let prog = info.mProgID.clone().unwrap_or_default();
    let text = format!(" {} {} ", KFieldType::FieldEmbed.toString(), prog);
    append_string(doc, &text, rpr);
    let ole_shape = {
        let drawing = doc.getDrawingContainer().expect("drawingContainer");
        IOHelper::createShape(drawing, false).into_shared()
    };
    {
        let mut ole = OLE::new_with_id(info.mMediaId);
        if let Some(prog) = &info.mProgID { ole.setProgId(prog.clone()); }
        ole.setIsIcon(info.mIsIcon);
        ole.setIsVba(info.mIsVba);
        if let Some(clsid) = &info.mVbaClsid { ole.setVbaCLSID(clsid.clone()); }
        if let Some(persistence) = &info.mPersistence { ole.setVbaPersistence(persistence.clone()); }
        ole_shape.lock().expect("ole shape").setOLE(Some(Arc::new(ole)));
    }
    if let Some(drawing) = doc.getDrawingContainer() {
        drawing.addShape(Some(ole_shape));
    }
    append_field_char(doc, '\u{0014}', rpr, info.mIsIcon);
    let mut props = rpr.clone_mps();
    let cp = IOHelper::appendControlChar(doc, '\u{0001}', &mut props);
    IOHelper::addTopLevelShape(doc, cp, pic.clone());
    append_field_char(doc, '\u{0015}', rpr, info.mIsIcon);
}

// Java Shapes2Core.appendToPLCField shares each control character's CP with
// the corresponding PLCField.FieldNode. Keep the product VML OLE path on that
// same model so FieldFilter can suppress field instructions during layout.
fn append_field_char(doc: &mut SubDocument, ch: char, rpr: &MutablePropertySet, is_icon: bool) {
    let mut props = rpr.clone_mps();
    let cp = IOHelper::appendControlChar(doc, ch, &mut props);
    let mut node = doc.getPLCFieldMut().expect("PLCField").addFieldChar__int(cp);
    node.setFieldChar(ch as i32);
    match ch {
        '\u{0013}' => node.setFieldType(KFieldType::FieldEmbed.getFlt()),
        '\u{0014}' if !is_icon => node.setFieldType(KFieldType::FieldRefOmitted.getFlt()),
        '\u{0015}' => {
            let has_separator = node.getPrev().is_some_and(|prev| prev.isFieldSeperator());
            node.setFieldFlags(if has_separator { 128 } else { 0 });
        }
        _ => {}
    }
}

fn append_string(doc: &mut SubDocument, text: &str, rpr: &MutablePropertySet) {
    let mut props = rpr.clone_mps();
    let frozen = props.freeze();
    CoreHelper::appendText__string_ps(Some(doc), Some(text), Some(&frozen));
}
