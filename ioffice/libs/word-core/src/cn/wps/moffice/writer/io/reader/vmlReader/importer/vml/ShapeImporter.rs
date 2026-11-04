// Java: cn.wps.moffice.writer.io.reader.vmlReader.importer.vml.ShapeImporter
#![allow(non_snake_case)]

use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::Shape::Shape;
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;

use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use super::ShapeCoreImporter::{ShapeCoreImporter, VmlShapeKind};

// Java: public class ShapeImporter extends CustomShapeBaseImporter
pub struct ShapeImporter;

impl ShapeImporter {
    // Java: ShapeImporter(part, document, parent, mode, xml07, xml03)
    pub fn create(part: *const POIXMLDocumentPart, document: *mut SubDocument, parent: Option<Arc<Mutex<Shape>>>, mode: i32) -> ShapeCoreImporter {
        ShapeCoreImporter::new(VmlShapeKind::Shape, 109399969, part, document, parent, mode)
    }
}
