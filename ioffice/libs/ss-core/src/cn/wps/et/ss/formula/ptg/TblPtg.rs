// java public abstract class TblPtg extends ControlPtg {
pub struct TblPtg;
impl TblPtg{
    // java public String toFormulaString(){throw new RuntimeException("Table and Arrays are not yet supported");}
    pub fn toFormulaString()->String{panic!("RuntimeException: Table and Arrays are not yet supported")}
}
impl super::Ptg::Ptg{
    // java public String toString() {
    pub fn toTblDebugString(&self)->String{
        // java StringBuffer buffer = new StringBuffer("[Data Table - Parent cell is an interior cell in a data table]\n");
        let mut buffer=String::from("[Data Table - Parent cell is an interior cell in a data table]\n");
        // rust: Tbl03/Tbl10 getRow/getColumn dispatch through their same carrier fields.
        let super::Ptg::PtgKind::Tbl{row,col,..}=self.kind else{panic!("ClassCastException: TblPtg")};
        // java buffer.append("top left row = ").append(this.getRow()).append("\n");
        buffer.push_str(&format!("top left row = {row}\n"));
        // java buffer.append("top left col = ").append(this.getColumn()).append("\n"); return buffer.toString();
        buffer.push_str(&format!("top left col = {col}\n"));buffer
    }
}
