#![allow(non_snake_case, dead_code)]

use super::LittleEndianInput::LittleEndianInput;

// Java: public interface LittleEndianRandomAccessInput extends LittleEndianInput
pub trait LittleEndianRandomAccessInput: LittleEndianInput {
    // Java: abstract long tell()
    fn tell(&self) -> i64;

    // Java: abstract long seek(long var1)
    fn seek(&mut self, var1: i64) -> i64;
}
