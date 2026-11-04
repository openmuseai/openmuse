use crate::java_runtime::Cell;
// java public abstract class AreaEvalBase implements IAreaEval
pub struct AreaEvalBase{pub firstRow:Cell<i32>,pub firstColumn:Cell<i32>,pub lastRow:Cell<i32>,pub lastColumn:Cell<i32>,pub nRows:Cell<i32>,pub nColumns:Cell<i32>,pub orgLastRow:Cell<i32>,pub orgLastColumn:Cell<i32>,pub rav:Cell<i8>}
impl AreaEvalBase{
// java protected AreaEvalBase(int firstRow, int firstColumn, int lastRow, int lastColumn, byte rav) {
// java this._firstColumn = firstColumn;
// java this._firstRow = firstRow;
// java this._lastColumn = lastColumn;
// java this._lastRow = lastRow;
// java this._nColumns = this._lastColumn - this._firstColumn + 1;
// java this._nRows = this._lastRow - this._firstRow + 1;
// java this._RAV = rav;
// java }
pub fn new(fr:i32,fc:i32,lr:i32,lc:i32,rav:i8)->Self {
Self{firstRow:Cell::new(fr),firstColumn:Cell::new(fc),lastRow:Cell::new(lr),lastColumn:Cell::new(lc),nRows:Cell::new(lr-fr+1),nColumns:Cell::new(lc-fc+1),orgLastRow:Cell::new(0),orgLastColumn:Cell::new(0),rav:Cell::new(rav)}
}
// java public final int getFirstRow() {
// java return this._firstRow;
// java }
pub fn getFirstRow(&self)->i32 {
self.firstRow.get()
}
// java public final int getFirstColumn() {
// java return this._firstColumn;
// java }
pub fn getFirstColumn(&self)->i32 {
self.firstColumn.get()
}
// java public final int getLastRow() {
// java return this._lastRow;
// java }
pub fn getLastRow(&self)->i32 {
self.lastRow.get()
}
// java public final int getLastColumn() {
// java return this._lastColumn;
// java }
pub fn getLastColumn(&self)->i32 {
self.lastColumn.get()
}
// java public final boolean contains(int row, int col) {
// java return this._firstRow <= row && this._lastRow >= row && this._firstColumn <= col && this._lastColumn >= col;
// java }
pub fn contains(&self,row:i32,col:i32)->bool {
self.firstRow.get()<=row&&self.lastRow.get()>=row&&self.firstColumn.get()<=col&&self.lastColumn.get()>=col
}
// java public final boolean containsRow(int row) {
// java return this._firstRow <= row && this._lastRow >= row;
// java }
pub fn containsRow(&self,row:i32)->bool {
self.firstRow.get()<=row&&self.lastRow.get()>=row
}
// java public final boolean containsColumn(int col) {
// java return this._firstColumn <= col && this._lastColumn >= col;
// java }
pub fn containsColumn(&self,col:i32)->bool {
self.firstColumn.get()<=col&&self.lastColumn.get()>=col
}
// java public final boolean isColumn() {
// java return this._firstColumn == this._lastColumn;
// java }
pub fn isColumn(&self)->bool {
self.firstColumn.get()==self.lastColumn.get()
}
// java public final boolean isRow() {
// java return this._firstRow == this._lastRow;
// java }
pub fn isRow(&self)->bool {
self.firstRow.get()==self.lastRow.get()
}
// java public int getHeight() {
// java return this._lastRow - this._firstRow + 1;
// java }
pub fn getHeight(&self)->i32 {
self.lastRow.get()-self.firstRow.get()+1
}
// java public int getWidth() {
// java return this._lastColumn - this._firstColumn + 1;
// java }
pub fn getWidth(&self)->i32 {
self.lastColumn.get()-self.firstColumn.get()+1
}
// java public byte getRAV() {
// java return this._RAV;
// java }
pub fn getRAV(&self)->i8 {
self.rav.get()
}
// java public void setRAV(byte rav) {
// java this._RAV = rav;
// java }
pub fn setRAV(&self,v:i8) {
self.rav.set(v);
}
// java public boolean clip(int minRow, int minCol, int maxRow, int maxCol) {
// java this._orgLastRow = this._orgLastRow <= this._lastRow ? this._lastRow : this._orgLastRow;
// java this._orgLastColumn = this._orgLastColumn <= this._lastColumn ? this._lastColumn : this._orgLastColumn;
// java this._firstRow = Math.min(Math.max(minRow, this._firstRow), maxRow);
// java this._firstColumn = Math.min(Math.max(minCol, this._firstColumn), maxCol);
// java this._lastRow = Math.min(maxRow, this._lastRow);
// java this._lastColumn = Math.min(maxCol, this._lastColumn);
// java int cols = this._lastColumn - this._firstColumn + 1;
// java int rows = this._lastRow - this._firstRow + 1;
// java if (this._nColumns == cols && this._nRows == rows) {
// java return false;
// java } else {
// java this._nColumns = cols;
// java this._nRows = rows;
// java return true;
// java }
// java }
pub fn clip(&self,minRow:i32,minCol:i32,maxRow:i32,maxCol:i32)->bool {
self.orgLastRow.set(self.orgLastRow.get().max(self.lastRow.get()));self.orgLastColumn.set(self.orgLastColumn.get().max(self.lastColumn.get()));self.firstRow.set(minRow.max(self.firstRow.get()).min(maxRow));self.firstColumn.set(minCol.max(self.firstColumn.get()).min(maxCol));self.lastRow.set(maxRow.min(self.lastRow.get()));self.lastColumn.set(maxCol.min(self.lastColumn.get()));let rows=self.getHeight();let cols=self.getWidth();if self.nColumns.get()==cols&&self.nRows.get()==rows{false}else{self.nColumns.set(cols);self.nRows.set(rows);true}
}
}
