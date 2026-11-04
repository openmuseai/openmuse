use super::Ptg::{Ptg,PtgRef,ptgRef};
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
// java public abstract class AreaPtg extends Area2DPtgBase {
// Static class facade; instances use the original coordinate fields in Ptg.
pub enum AreaPtg{}
impl AreaPtg{
    // java public static AreaPtg createFromNptg(AreaNPtg anptg, int row, int col, SpreadsheetVersion sv) {
    pub fn createFromNptg(anptg:Option<&PtgRef>,row:i32,col:i32,sv:Option<SpreadsheetVersion>)->PtgRef{
        // java int maxRows = sv.getMaxRows(); int maxCols = sv.getMaxColumns(); AreaPtg r = null;
        let sv=sv.expect("NullPointerException");let maxRows=sv.getMaxRows();let maxCols=sv.getMaxColumns();let mut r=None;
        // java boolean isFirstRowRelative = anptg.isFirstRowRelative(); boolean isLastRowRelative = anptg.isLastRowRelative(); boolean isFirstColRelative = anptg.isFirstColRelative(); boolean isLastColRelative = anptg.isLastColRelative();
        let anptg=anptg.expect("NullPointerException").borrow();let isFirstRowRelative=anptg.isFirstRowRelative();let isLastRowRelative=anptg.isLastRowRelative();let isFirstColRelative=anptg.isFirstColRelative();let isLastColRelative=anptg.isLastColRelative();
        // java if (sv == SpreadsheetVersion.EXCEL97) {
        if sv==SpreadsheetVersion::EXCEL97{
            // java r = new Area03Ptg(isFirstRowRelative ? (anptg.getFirstRow() + row) % maxRows : anptg.getFirstRow(), isLastRowRelative ? (anptg.getLastRow() + row) % maxRows : anptg.getLastRow(), isFirstColRelative ? (anptg.getFirstColumn() + col) % maxCols : anptg.getFirstColumn(), isLastColRelative ? (anptg.getLastColumn() + col) % maxCols : anptg.getLastColumn(), isFirstRowRelative, isLastRowRelative, isFirstColRelative, isLastColRelative);
            r=Some(Ptg::newArea2D(if isFirstRowRelative{anptg.getFirstRow().wrapping_add(row)%maxRows}else{anptg.getFirstRow()},if isLastRowRelative{anptg.getLastRow().wrapping_add(row)%maxRows}else{anptg.getLastRow()},if isFirstColRelative{anptg.getFirstColumn().wrapping_add(col)%maxCols}else{anptg.getFirstColumn()},if isLastColRelative{anptg.getLastColumn().wrapping_add(col)%maxCols}else{anptg.getLastColumn()},isFirstRowRelative,isLastRowRelative,isFirstColRelative,isLastColRelative,false));
        }else{
            // java r = new Area10Ptg(isFirstRowRelative ? (anptg.getFirstRow() + row) % maxRows : anptg.getFirstRow(), isLastRowRelative ? (anptg.getLastRow() + row) % maxRows : anptg.getLastRow(), isFirstColRelative ? (anptg.getFirstColumn() + col) % maxCols : anptg.getFirstColumn(), isLastColRelative ? (anptg.getLastColumn() + col) % maxCols : anptg.getLastColumn(), isFirstRowRelative, isLastRowRelative, isFirstColRelative, isLastColRelative);
            r=Some(Ptg::newArea2D(if isFirstRowRelative{anptg.getFirstRow().wrapping_add(row)%maxRows}else{anptg.getFirstRow()},if isLastRowRelative{anptg.getLastRow().wrapping_add(row)%maxRows}else{anptg.getLastRow()},if isFirstColRelative{anptg.getFirstColumn().wrapping_add(col)%maxCols}else{anptg.getFirstColumn()},if isLastColRelative{anptg.getLastColumn().wrapping_add(col)%maxCols}else{anptg.getLastColumn()},isFirstRowRelative,isLastRowRelative,isFirstColRelative,isLastColRelative,true));
        }
        // java r.setClass(anptg.getPtgClass()); return r;
        let mut r=r.unwrap();r.setClass(anptg.getPtgClass()as u8);ptgRef(r)
    }
}
