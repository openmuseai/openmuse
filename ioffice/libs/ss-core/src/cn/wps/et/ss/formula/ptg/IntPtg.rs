// java public final class IntPtg extends ScalarConstantPtg {
// rust: methods on the current Ptg carrier; scalar class inheritance is not implied.
use super::Ptg::{Ptg,PtgKind};
impl Ptg{
    // java public int getValue(){return this.field_1_value;}
    pub fn getIntValue(&self)->i32{match self.kind{PtgKind::Int(v)=>v as i32,_=>panic!("ClassCastException: IntPtg")}}
    // java public String toFormulaString(){return String.valueOf(this.getValue());}
    pub fn toIntFormulaString(&self)->String{self.getIntValue().to_string()}
}
