// java public final class ParenthesisPtg extends ControlPtg {
pub struct ParenthesisPtg;
impl ParenthesisPtg{
    // java public String toFormulaString(){return "()";}
    pub fn toFormulaString()->String{"()".into()}
    // java public String toFormulaString(String[] operands){return "("+operands[0]+")";}
    pub fn toFormulaString__operands(operands:Option<&[Option<String>]>)->String{format!("({})",super::ValueOperatorPtg::operand(operands,0))}
}
