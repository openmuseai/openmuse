// Java: public interface IResourceManager
pub trait IResourceManager {
    // Java: abstract int getAnimId(String var1)
    fn getAnimId(&self, var1: &str) -> i32;

    // Java: abstract int getArrayId(String var1)
    fn getArrayId(&self, var1: &str) -> i32;

    // Java: abstract int getDimenId(String var1)
    fn getDimenId(&self, var1: &str) -> i32;

    // Java: abstract int getDimen(int var1)
    fn getDimen(&self, var1: i32) -> i32;

    // Java: abstract int getDrawableId(String var1)
    fn getDrawableId(&self, var1: &str) -> i32;

    // Java: abstract int getId(String var1)
    fn getId(&self, var1: &str) -> i32;

    // Java: abstract int getLayoutId(String var1)
    fn getLayoutId(&self, var1: &str) -> i32;

    // Java: abstract String getString(int var1)
    fn getString(&self, var1: i32) -> String;

    // Java: abstract String getString(String var1)
    fn getString_by_name(&self, var1: &str) -> String;

    // Java: abstract int getStringId(String var1)
    fn getStringId(&self, var1: &str) -> i32;

    // Java: abstract int getStyleId(String var1)
    fn getStyleId(&self, var1: &str) -> i32;

    // Java: abstract int getColorId(String var1)
    fn getColorId(&self, var1: &str) -> i32;

    // Java: abstract int getColor(int var1)
    fn getColor(&self, var1: i32) -> i32;

    // Java: abstract int[] getStyleableArr(String var1)
    fn getStyleableArr(&self, var1: &str) -> Vec<i32>;

    // Java: abstract int getStyleableId(String var1)
    fn getStyleableId(&self, var1: &str) -> i32;

}
