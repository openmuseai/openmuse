use core_libs_io::cn::wps::io::xwpf::element_handler::RHandler::RHandler;
use core_libs_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use core_libs_io::cn::wps::io::opt::AttributesImpl::AttributesImpl;
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
struct RecordingImporter {
    received: RefCell<Vec<(i32, i32, bool, bool)>>,
}

impl IImporterBase for RecordingImporter {
    fn mode(&self) -> i32 { 0 }
}

impl IDocumentImporter for RecordingImporter {
    fn onImportRunContent(&self, sub_doc_type: i32, id: i32, prop: &dyn Any, attrs: &dyn Any) {
        self.received.borrow_mut().push((
            sub_doc_type,
            id,
            prop.downcast_ref::<MutablePropertySet>().is_some(),
            attrs.downcast_ref::<AttributesImpl>().is_some(),
        ));
    }
}

#[test]
fn java_content_handler_dispatches_five_run_content_elements() {
    let importer = Rc::new(RecordingImporter::default());
    let run = RHandler::new(Box::new(()), importer.clone(), 0, 0);
    let attrs = AttributesImpl::new_with_maps(HashMap::new(), None);
    for id in [-784674076, -777505580, 114375, 429190931, 692063750] {
        let handler = run.borrow().getElementHandler(id, "").expect("Java ContentHandler branch");
        handler.borrow_mut().onStart_with_id(id, "", "", "", &attrs).expect("handler onStart");
        assert_eq!(importer.received.borrow().last(), Some(&(0, id, true, true)));
    }
    assert_eq!(importer.received.borrow().len(), 5);
}
