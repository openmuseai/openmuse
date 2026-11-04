use crate::java_runtime::{Cell,RefCell,Rc};
use util::org::apache::poi::util::{LittleEndianOutput::LittleEndianOutput,HexDump::HexDump};
use super::{CellValueRecordInterface::CellValueRecordInterface,CellAttrRecord_BIFF2::CellAttrRecord_BIFF2};
// java public abstract class CellRecord extends StandardRecord implements CellValueRecordInterface, ReusableRecord {
pub struct CellRecord{
    // java protected int _rowIndex;
    pub(super) _rowIndex:Cell<i32>,
    // java protected int _columnIndex;
    pub(super) _columnIndex:Cell<i32>,
    // java protected int _formatIndex;
    pub(super) _formatIndex:Cell<i32>,
    // java protected CellAttrRecord_BIFF2 _cellAttr;
    pub(super) _cellAttr:RefCell<Option<Rc<CellAttrRecord_BIFF2>>>,
}
impl CellRecord{
    // java protected CellRecord() { }
    pub(super) fn new()->Self{Self{_rowIndex:Cell::new(0),_columnIndex:Cell::new(0),_formatIndex:Cell::new(0),_cellAttr:RefCell::new(None)}}
    // java protected CellRecord(int row, int col, int ixf) {
    pub(super) fn new__3(row:i32,col:i32,ixf:i32)->Self{
        // java this._rowIndex = row; this._columnIndex = col; this._formatIndex = ixf;
        Self{_rowIndex:Cell::new(row),_columnIndex:Cell::new(col),_formatIndex:Cell::new(ixf),_cellAttr:RefCell::new(None)}
    }
    // rust: BIFF reader constructors and reuseObject wait for RecordInputStream.
    // java public final void setRow(int row) { this._rowIndex = row; }
    pub fn setRow(&self,row:i32){self._rowIndex.set(row);}
    // java public final void setColumn(short col) { this._columnIndex = col; }
    pub fn setColumn(&self,col:i16){self._columnIndex.set(col as i32);}
    // java public final void setXFIndex(int xf) { this._formatIndex = xf; }
    pub fn setXFIndex(&self,xf:i32){self._formatIndex.set(xf);}
    // java public final int getRow() { return this._rowIndex; }
    pub fn getRow(&self)->i32{self._rowIndex.get()}
    // java public final short getColumn() { return (short)(this._columnIndex & 32767); }
    pub fn getColumn(&self)->i16{(self._columnIndex.get()&32767)as i16}
    // java public final int getXFIndex() { return this._formatIndex; }
    pub fn getXFIndex(&self)->i32{self._formatIndex.get()}
    // java public final String toString() {
    pub fn toString(&self,record:&dyn CellRecordText)->String{
        // java StringBuilder sb = new StringBuilder(); String recordName = this.getRecordName();
        let mut sb=String::new();let recordName=record.getRecordName();
        // java sb.append("[").append(recordName).append("]\n");
        sb.push('[');sb.push_str(recordName.as_deref().unwrap_or("null"));sb.push_str("]\n");
        // java sb.append("    .row    = ").append(HexDump.shortToHex(this.getRow())).append("\n");
        sb.push_str("    .row    = ");sb.extend(HexDump::shortToHex(self.getRow()));sb.push('\n');
        // java sb.append("    .col    = ").append(HexDump.shortToHex(this.getColumn())).append("\n");
        sb.push_str("    .col    = ");sb.extend(HexDump::shortToHex(self.getColumn()as i32));sb.push('\n');
        // java sb.append("    .xfindex= ").append(HexDump.shortToHex(this.getXFIndex())).append("\n");
        sb.push_str("    .xfindex= ");sb.extend(HexDump::shortToHex(self.getXFIndex()));sb.push('\n');
        // java this.appendValueText(sb); sb.append("\n");
        record.appendValueText(&mut sb);sb.push('\n');
        // java sb.append("[/").append(recordName).append("]\n"); return sb.toString();
        sb.push_str("[/");sb.push_str(recordName.as_deref().unwrap_or("null"));sb.push_str("]\n");sb
    }
    // java public final void serializeData(LittleEndianOutput out) {
    pub fn serializeData(&self,record:&dyn CellRecordValue,out:&mut dyn LittleEndianOutput){
        // java out.writeShort(this.getRow());
        out.writeShort(self.getRow());
        // java out.writeShort(this.getColumn());
        out.writeShort(self.getColumn()as i32);
        // java out.writeShort(this.getXFIndex());
        out.writeShort(self.getXFIndex());
        // java this.serializeValue(out);
        record.serializeValue(out);
    }
    // java protected final int getDataSize() { return 6 + this.getValueDataSize(); }
    pub fn getDataSize(&self,record:&dyn CellRecordValue)->i32{6i32.wrapping_add(record.getValueDataSize())}
    // java protected final void copyBaseFields(CellRecord rec) {
    pub(super) fn copyBaseFields(&self,rec:Option<&CellRecord>){
        let rec=rec.expect("NullPointerException");
        // java rec._rowIndex = this._rowIndex;
        rec._rowIndex.set(self._rowIndex.get());
        // java rec._columnIndex = this._columnIndex;
        rec._columnIndex.set(self._columnIndex.get());
        // java rec._formatIndex = this._formatIndex;
        rec._formatIndex.set(self._formatIndex.get());
    }
    // java public CellAttrRecord_BIFF2 getCellAttribute() { return this._cellAttr; }
    pub fn getCellAttribute(&self)->Option<Rc<CellAttrRecord_BIFF2>>{self._cellAttr.borrow().clone()}
}
// rust: The two halves of the original abstract dispatch. The explicit record
// argument is the derived Java `this`, never a second record or copy of fields.
pub trait CellRecordValue{
    // java protected abstract void serializeValue(LittleEndianOutput var1);
    fn serializeValue(&self,out:&mut dyn LittleEndianOutput);
    // java protected abstract int getValueDataSize();
    fn getValueDataSize(&self)->i32;
}
pub trait CellRecordText{
    // java protected abstract void appendValueText(StringBuilder var1);
    fn appendValueText(&self,sb:&mut String);
    // java protected abstract String getRecordName();
    fn getRecordName(&self)->Option<String>;
}
impl CellValueRecordInterface for CellRecord{
    fn getRow(&self)->i32{Self::getRow(self)}fn getColumn(&self)->i16{Self::getColumn(self)}
    fn setRow(&self,row:i32){Self::setRow(self,row)}fn setColumn(&self,col:i16){Self::setColumn(self,col)}
    fn setXFIndex(&self,xf:i32){Self::setXFIndex(self,xf)}fn getXFIndex(&self)->i32{Self::getXFIndex(self)}
}
#[cfg(test)]
#[path="CellRecord.review.rs"]
mod review;
