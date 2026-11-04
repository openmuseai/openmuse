use super::Ptg::{Ptg,PtgKind};
// java abstract class Area2DPtgBase extends AreaPtgBase {
impl Ptg{
    // java protected Area2DPtgBase(int firstRow, int lastRow, int firstColumn, int lastColumn, boolean firstRowRelative, boolean lastRowRelative, boolean firstColRelative, boolean lastColRelative) {
    // Native wide metadata selects the original Area03/Area10 concrete class.
    pub(crate) fn newArea2D(firstRow:i32,lastRow:i32,firstColumn:i32,lastColumn:i32,firstRowRelative:bool,lastRowRelative:bool,firstColRelative:bool,lastColRelative:bool,wide:bool)->Self{
        let mut ret=Self::base(PtgKind::Area{first_row:0,last_row:0,first_col_raw:0,last_col_raw:0,wide,relative:false});
        // java super(firstRow, lastRow, firstColumn, lastColumn, firstRowRelative, lastRowRelative, firstColRelative, lastColRelative);
        ret.initializeAreaCoordinates(firstRow,lastRow,firstColumn,lastColumn,firstRowRelative,lastRowRelative,firstColRelative,lastColRelative);ret
    }
    // java public final String toString() {
    pub fn toArea2DDebugString(&self)->String{
        // java StringBuffer sb = new StringBuffer();
        let mut sb=String::new();
        // java sb.append(this.getClass().getName()); sb.append(" [");
        sb.push_str(&self.javaClassName());sb.push_str(" [");
        // java sb.append(this.formatReferenceAsString());
        sb.push_str(&self.formatAreaReferenceAsString());
        // java sb.append("]"); return sb.toString();
        sb.push(']');sb
    }
}
