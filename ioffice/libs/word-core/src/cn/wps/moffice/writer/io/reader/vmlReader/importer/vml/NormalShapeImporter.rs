// Java: cn.wps.moffice.writer.io.reader.vmlReader.importer.vml.NormalShapeImporter
#![allow(non_snake_case)]

use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::Shape::Shape;
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;

use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use super::ShapeCoreImporter::{ShapeCoreImporter, VmlShapeKind};

// Java: class NormalShapeImporter extends ShapeBaseImporter
pub struct NormalShapeImporter;

impl NormalShapeImporter {
    pub fn create(element_id: i32, part: *const POIXMLDocumentPart, document: *mut SubDocument, parent: Option<Arc<Mutex<Shape>>>, mode: i32) -> ShapeCoreImporter {
        ShapeCoreImporter::new(VmlShapeKind::Normal, element_id, part, document, parent, mode)
    }
}
