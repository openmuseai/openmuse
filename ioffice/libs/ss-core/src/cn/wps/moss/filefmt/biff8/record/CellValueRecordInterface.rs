// java public interface CellValueRecordInterface {
pub trait CellValueRecordInterface{
    // java int getRow();
    fn getRow(&self)->i32;
    // java short getColumn();
    fn getColumn(&self)->i16;
    // java void setRow(int var1);
    fn setRow(&self,var1:i32);
    // java void setColumn(short var1);
    fn setColumn(&self,var1:i16);
    // java void setXFIndex(int var1);
    fn setXFIndex(&self,var1:i32);
    // java int getXFIndex();
    fn getXFIndex(&self)->i32;
}
