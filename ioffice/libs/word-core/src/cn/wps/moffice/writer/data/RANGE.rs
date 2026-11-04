#![allow(non_snake_case, dead_code, unused_imports)]

/// Soft usable RANGE (Document/math/comment wiring). Full pool semantics deferred.
#[derive(Clone, Debug, Default)]
pub struct RANGE {
    // java public int start;
    pub start: i32,
    // java public int end;
    pub end: i32,
    // java protected RANGE next;
    pub(crate) next: Option<Box<RANGE>>,
}

impl RANGE {
    pub fn obtain__int_int(start: i32, end: i32) -> Self {
        Self {
            start,
            end,
            next: None,
        }
    }
    pub fn obtain__int(cp: i32) -> Self {
        Self::obtain__int_int(cp, cp)
    }
    pub fn reset(&mut self) {
        self.start = 0;
        self.end = 0;
    }
    pub fn set__range(&mut self, range: &RANGE) {
        self.start = range.start;
        self.end = range.end;
    }
    pub fn set__int_int(&mut self, start: i32, end: i32) {
        self.start = start;
        self.end = end;
    }
    pub fn getLength(&self) -> i32 {
        self.end - self.start
    }
    pub fn contains(&self, cp: i32) -> bool {
        cp >= self.start && cp < self.end
    }
    pub fn isEmpty(&self) -> bool {
        self.start == self.end
    }
    // java: boolean noIntersection(int start, int end) / hasIntersection
    pub fn noIntersection(&self, start: i32, end: i32) -> bool {
        self.end <= start || self.start >= end
    }
    pub fn hasIntersection(&self, start: i32, end: i32) -> bool {
        !self.noIntersection(start, end)
    }
    pub fn recycle(self) {}
}

pub(crate) const DEFAULT_MAX_POOL_SIZE: i32 = 32;
