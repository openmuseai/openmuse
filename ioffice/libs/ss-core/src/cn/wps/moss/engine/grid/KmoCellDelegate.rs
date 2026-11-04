// java package cn.wps.moss.engine.grid;

// java import cn.wps.moss.util.objpool.SharedObjects;
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

use crate::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;

use super::KmoMatrix::KmoMatrix;

// java public class KmoCellDelegate
pub struct KmoCellDelegate {
    // java private KmoMatrix matrix;
    matrix: Rc<RefCell<KmoMatrix>>,
}

// java public class KmoCellDelegate
impl KmoCellDelegate {
    // java KmoCellDelegate(KmoMatrix rhs)
    pub fn KmoCellDelegate(matrix: Rc<RefCell<KmoMatrix>>) -> Self {
        // java this.matrix = rhs;
        let matrix = matrix;
        Self { matrix }
    }

    // java public boolean exists(int row, int col)
    pub fn exists(&self, row: i32, col: i32) -> bool {
        // java if (row >= 0 && col >= 0) {
        if row >= 0 && col >= 0 {
            // java return !this.matrix.isEmptyNode(row, col);
            return !self.matrix.borrow().isEmptyNode(row, col);
        } else {
            // java return false;
            return false;
        }
    }

    // java public void read(int row, int col, KmoCellData cell)
    pub fn read(&self, row: i32, col: i32, cell: &mut KmoCellData) {
        let matrix = self.matrix.borrow();
        // java KmoMatrix.KmoBlock blk = this.matrix.getBlk(row, col);
        // java if (blk != null) {
        if let Some(blk) = matrix.getBlk(row, col) {
            // java cell.read(blk.rawByteData(), this.matrix.getOffset(row, col));
            let off = matrix.getOffset(row, col);
            cell.read__bytearr_int(&blk.rawByteData(), off);
        } else {
            // java cell.clear();
            cell.clear();
        }
    }

    // java public void write(int row, int col, KmoCellData cell)
    pub fn write(&self, row: i32, col: i32, cell: &KmoCellData) {
        // java byte[] buff = SharedObjects.cellDataBuf.borrowObject();
        let mut buff = [0u8; 7];
        // java cell.write(buff);
        cell.write(&mut buff);
        // java this.matrix.set(row, col, buff);
        self.matrix.borrow_mut().set(row, col, &buff);
        // java buff = SharedObjects.cellDataBuf.returnObject(buff);
        // rust: no-stmt — borrow 得到的是 7 个 0，passivate 在归还时再填 0。write 已覆盖全部 7 字节，归还不再改格子。
        let _ = buff;
    }

    // java public void writeCalcResult(int row, int col, KmoCellData cell)
    pub fn writeCalcResult(&self, row: i32, col: i32, cell: &KmoCellData) {
        // java byte[] buff = SharedObjects.cellDataBuf.borrowObject();
        let mut buff = [0u8; 7];
        // java cell.write(buff);
        cell.write(&mut buff);
        // java this.matrix.setCalcResult(row, col, buff);
        self.matrix.borrow_mut().setCalcResult(row, col, &buff);
        // java buff = SharedObjects.cellDataBuf.returnObject(buff);
        let _ = buff;
    }

    // java public void delete(int row, int col)
    pub fn delete(&self, row: i32, col: i32) {
        // java byte[] empty_node = SharedObjects.cellDataBuf.borrowObject();
        let empty_node = [0u8; 7];
        // java this.matrix.set(row, col, empty_node);
        self.matrix.borrow_mut().set(row, col, &empty_node);
        // java empty_node = SharedObjects.cellDataBuf.returnObject(empty_node);
        let _ = empty_node;
    }
}
