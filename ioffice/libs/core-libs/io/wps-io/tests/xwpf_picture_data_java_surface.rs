use core_libs_io::io::opc::internal::ContentTypeManager::ContentTypeManager;
use core_libs_io::io::opc::OPCPackage::OPCPackage;
use core_libs_io::io::opc::PackageAccess::PackageAccess;
use core_libs_io::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use core_libs_io::io::opc::TargetMode::TargetMode;
use core_libs_io::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use core_libs_io::io::util::URI::URI;
use core_libs_io::io::xwpf::usermodel::XWPFFactory::XWPFFactory;
use core_libs_io::io::xwpf::usermodel::XWPFPictureData::XWPFPictureData;
use core_libs_io::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;
use core_libs_util::cn::wps::util::IOUtils::IOUtils;
use std::cell::RefCell;
use std::io::{Cursor, Read, Write};
use std::rc::Rc;

#[test]
fn java_picture_relation_reads_original_package_bytes() {
    let image = b"\x89PNG\r\n\x1a\nmedia-bytes";
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    writer.start_file("word/media/image1.png", zip::write::SimpleFileOptions::default()).unwrap();
    writer.write_all(image).unwrap();
    let zip_bytes = writer.finish().unwrap().into_inner();
    let source = ZipInputStreamZipEntrySource::try_new(&mut Cursor::new(zip_bytes)).unwrap();
    let mut package = Box::new(OPCPackage::new(PackageAccess::READ));
    let mut content_types = ContentTypeManager::new(None, None);
    content_types.addDefaultContentType("png", "image/png");
    package.setContentTypeManager(content_types);
    package.setZipArchive(Box::new(source));
    package.getPartsImpl();

    let name = PackagingURIHelper::createPartName_str("/word/media/image1.png").unwrap();
    let part = package.getPart(&name).expect("image package part");
    let mut relations = PackageRelationshipCollection::new();
    let relation = relations.addRelationship(
        URI::new("media/image1.png").unwrap(), TargetMode::INTERNAL,
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", Some("rId1"),
    );
    let parent = POIXMLDocumentPart::new();
    let child = XWPFFactory::getInstance().createDocumentPart(&parent, relation, part).expect("image relation part");
    let picture = XWPFPictureData::from_relation(Rc::new(RefCell::new(child)));
    assert_eq!(picture.getData(), image);
    assert_eq!(picture.getFileName().as_deref(), Some("image1.png"));
    assert_eq!(picture.suggestFileExtension(), "png");
    assert_eq!(picture.getChecksum(), IOUtils::calculateChecksum(image));
    assert_eq!(picture.getChecksum(), IOUtils::calculateChecksum(image));
    let unique = picture.getUniqueFileName();
    assert!(unique.ends_with(".png"));
    let data_source = picture.getDataSource().expect("zip picture data source");
    assert!(data_source.getFilename().ends_with(".png"));
    assert_eq!(data_source.getEntry().getName(), "word/media/image1.png");
    let mut from_entry = Vec::new();
    data_source.getSource().getInputStream(data_source.getEntry()).unwrap()
        .read_to_end(&mut from_entry).unwrap();
    assert_eq!(from_entry, image);
}
