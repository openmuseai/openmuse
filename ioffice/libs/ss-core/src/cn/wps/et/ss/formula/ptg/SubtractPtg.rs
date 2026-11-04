// java public final class SubtractPtg extends ValueOperatorPtg {
pub struct SubtractPtg;
impl SubtractPtg{
    // java public int getNumberOfOperands(){return 2;}
    pub fn getNumberOfOperands()->i32{2}
    // java public String toFormulaString(String[] operands){
    pub fn toFormulaString(operands:Option<&[Option<String>]>)->String{
        // java StringBuffer buffer=new StringBuffer();buffer.append(operands[0]);buffer.append("-");buffer.append(operands[1]);return buffer.toString();
        let mut buffer=String::new();buffer.push_str(super::ValueOperatorPtg::operand(operands,0));buffer.push_str("-");buffer.push_str(super::ValueOperatorPtg::operand(operands,1));buffer
    }
}
