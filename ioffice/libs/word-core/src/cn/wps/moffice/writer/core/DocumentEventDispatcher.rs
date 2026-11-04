#![allow(non_snake_case, dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

use super::DocumentListener::DocumentListener;

/// Shared callback log: `getDispatcher()` returns by value, Clone keeps the same Rc
/// so TextRopeListeners and the test Document observe one ordered stream.
/// Document listeners share the same Rc, matching Java's single dispatcher object.
#[derive(Clone)]
pub struct DocumentEventDispatcher {
    pub events: Rc<RefCell<Vec<String>>>,
    // Java field: ArrayList<DocumentListener> mListeners
    listeners: Rc<RefCell<Vec<*mut dyn DocumentListener>>>,
}

impl Default for DocumentEventDispatcher {
    fn default() -> Self {
        Self {
            events: Rc::new(RefCell::new(Vec::new())),
            listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl std::fmt::Debug for DocumentEventDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocumentEventDispatcher")
            .field("events", &self.events)
            .finish()
    }
}

impl DocumentEventDispatcher {
    pub fn new() -> Self { Self::default() }

    // Java: void addDocumentListener(DocumentListener listener)
    pub fn addDocumentListener(&self, listener: *mut dyn DocumentListener) {
        // Java: if (!this.mListeners.contains(listener)) this.mListeners.add(listener);
        let mut listeners = self.listeners.borrow_mut();
        if !listeners.iter().any(|existing| std::ptr::eq(*existing, listener)) {
            listeners.push(listener);
        }
    }

    pub fn fireEvent(&mut self, e: &super::DocumentEvent::DocumentEvent) {
        // Java: this.update(e);
        self.update(e);
    }

    pub fn update(&mut self, e: &super::DocumentEvent::DocumentEvent) {
        // Java: for (DocumentListener listener : this.mListeners) listener.update(e);
        let listeners: Vec<*mut dyn DocumentListener> = self.listeners.borrow().clone();
        for listener in listeners {
            if !listener.is_null() {
                unsafe { &mut *listener }.update(e);
            }
        }
    }
    // Java beforeInsertText is empty; log the call so multicast order is observable.
    pub fn beforeInsertText(&mut self, _cp: i32, _fc: i32, _length: i32) {
        self.events.borrow_mut().push("dispatcher.beforeInsert".to_string());
    }
    pub fn afterInsertText(&mut self, _cp: i32, _fc: i32, _length: i32) {
        self.events.borrow_mut().push("dispatcher.afterInsert".to_string());
    }
    pub fn beforeRemoveText(&mut self, _start: i32, _end: i32) {
        self.events.borrow_mut().push("dispatcher.beforeRemove".to_string());
    }
    pub fn afterRemoveText(&mut self, _start: i32, _end: i32) {
        self.events.borrow_mut().push("dispatcher.afterRemove".to_string());
    }
}
