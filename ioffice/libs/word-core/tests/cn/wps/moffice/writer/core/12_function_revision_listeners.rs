// function_id=12_function_revision_listeners
// cargo test --manifest-path libs/word-core/Cargo.toml --test 12_function_revision_listeners

use std::cell::Cell;
use std::path::PathBuf;
use std::process::Command;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::core::Document::{
    Document, HEADER_DOCUMENT, MAIN_DOCUMENT,
};
use word_core::cn::wps::moffice::writer::core::RevisionListener::{
    RevisionChange, RevisionListener, RevisionType,
};
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;

fn hello_world() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/p1_layout_document/hello-world.docx")
}

fn sha256_file(path: &PathBuf) -> String {
    let out = Command::new("shasum")
        .args(["-a", "256"])
        .arg(path)
        .output()
        .expect("shasum");
    String::from_utf8(out.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}

struct Probe {
    before_all: Cell<i32>,
    after_all: Cell<i32>,
    before: Cell<i32>,
    after: Cell<i32>,
}

impl Probe {
    fn new() -> Self {
        Self {
            before_all: Cell::new(0),
            after_all: Cell::new(0),
            before: Cell::new(0),
            after: Cell::new(0),
        }
    }
}

impl RevisionListener for Probe {
    fn beforeChangeAll(&mut self, _var1: i32, _var2: i32, _var3: &RevisionChange) {
        self.before_all.set(self.before_all.get() + 1);
    }
    fn afterChangeAll(&mut self, _var1: i32, _var2: i32, _var3: &RevisionChange) {
        self.after_all.set(self.after_all.get() + 1);
    }
    fn beforeChange(
        &mut self,
        _var1: i32,
        _var2: i32,
        _var3: &RevisionType,
        _var4: &RevisionChange,
    ) {
        self.before.set(self.before.get() + 1);
    }
    fn afterChange(
        &mut self,
        _var1: i32,
        _var2: i32,
        _var3: &RevisionType,
        _var4: &RevisionChange,
    ) {
        self.after.set(self.after.get() + 1);
    }
}

#[test]
fn RL1_add_then_fire_before_all() {
    let mut td = TextDocument::new();
    let sd = td.getSubdocumentMut(MAIN_DOCUMENT).expect("main");
    let mut probe = Probe::new();
    sd.addRevisionListener(&probe);
    sd.fireBeforeChangeAll(0, 1, &RevisionChange::ACCEPT);
    assert_eq!(probe.before_all.get(), 1);
    let _ = &mut probe;
}

#[test]
fn RL2_remove_stops_delivery() {
    let mut td = TextDocument::new();
    let sd = td.getSubdocumentMut(MAIN_DOCUMENT).expect("main");
    let mut probe = Probe::new();
    sd.addRevisionListener(&probe);
    sd.removeRevisionListener(&probe);
    sd.fireAfterChangeAll(0, 1, &RevisionChange::REJECT);
    assert_eq!(probe.after_all.get(), 0);
    let _ = &mut probe;
}

#[test]
fn RL3_two_listeners_both_fire() {
    let mut td = TextDocument::new();
    let sd = td.getSubdocumentMut(MAIN_DOCUMENT).expect("main");
    let mut a = Probe::new();
    let mut b = Probe::new();
    sd.addRevisionListener(&a);
    sd.addRevisionListener(&b);
    sd.fireBeforeChangeAll(2, 4, &RevisionChange::ACCEPT);
    assert_eq!(a.before_all.get(), 1);
    assert_eq!(b.before_all.get(), 1);
    let _ = (&mut a, &mut b);
}

#[test]
fn RL4_fire_before_change_when_not_change_all() {
    let mut td = TextDocument::new();
    let sd = td.getSubdocumentMut(MAIN_DOCUMENT).expect("main");
    let mut probe = Probe::new();
    sd.addRevisionListener(&probe);
    sd.fireBeforeChange(0, 3, &RevisionType::INSERT, &RevisionChange::ACCEPT);
    sd.fireAfterChange(0, 3, &RevisionType::DELETE, &RevisionChange::REJECT);
    assert_eq!(probe.before.get(), 1);
    assert_eq!(probe.after.get(), 1);
    let _ = &mut probe;
}

#[test]
fn RL5_header_story_accepts_listener() {
    let mut td = TextDocument::new();
    let sd = td.getSubdocumentMut(HEADER_DOCUMENT).expect("header");
    let mut probe = Probe::new();
    sd.addRevisionListener(&probe);
    sd.fireAfterChangeAll(0, 1, &RevisionChange::ACCEPT);
    assert_eq!(probe.after_all.get(), 1);
    let _ = &mut probe;
}

#[test]
fn RL6_pipeline_hello_world_fire_empty_ok() {
    let path = hello_world();
    assert_eq!(
        sha256_file(&path),
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let mut td = open_text_document(&bytes).expect("open_text_document");
    let sd = td.getSubdocumentMut(MAIN_DOCUMENT).expect("parsed main");
    sd.fireBeforeChangeAll(0, 0, &RevisionChange::ACCEPT);
    let _sd: &dyn Document = sd;
}
