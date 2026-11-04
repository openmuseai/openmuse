use super::Ptg::Ptg;
// java public abstract class ScalarConstantPtg extends ConstantPtg {
impl Ptg{
    // java public final String toString() {
    pub fn toScalarDebugString(&self)->String{
        // java StringBuffer sb = new StringBuffer(64);
        let mut sb=String::with_capacity(64);
        // java sb.append(this.getClass().getName()).append(" [");
        sb.push_str(&self.javaClassName());sb.push_str(" [");
        // java sb.append(this.toFormulaString());
        sb.push_str(self.toFormulaString().as_deref().unwrap_or("null"));
        // java sb.append("]"); return sb.toString();
        sb.push(']');sb
    }
}
