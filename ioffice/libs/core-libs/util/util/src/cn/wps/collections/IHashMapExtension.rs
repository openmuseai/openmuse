#![allow(non_snake_case, dead_code)]

use super::ITreeExtension::ITreeExtension;

// Java: public interface IHashMapExtension extends ITreeExtension
pub trait IHashMapExtension: ITreeExtension {
    // Java: abstract Listener getListener()
    fn getListener(&self) -> Option<&dyn Listener>;

    // Java: abstract void setListener(Listener var1)
    fn setListener(&mut self, var1: Option<Box<dyn Listener>>);
}

// Java: public interface Listener
pub trait Listener {
    // Java: abstract void beforeKeyChange(int var1)
    fn beforeKeyChange(&mut self, var1: i32);

    // Java: abstract void afterKeyChange(int var1, Object var2)
    fn afterKeyChange(&mut self, var1: i32, var2: &dyn std::any::Any);
}
