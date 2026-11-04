// Java: public interface AnyMatrix
pub trait AnyMatrix {
    // Java: abstract boolean isSquare()
    fn isSquare(&self) -> bool;

    // Java: abstract int getRowDimension()
    fn getRowDimension(&self) -> i32;

    // Java: abstract int getColumnDimension()
    fn getColumnDimension(&self) -> i32;

}
