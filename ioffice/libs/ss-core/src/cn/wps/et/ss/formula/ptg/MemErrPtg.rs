// java public final class MemErrPtg extends OperandPtg {
pub struct MemErrPtg;
impl MemErrPtg{
    // java public String toFormulaString(){return "ERR#";}
    pub fn toFormulaString()->String{"ERR#".into()}
}
