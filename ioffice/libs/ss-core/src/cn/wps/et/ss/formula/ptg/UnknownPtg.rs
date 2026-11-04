// java public final class UnknownPtg extends Ptg {
pub struct UnknownPtg;
impl UnknownPtg{
    // java public String toFormulaString(){return "UNKNOWN";}
    pub fn toFormulaString()->String{"UNKNOWN".into()}
}
