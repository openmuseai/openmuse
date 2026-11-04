// Java: cn.wps.moffice.writer.io.reader.vmlReader.importer.vml.ShapeTypeImporter
#![allow(non_snake_case)]

use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;

use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use super::ShapeCoreImporter::{ShapeCoreImporter, VmlShapeKind};

// Java: ShapeTypeImporter does not append a shape. It records the shapetype definition.
pub struct ShapeTypeImporter;

impl ShapeTypeImporter {
    pub fn create(part: *const POIXMLDocumentPart, document: *mut SubDocument, mode: i32) -> ShapeCoreImporter {
        ShapeCoreImporter::new(VmlShapeKind::ShapeType, -1638324645, part, document, None, mode)
    }
}
