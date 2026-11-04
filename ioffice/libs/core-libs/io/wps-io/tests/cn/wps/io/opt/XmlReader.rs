// function_id=06_function_xml_elementid
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 06_function_xml_elementid
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opt/XmlReader.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opt/XmlReader.rs

use core_libs_io::io::opt::XmlReader::{XmlReader, DEFAULT_ENCODING};
use core_libs_io::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use core_libs_io::io::simple::ElementIds;
use std::any::Any;
use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

/// Java String.hashCode: s[0]*31^(n-1) + ... + s[n-1]
fn java_string_hash(s: &str) -> i32 {
    s.bytes().fold(0i32, |h, b| h.wrapping_mul(31).wrapping_add(b as i32))
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Ev {
    Start(i32),
    Chars(String),
    End(i32),
}

struct Recording {
    events: Rc<RefCell<Vec<Ev>>>,
}

impl ElementHandlerSimple for Recording {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        -1
    }
    fn onStart(
        &mut self,
        _var1: &str,
        _var2: &str,
        _var3: &str,
        _var4: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onEnd(
        &mut self,
        _var1: &str,
        _var2: &str,
        _var3: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onStart_with_id(
        &mut self,
        id: i32,
        _ns: &str,
        _local: &str,
        _qname: &str,
        _attrs: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.events.borrow_mut().push(Ev::Start(id));
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        id: i32,
        _ns: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.events.borrow_mut().push(Ev::End(id));
        Ok(())
    }
    fn onCharacters(
        &mut self,
        value: &[char],
        start: i32,
        len: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let s = start as usize;
        let l = len as usize;
        let text: String = value.get(s..s + l).unwrap_or(&[]).iter().collect();
        self.events.borrow_mut().push(Ev::Chars(text));
        Ok(())
    }
    fn onCharacters_str(&mut self, persist_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.events
            .borrow_mut()
            .push(Ev::Chars(format!("path:{persist_path}")));
        Ok(())
    }
    fn getElementHandler(&self, _id: i32, _ns: &str) -> Option<ElementHandlerRef> {
        Some(Rc::new(RefCell::new(Recording {
            events: self.events.clone(),
        })))
    }
}

fn parse_bytes(bytes: &[u8], encoding: &str, ignore: i32) -> Vec<Ev> {
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut handler: ElementHandlerRef = Rc::new(RefCell::new(Recording {
        events: events.clone(),
    }));
    let mut cur = Cursor::new(bytes.to_vec());
    XmlReader::new()
        .parse(&mut cur, &mut handler, encoding, ignore, 0)
        .expect("parse");
    let out = events.borrow().clone();
    out
}

fn parse_xml(xml: &str, ignore: i32) -> Vec<Ev> {
    parse_bytes(xml.as_bytes(), "utf-8", ignore)
}

fn t_text(events: &[Ev]) -> Option<String> {
    let mut inside = false;
    let mut acc = String::new();
    let mut saw = false;
    for e in events {
        match e {
            Ev::Start(id) if *id == ElementIds::T => {
                inside = true;
                saw = true;
            }
            Ev::End(id) if *id == ElementIds::T => {
                inside = false;
            }
            Ev::Chars(s) if inside => acc.push_str(s),
            _ => {}
        }
    }
    if saw {
        Some(acc)
    } else {
        None
    }
}

fn assert_t_hi(events: &[Ev]) {
    assert!(
        events.iter().any(|e| matches!(e, Ev::Start(id) if *id == ElementIds::T)),
        "missing start(t=116): {events:?}"
    );
    assert_eq!(t_text(events).as_deref(), Some("Hi"), "{events:?}");
    assert!(
        events.iter().any(|e| matches!(e, Ev::End(id) if *id == ElementIds::T)),
        "missing end(t=116): {events:?}"
    );
}

#[test]
fn e1_wt_text_events() {
    let events = parse_xml("<w:t>Hi</w:t>", -1);
    assert_t_hi(&events);
}

#[test]
fn e2_bom_not_in_text() {
    let mut bytes = vec![0xEF, 0xBB, 0xBF];
    bytes.extend_from_slice(b"<w:t>Hi</w:t>");
    let events = parse_bytes(&bytes, "utf-8", -1);
    assert_t_hi(&events);
    assert!(
        !events.iter().any(|e| matches!(e, Ev::Chars(s) if s.contains('\u{feff}'))),
        "BOM leaked into characters: {events:?}"
    );
}

#[test]
fn e3_p_and_t_hash_match_java() {
    assert_eq!(java_string_hash("p"), 112);
    assert_eq!(java_string_hash("t"), 116);
    assert_eq!(java_string_hash("b"), 98);
    assert_eq!(ElementIds::P, 112);
    assert_eq!(ElementIds::T, 116);
    assert_eq!(ElementIds::RPR_B, 98);
}

#[test]
fn e4_empty_xml_no_events() {
    let events = parse_bytes(b"", "utf-8", -1);
    assert!(events.is_empty(), "{events:?}");
}

#[test]
fn e5_nested_p_and_t() {
    let events = parse_xml("<w:p><w:t>Hi</w:t></w:p>", -1);
    assert_t_hi(&events);
    let ids: Vec<&Ev> = events
        .iter()
        .filter(|e| matches!(e, Ev::Start(_) | Ev::End(_)))
        .collect();
    assert_eq!(
        ids,
        vec![
            &Ev::Start(ElementIds::P),
            &Ev::Start(ElementIds::T),
            &Ev::End(ElementIds::T),
            &Ev::End(ElementIds::P),
        ]
    );
}

#[test]
fn e6_ignore_t_still_parses_p() {
    let events = parse_xml("<w:p><w:t>Hi</w:t></w:p>", ElementIds::T);
    assert!(
        events.iter().any(|e| matches!(e, Ev::Start(id) if *id == ElementIds::P)),
        "ignore must not skip XmlParse: {events:?}"
    );
    assert!(
        events.iter().any(|e| matches!(e, Ev::End(id) if *id == ElementIds::P)),
        "missing end(p): {events:?}"
    );
}

#[test]
fn e7_self_closing_t() {
    let events = parse_xml("<w:t/>", -1);
    assert_eq!(
        events,
        vec![Ev::Start(ElementIds::T), Ev::End(ElementIds::T)]
    );
}

#[test]
fn e8_amp_entity_in_text() {
    let events = parse_xml("<w:t>A&amp;B</w:t>", -1);
    assert_eq!(t_text(&events).as_deref(), Some("A&B"), "{events:?}");
}

#[test]
fn e9_parse4_hw_memo() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut handler: ElementHandlerRef = Rc::new(RefCell::new(Recording {
        events: events.clone(),
    }));
    let mut cur = Cursor::new(b"<w:t>Hi</w:t>".to_vec());
    XmlReader::new()
        .parse4HwMemo(&mut cur, &mut handler, "utf-8")
        .expect("parse4HwMemo");
    let events = events.borrow().clone();
    assert_t_hi(&events);
}

#[test]
fn e10_default_encoding_utf8() {
    assert_eq!(DEFAULT_ENCODING, "utf-8");
}

#[test]
fn e11_utf16_le_bom() {
    let xml = "<w:t>Hi</w:t>";
    let mut bytes = vec![0xFF, 0xFE];
    for u in xml.encode_utf16() {
        bytes.extend_from_slice(&u.to_le_bytes());
    }
    let events = parse_bytes(&bytes, "utf-16", -1);
    assert_t_hi(&events);
}

#[test]
fn e12_xml_declaration_then_t() {
    let events = parse_xml("<?xml version=\"1.0\"?><w:t>Hi</w:t>", -1);
    assert_t_hi(&events);
}
