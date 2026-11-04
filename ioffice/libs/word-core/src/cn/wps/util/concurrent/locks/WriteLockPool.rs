#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::RangeWriteLock;

/// Soft usable pool until full java2rust of WriteLockPool.
#[derive(Default)]
pub struct WriteLockPool {
    pub(crate) n: i32,
}

impl WriteLockPool {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn dispose(&mut self) {}
    pub fn obtain(&mut self, _end: i32) -> RangeWriteLock {
        RangeWriteLock::default()
    }
    pub fn recycle(&mut self, _lock: RangeWriteLock) {}
}
