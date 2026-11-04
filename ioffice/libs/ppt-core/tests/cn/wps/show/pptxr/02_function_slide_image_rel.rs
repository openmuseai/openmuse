#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::slide_image_part_from_rels;
use visual_zip::{IMAGE_TYPE, REL_NS};

#[test]
fn t01_image_rel_from_slide() {
    let rels = format!(
        r#"<Relationships {REL_NS}><Relationship Id="rIdImg" Type="{IMAGE_TYPE}" Target="../media/image1.jpg"/></Relationships>"#
    );
    let part = slide_image_part_from_rels("ppt/slides/slide1.xml", &rels, "rIdImg").unwrap();
    assert_eq!(part, "ppt/media/image1.jpg");
}

#[test]
fn t02_unknown_rid_is_none() {
    let rels = format!(
        r#"<Relationships {REL_NS}><Relationship Id="rIdImg" Type="{IMAGE_TYPE}" Target="../media/image1.jpg"/></Relationships>"#
    );
    assert!(slide_image_part_from_rels("ppt/slides/slide1.xml", &rels, "rIdMissing").is_none());
}

#[test]
fn t03_non_image_type_ignored() {
    let rels = format!(
        r#"<Relationships {REL_NS}><Relationship Id="rIdImg" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="../media/image1.jpg"/></Relationships>"#
    );
    assert!(slide_image_part_from_rels("ppt/slides/slide1.xml", &rels, "rIdImg").is_none());
}
