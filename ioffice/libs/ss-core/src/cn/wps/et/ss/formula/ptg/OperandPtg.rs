// java package cn.wps.et.ss.formula.ptg;
use super::Ptg::Ptg;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
impl Ptg{
    // java public static int getRelativeRow(int row,int refRow,SpreadsheetVersion spreadsheetVersion){return (row+refRow)%spreadsheetVersion.getMaxRows();}
    pub fn getRelativeRow(row:i32,refRow:i32,spreadsheetVersion:SpreadsheetVersion)->i32{row.wrapping_add(refRow)%spreadsheetVersion.getMaxRows()}
    // java public static int getRelativeCol(int col,int refCol,SpreadsheetVersion spreadsheetVersion){return (col+refCol)%spreadsheetVersion.getMaxColumns();}
    pub fn getRelativeCol(col:i32,refCol:i32,spreadsheetVersion:SpreadsheetVersion)->i32{col.wrapping_add(refCol)%spreadsheetVersion.getMaxColumns()}
}
