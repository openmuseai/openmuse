use lopdf::{
    Document, Object, Stream,
    content::{Content, Operation},
    dictionary,
};
use openmuse_office_viewers::{
    ViewerError, inspect_pdf, openmuse_office_viewer_buffer_free, openmuse_pdf_inspect,
};

fn pdf(active: bool) -> Vec<u8> {
    let mut document = Document::with_version("1.5");
    let pages_id = document.new_object_id();
    let page_id = document.new_object_id();
    let font_id = document.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), 14.into()]),
            Operation::new("Td", vec![72.into(), 720.into()]),
            Operation::new("Tj", vec![Object::string_literal("Hello PDF")]),
            Operation::new("ET", vec![]),
        ],
    }
    .encode()
    .unwrap();
    let content_id = document.add_object(Stream::new(dictionary! {}, content));
    document.objects.insert(
        page_id,
        Object::Dictionary(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
            "Resources" => dictionary! { "Font" => dictionary! { "F1" => font_id } },
            "Contents" => content_id,
        }),
    );
    document.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        }),
    );
    let mut catalog = dictionary! { "Type" => "Catalog", "Pages" => pages_id };
    if active {
        catalog.set(
            "OpenAction",
            dictionary! { "S" => "JavaScript", "JS" => Object::string_literal("app.alert(1)") },
        );
    }
    let catalog_id = document.add_object(catalog);
    document.trailer.set("Root", catalog_id);
    let mut bytes = Vec::new();
    document.save_to(&mut bytes).unwrap();
    bytes
}

#[test]
fn pdf_extracts_a_bounded_text_compatibility_view() {
    let inspected = inspect_pdf(&pdf(false)).unwrap();
    assert_eq!(inspected.schema, "openmuse.office.pdf-inspection@1");
    assert_eq!(inspected.profile, "text-view-only");
    assert_eq!(inspected.capabilities, ["view"]);
    assert!(
        inspected
            .paragraphs
            .iter()
            .any(|line| line.contains("Hello PDF"))
    );
}

#[test]
fn active_and_invalid_pdf_inputs_fail_closed() {
    assert_eq!(inspect_pdf(&pdf(true)), Err(ViewerError::ActivePdfDenied));
    assert_eq!(inspect_pdf(b"not-pdf"), Err(ViewerError::InvalidPdf));
}

#[test]
fn pdf_c_abi_returns_owned_versioned_json() {
    let input = pdf(false);
    let buffer = unsafe { openmuse_pdf_inspect(input.as_ptr(), input.len()) };
    assert_eq!(buffer.status, 0);
    let json: serde_json::Value =
        serde_json::from_slice(unsafe { std::slice::from_raw_parts(buffer.ptr, buffer.len) })
            .unwrap();
    assert_eq!(json["schema"], "openmuse.office.pdf-inspection@1");
    openmuse_office_viewer_buffer_free(buffer);
}
