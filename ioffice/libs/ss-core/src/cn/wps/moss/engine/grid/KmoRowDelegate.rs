// java package cn.wps.moss.engine.grid;

// java import cn.wps.moss.util.KmoRect;
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

use crate::cn::wps::moss::util::KmoRect::KmoRect;

use super::KmoMatrix::KmoMatrix;

// java public class KmoRowDelegate
pub struct KmoRowDelegate {
    // java private KmoMatrix matrix;
    matrix: Rc<RefCell<KmoMatrix>>,
    // java private final KmoRect usedRange;
    usedRange: Rc<RefCell<KmoRect>>,
}

// java public class KmoRowDelegate
impl KmoRowDelegate {
    // java private void adjustUsedRange(int row, int count)
    fn adjustUsedRange(&self, _row: i32, _count: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (row <= this.usedRange.br.row) {
        // java if (row < this.usedRange.tl.row) {
        // java this.usedRange.tl.row = row;
        // java this.usedRange.br.row = Math.min(this.usedRange.br.row + count, this.matrix.getMaxRow());
        */
    }

    // java public void insRow(int begin, int count)
    pub fn insRow(&self, _begin: i32, _count: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.insNode(new KmoRect(begin, 0, begin + count - 1, this.usedRange.br.col));
        */
    }

    // java public void delRow(int begin, int count)
    pub fn delRow(&self, _begin: i32, _count: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.delNode(new KmoRect(begin, 0, begin + count - 1, this.usedRange.br.col));
        */
    }

    // java public void insNode(KmoRect range)
    pub fn insNode(&self, _range: &KmoRect) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java InsRowCmd cmd = new InsRowCmd(this, this.matrix, range);
        // java cmd.beforeWrite();
        // java this.matrix.startCmdMode();
        // java this.insNode(range.tl.row, range.tl.col, range.height(), range.width());
        // java this.matrix.endCmdMode();
        */
    }

    // java protected void insNode(int row, int col, int height, int width)
    pub fn insNode__ints(&self, _row: i32, _col: i32, _height: i32, _width: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (row <= this.usedRange.br.row) {
        // java this.adjustUsedRange(row, height);
        // java for(int i = this.usedRange.br.row; i >= row + height; i -= this.matrix.moveRow(i - height, i, col, col + width - 1, false)) {
        // java for(int i = row; i < row + height; i += this.matrix.clearRow(i, col, col + width - 1)) {
        */
    }

    // java public void delNode(KmoRect range)
    pub fn delNode(&self, _range: &KmoRect) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java DelRowCmd cmd = new DelRowCmd(this, this.matrix, range);
        // java cmd.beforeWrite();
        // java this.matrix.startCmdMode();
        // java this.delNode(range.tl.row, range.tl.col, range.height(), range.width());
        // java this.matrix.endCmdMode();
        */
    }

    // java protected void delNode(int row, int col, int height, int width)
    pub fn delNode__ints(&self, _row: i32, _col: i32, _height: i32, _width: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (row < this.usedRange.br.row) {
        // java this.adjustUsedRange(row, height);
        // java for(int i = 0; i < this.usedRange.br.row - height; ++i) {
        // java this.matrix.moveRow(i + row + height, i + row, col, col + width - 1, true);
        // java for(int i = 0; i < height; ++i) {
        // java int delRow = this.usedRange.br.row - 1 - i;
        // java if (delRow >= row) {
        // java this.matrix.clearRow(delRow, col, col + width - 1);
        */
    }

    // java KmoRowDelegate(KmoMatrix matrix, KmoRect usedRange)
    pub fn KmoRowDelegate(matrix: Rc<RefCell<KmoMatrix>>, usedRange: Rc<RefCell<KmoRect>>) -> Self {
        // java this.matrix = matrix;
        let matrix = matrix;
        // java this.usedRange = usedRange;
        let usedRange = usedRange;
        Self { matrix, usedRange }
    }
}
