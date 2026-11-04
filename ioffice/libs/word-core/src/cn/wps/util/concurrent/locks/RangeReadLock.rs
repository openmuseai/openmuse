#![allow(non_snake_case, dead_code, unused_imports)]

/// Soft usable RangeReadLock (Document / KTextStream call sites need unlock).
#[derive(Default, Clone)]
pub struct RangeReadLock {
    pub(crate) end: i32,
    pub(crate) threadId: i64,
    pub(crate) threadName: Option<String>,
}

impl RangeReadLock {
    pub fn RangeReadLock(_lock: &super::RangeLock::RangeLock, end: i32) -> Self {
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
    pub fn lock(&mut self) {}
    pub fn tryLock(&mut self) -> bool {
        true
    }
}
