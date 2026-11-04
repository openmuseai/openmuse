use super::ValueType::ValueType;
#[derive(Clone,Debug)]pub enum Value{Number(f64),Bool(bool),Text(Vec<u16>)}
// java public class RecogniseResultRl
pub struct RecogniseResultRl{pub rtype:Option<ValueType>,pub value:Option<Value>,suggestedNumfmt:Option<String>,builtinNumfmtIdx:i32,pub sqleading:bool}impl RecogniseResultRl{pub fn new()->Self{Self{rtype:None,value:None,suggestedNumfmt:None,builtinNumfmtIdx:-1,sqleading:false}}pub fn double(&self)->f64{match self.value.as_ref().unwrap(){Value::Number(v)=>*v,_=>panic!("ClassCastException Double")}}
// java public void setSuggestedNumfmt(String fmt) {
// java this.suggestedNumfmt = fmt;
// java this.builtinNumfmtIdx = -1;
// java }
pub fn setSuggestedNumfmtS(&mut self,v:&str) {
self.suggestedNumfmt=Some(v.to_string());self.builtinNumfmtIdx= -1;
}
// java public void setSuggestedNumfmt(int numfmt) {
// java this.builtinNumfmtIdx = numfmt;
// java this.suggestedNumfmt = null;
// java }
pub fn setSuggestedNumfmtI(&mut self,v:i32) {
self.builtinNumfmtIdx=v;self.suggestedNumfmt=None;
}
// java public String getSuggestedNumfmt() {
// java return this.suggestedNumfmt;
// java }
pub fn getSuggestedNumfmt(&self)->Option<String> {
self.suggestedNumfmt.clone()
}
// java public int getBuiltInNumfmtIdx() {
// java return this.builtinNumfmtIdx;
// java }
pub fn getBuiltInNumfmtIdx(&self)->i32 {
self.builtinNumfmtIdx
}
// java public ValueType getValueType() {
// java return this.type;
// java }
pub fn getValueType(&self)->Option<ValueType> {
self.rtype
}
// java public Object getValue() {
// java return this.value;
// java }
pub fn getValue(&self)->Option<Value> {
self.value.clone()
}
}
