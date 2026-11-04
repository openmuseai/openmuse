// java public interface KCellIterator
pub trait KCellIterator {
    // java  boolean hasNext()
    fn hasNext(&self) -> bool;

    // java  boolean next()
    fn next(&mut self) -> bool;

    // java  int getCellEndIndex()
    fn getCellEndIndex(&self) -> i32;

}
