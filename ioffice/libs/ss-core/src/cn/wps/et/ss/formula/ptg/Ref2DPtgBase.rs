use super::Ptg::{Ptg,PtgKind};
// java abstract class Ref2DPtgBase extends RefPtgBase {
impl Ptg{
    // java protected Ref2DPtgBase(int row, int column, boolean isRowRelative, boolean isColumnRelative) {
    // Native wide/relative metadata chooses the actual Ref03/Ref10 concrete class.
    pub(crate) fn newRef2D(row:i32,column:i32,isRowRelative:bool,isColumnRelative:bool,wide:bool,relative:bool)->Self{
        let mut ret=Self::base(PtgKind::Ref{row:0,col_raw:0,wide,relative});
        // java this.setRow(row); this.setColumn(column); this.setRowRelative(isRowRelative); this.setColRelative(isColumnRelative);
        ret.setRow(row);ret.setColumn(column);ret.setRowRelative(isRowRelative);ret.setColRelative(isColumnRelative);ret
    }
    // java public final String toString() {
    pub fn toRef2DDebugString(&self)->String{
        // java StringBuffer sb = new StringBuffer();
        let mut sb=String::new();
        // java sb.append(this.getClass().getName()); sb.append(" [");
        sb.push_str(&self.javaClassName());sb.push_str(" [");
        // java sb.append(this.formatReferenceAsString());
        sb.push_str(&self.formatReferenceAsString());
        // java sb.append("]"); return sb.toString();
        sb.push(']');sb
    }
}
