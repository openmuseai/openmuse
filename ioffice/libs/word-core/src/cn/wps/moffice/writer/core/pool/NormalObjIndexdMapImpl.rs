// Rust stub for NormalObjIndexdMapImpl (auto-generated)
// Original: cn/wps/moffice/writer/core/pool/NormalObjIndexdMapImpl.java
use crate::*;

pub struct NormalObjIndexdMapImpl {
    // TODO: fields
}

impl NormalObjIndexdMapImpl {
    pub fn new() -> Self { todo!() }
    pub fn addRecycleIndexs(&self, recyleIndexs: Box<dyn IIntList>) -> i32 { todo!() }
    pub fn recycleCopy(&self, reset: bool) -> Box<dyn INormalObjIndexdMap> { todo!() }
    pub fn prepareAddObj(&self, obj: Box<dyn std::any::Any + Send + Sync>) -> i32 { todo!() }
    pub fn cancleAddObj(&self, index: i32) -> bool { todo!() }
    pub fn submitAddObjs(&self) { todo!() }
    pub fn getObjValue(&self, index: i32) -> Box<dyn std::any::Any + Send + Sync> { todo!() }
    pub fn replaceObj(&self, index: i32, obj: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn getObjIndex(&self, obj: Box<dyn std::any::Any + Send + Sync>) -> i32 { todo!() }
    pub fn getObjIndex2(&self, obj: Box<dyn std::any::Any + Send + Sync>) -> i32 { todo!() }
    pub fn size(&self) -> i32 { todo!() }
    pub fn addObj(&self, obj: Box<dyn std::any::Any + Send + Sync>, addUnityCheck: bool) -> i32 { todo!() }
    pub fn growCapacityInternal(&self, minCapacity: i32) -> bool { todo!() }
    pub fn setFrozenSize(&self) { todo!() }
    pub fn getFrozenSize(&self) -> i32 { todo!() }
    pub fn toString(&self) -> String { todo!() }
    pub fn getTag(&self) -> String { todo!() }
}
