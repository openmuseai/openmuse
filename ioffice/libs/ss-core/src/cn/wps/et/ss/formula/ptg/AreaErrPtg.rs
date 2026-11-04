// java public final class AreaErrPtg extends OperandPtg {
pub struct AreaErrPtg;
impl AreaErrPtg{
    // java public String toFormulaString(){return ErrorConstants.getText(23);}
    pub fn toFormulaString()->String{crate::org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants::getText(23)}
}
