use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use ppt_core::cn::wps::show::pptxr::util::UnifiedXmlReader::UnifiedXmlReader;

struct RecordingHandler {
    starts: RefCell<Vec<i32>>,
    ends: RefCell<Vec<i32>>,
    calls: RefCell<i32>,
}

impl Default for RecordingHandler {
    fn default() -> Self {
        Self {
            starts: RefCell::new(Vec::new()),
            ends: RefCell::new(Vec::new()),
            calls: RefCell::new(0),
        }
    }
}

#[derive(Clone, Default)]
struct RcHandler {
    starts: Rc<RefCell<Vec<i32>>>,
    ends: Rc<RefCell<Vec<i32>>>,
}

impl ElementHandler for RcHandler {
    fn startElement(&self, var1: i32, _var2: &dyn XmlRoAttr) {
        self.starts.borrow_mut().push(var1);
    }
    fn endElement(&self, var1: i32) {
        self.ends.borrow_mut().push(var1);
    }
    fn characters(&self, _var1: &str) {}
    fn enterSubElement(&self, _var1: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

impl ElementHandler for RecordingHandler {
    fn startElement(&self, var1: i32, _var2: &dyn XmlRoAttr) {
        *self.calls.borrow_mut() += 1;
        self.starts.borrow_mut().push(var1);
    }
    fn endElement(&self, var1: i32) {
        *self.calls.borrow_mut() += 1;
        self.ends.borrow_mut().push(var1);
    }
    fn characters(&self, _var1: &str) {}
    fn enterSubElement(&self, _var1: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

const SLD: &[u8] = br#"<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"/>"#;

#[test]
fn t01_stream_overload_parser_once() {
    let handler = RecordingHandler::default();
    let mut cur = Cursor::new(SLD);
    UnifiedXmlReader::read__inputstream_elementhandler(&mut cur, Box::new(handler))
        .expect("stream read");
}

#[test]
fn t04_stream_emits_sld_element() {
    let handler = RcHandler::default();
    let mut cur = Cursor::new(SLD);
    UnifiedXmlReader::read__inputstream_elementhandler(&mut cur, Box::new(handler.clone()))
        .expect("stream read");
    assert_eq!(handler.starts.borrow().as_slice(), &[3211440]);
    assert_eq!(handler.ends.borrow().as_slice(), &[3211440]);
}

#[test]
fn t02_path_overload_and_missing_file() {
    let dir = std::env::temp_dir().join("01_function_unified_xml_reader");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("sld.xml");
    std::fs::write(&path, SLD).unwrap();
    UnifiedXmlReader::read__string_elementhandler(
        path.to_str().unwrap(),
        Box::new(RecordingHandler::default()),
    )
    .expect("path read");
    let missing = dir.join("no-such.xml");
    let err = UnifiedXmlReader::read__string_elementhandler(
        missing.to_str().unwrap(),
        Box::new(RecordingHandler::default()),
    );
    assert!(err.is_err());
}

#[test]
fn t03_path_slice_begin_end() {
    let dir = std::env::temp_dir().join("01_function_unified_xml_reader");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("sld-slice.xml");
    std::fs::write(&path, SLD).unwrap();
    let end = SLD.len() as i32;
    UnifiedXmlReader::read__string_int_int_elementhandler_boolean(
        path.to_str().unwrap(),
        0,
        end,
        Box::new(RecordingHandler::default()),
        true,
    )
    .expect("slice read");
}
