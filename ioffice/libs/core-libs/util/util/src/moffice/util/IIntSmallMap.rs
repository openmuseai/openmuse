// Java: import cn.wps.moffice.property.PropBaseSerializable
use util::cn::wps::moffice::property::PropBaseSerializable;

// Java: public class IIntSmallMap implements Cloneable, PropBaseSerializable
pub struct IIntSmallMap {
}

// Java: public class IIntSmallMap implements Cloneable, PropBaseSerializable
impl IIntSmallMap {
    // Java: IIntSmallMap()
    pub fn IIntSmallMap(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: abstract int size()
    pub fn size(&self) -> i32 {
    // TODO_RUST_CODE
    }

    // Java: abstract boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
    // TODO_RUST_CODE
    }

    // Java: abstract Object[] values()
    pub fn values(&self) -> Object[] {
    // TODO_RUST_CODE
    }

    // Java: abstract Object get(int var1)
    pub fn get(&self, _var1: i32) -> Object {
    // TODO_RUST_CODE
    }

    // Java: abstract void put(int var1, Object var2)
    pub fn put(&self, _var1: i32, _var2: i32) {
    // TODO_RUST_CODE
    }

    // Java: abstract void remove(int var1)
    pub fn remove(&self, _var1: i32) {
    // TODO_RUST_CODE
    }

    // Java: abstract void add(IIntSmallMap var1)
    pub fn add(&self, _var1: i32) {
    // TODO_RUST_CODE
    }

    // Java: abstract void clear()
    pub fn clear(&self) {
    // TODO_RUST_CODE
    }

    // Java: abstract IIntSmallMap clone()
    pub fn clone(&self) -> IIntSmallMap {
    // TODO_RUST_CODE
    }

}

impl Cloneable for IIntSmallMap {
}

impl PropBaseSerializable for IIntSmallMap {
}
