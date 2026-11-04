#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::util::concurrent::locks::RangeReadLock::RangeReadLock;

/// Soft usable pool until full java2rust of ReadLockPool.
#[derive(Default)]
pub struct ReadLockPool {
    pub(crate) n: i32,
}

impl ReadLockPool {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn dispose(&mut self) {}
    pub fn obtain(&mut self, _end: i32) -> RangeReadLock {
        RangeReadLock::default()
    }
    pub fn recycle(&mut self, _lock: RangeReadLock) {}
}
