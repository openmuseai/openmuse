#![allow(non_snake_case, dead_code, unused_imports)]

/// Soft usable RangeWriteLock (Document / TextDocument call sites need unlock).
#[derive(Default, Clone)]
pub struct RangeWriteLock {
    pub(crate) end: i32,
    pub(crate) threadId: i64,
    pub(crate) threadName: Option<String>,
}

impl RangeWriteLock {
    pub fn RangeWriteLock(_lock: &super::RangeLock::RangeLock, end: i32) -> Self {
        Self {
            end,
            ..Default::default()
        }
    }
    pub fn reset(&mut self, _lock: &super::RangeLock::RangeLock, end: i32) {
        self.end = end;
    }
    pub fn getEnd(&self) -> i32 {
        self.end
    }
    pub fn unlock(&mut self) {}
    pub fn writeUnlock(&mut self) {
        self.unlock();
    }
    pub fn lock(&mut self) {}
    pub fn tryLock(&mut self) -> bool {
        true
    }
}
