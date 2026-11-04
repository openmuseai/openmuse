#![allow(non_snake_case, dead_code, unused_imports)]

/// Soft usable KTableRange (Document.getTableRange) — table subtypes deferred.
#[derive(Default)]
pub struct KTableRange {
    pub(crate) start: i32,
    pub(crate) end: i32,
}

impl KTableRange {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn getStart(&self) -> i32 {
        self.start
    }
    pub fn getEnd(&self) -> i32 {
        self.end
    }
    pub fn setRange__int_int(&mut self, start: i32, end: i32) {
        self.start = start;
        self.end = end;
    }
}
