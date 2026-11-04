// java public final class BoolPtg extends ScalarConstantPtg {
use super::Ptg::{Ptg,PtgKind};
impl Ptg{
    // java public boolean getValue(){return this._value;}
    pub fn getBoolValue(&self)->bool{match self.kind{PtgKind::Bool(v)=>v,_=>panic!("ClassCastException: BoolPtg")}}
    // java public String toFormulaString(){return this._value?"TRUE":"FALSE";}
    pub fn toBoolFormulaString(&self)->String{if self.getBoolValue(){"TRUE"}else{"FALSE"}.into()}
}
