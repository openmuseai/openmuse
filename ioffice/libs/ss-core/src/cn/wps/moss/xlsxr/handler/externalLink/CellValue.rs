use crate::java_runtime::JavaObject::NullableObject;
// java class CellValue { int column; Object value;
pub(super) struct CellValue{pub column:i32,pub value:NullableObject}
impl CellValue{
    // java public CellValue(int column,Object value){this.column=column;this.value=value;}
    pub fn CellValue(column:i32,value:NullableObject)->Self{Self{column,value}}
}
