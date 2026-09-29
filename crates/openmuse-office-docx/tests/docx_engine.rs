use openmuse_office_docx::{
    DocxError, DocxLimits, DocxProfile, OpenMuseDocxBuffer, export_simple, inspect,
    inspect_with_limits, openmuse_docx_abi_version, openmuse_docx_buffer_free,
    openmuse_docx_inspect,
};
use std::io::{Cursor, Write};
use zip::{ZipWriter, write::SimpleFileOptions};

fn package(document: &str, extra: &[(&str, &str)]) -> Vec<u8> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default();
    for (name, content) in [
        (
            "[Content_Types].xml",
            r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>"#,
        ),
        (
            "_rels/.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"/>"#,
        ),
        ("word/document.xml", document),
    ]
    .into_iter()
    .chain(extra.iter().copied())
    {
        writer.start_file(name, options).unwrap();
        writer.write_all(content.as_bytes()).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

fn simple_docx() -> Vec<u8> {
    package(
        r#"<?xml version="1.0"?><w:document xmlns:w="urn:w"><w:body><w:p><w:r><w:t>Hello &amp; 世界</w:t><w:tab/><w:t>tab</w:t></w:r></w:p><w:p><w:r><w:t>Second</w:t><w:br/><w:t>line</w:t></w:r></w:p></w:body></w:document>"#,
        &[],
    )
}

#[test]
fn simple_docx_inspects_and_round_trips_through_original_format_export() {
    let input = simple_docx();
    let inspected = inspect(&input).unwrap();
    assert_eq!(inspected.profile, DocxProfile::SimpleText);
    assert_eq!(inspected.paragraphs, ["Hello & 世界\ttab", "Second\nline"]);
    assert_eq!(inspected.capabilities, ["view", "edit", "export"]);

    let replacement = vec!["Changed <safe> & preserved".to_owned(), "第二段".to_owned()];
    let output = export_simple(&input, &replacement).unwrap();
    assert_eq!(inspect(&output).unwrap().paragraphs, replacement);
}

#[test]
fn complex_document_is_view_only_and_cannot_claim_export() {
    let input = package(
        r#"<w:document xmlns:w="urn:w"><w:body><w:tbl><w:tr><w:tc><w:p><w:r><w:t>cell</w:t></w:r></w:p></w:tc></w:tr></w:tbl></w:body></w:document>"#,
        &[],
    );
    let inspected = inspect(&input).unwrap();
    assert_eq!(inspected.profile, DocxProfile::ViewOnly);
    assert_eq!(inspected.capabilities, ["view"]);
    assert_eq!(
        export_simple(&input, &["replacement".into()]),
        Err(DocxError::ExportUnsupported)
    );
}

#[test]
fn corrupt_external_and_doctype_inputs_fail_closed() {
    assert_eq!(inspect(b"not a zip"), Err(DocxError::InvalidArchive));
    let external = package(
        r#"<w:document xmlns:w="urn:w"><w:body/></w:document>"#,
        &[(
            "word/_rels/document.xml.rels",
            r#"<Relationships><Relationship TargetMode="External" Target="https://evil.test"/></Relationships>"#,
        )],
    );
    assert_eq!(inspect(&external), Err(DocxError::ExternalRelationship));
    let doctype = package(
        r#"<!DOCTYPE x [<!ENTITY e "boom">]><w:document xmlns:w="urn:w"><w:body><w:p><w:r><w:t>&e;</w:t></w:r></w:p></w:body></w:document>"#,
        &[],
    );
    assert_eq!(inspect(&doctype), Err(DocxError::DocTypeDenied));
}

#[test]
fn package_traversal_and_non_portable_paths_fail_closed() {
    let traversal = package(
        r#"<w:document xmlns:w="urn:w"><w:body/></w:document>"#,
        &[("../outside.xml", "secret")],
    );
    assert_eq!(inspect(&traversal), Err(DocxError::UnsafePackagePath));
    let non_portable = package(
        r#"<w:document xmlns:w="urn:w"><w:body/></w:document>"#,
        &[(r"word\outside.xml", "secret")],
    );
    assert_eq!(inspect(&non_portable), Err(DocxError::UnsafePackagePath));
}

#[test]
fn archive_limits_are_enforced_before_unbounded_allocation() {
    let input = simple_docx();
    assert_eq!(
        inspect_with_limits(
            &input,
            DocxLimits {
                max_archive_bytes: input.len() - 1,
                max_entry_bytes: 1024,
                max_total_uncompressed_bytes: 4096,
                max_entries: 10,
            }
        ),
        Err(DocxError::LimitExceeded)
    );
}

#[test]
fn c_abi_returns_owned_json_and_explicit_free() {
    assert_eq!(openmuse_docx_abi_version(), 1);
    let input = simple_docx();
    let result: OpenMuseDocxBuffer = unsafe { openmuse_docx_inspect(input.as_ptr(), input.len()) };
    assert_eq!(result.status, 0);
    let json = unsafe { std::slice::from_raw_parts(result.ptr, result.len) };
    let value: serde_json::Value = serde_json::from_slice(json).unwrap();
    assert_eq!(value["schema"], "openmuse.office.docx-inspection@1");
    openmuse_docx_buffer_free(result);
}

#[test]
fn multilingual_corpus_round_trips_deterministically() {
    let original = simple_docx();
    let paragraphs = vec![
        "English <tag> & apostrophe ' quote \"".to_owned(),
        "简体中文、繁體中文、日本語、한국어".to_owned(),
        "العربية עברית हिन्दी ไทย".to_owned(),
        "emoji 👩🏽‍💻🚀 and combining e\u{301}".to_owned(),
        "tabs\tand\nline breaks".to_owned(),
    ];
    let first = export_simple(&original, &paragraphs).unwrap();
    let second = export_simple(&original, &paragraphs).unwrap();
    assert_eq!(first, second);
    let inspected = inspect(&first).unwrap();
    assert_eq!(inspected.profile, DocxProfile::SimpleText);
    assert_eq!(inspected.paragraphs, paragraphs);
}

#[test]
fn relationship_parser_rejects_spacing_case_doctype_and_malformed_xml() {
    for relationships in [
        r#"<Relationships><Relationship TargetMode = "External" Target="https://evil.test"/></Relationships>"#,
        r#"<Relationships><Relationship TargetMode="external" Target="https://evil.test"/></Relationships>"#,
    ] {
        let input = package(
            r#"<w:document xmlns:w="urn:w"><w:body/></w:document>"#,
            &[("word/_rels/document.xml.rels", relationships)],
        );
        assert_eq!(inspect(&input), Err(DocxError::ExternalRelationship));
    }
    for relationships in [
        r#"<!DOCTYPE x><Relationships/>"#,
        r#"<Relationships><Relationship></Relationships>"#,
    ] {
        let input = package(
            r#"<w:document xmlns:w="urn:w"><w:body/></w:document>"#,
            &[("word/_rels/document.xml.rels", relationships)],
        );
        assert!(matches!(
            inspect(&input),
            Err(DocxError::DocTypeDenied | DocxError::InvalidXml)
        ));
    }
}

#[test]
fn known_complex_office_structures_remain_view_only() {
    for element in [
        "<w:hyperlink><w:r><w:t>link</w:t></w:r></w:hyperlink>",
        "<w:ins><w:r><w:t>tracked</w:t></w:r></w:ins>",
        "<w:drawing/>",
        "<m:oMath xmlns:m=\"urn:m\"/>",
    ] {
        let document = format!(
            r#"<w:document xmlns:w="urn:w"><w:body><w:p>{element}</w:p></w:body></w:document>"#
        );
        let input = package(&document, &[]);
        let inspected = inspect(&input).unwrap();
        assert_eq!(inspected.profile, DocxProfile::ViewOnly);
        assert_eq!(inspected.capabilities, ["view"]);
        assert_eq!(
            export_simple(&input, &["replacement".into()]),
            Err(DocxError::ExportUnsupported)
        );
    }
}
