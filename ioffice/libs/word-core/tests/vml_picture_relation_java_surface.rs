use io_wps_io::io::opc::internal::ContentTypeManager::ContentTypeManager;
use io_wps_io::io::opc::OPCPackage::OPCPackage;
use io_wps_io::io::opc::PackageAccess::PackageAccess;
use io_wps_io::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use io_wps_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use io_wps_io::io::opc::TargetMode::TargetMode;
use io_wps_io::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use io_wps_io::io::util::URI::URI;
use io_wps_io::io::xwpf::usermodel::XWPFFactory::XWPFFactory;
use io_wps_io::io::xwpf::PictureData::PictureData;
use io_wps_io::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;
use std::cell::RefCell;
use std::io::{Cursor, Write};
use std::rc::Rc;
use word_core::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::helper::VmlHelper::VmlHelper;
use word_core::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;
use word_core::cn::wps::moffice::writer::core::Document::MAIN_DOCUMENT;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::data::IChpxTable::{Entry as IChpxEntry, IChpxTable};
use util::moffice::property::MutablePropertySet::MutablePropertySet;

#[test]
fn java_vml_picture_lookup_preserves_image_part_bytes() {
    let image = b"\x89PNG\r\n\x1a\noriginal-media";
    let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
    zip.start_file("word/media/image1.png", zip::write::SimpleFileOptions::default()).unwrap();
    zip.write_all(image).unwrap();
    let source = ZipInputStreamZipEntrySource::try_new(&mut Cursor::new(zip.finish().unwrap().into_inner())).unwrap();
    let mut package = Box::new(OPCPackage::new(PackageAccess::READ));
    let mut types = ContentTypeManager::new(None, None);
    types.addDefaultContentType("png", "image/png");
    package.setContentTypeManager(types);
    package.setZipArchive(Box::new(source));
    package.getPartsImpl();

    let name = PackagingURIHelper::createPartName_str("/word/media/image1.png").unwrap();
    let part = package.getPart(&name).expect("package image part");
    let mut relations = PackageRelationshipCollection::new();
    let relation = relations.addRelationship(
        URI::new("media/image1.png").unwrap(), TargetMode::INTERNAL,
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", Some("rId1"),
    );
    let mut parent = POIXMLDocumentPart::new();
    let child = Rc::new(RefCell::new(XWPFFactory::getInstance()
        .createDocumentPart(&parent, relation, part).expect("picture child part")));
    parent.addRelation("rId1".into(), child.clone());

    assert!(VmlHelper::getPictureData(None, &parent).is_none());
    assert!(VmlHelper::getPictureData(Some("missing"), &parent).is_none());
    let picture = VmlHelper::getPictureData(Some("rId1"), &parent).expect("image relation");
    assert!(Rc::ptr_eq(&picture.document_part(), &child));
    assert_eq!(picture.getData(), image);
    let data: &dyn PictureData = &picture;
    assert_eq!(data.getData(), image);
    assert_eq!(data.suggestFileExtension(), "png");
    assert!(data.getUniqueFileName().ends_with(".png"));
    let source = data.getDataSource().expect("PictureData zip source");
    assert_eq!(source.getEntry().getName(), "word/media/image1.png");
    assert!(!IOHelper::stringHasValue(None));
    assert!(IOHelper::getPictureData(Some("missing"), &parent).is_none());
    let helper_picture = IOHelper::getPictureData(Some("rId1"), &parent).expect("IOHelper image relation");
    assert!(Rc::ptr_eq(&helper_picture.document_part(), &child));
    assert_eq!(helper_picture.getData(), image);
    assert!(Rc::ptr_eq(&VmlHelper::getOleObject(Some("rId1"), &parent).unwrap(), &child));
}

#[test]
fn java_io_helper_creates_picture_path_in_requested_folder() {
    let folder = std::env::temp_dir().join(format!("word-core-picture-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&folder).unwrap();
    let path = IOHelper::createPicturePath(folder.to_str().unwrap(), "image.png").unwrap();
    assert_eq!(std::path::Path::new(&path), folder.join("image.png"));
    assert!(std::path::Path::new(&path).exists());
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn java_io_helper_appends_picture_control_character_with_property_28() {
    let mut document = TextDocument::new();
    let main = document.getSubdocumentMut(MAIN_DOCUMENT).unwrap();
    let mut props = MutablePropertySet::new();
    let cp = IOHelper::appendControlChar(main, '\u{0001}', &mut props);
    assert_eq!(cp, 0);
    assert_eq!(main.getLength(), 1);
    assert!(props.getBoolean(28));
    let entry = IChpxTable::seek(main.getChpxTable().unwrap(), cp).unwrap();
    assert!(IChpxEntry::getProperty(entry.as_ref()).unwrap().getBoolean(28));
}
