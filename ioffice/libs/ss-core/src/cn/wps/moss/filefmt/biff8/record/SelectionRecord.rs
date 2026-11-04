use super::super::util::CellRangeAddress::CellRangeAddress;
// java public final class SelectionRecord extends StandardRecord
pub struct SelectionRecord{pane:i8,row:i32,col:i32,activeRef:i32,refs:Vec<CellRangeAddress>}
impl SelectionRecord{
// java public SelectionRecord(int activeCellRow, int activeCellCol) {
// java this.field_1_pane = 3;
// java this.field_2_row_active_cell = activeCellRow;
// java this.field_3_col_active_cell = activeCellCol;
// java this.field_4_active_cell_ref_index = 0;
// java this.field_6_refs = new CellRangeAddress8Bit[]{new CellRangeAddress8Bit(activeCellRow, activeCellRow, activeCellCol, activeCellCol)};
// java }
pub fn new(row:i32,col:i32)->Self{Self{pane:3,row,col,activeRef:0,refs:vec![CellRangeAddress::new(row,row,col,col)]}}
// java public byte getPane() {
// java return this.field_1_pane;
// java }
pub fn getPane(&self)->i8{self.pane}
// java public void setPane(byte pane) {
// java this.field_1_pane = pane;
// java }
pub fn setPane(&mut self,v:i8){self.pane=v as i8;}
// java public int getActiveCellRow() {
// java return this.field_2_row_active_cell;
// java }
pub fn getActiveCellRow(&self)->i32{self.row}
// java public void setActiveCellRow(int row) {
// java this.field_2_row_active_cell = row;
// java }
pub fn setActiveCellRow(&mut self,v:i32){self.row=v as i32;}
// java public int getActiveCellCol() {
// java return this.field_3_col_active_cell;
// java }
pub fn getActiveCellCol(&self)->i32{self.col}
// java public void setActiveCellCol(short col) {
// java this.field_3_col_active_cell = col;
// java }
pub fn setActiveCellCol(&mut self,v:i16){self.col=v as i32;}
// java public int getActiveCellRef() {
// java return this.field_4_active_cell_ref_index;
// java }
pub fn getActiveCellRef(&self)->i32{self.activeRef}
// java public void setActiveCellRef(int ref) {
// java this.field_4_active_cell_ref_index = ref;
// java }
pub fn setActiveCellRef(&mut self,v:i32){self.activeRef=v as i32;}
// java public void setRefs(List<CellRangeAddress> rgs) {
// java this.field_6_refs = new CellRangeAddress8Bit[rgs.size()];
// java int i = 0;
// java
// java for(int len = rgs.size(); i < len; ++i) {
// java CellRangeAddress rg = (CellRangeAddress)rgs.get(i);
// java this.field_6_refs[i] = new CellRangeAddress8Bit(rg.getFirstRow(), rg.getLastRow(), rg.getFirstColumn(), rg.getLastColumn());
// java }
// java
// java }
pub fn setRefs(&mut self,refs:Vec<CellRangeAddress>){self.refs=refs;}
pub fn refs(&self)->&[CellRangeAddress]{&self.refs}
}
