// function_id=08_function_xml_dispatch
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 08_function_xml_dispatch
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/simple/DispatchHandler2.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/simple/DispatchHandler2.rs

use core_libs_io::io::dom::io::WMap::WMap;
use core_libs_io::io::opt::ContentHandlerImpl::ContentHandlerImpl;
use core_libs_io::io::simple::DispatchHandler2::DispatchHandler2;
use core_libs_io::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use core_libs_io::io::simple::ElementIds;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Ev {
    Start(i32),
    Chars(String),
    End(i32),
}

struct Recording {
    events: Rc<RefCell<Vec<Ev>>>,
    child_for: Option<i32>,
    child: Option<ElementHandlerRef>,
}

impl Recording {
    fn leaf(events: Rc<RefCell<Vec<Ev>>>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Recording {
            events,
            child_for: None,
            child: None,
        }))
    }

    fn parent(events: Rc<RefCell<Vec<Ev>>>, child_for: i32, child: ElementHandlerRef) -> ElementHandlerRef {
        Rc::new(RefCell::new(Recording {
            events,
            child_for: Some(child_for),
            child: Some(child),
        }))
    }
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
        _a: &str,
        _b: &str,
        _c: &str,
        _d: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onEnd(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onStart_with_id(
        &mut self,
        id: i32,
        _ns: &str,
        _local: &str,
        _q: &str,
        _attrs: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.events.borrow_mut().push(Ev::Start(id));
        Ok(())
    }
    fn onEnd_with_id(&mut self, id: i32, _ns: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    fn getElementHandler(&self, id: i32, _ns: &str) -> Option<ElementHandlerRef> {
        if self.child_for == Some(id) {
            self.child.clone()
        } else {
            None
        }
    }
}

fn w_ns() -> &'static str {
    WMap::MAIN_NAMESPACING()
}

fn p_then_t(events: Rc<RefCell<Vec<Ev>>>) -> ElementHandlerRef {
    let t = Recording::leaf(events.clone());
    Recording::parent(events, ElementIds::T, t)
}

#[test]
fn d1_nested_p_then_t() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let p = p_then_t(events.clone());
    let mut d = DispatchHandler2::new();
    d.addHandler(ElementIds::P, p);
    d.onStart_str(w_ns(), "p", "w:p", &());
    d.onStart_str(w_ns(), "t", "w:t", &());
    d.onEnd_str(w_ns(), "t", "w:t");
    d.onEnd_str(w_ns(), "p", "w:p");
    let got = events.borrow().clone();
    assert_eq!(
        got,
        vec![
            Ev::Start(ElementIds::P),
            Ev::Start(ElementIds::T),
            Ev::End(ElementIds::T),
            Ev::End(ElementIds::P),
        ]
    );
}

#[test]
fn d2_unknown_id_does_not_panic() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let p = p_then_t(events.clone());
    let mut d = DispatchHandler2::new();
    d.addHandler(ElementIds::P, p);
    d.onStart_str(w_ns(), "unknown", "w:unknown", &());
    d.onEnd_str(w_ns(), "unknown", "w:unknown");
    assert!(events.borrow().is_empty());
}

#[test]
fn d3_content_handler_impl_nested() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let p = p_then_t(events.clone());
    let mut ch = ContentHandlerImpl::new(p);
    ch.startElement(ElementIds::P, w_ns(), "p", "w:p", &());
    ch.startElement(ElementIds::T, w_ns(), "t", "w:t", &());
    ch.endElement(ElementIds::T, w_ns(), "t", "w:t");
    ch.endElement(ElementIds::P, w_ns(), "p", "w:p");
    let got = events.borrow().clone();
    assert_eq!(
        got,
        vec![
            Ev::Start(ElementIds::P),
            Ev::Start(ElementIds::T),
            Ev::End(ElementIds::T),
            Ev::End(ElementIds::P),
        ]
    );
}

#[test]
fn d4_characters_go_to_t() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let p = p_then_t(events.clone());
    let mut d = DispatchHandler2::new();
    d.addHandler(ElementIds::P, p);
    d.onStart_str(w_ns(), "p", "w:p", &());
    d.onStart_str(w_ns(), "t", "w:t", &());
    let chars: Vec<char> = "Hi".chars().collect();
    d.onCharacters_buf(&chars, 0, 2);
    d.onEnd_str(w_ns(), "t", "w:t");
    d.onEnd_str(w_ns(), "p", "w:p");
    let got = events.borrow().clone();
    assert!(got.contains(&Ev::Chars("Hi".into())), "{got:?}");
    assert_eq!(got.iter().filter(|e| matches!(e, Ev::Chars(_))).count(), 1);
}

#[test]
fn d5_reset_handlers_clears_table() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let p = p_then_t(events.clone());
    let mut d = DispatchHandler2::new();
    d.addHandler(ElementIds::P, p);
    d.resetHandlers();
    d.onStart_str(w_ns(), "p", "w:p", &());
    d.onEnd_str(w_ns(), "p", "w:p");
    assert!(events.borrow().is_empty());
}

#[test]
fn d6_unknown_child_does_not_panic() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let p = p_then_t(events.clone());
    let mut d = DispatchHandler2::new();
    d.addHandler(ElementIds::P, p);
    d.onStart_str(w_ns(), "p", "w:p", &());
    d.onStart_str(w_ns(), "r", "w:r", &());
    d.onEnd_str(w_ns(), "r", "w:r");
    d.onEnd_str(w_ns(), "p", "w:p");
    let got = events.borrow().clone();
    assert_eq!(
        got,
        vec![Ev::Start(ElementIds::P), Ev::End(ElementIds::P)]
    );
}

#[test]
fn d7_characters_str_to_stack_top() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let p = p_then_t(events.clone());
    let mut d = DispatchHandler2::new();
    d.addHandler(ElementIds::P, p);
    d.onStart_str(w_ns(), "p", "w:p", &());
    d.onCharacters_str("tmp");
    d.onEnd_str(w_ns(), "p", "w:p");
    assert!(events.borrow().contains(&Ev::Chars("path:tmp".into())));
}

#[test]
fn d8_filter_index_zero() {
    assert_eq!(DispatchHandler2::new().filterIndex(), 0);
}

#[test]
fn d9_root_table_miss() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let t = Recording::leaf(events.clone());
    let mut d = DispatchHandler2::new();
    d.addHandler(ElementIds::T, t);
    d.onStart_str(w_ns(), "p", "w:p", &());
    d.onEnd_str(w_ns(), "p", "w:p");
    assert!(events.borrow().is_empty());
}

#[test]
fn d10_content_handler_unknown_child() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let p = p_then_t(events.clone());
    let mut ch = ContentHandlerImpl::new(p);
    ch.startElement(ElementIds::P, w_ns(), "p", "w:p", &());
    ch.startElement(99, w_ns(), "x", "w:x", &());
    ch.endElement(99, w_ns(), "x", "w:x");
    ch.endElement(ElementIds::P, w_ns(), "p", "w:p");
    let got = events.borrow().clone();
    assert_eq!(
        got,
        vec![Ev::Start(ElementIds::P), Ev::End(ElementIds::P)]
    );
}
