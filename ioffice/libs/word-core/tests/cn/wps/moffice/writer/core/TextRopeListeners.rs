// function_id=08_function_listeners_io_skip
// cargo test --manifest-path libs/word-core/Cargo.toml --test 08_function_listeners_io_skip
// src:  libs/word-core/src/cn/wps/moffice/writer/core/TextRopeListeners.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/core/TextRopeListeners.rs

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use util::moffice::transaction::Transaction::Transaction;
use word_core::cn::wps::collections::textrope::INotifyTextRope::Listener;
use word_core::cn::wps::moffice::writer::core::Document::{Document, SoftTextDocument};
use word_core::cn::wps::moffice::writer::core::DocumentEventDispatcher::DocumentEventDispatcher;
use word_core::cn::wps::moffice::writer::core::TextRopeListeners::TextRopeListeners;

struct FakeTd {
    io: Cell<bool>,
    user: Cell<bool>,
}

impl SoftTextDocument for FakeTd {
    fn getTransaction(&self) -> Option<&Transaction> {
        None
    }
    fn isInIOState(&self) -> bool {
        self.io.get()
    }
    fn isInUserAction(&self) -> bool {
        self.user.get()
    }
}

struct FakeDoc {
    td: FakeTd,
    disp: DocumentEventDispatcher,
}

impl IDocument for FakeDoc {
    fn getType(&self) -> i32 {
        0
    }
}

impl Document for FakeDoc {
    // rust: ★门禁修复 —— `Document::asAny`/`asAny_mut`（Java 强制类型转换入口，
    //   见 `core/Document.rs:410-415`）是**无默认体**的必需项；测试替身必须实现，
    //   否则测试目标无法编译（`E0046`），全量回归门禁形同虚设。
    // Java: 等价于 Java 的强制类型转换入口
    fn asAny(&self) -> &dyn std::any::Any {
        self
    }
    fn asAny_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn getLength(&self) -> i32 {
        0
    }
    fn getTextDocument(&self) -> Option<&dyn SoftTextDocument> {
        Some(&self.td)
    }
    fn getDispatcher(&self) -> Option<DocumentEventDispatcher> {
        Some(self.disp.clone())
    }
}

struct RecordingListener {
    name: &'static str,
    events: Rc<RefCell<Vec<String>>>,
}

impl Listener for RecordingListener {
    fn beforeInsertText(&mut self, _cp: i32, _fc: i32, _length: i32) {
        self.events
            .borrow_mut()
            .push(format!("{}.beforeInsert", self.name));
    }
    fn afterInsertText(&mut self, _cp: i32, _fc: i32, _length: i32) {
        self.events
            .borrow_mut()
            .push(format!("{}.afterInsert", self.name));
    }
    fn beforeRemoveText(&mut self, _start: i32, _end: i32) {
        self.events
            .borrow_mut()
            .push(format!("{}.beforeRemove", self.name));
    }
    fn afterRemoveText(&mut self, _start: i32, _end: i32) {
        self.events
            .borrow_mut()
            .push(format!("{}.afterRemove", self.name));
    }
}

fn harness() -> (FakeDoc, Rc<RefCell<Vec<String>>>) {
    let events = Rc::new(RefCell::new(Vec::new()));
    let doc = FakeDoc {
        td: FakeTd {
            io: Cell::new(false),
            user: Cell::new(false),
        },
        disp: {
            let mut disp = DocumentEventDispatcher::new();
            disp.events = events.clone();
            disp
        },
    };
    (doc, events)
}

fn rec(name: &'static str, events: &Rc<RefCell<Vec<String>>>) -> Box<dyn Listener> {
    Box::new(RecordingListener {
        name,
        events: events.clone(),
    })
}

fn with_l0_l1(doc: &dyn Document, events: &Rc<RefCell<Vec<String>>>) -> TextRopeListeners {
    let mut lis = TextRopeListeners::TextRopeListeners(doc);
    lis.add(rec("L0", events));
    lis.add(rec("L1", events));
    lis
}

#[test]
fn TL1_before_insert_dispatcher_then_reverse_list() {
    let (doc, events) = harness();
    let mut lis = with_l0_l1(&doc, &events);
    lis.beforeInsertText(0, 0, 1);
    assert_eq!(
        *events.borrow(),
        [
            "dispatcher.beforeInsert",
            "L1.beforeInsert",
            "L0.beforeInsert"
        ]
    );
}

#[test]
fn TL2_after_insert_list_then_dispatcher() {
    let (doc, events) = harness();
    let mut lis = with_l0_l1(&doc, &events);
    lis.afterInsertText(0, 0, 1);
    assert_eq!(
        *events.borrow(),
        ["L0.afterInsert", "L1.afterInsert", "dispatcher.afterInsert"]
    );
}

#[test]
fn TL3_before_remove_dispatcher_then_reverse_list() {
    let (doc, events) = harness();
    let mut lis = with_l0_l1(&doc, &events);
    lis.beforeRemoveText(0, 1);
    assert_eq!(
        *events.borrow(),
        [
            "dispatcher.beforeRemove",
            "L1.beforeRemove",
            "L0.beforeRemove"
        ]
    );
}

#[test]
fn TL4_after_remove_list_then_dispatcher() {
    let (doc, events) = harness();
    let mut lis = with_l0_l1(&doc, &events);
    lis.afterRemoveText(0, 1);
    assert_eq!(
        *events.borrow(),
        ["L0.afterRemove", "L1.afterRemove", "dispatcher.afterRemove"]
    );
}

#[test]
fn TL5_io_state_skips_all_callbacks() {
    let (doc, events) = harness();
    let mut lis = with_l0_l1(&doc, &events);
    doc.td.io.set(true);
    doc.td.user.set(false);
    lis.beforeInsertText(0, 0, 1);
    lis.afterInsertText(0, 0, 1);
    assert!(events.borrow().is_empty(), "IO && !user must skip all callbacks, got {:?}", events.borrow());
}

#[test]
fn TL6_io_plus_user_action_still_forwards() {
    let (doc, events) = harness();
    let mut lis = with_l0_l1(&doc, &events);
    doc.td.io.set(true);
    doc.td.user.set(true);
    lis.beforeInsertText(0, 0, 1);
    assert_eq!(
        *events.borrow(),
        [
            "dispatcher.beforeInsert",
            "L1.beforeInsert",
            "L0.beforeInsert"
        ]
    );
}

#[test]
fn TL7_not_io_forwards() {
    let (doc, events) = harness();
    let mut lis = with_l0_l1(&doc, &events);
    doc.td.io.set(false);
    lis.beforeInsertText(0, 0, 1);
    assert_eq!(
        *events.borrow(),
        [
            "dispatcher.beforeInsert",
            "L1.beforeInsert",
            "L0.beforeInsert"
        ]
    );
}

#[test]
fn TL8_add_size_two() {
    let (doc, events) = harness();
    let lis = with_l0_l1(&doc, &events);
    assert_eq!(lis.size(), 2);
}

#[test]
fn TL9_empty_list_only_dispatcher() {
    let (doc, events) = harness();
    let mut lis = TextRopeListeners::TextRopeListeners(&doc);
    lis.beforeInsertText(0, 0, 1);
    assert_eq!(*events.borrow(), ["dispatcher.beforeInsert"]);
}
