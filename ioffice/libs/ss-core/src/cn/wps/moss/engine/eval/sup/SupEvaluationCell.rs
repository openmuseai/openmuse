use crate::{java_runtime::JavaObject::NullableObject,cn::wps::et::ss::{interfaces::IEvalCell::IEvalCell,formula::evaluator::EvalResult}};
// java public final class SupEvaluationCell implements IEvalCell {
pub struct SupEvaluationCell{
    // java private Object value;
    value:NullableObject,
    // java private int sheetIndex;
    sheetIndex:i32,
    // java private int row;
    row:i32,
    // java private int col;
    col:i32,
}
impl SupEvaluationCell{
    // java public SupEvaluationCell(int sheetIndex,int row,int col,Object v){this.sheetIndex=sheetIndex;this.row=row;this.col=col;this.value=v;}
    pub fn SupEvaluationCell(sheetIndex:i32,row:i32,col:i32,v:NullableObject)->Self{Self{value:v,sheetIndex,row,col}}
    // java public Boolean checkRecalculationFlag(){return null;}
    pub fn checkRecalculationFlag(&self)->Option<bool>{None}
    // java public int getFinalCellType(){if(this.value==null){return 3;}else if(this.value instanceof Double){return 0;}else if(this.value instanceof Boolean){return 4;}else if(this.value instanceof Integer){return 5;}else if(this.value instanceof String){return 1;}else{throw new IllegalStateException();}}
    pub fn getFinalCellType(&self)->i32{let Some(value)=&self.value else{return 3;};if value.as_any().is::<f64>(){0}else if value.as_any().is::<bool>(){4}else if value.as_any().is::<i32>(){5}else if value.as_any().is::<String>(){1}else{panic!("IllegalStateException")}}
    // java public void updateCell() { }
    pub fn updateCell(&self){}
    // java public double getNumericCellValue(){return (Double)this.value;}
    pub fn getNumericCellValue(&self)->f64{*self.value.as_ref().expect("NullPointerException").as_any().downcast_ref::<f64>().expect("ClassCastException")}
    // java public String getStringCellValue(){return (String)this.value;}
    pub fn getStringCellValue(&self)->Option<String>{self.value.as_ref().map(|v|v.as_any().downcast_ref::<String>().expect("ClassCastException").clone())}
    // java public boolean getBooleanCellValue(){return (Boolean)this.value;}
    pub fn getBooleanCellValue(&self)->bool{*self.value.as_ref().expect("NullPointerException").as_any().downcast_ref::<bool>().expect("ClassCastException")}
    // java public byte getErrorCellValue(){return ((Integer)this.value).byteValue();}
    pub fn getErrorCellValue(&self)->i8{*self.value.as_ref().expect("NullPointerException").as_any().downcast_ref::<i32>().expect("ClassCastException")as i8}
    // java public int getSheetUid(){return 65536+this.sheetIndex;}
    pub fn getSheetUid(&self)->i32{65536i32.wrapping_add(self.sheetIndex)}
    // java public int getColIndex(){return this.col;}
    pub fn getColIndex(&self)->i32{self.col}
    // java public int getRowIndex(){return this.row;}
    pub fn getRowIndex(&self)->i32{self.row}
}
// The interface preserves the same nullable String reference as the concrete getter.
impl IEvalCell for SupEvaluationCell{
    fn checkRecalculationFlag(&self)->EvalResult<Option<bool>>{Ok(Self::checkRecalculationFlag(self))}
    fn getFinalCellType(&self)->i32{Self::getFinalCellType(self)}fn updateCell(&self){Self::updateCell(self)}fn getNumericCellValue(&self)->f64{Self::getNumericCellValue(self)}fn getStringCellValue(&self)->Option<String>{Self::getStringCellValue(self)}fn getBooleanCellValue(&self)->bool{Self::getBooleanCellValue(self)}fn getErrorCellValue(&self)->i8{Self::getErrorCellValue(self)}fn getSheetUid(&self)->i32{Self::getSheetUid(self)}fn getColIndex(&self)->i32{Self::getColIndex(self)}fn getRowIndex(&self)->i32{Self::getRowIndex(self)}
}
