// function_id=02_function_notify_order
// cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_notify_order
// src:  libs/word-core/src/cn/wps/collections/textrope/{KNotifyTextRope,INotifyTextRope}.rs
// test: libs/word-core/tests/cn/wps/collections/textrope/INotifyTextRope.rs

use std::cell::RefCell;
use std::rc::Rc;

use word_core::cn::wps::collections::textrope::INotifyTextRope::{INotifyTextRope, Listener};
use word_core::cn::wps::collections::textrope::ITextRope::ITextRope;
use word_core::cn::wps::collections::textrope::KNotifyTextRope::KNotifyTextRope;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Event {
    BeforeInsert { cp: i32, fc: i32, length: i32 },
    AfterInsert { cp: i32, fc: i32, length: i32, rope_len: i32 },
    BeforeRemove { start: i32, end: i32 },
    AfterRemove { start: i32, end: i32, rope_len: i32 },
}

/// Records listener callbacks. `after*` also snapshots rope length to prove mutate already happened.
struct RecordingListener {
    events: Rc<RefCell<Vec<Event>>>,
    rope: *mut KNotifyTextRope,
}

impl Listener for RecordingListener {
    fn beforeInsertText(&mut self, cp: i32, fc: i32, length: i32) {
        self.events.borrow_mut().push(Event::BeforeInsert { cp, fc, length });
    }
    fn afterInsertText(&mut self, cp: i32, fc: i32, length: i32) {
        let rope_len = unsafe { ITextRope::length(&*self.rope) };
        self.events
            .borrow_mut()
            .push(Event::AfterInsert { cp, fc, length, rope_len });
    }
    fn beforeRemoveText(&mut self, start: i32, end: i32) {
        self.events.borrow_mut().push(Event::BeforeRemove { start, end });
    }
    fn afterRemoveText(&mut self, start: i32, end: i32) {
        let rope_len = unsafe { ITextRope::length(&*self.rope) };
        self.events
            .borrow_mut()
            .push(Event::AfterRemove { start, end, rope_len });
    }
}

fn attach(rope: &mut KNotifyTextRope) -> Rc<RefCell<Vec<Event>>> {
    let events = Rc::new(RefCell::new(Vec::new()));
    let listener = RecordingListener {
        events: events.clone(),
        rope: rope as *mut KNotifyTextRope,
    };
    INotifyTextRope::setListener(rope, Some(Box::new(listener)));
    events
}

#[test]
fn n1_append_before_then_after_and_length_already_updated() {
    let mut rope = KNotifyTextRope::default();
    let events = attach(&mut rope);
    ITextRope::append__string_int_int(&mut rope, "ab", 0, 2);
    let ev = events.borrow().clone();
    assert!(
        matches!(ev.as_slice(), [Event::BeforeInsert { .. }, Event::AfterInsert { rope_len: 2, .. }]),
        "expected before→after with after seeing length=2, got {ev:?}"
    );
}

#[test]
fn n2_append_reports_old_length_as_cp() {
    let mut rope = KNotifyTextRope::default();
    let events = attach(&mut rope);
    ITextRope::append__string_int_int(&mut rope, "xy", 0, 2);
    let ev = events.borrow().clone();
    match &ev[0] {
        Event::BeforeInsert { cp, length, .. } => {
            assert_eq!(*cp, 0);
            assert_eq!(*length, 2);
        }
        other => panic!("expected BeforeInsert, got {other:?}"),
    }
}

#[test]
fn n3_null_listener_append_still_writes() {
    let mut rope = KNotifyTextRope::default();
    INotifyTextRope::setListener(&mut rope, None);
    ITextRope::append__char(&mut rope, 'Z');
    assert_eq!(ITextRope::length(&rope), 1);
    assert_eq!(ITextRope::charAt(&mut rope, 0), 'Z');
}

#[test]
fn n4_erase_before_then_after() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__string_int_int(&mut rope, "ab", 0, 2);
    let events = attach(&mut rope);
    ITextRope::erase(&mut rope, 0, 2);
    let ev = events.borrow().clone();
    assert!(
        matches!(ev.as_slice(), [Event::BeforeRemove { .. }, Event::AfterRemove { .. }]),
        "got {ev:?}"
    );
}

#[test]
fn n5_erase_args_and_final_length() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__string_int_int(&mut rope, "ab", 0, 2);
    let events = attach(&mut rope);
    ITextRope::erase(&mut rope, 0, 1);
    let ev = events.borrow().clone();
    match &ev[0] {
        Event::BeforeRemove { start, end } => {
            assert_eq!(*start, 0);
            assert_eq!(*end, 1);
        }
        other => panic!("expected BeforeRemove, got {other:?}"),
    }
    match &ev[1] {
        Event::AfterRemove { rope_len, .. } => assert_eq!(*rope_len, 1),
        other => panic!("expected AfterRemove, got {other:?}"),
    }
}

#[test]
fn n6_insert_middle_reports_target_cp() {
    let mut rope = KNotifyTextRope::default();
    ITextRope::append__string_int_int(&mut rope, "AC", 0, 2);
    let events = attach(&mut rope);
    ITextRope::insert__int_char(&mut rope, 1, 'B');
    let ev = events.borrow().clone();
    match &ev[0] {
        Event::BeforeInsert { cp, length, .. } => {
            assert_eq!(*cp, 1);
            assert_eq!(*length, 1);
        }
        other => panic!("expected BeforeInsert at cp=1, got {other:?}"),
    }
}

#[test]
fn n7_set_listener_get_listener_is_some() {
    let mut rope = KNotifyTextRope::default();
    assert!(INotifyTextRope::getListener(&rope).is_none());
    attach(&mut rope);
    assert!(INotifyTextRope::getListener(&rope).is_some());
}
