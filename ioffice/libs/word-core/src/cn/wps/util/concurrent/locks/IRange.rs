// java public interface IRange
pub trait IRange {
    // java  int getMax()
    fn getMax(&self) -> i32;

    // java  int getLastParaStart()
    fn getLastParaStart(&self) -> i32;
}
