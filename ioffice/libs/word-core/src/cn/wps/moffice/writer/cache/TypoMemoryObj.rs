#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;

// java public abstract class TypoMemoryObj
pub struct TypoMemoryObj {
    mPointer: i32,
    mSnapshot: *mut TypoSnapshot,
}

impl Default for TypoMemoryObj {
    fn default() -> Self {
        Self { mPointer: 0, mSnapshot: std::ptr::null_mut() }
    }
}

impl Clone for TypoMemoryObj {
    fn clone(&self) -> Self {
        Self { mPointer: self.mPointer, mSnapshot: self.mSnapshot }
    }
}

impl TypoMemoryObj {
    // java public static int getType(int pointer, TypoSnapshot snapshot)
    pub fn getType__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        if pointer != 0 {
            snapshot.get__int(pointer)
        } else {
            -1
        }
    }

    // java public static int getMemSize(int pointer, TypoSnapshot snapshot)
    pub fn getMemSize__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        // java return snapshot.get(pointer + 1);
        snapshot.get__int(pointer + 1)
    }

    pub fn getPointer(&self) -> i32 { self.mPointer }

    // java public final void assignment(int pointer, TypoSnapshot snapshot)
    pub fn assignment(&mut self, pointer: i32, snapshot: *mut TypoSnapshot) {
        self.mPointer = pointer;
        self.mSnapshot = snapshot;
    }

    // java public void clear()
    pub fn clear(&mut self) {
        self.mPointer = 0;
        self.mSnapshot = std::ptr::null_mut();
    }

    // java public int getRawType()
    pub fn getRawType(&self) -> i32 {
        if self.mSnapshot.is_null() || self.mPointer == 0 {
            -1
        } else {
            unsafe { (*self.mSnapshot).get__int(self.mPointer) }
        }
    }

    // java public int cloneObj(TypoSnapshot snapshot)
    pub fn cloneObj(&self, snapshot: &mut TypoSnapshot) -> i32 {
        if self.mPointer == 0 || self.mSnapshot.is_null() {
            0
        } else {
            let len = unsafe { (*self.mSnapshot).get__int(self.mPointer + 1) };
            snapshot.copy__int_int(self.mPointer, len)
        }
    }
}
