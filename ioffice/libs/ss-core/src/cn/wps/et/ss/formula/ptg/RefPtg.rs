use super::Ptg::{Ptg,PtgRef,ptgRef};
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
// java public abstract class RefPtg extends Ref2DPtgBase {
// Static class facade; instances remain the same typed Ptg field/reference carrier.
pub enum RefPtg{}
impl RefPtg{
    // java public static RefPtg createFromNptg(RefNPtg rnptg, int row, int col, SpreadsheetVersion sv) {
    pub fn createFromNptg(rnptg:Option<&PtgRef>,row:i32,col:i32,sv:Option<SpreadsheetVersion>)->PtgRef{
        // java int maxRows = sv.getMaxRows(); int maxCols = sv.getMaxColumns(); RefPtg r = null;
        let sv=sv.expect("NullPointerException");let maxRows=sv.getMaxRows();let maxCols=sv.getMaxColumns();let mut r=None;
        // java boolean isRowRelative = rnptg.isRowRelative(); boolean isColRelative = rnptg.isColRelative();
        let rnptg=rnptg.expect("NullPointerException").borrow();let isRowRelative=rnptg.isRowRelative();let isColRelative=rnptg.isColRelative();
        // java if (sv == SpreadsheetVersion.EXCEL97) {
        if sv==SpreadsheetVersion::EXCEL97{
            // java r = new Ref03Ptg(isRowRelative ? (rnptg.getRow() + row) % maxRows : rnptg.getRow(), isColRelative ? (rnptg.getColumn() + col) % maxCols : rnptg.getColumn(), isRowRelative, isColRelative);
            r=Some(Ptg::newRef2D(if isRowRelative{rnptg.getRow().wrapping_add(row)%maxRows}else{rnptg.getRow()},if isColRelative{rnptg.getColumn().wrapping_add(col)%maxCols}else{rnptg.getColumn()},isRowRelative,isColRelative,false,false));
        }else{
            // java r = new Ref10Ptg(isRowRelative ? (rnptg.getRow() + row) % maxRows : rnptg.getRow(), isColRelative ? (rnptg.getColumn() + col) % maxCols : rnptg.getColumn(), isRowRelative, isColRelative);
            r=Some(Ptg::newRef2D(if isRowRelative{rnptg.getRow().wrapping_add(row)%maxRows}else{rnptg.getRow()},if isColRelative{rnptg.getColumn().wrapping_add(col)%maxCols}else{rnptg.getColumn()},isRowRelative,isColRelative,true,false));
        }
        // java r.setClass(rnptg.getPtgClass()); return r;
        let mut r=r.unwrap();r.setClass(rnptg.getPtgClass()as u8);ptgRef(r)
    }
}
