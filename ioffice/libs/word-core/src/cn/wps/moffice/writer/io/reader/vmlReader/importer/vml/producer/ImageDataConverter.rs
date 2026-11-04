// Java: cn.wps.moffice.writer.io.reader.vmlReader.importer.vml.producer.ImageDataConverter
#![allow(non_snake_case)]

use std::any::Any;
use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::graphics::Picture::Picture;
use drawing::cn::wps::moffice::drawing::media::IMediaLib::IMediaLib;
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::MediaTypeEnum;
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use io_wps_io::cn::wps::io::xwpf::element_handler::vml::ConvertHelper::ConvertHelper;

use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;
use crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::helper::VmlHelper::VmlHelper;

const REL_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

// Java: public class ImageDataConverter
pub struct ImageDataConverter;

impl ImageDataConverter {
    // Java: public Picture getPicture(Shape, Attributes, int) — docx branch.
    pub fn getPicture(part: Option<&POIXMLDocumentPart>, doc: &mut TextDocument, shape: &Arc<Mutex<Shape>>, image_attr: &dyn Any, mode: i32) -> Option<Picture> {
        // Java: String rId / rPict / src
        let r_id = ConvertHelper::getString__Attributes_String_String(image_attr, "id", REL_NS);
        let r_pict = ConvertHelper::getString__Attributes_String_String(image_attr, "pict", REL_NS);
        let src = ConvertHelper::getString4EmptyNamespace(image_attr, "src");
        // Java: isXml07 is mode 1. The complex sample is docx (mode 0).
        if mode == 1 {
            return None;
        }
        // Java: isXml03 is mode 3.
        if mode == 3 {
            let Some(src) = src else { return None; };
            let media_id = IMediaLib::addMedia(doc.getMediaLib(), &src, &src, MediaTypeEnum::PICTURE);
            return Some(Picture::new_with_media_id(media_id));
        }
        let id = r_id.or(r_pict);
        let picture_data = part.and_then(|part| VmlHelper::getPictureData(id.as_deref(), part));
        let picture = if src.is_some() {
            None
        } else if let Some(picture_data) = picture_data.as_ref() {
            // Java: picture = IOHelper.dumpPicture(shape, pictureData, this.mDoc);
            Some(IOHelper::dumpPicture(shape, picture_data, doc))
        } else {
            None
        };
        picture
    }
}
