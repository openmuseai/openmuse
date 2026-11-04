use super::IPropBaseHandlerHelper::IPropBaseHandlerHelper;

// Java: public interface IPropHandlerHelper extends IPropBaseHandlerHelper
pub trait IPropHandlerHelper: IPropBaseHandlerHelper {
    // Java: abstract boolean isBaseElement(int var1)
    fn isBaseElement(&self, var1: i32) -> bool;

    // Java: abstract boolean isChangeElement(int var1)
    fn isChangeElement(&self, var1: i32) -> bool;

    // Java: abstract int filterChangeIndex()
    fn filterChangeIndex(&self) -> i32;
}
