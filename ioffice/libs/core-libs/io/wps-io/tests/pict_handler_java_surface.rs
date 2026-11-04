use core_libs_io::cn::wps::io::xwpf::element_handler::RHandler::RHandler;
use core_libs_io::cn::wps::io::xwpf::element_handler::DocumentHandler::DocumentHandler;
use core_libs_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use core_libs_io::cn::wps::io::xwpf::IVmlImporter::{IVmlImporter, VmlImporterRef};
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

struct RecordingVml {
    events: Rc<RefCell<Vec<String>>>,
}

#[test]
fn java_document_body_paragraph_run_keeps_same_part_for_pict() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let vml: VmlImporterRef = Rc::new(RefCell::new(RecordingVml { events: events.clone() }));
    let importer = Rc::new(RecordingImporter { vml });
    let document = DocumentHandler::new_rc(importer, Some(Box::new("document-part")));
    let body = document.getElementHandler(3029410, "").expect("Java body");
    let paragraph = body.borrow().getElementHandler(112, "").expect("Java paragraph");
    let run = paragraph.borrow().getElementHandler(114, "").expect("Java run");
    let pict = run.borrow().getElementHandler(3440682, "").expect("Java pict");
    pict.borrow_mut().onStart_with_id(3440682, "", "", "", &()).unwrap();
    assert_eq!(events.borrow().as_slice(), ["vml-start"]);
}

impl IImporterBase for RecordingVml { fn mode(&self) -> i32 { 0 } }

impl IVmlImporter for RecordingVml {
    fn getDocumentImporter(&self) -> &dyn Any { &() }
    fn onImportVmlStart(&mut self, part: &dyn Any, prop: &dyn Any) {
        assert_eq!(part.downcast_ref::<&str>(), Some(&"document-part"));
        assert!(prop.downcast_ref::<MutablePropertySet>().is_some());
        self.events.borrow_mut().push("vml-start".into());
    }
    fn onImportVmlEnd(&mut self) { self.events.borrow_mut().push("vml-end".into()); }
    fn onImportPictChildStart(&mut self, id: i32, _: &dyn Any) { self.events.borrow_mut().push(format!("shape-start:{id}")); }
    fn onImportPictChildEnd(&mut self, id: i32) { self.events.borrow_mut().push(format!("shape-end:{id}")); }
    fn onImportEGShapeElementsStart(&mut self, id: i32, _: &str, _: &dyn Any) { self.events.borrow_mut().push(format!("eg:{id}")); }
    fn onImportTextboxEnd(&mut self, _: bool) {}
    fn onImportWBackgroundStart(&mut self, _: &dyn Any, _: i32, _: &dyn Any) {}
    fn onImportVBackgroundStart(&mut self, _: i32, _: &str, _: &dyn Any) {}
    fn onImportBackgroundEnd(&mut self, _: i32, _: &str) {}
    fn onImportInk(&mut self, _: &dyn Any) {}
    fn onImportOLEObject(&mut self, _: &dyn Any) { self.events.borrow_mut().push("ole".into()); }
    fn onImportControlObject(&mut self, _: &dyn Any) {}
}

struct RecordingImporter { vml: VmlImporterRef }
impl IImporterBase for RecordingImporter { fn mode(&self) -> i32 { 0 } }
impl IDocumentImporter for RecordingImporter {
    fn getVmlImporter(&self, sub_doc_type: i32) -> Option<VmlImporterRef> {
        assert_eq!(sub_doc_type, 0);
        Some(self.vml.clone())
    }
}

#[test]
fn java_pict_shape_eg_and_ole_callbacks_keep_part_and_run_properties() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let vml: VmlImporterRef = Rc::new(RefCell::new(RecordingVml { events: events.clone() }));
    let importer = Rc::new(RecordingImporter { vml });
    let run = RHandler::new(Box::new("document-part"), importer, 0, 0);
    let pict = run.borrow().getElementHandler(-1023368385, "").expect("Java PictHandler");
    let same_pict = run.borrow().getElementHandler(3440682, "").expect("cached PictHandler");
    assert!(Rc::ptr_eq(&pict, &same_pict));
    pict.borrow_mut().onStart_with_id(-1023368385, "", "", "", &()).unwrap();
    let shape = pict.borrow().getElementHandler(109399969, "").expect("Java ShapeHandler");
    shape.borrow_mut().onStart_with_id(109399969, "", "", "", &()).unwrap();
    // VMap.java: "imagedata" → -877694075.
    let eg = shape.borrow().getElementHandler(-877694075, "").expect("Java EGShapeElementsHandler");
    eg.borrow_mut().onStart_with_id(-877694075, "urn:schemas-microsoft-com:vml", "", "", &()).unwrap();
    shape.borrow_mut().onEnd_with_id(109399969, "").unwrap();
    let ole = pict.borrow().getElementHandler(1718295399, "").expect("Java OLEObjectHandler");
    ole.borrow_mut().onStart_with_id(1718295399, "", "", "", &()).unwrap();
    pict.borrow_mut().onEnd_with_id(-1023368385, "").unwrap();
    assert_eq!(events.borrow().as_slice(), ["vml-start", "shape-start:109399969", "eg:-877694075", "shape-end:109399969", "ole", "vml-end"]);
}
