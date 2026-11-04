#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::util::concurrent::locks::RangeReadLock::RangeReadLock;
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::RangeWriteLock;
use crate::cn::wps::util::concurrent::locks::ReadLockPool::ReadLockPool;
use crate::cn::wps::util::concurrent::locks::WriteLockPool::WriteLockPool;

/// Soft usable RangeLock until full java2rust (Document.getRangeLock / lock APIs).
#[derive(Default)]
pub struct RangeLock {
    pub(crate) rpool: ReadLockPool,
    pub(crate) wpool: WriteLockPool,
    pub(crate) mIoThreadId: i64,
    pub(crate) mInIoState: bool,
    pub locked: bool,
}

impl RangeLock {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn dispose(&mut self) {
        self.rpool.dispose();
        self.wpool.dispose();
    }
    pub fn readLock(&mut self, end: i32) -> RangeReadLock {
        self.rpool.obtain(end)
    }
    pub fn readLockAll(&mut self) -> RangeReadLock {
        self.readLock(i32::MAX)
    }
    pub fn writeLock(&mut self, end: i32) -> RangeWriteLock {
        RangeWriteLock::RangeWriteLock(self, end)
    }
    pub fn tryReadLock(&mut self, _timeout: i64) -> Option<RangeReadLock> {
        Some(self.readLock(i32::MAX))
    }
    pub fn tryWriteLock(&mut self, _timeout: i64) -> Option<RangeWriteLock> {
        Some(self.writeLock(i32::MAX))
    }
    pub fn isLocked(&self) -> bool {
        self.locked
    }
    pub fn setIoThreadId(&mut self, id: i64) {
        self.mIoThreadId = id;
    }
    pub fn unLockCurThreadWhenCrash(&mut self) {}
    pub fn collectLockInfo(&self, _curTime: i64) -> String {
        String::new()
    }
}
