// java package cn.wps.et.ss.formula.ptg;

// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
// java import org.apache.poi.util.LittleEndian;
use util::cn::wps::moffice::util::LittleEndian::LittleEndian;

// java public abstract class ExpPtg extends ControlPtg
pub struct ExpPtg;
impl super::Ptg::Ptg{
    // java public int getRow() { return this.field_1_first_row; }
    pub fn getExpRow(&self)->i32{let super::Ptg::PtgKind::Exp{row,..}=self.kind else{panic!("ClassCastException: ExpPtg")};row}
    // java public int getColumn() { return this.field_2_first_col; }
    pub fn getExpColumn(&self)->i32{let super::Ptg::PtgKind::Exp{col,..}=self.kind else{panic!("ClassCastException: ExpPtg")};col as i32}
    // java public String toString() {
    pub fn toExpDebugString(&self)->String{
        // java StringBuffer buffer = new StringBuffer("[Array Formula or Shared Formula]\n");
        let mut buffer=String::from("[Array Formula or Shared Formula]\n");
        // rust: Exp03/Exp10 getRow/getColumn dispatch through their same carrier fields.
        let super::Ptg::PtgKind::Exp{row,col,..}=self.kind else{panic!("ClassCastException: ExpPtg")};
        // java buffer.append("row = ").append(this.getRow()).append("\n");
        buffer.push_str(&format!("row = {row}\n"));
        // java buffer.append("col = ").append(this.getColumn()).append("\n"); return buffer.toString();
        buffer.push_str(&format!("col = {col}\n"));buffer
    }
}

impl ExpPtg {
    // java public static final short sid = 1;
    pub const sid: i16 = 1;

    // java public static int[] getRowCol(SpreadsheetVersion sv, byte[] tokens)
    pub fn getRowCol(sv: SpreadsheetVersion, tokens: &[u8]) -> [i32; 2] {
        // java int[] rowCol = new int[2];
        let mut rowCol = [0; 2];
        // java if (sv == SpreadsheetVersion.EXCEL97)
        if sv == SpreadsheetVersion::EXCEL97 {
            // java rowCol[0] = LittleEndian.getUShort(tokens, 1);
            rowCol[0] = LittleEndian::getUShort(tokens, 1);
            // java rowCol[1] = LittleEndian.getUShort(tokens, 3);
            rowCol[1] = LittleEndian::getUShort(tokens, 3);
        } else {
            // java rowCol[0] = LittleEndian.getInt(tokens, 1);
            rowCol[0] = LittleEndian::getInt(tokens, 1);
            // java rowCol[1] = LittleEndian.getUShort(tokens, 5);
            rowCol[1] = LittleEndian::getUShort(tokens, 5);
        }
        // java return rowCol;
        rowCol
    }

    // java public static int getSize(SpreadsheetVersion sv)
    pub fn getSize(sv: SpreadsheetVersion) -> i32 {
        // java return sv == SpreadsheetVersion.EXCEL97 ? 5 : 7;
        if sv == SpreadsheetVersion::EXCEL97 { 5 } else { 7 }
    }
    // java public String toFormulaString(){throw new RuntimeException("Coding Error: Expected ExpPtg to be converted from Shared to Non-Shared Formula by ValueRecordsAggregate, but it wasn't");}
    pub fn toFormulaString()->String{panic!("RuntimeException: Coding Error: Expected ExpPtg to be converted from Shared to Non-Shared Formula by ValueRecordsAggregate, but it wasn't")}

}
