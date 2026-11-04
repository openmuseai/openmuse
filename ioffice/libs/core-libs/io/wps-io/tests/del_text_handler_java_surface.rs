use core_libs_io::cn::wps::io::xwpf::element_handler::RHandler::RHandler;
use core_libs_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct RecordingImporter(RefCell<Vec<(i32, String, bool)>>);

impl IImporterBase for RecordingImporter {
    fn mode(&self) -> i32 { 0 }
}

impl IDocumentImporter for RecordingImporter {
    fn onImportRun_str(&self, sub_doc_type: i32, prop: &dyn Any, text: &str) {
        self.0.borrow_mut().push((sub_doc_type, text.to_string(), prop.is::<MutablePropertySet>()));
    }
}

#[test]
fn java_deleted_text_is_dispatched_to_the_real_run_callback() {
    let importer = Rc::new(RecordingImporter::default());
    let run = RHandler::new(Box::new(()), importer.clone(), 0, 0);
    let handler = run.borrow().getElementHandler(1549942744, "").expect("w:delText handler");
    let characters: Vec<char> = "_deleted_".chars().collect();
    handler.borrow_mut().onCharacters(&characters, 1, 7).unwrap();
    assert_eq!(&*importer.0.borrow(), &[(0, "deleted".to_string(), true)]);
}
