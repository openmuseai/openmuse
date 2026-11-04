// java public final class ErrPtg extends ScalarConstantPtg {
use super::Ptg::{Ptg,PtgKind};
use crate::org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants;
impl Ptg{
    // java public String toFormulaString(){return ErrorConstants.getText(this.field_1_error_code);}
    pub fn toErrFormulaString(&self)->String{ErrorConstants::getText(self.getErrorCode())}
    // java public int getErrorCode(){return this.field_1_error_code;}
    pub fn getErrorCode(&self)->i32{match self.kind{PtgKind::Err(v)=>v as i32,_=>panic!("ClassCastException: ErrPtg")}}
}
