use core_libs_io::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use core_libs_io::io::opc::TargetMode::TargetMode;
use core_libs_io::io::util::URI::URI;
use core_libs_io::cn::wps::io::xwpf::element_handler::SubDocumentHandler::SubDocumentHandler;

#[test]
fn java_has_header_footer_checks_transitional_and_strict_relations() {
    let mut rels = PackageRelationshipCollection::new();
    rels.addRelationship(
        URI::new("media/image1.png").unwrap(), TargetMode::INTERNAL,
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image", Some("rId1"),
    );
    assert!(!SubDocumentHandler::hasHeaderFooter(&rels));
    for (index, relation) in [
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
        "http://purl.oclc.org/ooxml/officeDocument/relationships/header",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
        "http://purl.oclc.org/ooxml/officeDocument/relationships/footer",
    ].iter().enumerate() {
        let mut one = PackageRelationshipCollection::new();
        one.addRelationship(URI::new("header1.xml").unwrap(), TargetMode::INTERNAL, relation,
            Some(&format!("rId{index}")));
        assert!(SubDocumentHandler::hasHeaderFooter(&one));
    }
}
