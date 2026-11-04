// java package cn.wps.moss.engine.grid;

// java import cn.wps.moss.util.KmoRect;
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

use crate::cn::wps::moss::util::KmoRect::KmoRect;

use super::KmoMatrix::KmoMatrix;

// java public class KmoColDelegate
pub struct KmoColDelegate {
    // java private KmoMatrix matrix;
    matrix: Rc<RefCell<KmoMatrix>>,
    // java private final KmoRect usedRange;
    usedRange: Rc<RefCell<KmoRect>>,
}

// java public class KmoColDelegate
impl KmoColDelegate {
    // java private void adjustUsedRange(int col, int count)
    fn adjustUsedRange(&self, _col: i32, _count: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (col <= this.usedRange.br.col) {
        // java if (col < this.usedRange.tl.col) {
        // java this.usedRange.tl.col = col;
        // java this.usedRange.br.col = Math.min(this.usedRange.br.col + count, this.matrix.getMaxCol());
        */
    }

    // java public void insCol(int begin, int count)
    pub fn insCol(&self, _begin: i32, _count: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.insNode(new KmoRect(this.usedRange.tl.row, begin, this.usedRange.br.row, begin + count - 1));
        */
    }

    // java public void delCol(int begin, int count)
    pub fn delCol(&self, _begin: i32, _count: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.delNode(new KmoRect(this.usedRange.tl.row, begin, this.usedRange.br.row, begin + count - 1));
        */
    }

    // java public void insNode(KmoRect range)
    pub fn insNode(&self, _range: &KmoRect) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java InsColCmd cmd = new InsColCmd(this, this.matrix, range);
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
        // java this.adjustUsedRange(col, width);
        // java for(int j = this.usedRange.br.col; j >= col + width; --j) {
        // java this.matrix.moveCol(j - width, j, row, row + height - 1);
        // java for(int j = col; j < col + width; ++j) {
        // java this.matrix.clearCol(j, row, row + height - 1);
        */
    }

    // java public void delNode(KmoRect range)
    pub fn delNode(&self, _range: &KmoRect) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java DelColCmd cmd = new DelColCmd(this, this.matrix, range);
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
        // java if (col < this.usedRange.br.col) {
        // java this.adjustUsedRange(col, width);
        // java for(int j = 0; j < this.usedRange.br.col - width; ++j) {
        // java this.matrix.moveCol(j + col + width, j + col, row, row + height - 1);
        // java for(int i = 0; i < width; ++i) {
        // java int delCol = this.usedRange.br.col - 1 - i;
        // java if (delCol >= col) {
        // java this.matrix.clearCol(delCol, row, row + height - 1);
        */
    }

    // java KmoColDelegate(KmoMatrix matrix, KmoRect usedRange)
    pub fn KmoColDelegate(matrix: Rc<RefCell<KmoMatrix>>, usedRange: Rc<RefCell<KmoRect>>) -> Self {
        // java this.matrix = matrix;
        let matrix = matrix;
        // java this.usedRange = usedRange;
        let usedRange = usedRange;
        Self { matrix, usedRange }
    }
}
