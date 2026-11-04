#![allow(non_snake_case, dead_code, unused_imports)]

/// Soft usable ErrorCollector for Document.getErrorCollector().
#[derive(Default, Clone, Copy)]
pub enum ErrorType {
    #[default]
    Unknown,
}

#[derive(Default, Clone)]
pub struct ErrorCollector {
    pub(crate) has: bool,
    pub(crate) min_cp: i32,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn hasError(&self) -> bool {
        self.has
    }
    pub fn getMinErrorCp(&self) -> i32 {
        self.min_cp
    }
    pub fn reportError(&mut self, cp: i32, _ty: ErrorType) {
        self.has = true;
        if self.min_cp == 0 || cp < self.min_cp {
            self.min_cp = cp;
        }
    }
    pub fn recordLockCp(&mut self, _mainLockCp: i32) {}
    pub fn clean(&mut self) {
        self.has = false;
        self.min_cp = 0;
    }
    pub fn needPopSaveDialog(&self) -> bool {
        false
    }
}
