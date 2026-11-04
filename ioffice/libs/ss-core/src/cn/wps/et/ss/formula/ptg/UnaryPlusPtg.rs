// java public final class UnaryPlusPtg extends ValueOperatorPtg {
pub struct UnaryPlusPtg;
impl UnaryPlusPtg{
    // java public int getNumberOfOperands(){return 1;}
    pub fn getNumberOfOperands()->i32{1}
    // java public String toFormulaString(String[] operands){
    pub fn toFormulaString(operands:Option<&[Option<String>]>)->String{
        // java StringBuffer buffer=new StringBuffer();buffer.append("+");buffer.append(operands[0]);return buffer.toString();
        let mut buffer=String::new();buffer.push_str("+");buffer.push_str(super::ValueOperatorPtg::operand(operands,0));buffer
    }
}
