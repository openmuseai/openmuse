// java public final class MissingArgPtg extends ScalarConstantPtg {
pub struct MissingArgPtg;
impl MissingArgPtg{
    // java public String toFormulaString(){return " ";}
    pub fn toFormulaString()->String{" ".into()}
}
