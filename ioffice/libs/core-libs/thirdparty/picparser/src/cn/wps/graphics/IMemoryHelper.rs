#![allow(non_snake_case, dead_code)]

// Java: public interface IMemoryHelper
pub trait IMemoryHelper {
    // Java: abstract void onOutOfMemory()
    fn onOutOfMemory(&self);

    // Java: abstract void gcForNewObject(int var1)
    fn gcForNewObject(&self, var1: i32);
}
