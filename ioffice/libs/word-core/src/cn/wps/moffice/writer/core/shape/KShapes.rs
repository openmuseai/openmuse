#![allow(non_snake_case, dead_code, unused_imports)]

/// Soft usable KShapes (Document.getShapes) — full drawing graph deferred.
#[derive(Default)]
pub struct KShapes {
    pub(crate) n: i32,
}

impl KShapes {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn size(&self) -> i32 {
        self.n
    }
}
