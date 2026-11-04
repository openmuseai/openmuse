// Rust stub for LBCOperand (auto-generated)
// Original: cn/wps/moffice/writer/data/value/LBCOperand.java
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LBCOperand {
    lbrNone,
    lbrLeft,
    lbrRight,
    lbrBoth,
    KSAssert,
}

impl LBCOperand {
    pub fn fromValue(value: i32) -> LBCOperand { todo!() }
    pub fn getValue(&self) -> i32 { todo!() }
}
