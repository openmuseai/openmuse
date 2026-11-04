use super::PropHandlerHelper::PropType;

// Java: public interface IPropBaseHandlerHelper
pub trait IPropBaseHandlerHelper {
    // Java: abstract PropHandlerHelper.PropType getPropType()
    fn getPropType(&self) -> PropType;

    // Java: abstract int convertProp(int var1, int var2)
    fn convertProp(&self, var1: i32, var2: i32) -> i32;

    // Java: abstract boolean isGetThirdSubElementHandler(int var1)
    fn isGetThirdSubElementHandler(&self, var1: i32) -> bool;
}
