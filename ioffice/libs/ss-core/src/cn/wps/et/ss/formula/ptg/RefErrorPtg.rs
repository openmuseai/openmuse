// java public final class RefErrorPtg extends OperandPtg {
pub struct RefErrorPtg;
impl RefErrorPtg{
    // java public String toFormulaString(){return ErrorConstants.getText(23);}
    pub fn toFormulaString()->String{crate::org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants::getText(23)}
}
impl super::Ptg::Ptg{
    // java public String toString() { return this.getClass().getName(); }
    pub fn toRefErrorDebugString(&self)->String{self.javaClassName()}
}
