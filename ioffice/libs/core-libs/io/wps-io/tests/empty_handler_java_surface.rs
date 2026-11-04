use core_libs_io::cn::wps::io::simple::ElementHandlerSimple::ElementHandlerSimple;
use core_libs_io::cn::wps::io::xwpf::element_handler::RHandler::RHandler;
use core_libs_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;

#[derive(Default)]
struct RecordingImporter {
    received: RefCell<Vec<(i32, String, bool)>>,
}

impl IImporterBase for RecordingImporter {
    fn mode(&self) -> i32 { 0 }
}

impl IDocumentImporter for RecordingImporter {
    fn onImportRun_str(&self, sub_doc_type: i32, prop: &dyn Any, text: &str) {
        self.received.borrow_mut().push((
            sub_doc_type,
            text.to_string(),
            prop.downcast_ref::<MutablePropertySet>().is_some(),
        ));
    }
}

#[test]
fn java_empty_handler_dispatches_control_runs_through_rhandler() {
    let importer = Rc::new(RecordingImporter::default());
    let run = RHandler::new(Box::new(()), importer.clone(), 0, 0);
    for (id, expected) in [
        (-462474756, "\u{001f}"),
        (3183, "\r"),
        (114581, "\t"),
        (2001013392, "\u{001e}"),
    ] {
        let handler = run.borrow().getElementHandler(id, "").expect("Java EmptyHandler branch");
        handler.borrow_mut().onEnd_with_id(id, "").expect("handler onEnd");
        assert_eq!(importer.received.borrow().last().unwrap(), &(0, expected.to_string(), true));
    }
    assert_eq!(importer.received.borrow().len(), 4);
}
