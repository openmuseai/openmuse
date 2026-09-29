use openmuse_office_viewers::{
    OpenMuseOfficeViewerBuffer, ViewerError, inspect_xlsx, openmuse_office_viewer_buffer_free,
    openmuse_office_viewers_abi_version, openmuse_xlsx_inspect,
};
use std::io::{Cursor, Write};
use zip::{ZipWriter, write::SimpleFileOptions};

fn package(extra_relationship: &str, worksheet: &str, unsafe_path: bool) -> Vec<u8> {
    let mut output = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut output);
    let options = SimpleFileOptions::default();
    let parts = [
        (
            "[Content_Types].xml",
            r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>"#,
        ),
        (
            "_rels/.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"/>"#,
        ),
        (
            "xl/workbook.xml",
            r#"<workbook xmlns:r="urn:r"><sheets><sheet name="Sheet A" r:id="rId1"/></sheets></workbook>"#,
        ),
        ("xl/_rels/workbook.xml.rels", extra_relationship),
        (
            "xl/sharedStrings.xml",
            r#"<sst><si><t>Hello</t></si><si><r><t>World</t></r></si></sst>"#,
        ),
        ("xl/worksheets/sheet1.xml", worksheet),
    ];
    for (name, bytes) in parts {
        zip.start_file(name, options).unwrap();
        zip.write_all(bytes.as_bytes()).unwrap();
    }
    if unsafe_path {
        zip.start_file("../escape.xml", options).unwrap();
        zip.write_all(b"escape").unwrap();
    }
    zip.finish().unwrap();
    output.into_inner()
}

fn valid_xlsx() -> Vec<u8> {
    package(
        r#"<Relationships><Relationship Id="rId1" Target="worksheets/sheet1.xml"/></Relationships>"#,
        r#"<worksheet><sheetData><row r="1"><c r="A1" t="s"><v>0</v></c><c r="B1"><v>42</v></c><c r="C1" t="inlineStr"><is><t>Inline</t></is></c></row><row r="2"><c r="B2" t="s"><v>1</v></c></row></sheetData></worksheet>"#,
        false,
    )
}

#[test]
fn xlsx_is_bounded_view_only_and_preserves_sparse_rows() {
    let inspected = inspect_xlsx(&valid_xlsx()).unwrap();
    assert_eq!(inspected.profile, "view-only");
    assert_eq!(inspected.capabilities, ["view"]);
    assert_eq!(
        inspected.paragraphs,
        ["Sheet A\tHello\t42\tInline", "Sheet A\t\tWorld",]
    );
}

#[test]
fn external_relationships_paths_and_doctypes_fail_closed() {
    let external = package(
        r#"<Relationships><Relationship Id="rId1" TargetMode = "External" Target="https://example.test"/></Relationships>"#,
        r#"<worksheet><sheetData/></worksheet>"#,
        false,
    );
    assert_eq!(
        inspect_xlsx(&external),
        Err(ViewerError::ExternalRelationship)
    );
    assert_eq!(
        inspect_xlsx(&package(
            r#"<Relationships><Relationship Id="rId1" Target="worksheets/sheet1.xml"/></Relationships>"#,
            r#"<worksheet><sheetData/></worksheet>"#,
            true,
        )),
        Err(ViewerError::UnsafePackagePath)
    );
    assert_eq!(
        inspect_xlsx(&package(
            r#"<Relationships><Relationship Id="rId1" Target="worksheets/sheet1.xml"/></Relationships>"#,
            r#"<!DOCTYPE x [<!ENTITY y "z">]><worksheet/>"#,
            false,
        )),
        Err(ViewerError::DocTypeDenied)
    );
}

#[test]
fn xlsx_c_abi_returns_owned_json() {
    let input = valid_xlsx();
    let buffer = unsafe { openmuse_xlsx_inspect(input.as_ptr(), input.len()) };
    assert_eq!(openmuse_office_viewers_abi_version(), 1);
    assert_eq!(buffer.status, 0);
    let bytes = unsafe { std::slice::from_raw_parts(buffer.ptr, buffer.len) };
    let json: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    assert_eq!(json["schema"], "openmuse.office.xlsx-inspection@1");
    openmuse_office_viewer_buffer_free(buffer);

    let invalid = unsafe { openmuse_xlsx_inspect([1_u8].as_ptr(), 1) };
    assert_ne!(invalid.status, 0);
    openmuse_office_viewer_buffer_free(invalid);
}

#[allow(dead_code)]
fn assert_buffer_layout(_: OpenMuseOfficeViewerBuffer) {}
