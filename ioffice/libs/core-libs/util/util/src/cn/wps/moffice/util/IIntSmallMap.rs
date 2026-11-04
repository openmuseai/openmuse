#![allow(dead_code, non_snake_case)]

use std::any::Any;

// Java: public class IIntSmallMap implements Cloneable, PropBaseSerializable
pub struct IIntSmallMap;

// Java: public class IIntSmallMap implements Cloneable, PropBaseSerializable
impl IIntSmallMap {
    // Java: IIntSmallMap()
    pub fn new() -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: merged into struct initialization
        Self
    }

    // Java: int size()
    pub fn size(&self) -> i32 { 0 }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool { true }

    // Java: Object[] values()
    pub fn values(&self) -> Vec<Option<Box<dyn Any>>> { vec![] }

    // Java: Object get(int var1)
    pub fn get(&self, _var1: i32) -> Option<Box<dyn Any>> { None }

    // Java: void put(int var1, Object var2)
    pub fn put(&mut self, _var1: i32, _var2: Box<dyn Any>) {}

    // Java: void remove(int var1)
    pub fn remove(&mut self, _var1: i32) {}

    // Java: void add(IIntSmallMap var1)
    pub fn add(&mut self, _var1: &IIntSmallMap) {}

    // Java: void clear()
    pub fn clear(&mut self) {}

    // Java: IIntSmallMap clone()
    pub fn clone(&self) -> Result<IIntSmallMap, String> {
        // LLM Rust Trait: abstract class, clone returns new empty instance
        Ok(IIntSmallMap)
    }
}

pub trait Cloneable {}
impl Cloneable for IIntSmallMap {}

pub trait PropBaseSerializable {}
impl PropBaseSerializable for IIntSmallMap {}
