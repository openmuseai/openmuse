// Java: cn.wps.moffice.writer.io.reader.vmlReader.importer.vml.GroupShapeImporter
#![allow(non_snake_case)]

use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::Shape::Shape;
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;

use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use super::ShapeCoreImporter::{ShapeCoreImporter, VmlShapeKind};

// Java: class GroupShapeImporter extends ShapeCoreImporter
pub struct GroupShapeImporter;

impl GroupShapeImporter {
    pub fn create(part: *const POIXMLDocumentPart, document: *mut SubDocument, parent: Option<Arc<Mutex<Shape>>>, mode: i32) -> ShapeCoreImporter {
        ShapeCoreImporter::new(VmlShapeKind::Group, 98629247, part, document, parent, mode)
    }
}
