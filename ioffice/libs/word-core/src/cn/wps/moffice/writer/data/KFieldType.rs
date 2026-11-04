#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! Soft KFieldType for CPieceTable.

#[derive(Default, Clone, Copy)]
pub struct KFieldType {
    pub(crate) flt: i32,
}

impl KFieldType {
    pub fn getFlt(&self) -> i32 {
        self.flt
    }
}
