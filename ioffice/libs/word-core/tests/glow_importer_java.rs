use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use word_core::cn::wps::moffice::writer::io::reader::dmlReader::{
    context::{DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
        ThemeContext::ThemeContext},
    GlowImporter::GlowImporter,
    helper::ColorsTrans::ColorsTrans,
};

struct ParentImporter;
impl IImporterBase for ParentImporter { fn mode(&self) -> i32 { 0 } }

#[derive(Default)]
struct Attrs(HashMap<(String, String), String>);
impl Attrs {
    fn with(mut self, name: &str, value: &str) -> Self {
        self.0.insert((String::new(), name.into()), value.into()); self
    }
}
impl SaxAttributes for Attrs {
    fn getLength(&self) -> usize { self.0.len() }
    fn getURI(&self, _: usize) -> Option<String> { None }
    fn getLocalName(&self, _: usize) -> Option<String> { None }
    fn getQName(&self, _: usize) -> Option<String> { None }
    fn getType_by_index(&self, _: usize) -> Option<String> { None }
    fn getValue_by_index(&self, _: usize) -> Option<String> { None }
    fn getIndex_by_uri(&self, _: &str, _: &str) -> i32 { -1 }
    fn getIndex_by_qname(&self, _: &str) -> i32 { -1 }
    fn getType_by_uri(&self, _: &str, _: &str) -> Option<String> { None }
    fn getType_by_qname(&self, _: &str) -> Option<String> { None }
    fn getValue_by_uri(&self, uri: &str, name: &str) -> Option<String> {
        self.0.get(&(uri.into(), name.into())).cloned()
    }
    fn getValue_by_qname(&self, _: &str) -> Option<String> { None }
}

#[test]
fn glow_preserves_java_color_alpha_size_and_context_lifetime() {
    let subdoc = Rc::new(RefCell::new(DmlSubDocCtxt::<()>::new()));
    let shape = Rc::new(RefCell::new(DmlShapeCtxt::new()));
    let theme = Rc::new(RefCell::new(ThemeContext::default()));
    let parent = Rc::new(RefCell::new(ParentImporter));
    let mut importer = GlowImporter::new(subdoc, shape.clone(), theme, None, parent, false);
    importer.createGlowContext(&Attrs::default().with("rad", "12700"));
    importer.createSrgbClr(&Attrs::default().with("val", "123456"));
    importer.setAlpha(&Attrs::default().with("val", "50000"));
    importer.setSrgbClr();
    assert!(shape.borrow().mClrTransform.is_none());
    importer.excute();
    let result = importer.result().expect("Glow result");
    assert_eq!(result.borrow().getGlowSize(), 1);
    assert_eq!(result.borrow().getGlowOpacity(), 0.5);
    assert_eq!(result.borrow().getGlowColor(), ColorsTrans::rgb2Argb(0x123456, 0.5));
    importer.clearContext();
    assert!(importer.result().is_none());
    assert!(shape.borrow().mGlow.is_none());
    assert_eq!(result.borrow().getGlowSize(), 1);
}
