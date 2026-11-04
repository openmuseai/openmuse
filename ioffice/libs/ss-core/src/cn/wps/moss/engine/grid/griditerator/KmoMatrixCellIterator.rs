// java package cn.wps.moss.engine.grid.griditerator;

// java public interface KmoMatrixCellIterator
pub trait KmoMatrixCellIterator {
    // java boolean hasNext();
    fn hasNext(&self) -> bool;
    // java void next();
    fn next(&mut self);
    // java int row();
    fn row(&self) -> i32;
    // java int col();
    fn col(&self) -> i32;
    // java byte[] data();
    fn data(&self) -> &[u8];
}
