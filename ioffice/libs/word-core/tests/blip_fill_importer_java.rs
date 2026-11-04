use std::{cell::RefCell, rc::Rc, sync::Arc};
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use word_core::cn::wps::moffice::writer::io::reader::dmlReader::{
    BlipFillImporter::BlipFillImporter,
    context::{DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
        ThemeContext::ThemeContext},
};

struct Parent;
impl IImporterBase for Parent { fn mode(&self) -> i32 { 0 } }

struct Attrs(Vec<(&'static str, &'static str)>);
impl SaxAttributes for Attrs {
    fn getLength(&self) -> usize { self.0.len() }
    fn getURI(&self, _: usize) -> Option<String> { None }
    fn getLocalName(&self, i: usize) -> Option<String> { self.0.get(i).map(|x| x.0.into()) }
    fn getQName(&self, i: usize) -> Option<String> { self.getLocalName(i) }
    fn getType_by_index(&self, _: usize) -> Option<String> { None }
    fn getValue_by_index(&self, i: usize) -> Option<String> { self.0.get(i).map(|x| x.1.into()) }
    fn getIndex_by_uri(&self, _: &str, name: &str) -> i32 { self.getIndex_by_qname(name) }
    fn getIndex_by_qname(&self, name: &str) -> i32 { self.0.iter().position(|x| x.0 == name).map(|i| i as i32).unwrap_or(-1) }
    fn getType_by_uri(&self, _: &str, _: &str) -> Option<String> { None }
    fn getType_by_qname(&self, _: &str) -> Option<String> { None }
    fn getValue_by_uri(&self, _: &str, name: &str) -> Option<String> { self.getValue_by_qname(name) }
    fn getValue_by_qname(&self, name: &str) -> Option<String> { self.0.iter().find(|x| x.0 == name).map(|x| x.1.into()) }
}

#[test]
fn blip_fill_matches_java_context_transfer_and_target_lifetime() {
    let shape = Rc::new(RefCell::new(DmlShapeCtxt::new()));
    let mut importer = BlipFillImporter::new(
        Rc::new(RefCell::new(DmlSubDocCtxt::<()>::new())), shape.clone(),
        Rc::new(RefCell::new(ThemeContext::default())), None,
        Rc::new(RefCell::new(Parent)));
    importer.setFillType(2);
    importer.setFillRect(&Attrs(vec![("l", "10000"), ("t", "20000")]));
    importer.excute();
    let result = importer.result().expect("Java initTarget creates BlipFill");
    assert_eq!(result.lock().unwrap().getFillType(), 2);
    let rect = result.lock().unwrap().getFillRect().unwrap();
    assert_eq!((rect.left, rect.top, rect.right, rect.bottom), (0.1, 0.2, 0.1, 0.2));
    assert!(shape.borrow().mFillType.is_none());
    assert!(shape.borrow().mFillRect.is_none());
    importer.excute();
    assert!(Arc::ptr_eq(&result, &importer.result().unwrap()));
    importer.clearContext();
    assert!(importer.result().is_none());
    assert_eq!(result.lock().unwrap().getFillType(), 2);
}

#[test]
fn blip_fill_effects_use_java_percent_and_brightness_conversion() {
    let mut importer = BlipFillImporter::new(
        Rc::new(RefCell::new(DmlSubDocCtxt::<()>::new())),
        Rc::new(RefCell::new(DmlShapeCtxt::new())),
        Rc::new(RefCell::new(ThemeContext::default())), None,
        Rc::new(RefCell::new(Parent)));
    importer.initTarget();
    importer.importBiLevelThresh(&Attrs(vec![("thresh", "45000")]));
    importer.importLum(&Attrs(vec![("bright", "100000"), ("contrast", "-50000")]));
    importer.importGrayscl();
    let result = importer.result().unwrap();
    let mut fill = result.lock().unwrap();
    assert!(fill.getPicBiLevel());
    assert_eq!(fill.getBiLevelThresh(), 0.45);
    assert_eq!(fill.getPicBrightness(), 100.0);
    assert_eq!(fill.getPicContrast(), 25.0);
    assert!(fill.getPictureGray());
}
