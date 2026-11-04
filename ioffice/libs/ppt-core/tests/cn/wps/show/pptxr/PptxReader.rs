use std::io::{Cursor, Write};

use ppt_core::cn::wps::show::pptxr::PptxReader::{open_pptx_bytes, open_pptx_document_bytes};
use ppt_core::cn::wps::show::render::mvp_pipeline::{
    render_pptx_bytes, render_pptx_envelope_json,
};
use ppt_core::cn::wps::show::render::rendercommand::RenderCommand;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

fn hello_pptx() -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    {
        let mut z = ZipWriter::new(&mut buf);
        let opt = SimpleFileOptions::default();
        z.start_file("[Content_Types].xml", opt).unwrap();
        z.write_all(br#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"></Types>"#).unwrap();
        z.start_file("ppt/presentation.xml", opt).unwrap();
        z.write_all(br#"<?xml version="1.0"?><p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"><p:sldSz cx="12192000" cy="6858000"/></p:presentation>"#).unwrap();
        z.start_file("ppt/slides/slide1.xml", opt).unwrap();
        z.write_all(br#"<?xml version="1.0"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld><p:spTree>
<p:sp>
<p:spPr bwMode="clr">
<a:xfrm><a:off x="1270000" y="1270000"/><a:ext cx="5080000" cy="2540000"/></a:xfrm>
<a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
<a:prstGeom prst="rect"/>
</p:spPr>
<p:txBody>
<a:p><a:r><a:rPr sz="2400" b="1"/><a:t>Hello PPT</a:t></a:r></a:p>
</p:txBody>
</p:sp>
</p:spTree></p:cSld>
</p:sld>"#).unwrap();
        z.finish().unwrap();
    }
    buf.into_inner()
}

fn ordered_two_slide_pptx() -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    {
        let mut z = ZipWriter::new(&mut buf);
        let opt = SimpleFileOptions::default();
        z.start_file("[Content_Types].xml", opt).unwrap();
        z.write_all(br#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>"#).unwrap();
        z.start_file("ppt/presentation.xml", opt).unwrap();
        z.write_all(br#"<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:sldSz cx="12192000" cy="6858000"/><p:sldIdLst><p:sldId id="256" r:id="rIdSecond"/><p:sldId id="257" r:id="rIdFirst"/></p:sldIdLst></p:presentation>"#).unwrap();
        z.start_file("ppt/_rels/presentation.xml.rels", opt).unwrap();
        z.write_all(br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rIdFirst" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/><Relationship Id="rIdSecond" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide2.xml"/></Relationships>"#).unwrap();
        for (name, text) in [("ppt/slides/slide1.xml", "First"), ("ppt/slides/slide2.xml", "Second")] {
            z.start_file(name, opt).unwrap();
            z.write_all(format!(r#"<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><p:cSld><p:spTree><p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="100" cy="100"/></a:xfrm></p:spPr><p:txBody><a:p><a:r><a:t>{text}</a:t></a:r></a:p></p:txBody></p:sp></p:spTree></p:cSld></p:sld>"#).as_bytes()).unwrap();
        }
        z.finish().unwrap();
    }
    buf.into_inner()
}

#[test]
fn t01_open_pptx_xfrm_fill_text() {
    let slide = open_pptx_bytes(&hello_pptx()).expect("open");
    assert_eq!(slide.slide_width, 12_192_000);
    assert_eq!(slide.sp_tree.children.len(), 1);
    let sp = &slide.sp_tree.children[0];
    assert_eq!(sp.transform2DX, 1_270_000);
    assert_eq!(sp.transform2DW, 5_080_000);
    assert!(sp.sp_pr.has_fill);
    assert!(sp
        .text_op
        .as_ref()
        .map(|o| o.paras.iter().any(|p| p.runs.iter().any(|r| r.text.contains("Hello"))))
        .unwrap_or(false));
}

#[test]
fn t03_sample_ppt_opens() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../samples/ppt-render/flutter/assets/Sample_ppt.pptx"
    );
    let bytes = std::fs::read(path).expect("Sample_ppt.pptx");
    let document = open_pptx_document_bytes(&bytes).expect("open Sample_ppt document");
    assert_eq!(document.slides.len(), 23);
    assert_eq!(document.slide_names.len(), 23);
    let slide = open_pptx_bytes(&bytes).expect("open Sample_ppt");
    assert_eq!(slide.slide_width, 12_192_000);
    assert_eq!(slide.slide_height, 6_858_000);
    assert!(
        slide.sp_tree.children.len() >= 2,
        "shapes {}",
        slide.sp_tree.children.len()
    );
    let has_title = slide.sp_tree.children.iter().any(|sp| {
        sp.text_op.as_ref().is_some_and(|o| {
            o.paras
                .iter()
                .any(|p| p.runs.iter().any(|r| r.text.contains("年终") || r.text.contains("蓝色")))
        })
    });
    assert!(has_title, "title runs missing");
    let cmds = render_pptx_bytes(&bytes).expect("render");
    assert!(cmds.iter().any(|c| match c {
        RenderCommand::DrawText { text, .. } => text.contains("年终") || text.contains("蓝色"),
        _ => false,
    }));
}

#[test]
fn t04_document_uses_presentation_relationship_order() {
    let bytes = ordered_two_slide_pptx();
    let document = open_pptx_document_bytes(&bytes).expect("open ordered document");
    assert_eq!(
        document.slide_names,
        vec!["ppt/slides/slide2.xml", "ppt/slides/slide1.xml"]
    );
    let page_text = |index: usize| {
        document.slides[index].sp_tree.children[0]
            .text_op
            .as_ref()
            .unwrap()
            .paras[0]
            .runs[0]
            .text
            .clone()
    };
    assert_eq!(page_text(0), "Second");
    assert_eq!(page_text(1), "First");

    let json = render_pptx_envelope_json(&bytes).expect("document envelope");
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value["page_count"], 2);
    assert_eq!(value["selected_page"], 0);
    assert_eq!(value["pages"].as_array().unwrap().len(), 2);
    assert!(value["pages"][0]["commands"].as_array().unwrap().len() >= 2);
}

#[test]
fn t02_render_pptx_commands() {
    let cmds = render_pptx_bytes(&hello_pptx()).expect("render");
    assert!(cmds.iter().any(|c| matches!(c, RenderCommand::DrawRect { fill: Some(_), .. })));
    assert!(cmds.iter().any(|c| match c {
        RenderCommand::DrawText { text, .. } => text.contains("Hello"),
        _ => false,
    }));
}

use std::cell::RefCell;
use std::rc::Rc;

use ppt_core::cn::wps::show::app::event::KmoPresOpenEventListener::KmoPresOpenEventListener;
use ppt_core::cn::wps::show::pptxr::PptxReader::{PptxReader, PresPartError};
use ppt_core::cn::wps::show::util::layout_shape::KmoPresentation;

const C1: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../samples/ppt-render/flutter/assets/Sample_ppt.pptx"
);

struct OrderListener {
    events: Rc<RefCell<Vec<&'static str>>>,
}

impl KmoPresOpenEventListener for OrderListener {
    fn docDidFinishInitializing(&mut self, _pres: &mut KmoPresentation, _has_blank: bool) {
        self.events.borrow_mut().push("docDidFinishInitializing");
    }
    fn slideDidFinishReadingEverything(&mut self, _slide_index: i32) {}
    fn docDidFinishReadingEverything(&mut self) {
        self.events.borrow_mut().push("docDidFinishReadingEverything");
    }
}

fn word_office_pptx() -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    {
        let mut z = ZipWriter::new(&mut buf);
        let opt = SimpleFileOptions::default();
        z.start_file("[Content_Types].xml", opt).unwrap();
        z.write_all(br#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>"#).unwrap();
        z.start_file("_rels/.rels", opt).unwrap();
        z.write_all(br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#).unwrap();
        z.start_file("word/document.xml", opt).unwrap();
        z.write_all(br#"<w:document/>"#).unwrap();
        z.finish().unwrap();
    }
    buf.into_inner()
}

fn temp_pptx(name: &str, bytes: &[u8]) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("pptx-reader-{name}-{}.pptx", std::process::id()));
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn java_t01_null_office_file_damaged() {
    let path = temp_pptx("not-zip", b"this is not a package");
    let mut r = PptxReader::PptxReader(None, &path.to_string_lossy(), None, false, None);
    match r.open() {
        Err(PresPartError::FileDamaged(s)) => assert_eq!(s, "Not OOXML file"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn java_t02_word_part_suffix() {
    let path = temp_pptx("word-office", &word_office_pptx());
    let mut r = PptxReader::PptxReader(None, &path.to_string_lossy(), None, false, None);
    match r.open() {
        Err(PresPartError::SuffixError(s)) => assert_eq!(s, "Not pptx file"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn java_t03_success_order() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let listener = Rc::new(RefCell::new(OrderListener { events: events.clone() }));
    let mut r = PptxReader::PptxReader(None, C1, Some(listener), false, None);
    r.open().unwrap();
    let model = r.assembled_pres().unwrap();
    let p = model.borrow();
    assert_eq!(p.file_format, 32);
    assert_eq!(p.app_application.as_deref(), Some("Microsoft Office PowerPoint"));
    assert!(p.hasInitialized());
    assert_eq!(p.core_title.as_deref(), Some("PowerPoint 演示文稿"));
    assert!(!p.read_only);
    assert_eq!(
        events.borrow().as_slice(),
        ["docDidFinishInitializing", "docDidFinishReadingEverything"]
    );
}

#[test]
fn java_t04_finally_closes_package() {
    let mut r = PptxReader::PptxReader(None, C1, None, false, None);
    r.open().unwrap();
    assert!(r.mPackage.is_none());
}

#[test]
fn java_t05_path_vs_raf() {
    let mut a = PptxReader::PptxReader(None, C1, None, false, None);
    a.open().unwrap();
    let path_app = a.assembled_pres().unwrap().borrow().app_application.clone();
    let bytes = std::fs::read(C1).unwrap();
    let mut b = PptxReader::PptxReader__raf(None, Some(bytes), None, false, None);
    b.open().unwrap();
    assert_eq!(
        b.assembled_pres().unwrap().borrow().app_application,
        path_app
    );
    assert!(b.assembled_pres().unwrap().borrow().hasInitialized());
}

#[test]
fn java_t06_package_ctor_file_damaged() {
    let mut r = PptxReader::PptxReader(None, "/no/such/pptx-reader.pptx", None, false, None);
    match r.open() {
        Err(PresPartError::FileDamaged(_)) => {}
        other => panic!("{other:?}"),
    }
}

#[test]
#[ignore = "spec: post-MVP"]
fn java_t07_custom_full() {}

