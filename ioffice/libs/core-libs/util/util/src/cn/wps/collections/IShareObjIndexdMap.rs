#![allow(non_snake_case, dead_code, unused_variables)]

// Java: public interface IShareObjIndexdMap extends IShareObjIndexdMap
// LLM Rust Pattern: Java interface extends -> Rust trait supertrait (no Deref needed; parent: IShareObjIndexdMap is the supertrait bound)
// impl Deref for IShareObjIndexdMap not needed: trait inheritance via supertrait bound (self-referential generic constraint)

pub struct IShareObjIndexdMap;

pub trait IShareObjIndexdMapTrait<T: IShareObjIndexedTrait> {
    // Java: abstract T getObjValue(int var1)
    fn getObjValue(&self, index: i32) -> Option<i32>;

    // Java: abstract int getObjIndex(T var1)
    fn getObjIndex(&self, obj: i32) -> i32;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract void setFrozenSize()
    fn setFrozenSize(&mut self);

    // Java: abstract int getFrozenSize()
    fn getFrozenSize(&self) -> i32;

    // Java: abstract void clear()
    fn clear(&mut self);
}

// Java: public interface IShareObjIndexed
pub struct IShareObjIndexed;

pub trait IShareObjIndexedTrait {
    // Java field: int INVALID_INDEX
    // LLM JavaLine Aggregation: declared as const

    // Java: abstract void setIndex(int var1)
    fn setIndex(&mut self, index: i32);

    // Java: abstract int getIndex()
    fn getIndex(&self) -> i32;

    // Java: abstract Object getHostObj()
    fn getHostObj(&self) -> i32;
}

pub const INVALID_INDEX: i32 = 0;
