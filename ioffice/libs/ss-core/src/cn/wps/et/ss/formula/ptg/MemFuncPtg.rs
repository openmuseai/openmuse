// java public final class MemFuncPtg extends OperandPtg {
pub struct MemFuncPtg;
impl MemFuncPtg{
    // java public String toFormulaString(){return "";}
    pub fn toFormulaString()->String{String::new()}
}
impl super::Ptg::Ptg{
    // java public final String toString() {
    pub fn toMemFuncDebugString(&self)->String{
        // java StringBuffer sb = new StringBuffer(64); sb.append(this.getClass().getName()).append(" [len=");
        let mut sb=String::with_capacity(64);sb.push_str(&self.javaClassName());sb.push_str(" [len=");
        // java sb.append(this.field_1_len_ref_subexpression); sb.append("]"); return sb.toString();
        let super::Ptg::PtgKind::MemFunc{subex_len}=self.kind else{panic!("ClassCastException: MemFuncPtg")};sb.push_str(&subex_len.to_string());sb.push(']');sb
    }
}
