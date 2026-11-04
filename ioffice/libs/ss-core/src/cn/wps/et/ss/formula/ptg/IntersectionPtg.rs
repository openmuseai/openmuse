// java public final class IntersectionPtg extends OperationPtg {
pub struct IntersectionPtg;
impl IntersectionPtg{
    // java public String toFormulaString(){return " ";}
    pub fn toFormulaString()->String{" ".into()}
    // java public String toFormulaString(String[] operands,char argSeparator,char unionSeparator){
    pub fn toFormulaString__operands(operands:Option<&[Option<String>]>,_argSeparator:char,_unionSeparator:char)->String{
        // java StringBuffer buffer=new StringBuffer();buffer.append(operands[0]);buffer.append(" ");buffer.append(operands[1]);return buffer.toString();
        let mut buffer=String::new();buffer.push_str(super::ValueOperatorPtg::operand(operands,0));buffer.push_str(" ");buffer.push_str(super::ValueOperatorPtg::operand(operands,1));buffer
    }
    // java public int getNumberOfOperands(){return 2;}
    pub fn getNumberOfOperands()->i32{2}
}
