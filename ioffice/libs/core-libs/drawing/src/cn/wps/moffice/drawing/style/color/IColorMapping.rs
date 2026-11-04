// Java: public interface IColorMapping
pub trait IColorMapping {
    // Java: abstract int mapColor(int var1)
    fn mapColor(&self, var1: i32) -> i32;
}
