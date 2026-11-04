//! Java `java.util.ArrayDeque` → Rust stub (JDK whitelist).

use std::any::Any;
use std::collections::VecDeque;

/// Resizable-array double-ended queue (Java `ArrayDeque`).
#[derive(Default)]
pub struct ArrayDeque {
    buf: VecDeque<Box<dyn Any>>,
}

impl ArrayDeque {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ArrayDeque(&self) {}

    /// Java `boolean add(E e)` — append to tail.
    pub fn add<T: Any + 'static>(&mut self, e: T) -> bool {
        self.buf.push_back(Box::new(e));
        true
    }

    /// Java `void addLast(E e)`.
    pub fn addLast<T: Any + 'static>(&mut self, e: T) {
        self.buf.push_back(Box::new(e));
    }

    /// Java `void addFirst(E e)`.
    pub fn addFirst<T: Any + 'static>(&mut self, e: T) {
        self.buf.push_front(Box::new(e));
    }

    /// Java `int size()`.
    pub fn size(&self) -> i32 {
        self.buf.len() as i32
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn isEmpty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Java `E getLast()` — returns `&dyn Any` for caller `.downcast_ref`.
    pub fn getLast(&self) -> &dyn Any {
        self.buf
            .back()
            .map(|b| b.as_ref() as &dyn Any)
            .expect("NoSuchElementException")
    }

    /// Java `E getFirst()`.
    pub fn getFirst(&self) -> &dyn Any {
        self.buf
            .front()
            .map(|b| b.as_ref() as &dyn Any)
            .expect("NoSuchElementException")
    }

    /// Java `E pollLast()`.
    pub fn pollLast(&mut self) -> Option<Box<dyn Any>> {
        self.buf.pop_back()
    }

    /// Java `E pollFirst()`.
    pub fn pollFirst(&mut self) -> Option<Box<dyn Any>> {
        self.buf.pop_front()
    }

    /// Java `void clear()`.
    pub fn clear(&mut self) {
        self.buf.clear();
    }
}
