// java public final class NumberPtg extends ScalarConstantPtg {
use super::Ptg::{Ptg,PtgKind};
use crate::org::apache::poi::ss::util::NumberToTextConverter::NumberToTextConverter;
impl Ptg{
    // java public double getValue(){return this.field_1_value;}
    pub fn getNumberValue(&self)->f64{match self.kind{PtgKind::Number(v)=>v,_=>panic!("ClassCastException: NumberPtg")}}
    // java public String toFormulaString(){return NumberToTextConverter.toText(this.field_1_value,'.');}
    pub fn toNumberFormulaString(&self)->String{NumberToTextConverter::toText(self.getNumberValue(),'.')}
    // java public String toFormulaString(char decimalSeparator){return NumberToTextConverter.toText(this.field_1_value,decimalSeparator);}
    pub fn toNumberFormulaString__separator(&self,decimalSeparator:char)->String{NumberToTextConverter::toText(self.getNumberValue(),decimalSeparator)}
}
