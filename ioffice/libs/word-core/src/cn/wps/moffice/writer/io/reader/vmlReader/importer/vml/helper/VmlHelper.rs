#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

use io_wps_io::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use io_wps_io::io::xwpf::usermodel::XWPFPictureData::XWPFPictureData;
use io_wps_io::io::xwpf::usermodel::XWPFRelation::XWPFRelation;
use io_wps_io::io::xwpf::usermodel::XWPFRelationPurl::XWPFRelationPurl;

// Java: public class VmlHelper
pub struct VmlHelper;

impl VmlHelper {
    // Java: implicit public VmlHelper()
    pub fn new() -> Self { Self }

    // Java: public static boolean stringHasValue(String value)
    pub fn stringHasValue(value: Option<&str>) -> bool {
        // Java: return null != value && value.length() > 0;
        value.is_some_and(|value| !value.is_empty())
    }

    // Java: public static boolean isFilledByDefault(int shapeType)
    pub fn isFilledByDefault(shapeType: i32) -> bool {
        // Java: cases 19, 85–88, 185, 186 return false; default returns true.
        !matches!(shapeType, 19 | 85 | 86 | 87 | 88 | 185 | 186)
    }

    // Java: public static PictureData getPictureData(String rId, POIXMLDocumentPart parent)
    pub fn getPictureData(rId: Option<&str>, parent: &POIXMLDocumentPart) -> Option<XWPFPictureData> {
        // Java: if (!stringHasValue(rId)) return null;
        if !Self::stringHasValue(rId) { return None; }
        // Java: POIXMLDocumentPart part = parent.getRelationById(rId);
        let part = parent.getRelationById(rId?)?;
        // Java: PictureData pictureData = (XWPFPictureData)part;
        // XWPFFactory stores a base relation node, so check the same descriptor
        // class used by its Java constructor before the downcast adapter.
        let is_picture = {
            let borrowed = part.borrow();
            let relation_type = borrowed.getPackageRelationship().as_ref()?.getRelationshipType();
            XWPFRelation::getInstance(relation_type).getRelationClass() == Some("XWPFPictureData")
                || XWPFRelationPurl::getInstance(relation_type).getRelationClass() == Some("XWPFPictureData")
        };
        assert!(is_picture, "VML picture relation is not XWPFPictureData");
        // The Rust factory stores the subclass as a shared base relation entry;
        // from_relation retains that same entry and its PackagePart/zip stream.
        Some(XWPFPictureData::from_relation(part))
    }

    // Java: public static POIXMLDocumentPart getOleObject(String rId, POIXMLDocumentPart parent)
    pub fn getOleObject(rId: Option<&str>, parent: &POIXMLDocumentPart) -> Option<Rc<RefCell<POIXMLDocumentPart>>> {
        // Java: if (!stringHasValue(rId)) return null;
        if !Self::stringHasValue(rId) { return None; }
        // Java: return parent.getRelationById(rId);
        parent.getRelationById(rId?)
    }
}
