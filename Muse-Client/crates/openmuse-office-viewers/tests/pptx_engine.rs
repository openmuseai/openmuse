use openmuse_office_viewers::{
    ViewerError, inspect_pptx, openmuse_office_viewer_buffer_free, openmuse_pptx_inspect,
};
use std::io::{Cursor, Write};
use zip::{ZipWriter, write::SimpleFileOptions};

fn pptx(slide_relationship: &str, slide: &str) -> Vec<u8> {
    let mut output = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut output);
    let options = SimpleFileOptions::default();
    for (name, bytes) in [
        (
            "[Content_Types].xml",
            r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>"#,
        ),
        ("_rels/.rels", r#"<Relationships/>"#),
        (
            "ppt/presentation.xml",
            r#"<p:presentation xmlns:p="urn:p" xmlns:r="urn:r"><p:sldIdLst><p:sldId id="256" r:id="rId1"/></p:sldIdLst></p:presentation>"#,
        ),
        (
            "ppt/_rels/presentation.xml.rels",
            r#"<Relationships><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/><Relationship Id="theme" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/></Relationships>"#,
        ),
        ("ppt/slides/slide1.xml", slide),
        ("ppt/slides/_rels/slide1.xml.rels", slide_relationship),
    ] {
        zip.start_file(name, options).unwrap();
        zip.write_all(bytes.as_bytes()).unwrap();
    }
    zip.finish().unwrap();
    output.into_inner()
}

fn valid() -> Vec<u8> {
    pptx(
        r#"<Relationships/>"#,
        r#"<p:sld xmlns:p="urn:p" xmlns:a="urn:a"><p:cSld><p:spTree><p:sp><p:txBody><a:p><a:r><a:t>Hello </a:t></a:r><a:r><a:t>Slides</a:t></a:r></a:p><a:p><a:r><a:t>Second line</a:t></a:r></a:p></p:txBody></p:sp></p:spTree></p:cSld></p:sld>"#,
    )
}

#[test]
fn pptx_follows_slide_order_and_extracts_text_view_only() {
    let inspected = inspect_pptx(&valid()).unwrap();
    assert_eq!(inspected.schema, "openmuse.office.pptx-inspection@1");
    assert_eq!(inspected.profile, "view-only");
    assert_eq!(inspected.capabilities, ["view"]);
    assert_eq!(
        inspected.paragraphs,
        ["Slide 1\tHello Slides", "Slide 1\tSecond line"]
    );
}

#[test]
fn pptx_external_relationship_and_doctype_fail_closed() {
    assert_eq!(
        inspect_pptx(&pptx(
            r#"<Relationships><Relationship Id="x" TargetMode="External" Target="https://example.test"/></Relationships>"#,
            r#"<p:sld/>"#,
        )),
        Err(ViewerError::ExternalRelationship)
    );
    assert_eq!(
        inspect_pptx(&pptx(
            r#"<Relationships/>"#,
            r#"<!DOCTYPE x [<!ENTITY y "z">]><p:sld/>"#,
        )),
        Err(ViewerError::DocTypeDenied)
    );
}

#[test]
fn pptx_c_abi_is_owned_and_versioned_by_shared_viewer_abi() {
    let input = valid();
    let buffer = unsafe { openmuse_pptx_inspect(input.as_ptr(), input.len()) };
    assert_eq!(buffer.status, 0);
    let json: serde_json::Value =
        serde_json::from_slice(unsafe { std::slice::from_raw_parts(buffer.ptr, buffer.len) })
            .unwrap();
    assert_eq!(json["schema"], "openmuse.office.pptx-inspection@1");
    openmuse_office_viewer_buffer_free(buffer);
}
