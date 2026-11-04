#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
//! Compile-surface hosts for parked KoBin signatures (pp-19 function 03).
//! Not under libs/core-libs/stubs.

#[derive(Default, Clone, Debug)]
pub struct KoZone;
#[derive(Default, Clone, Debug)]
pub struct KoAr;
#[derive(Default, Clone, Debug)]
pub struct KpBytePool;
#[derive(Default, Clone, Debug)]
pub struct KpStringPool;
#[derive(Default, Clone, Debug)]
pub struct KcByte;
#[derive(Default, Clone, Debug)]
pub struct Value1;
#[derive(Default, Clone, Debug)]
pub struct IllegalArgumentException;
impl IllegalArgumentException {
    pub fn new() -> Self { Self }
}
#[derive(Default, Clone, Debug)]
pub struct UnsupportedOperationException;
impl UnsupportedOperationException {
    pub fn new() -> Self { Self }
}
pub struct System;
impl System {
    pub fn arraycopy(_src: Option<Vec<u8>>, _src_pos: i32, _dest: &Vec<u8>, _dest_pos: i32, _len: i32) {}
}
