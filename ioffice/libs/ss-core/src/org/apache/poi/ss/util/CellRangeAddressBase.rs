// java package org.apache.poi.ss.util;
// java public abstract class CellRangeAddressBase implements Cloneable {
#[derive(Clone)]
pub struct CellRangeAddressBase{
    // java private int _firstRow;
    _firstRow:i32,
    // java private int _firstCol;
    _firstCol:i32,
    // java private int _lastRow;
    _lastRow:i32,
    // java private int _lastCol;
    _lastCol:i32,
}
impl CellRangeAddressBase{
    // java protected CellRangeAddressBase(int firstRow,int lastRow,int firstCol,int lastCol) {
    pub(crate) fn CellRangeAddressBase(firstRow:i32,lastRow:i32,firstCol:i32,lastCol:i32)->Self{
        // java this._firstRow=firstRow; this._lastRow=lastRow; this._firstCol=firstCol; this._lastCol=lastCol;
        Self{_firstRow:firstRow,_lastRow:lastRow,_firstCol:firstCol,_lastCol:lastCol}
    }
    // java public final int getFirstColumn() { return this._firstCol; }
    pub fn getFirstColumn(&self)->i32{self._firstCol}
    // java public final int getFirstRow() { return this._firstRow; }
    pub fn getFirstRow(&self)->i32{self._firstRow}
    // java public final int getLastColumn() { return this._lastCol; }
    pub fn getLastColumn(&self)->i32{self._lastCol}
    // java public final int getLastRow() { return this._lastRow; }
    pub fn getLastRow(&self)->i32{self._lastRow}
    // java public final void setFirstColumn(int firstCol) { this._firstCol=firstCol; }
    pub fn setFirstColumn(&mut self,firstCol:i32){self._firstCol=firstCol;}
    // java public final void setFirstRow(int firstRow) { this._firstRow=firstRow; }
    pub fn setFirstRow(&mut self,firstRow:i32){self._firstRow=firstRow;}
    // java public final void setLastColumn(int lastCol) { this._lastCol=lastCol; }
    pub fn setLastColumn(&mut self,lastCol:i32){self._lastCol=lastCol;}
    // java public final void setLastRow(int lastRow) { this._lastRow=lastRow; }
    pub fn setLastRow(&mut self,lastRow:i32){self._lastRow=lastRow;}
    // java public boolean isInRange(int rowInd,int colInd) { return this._firstRow<=rowInd && rowInd<=this._lastRow && this._firstCol<=colInd && colInd<=this._lastCol; }
    pub fn isInRange(&self,rowInd:i32,colInd:i32)->bool{self._firstRow<=rowInd&&rowInd<=self._lastRow&&self._firstCol<=colInd&&colInd<=self._lastCol}
}
