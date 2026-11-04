use super::super::Ptg::{Ptg,PtgKind};
// java public abstract class Area3DExtPtg extends AreaPtgBase implements Cloneable {
impl Ptg{
    // java public String toString() {
    pub fn toArea3DExtDebugString(&self)->String{
        // java StringBuffer sb = new StringBuffer(); sb.append(this.getClass().getName()); sb.append(" [");
        let mut sb=String::new();sb.push_str(&self.javaClassName());sb.push_str(" [");
        let PtgKind::Area3DExt{sheet_index1,sheet_index2,..}=self.kind else{panic!("ClassCastException: Area3DExtPtg")};
        // java sb.append("sheetIx=").append(this.field_1_sheet_index1);
        sb.push_str(&format!("sheetIx={sheet_index1}"));
        // java if (this.field_2_sheet_index2 != this.field_1_sheet_index1) { sb.append(':').append(this.field_2_sheet_index2); }
        if sheet_index2!=sheet_index1{sb.push(':');sb.push_str(&sheet_index2.to_string());}
        // java sb.append(" ! "); sb.append(this.formatReferenceAsString());
        sb.push_str(" ! ");sb.push_str(&self.formatAreaReferenceAsString());
        // java sb.append("]"); return sb.toString();
        sb.push(']');sb
    }
}
