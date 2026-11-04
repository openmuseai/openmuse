use ss_core::java_runtime::RefCell;
use std::io::Cursor;
use ss_core::java_runtime::Rc;

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use ss_core::cn::wps::moss::xlsxr::util::UnifiedXmlReader::UnifiedXmlReader;
use ss_core::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;

struct Rec {
    events: Rc<RefCell<Vec<String>>>,
}

impl Rec {
    fn push(&self, line: String) {
        self.events.borrow_mut().push(line);
    }
}

impl ElementHandler for Rec {
    fn startElement(&self, id: i32, attr: &dyn XmlRoAttr) {
        let mut line = format!("start {id}");
        for i in 0..attr.count() {
            let name = attr.get_name_at(i);
            let value = attr
                .get_attr_at(i)
                .map(|leaf| leaf.to_string())
                .unwrap_or_default();
            line.push_str(&format!(" {name}={value}"));
        }
        self.push(line);
    }

    fn endElement(&self, id: i32) {
        self.push(format!("end {id}"));
    }

    fn characters(&self, text: &str) {
        self.push(format!("text {text}"));
    }

    fn enterSubElement(&self, _id: i32) -> Option<Box<dyn ElementHandler>> {
        Some(Box::new(Rec {
            events: Rc::clone(&self.events),
        }))
    }
}

fn collect(xml: &str) -> Vec<String> {
    let events = Rc::new(RefCell::new(Vec::new()));
    let handler = Rec {
        events: Rc::clone(&events),
    };
    UnifiedXmlReader::read(&mut Cursor::new(xml.as_bytes()), Box::new(handler)).unwrap();
    let out = events.borrow().clone();
    out
}

#[test]
fn xml1_workbook_ids_and_attrs() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets>
    <sheet name="Sheet1" sheetId="1" r:id="rId1"/>
  </sheets>
</workbook>
"#;
    let events = collect(xml);
    assert!(
        events.iter().any(|e| e.starts_with("start 5252")),
        "{events:?}"
    );
    assert!(events.iter().any(|e| e.starts_with("start 5258")), "{events:?}");
    assert!(
        events.iter().any(|e| e.contains("start 4329")
            && e.contains("4217=Sheet1")
            && e.contains("4704=1")
            && e.contains("4259841=rId1")),
        "{events:?}"
    );
    assert!(events.iter().any(|e| e == "end 4329"), "{events:?}");
}

#[test]
fn xml2_text_and_fixture_workbook() {
    let events = collect("<t>hi</t>");
    assert!(events.iter().any(|e| e.starts_with("start ")), "{events:?}");
    assert!(events.iter().any(|e| e == "text hi"), "{events:?}");

    let path = format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/basic-types.xlsx",
        env!("CARGO_MANIFEST_DIR")
    );
    let mut pack = OxfrPackage::OxfrPackage(&path).unwrap();
    let book = XlsxrPackage::getBookPart(&mut pack);
    let mut stream = book.getInputStream_default().unwrap();
    let events = Rc::new(RefCell::new(Vec::new()));
    UnifiedXmlReader::read(
        &mut stream,
        Box::new(Rec {
            events: Rc::clone(&events),
        }),
    )
    .unwrap();
    let events = events.borrow().clone();
    assert!(
        events.iter().any(|e| e.starts_with("start 5252")),
        "{events:?}"
    );
    assert!(
        events.iter().any(|e| e.contains("start 4329")),
        "{events:?}"
    );
}
