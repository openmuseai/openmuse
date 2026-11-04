// java public abstract class Area3DPtg extends AreaPtgBase implements IWorkbookDependentPtg {
use super::{Ptg::{Ptg,PtgKind},ExternSheetNameResolver::ExternSheetNameResolver};
use crate::{cn::wps::et::ss::{interfaces::IEvalWorkbook::IEvalWorkbook,formula::renderer::RendererOption::RendererOption},org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::CellRangeAddressBase::CellRangeAddressBase}};
impl Ptg{
    // java public Area3DPtg(int firstRow, int lastRow, int firstColumn, int lastColumn, boolean firstRowRelative, boolean lastRowRelative, boolean firstColRelative, boolean lastColRelative, int externalSheetIndex) {
    // Native wide metadata selects Area3D03/Area3D10, retaining the base constructor.
    pub(crate) fn newArea3D(firstRow:i32,lastRow:i32,firstColumn:i32,lastColumn:i32,firstRowRelative:bool,lastRowRelative:bool,firstColRelative:bool,lastColRelative:bool,externalSheetIndex:i32,wide:bool)->Self{
        let mut ret=Self::base(PtgKind::Area3D{extern_index:0,first_row:0,last_row:0,first_col_raw:0,last_col_raw:0,wide});
        // java super(firstRow, lastRow, firstColumn, lastColumn, firstRowRelative, lastRowRelative, firstColRelative, lastColRelative);
        ret.initializeAreaCoordinates(firstRow,lastRow,firstColumn,lastColumn,firstRowRelative,lastRowRelative,firstColRelative,lastColRelative);
        // java this.setExternSheetIndex(externalSheetIndex);
        if let PtgKind::Area3D{extern_index,..}=&mut ret.kind{*extern_index=externalSheetIndex;}ret
    }
    // java public String toString() {
    pub fn toArea3DDebugString(&self)->String{
        // java StringBuffer sb = new StringBuffer(); sb.append(this.getClass().getName()); sb.append(" [");
        let mut sb=String::new();sb.push_str(&self.javaClassName());sb.push_str(" [");
        // java sb.append("sheetIx=").append(this.getExternSheetIndex());
        let PtgKind::Area3D{extern_index,..}=self.kind else{panic!("ClassCastException: Area3DPtg")};sb.push_str(&format!("sheetIx={extern_index}"));
        // java sb.append(" ! "); sb.append(this.formatReferenceAsString());
        sb.push_str(" ! ");sb.push_str(&self.formatAreaReferenceAsString());
        // java sb.append("]"); return sb.toString();
        sb.push(']');sb
    }
    // java public String toFormulaString(IEvalWorkbook book,RendererOption opt){return ExternSheetNameResolver.prependSheetName(book,this.field_1_index_extern_sheet,this.formatReferenceAsString(),opt);}
    pub fn toArea3DFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>)->String{let PtgKind::Area3D{extern_index,..}=self.kind else{panic!("ClassCastException: Area3DPtg")};let text=self.formatAreaReferenceAsString();ExternSheetNameResolver::prependSheetName(book,extern_index,Some(&text),opt)}
    // java public String toRCFormulaString(IEvalWorkbook book,RendererOption opt,CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress,SpreadsheetVersion version,boolean isName){return ExternSheetNameResolver.prependSheetName(book,this.field_1_index_extern_sheet,this.formatRCReferenceAsString(formulaAddress,originAddress,version,isName),opt);}
    pub fn toArea3DRCFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>,formulaAddress:Option<&CellRangeAddressBase>,originAddress:Option<&CellRangeAddressBase>,version:SpreadsheetVersion,isName:bool)->String{let PtgKind::Area3D{extern_index,..}=self.kind else{panic!("ClassCastException: Area3DPtg")};let text=self.formatAreaRCReferenceAsString(formulaAddress,originAddress,version,isName);ExternSheetNameResolver::prependSheetName(book,extern_index,Some(&text),opt)}
    // java public String toFormulaString(){throw new RuntimeException("3D references need a workbook to determine formula text");}
    pub fn toArea3DFormulaString__plain(&self)->String{panic!("RuntimeException: 3D references need a workbook to determine formula text")}
}
// java public abstract class Area3DPtg extends AreaPtgBase ...
pub enum Area3DPtg{}
impl Area3DPtg{
    // java public static Area3DPtg createFromNptg(Area3DPtg aptg, int row, int col, SpreadsheetVersion sv) {
    pub fn createFromNptg(aptg:Option<&super::Ptg::PtgRef>,row:i32,col:i32,sv:Option<SpreadsheetVersion>)->super::Ptg::PtgRef{
        // java int maxRows = sv.getMaxRows(); int maxCols = sv.getMaxColumns(); Area3DPtg aaptg = null;
        let sv=sv.expect("NullPointerException");let maxRows=sv.getMaxRows();let maxCols=sv.getMaxColumns();let mut aaptg=None;
        // java boolean isFirstRowRelative = aptg.isFirstRowRelative(); boolean isLastRowRelative = aptg.isLastRowRelative(); boolean isFirstColRelative = aptg.isFirstColRelative(); boolean isLastColRelative = aptg.isLastColRelative();
        let original=aptg.expect("NullPointerException");let aptg=original.borrow();let isFirstRowRelative=aptg.isFirstRowRelative();let isLastRowRelative=aptg.isLastRowRelative();let isFirstColRelative=aptg.isFirstColRelative();let isLastColRelative=aptg.isLastColRelative();
        // java if (isFirstRowRelative || isLastRowRelative || isFirstColRelative || isLastColRelative) {
        if isFirstRowRelative||isLastRowRelative||isFirstColRelative||isLastColRelative{
            let PtgKind::Area3D{extern_index,..}=aptg.kind else{panic!("ClassCastException: Area3DPtg")};
            // java if (sv == SpreadsheetVersion.EXCEL97) {
            if sv==SpreadsheetVersion::EXCEL97{
                // java aaptg = new Area3D03Ptg(isFirstRowRelative ? (aptg.getFirstRow() + row) % maxRows : aptg.getFirstRow(), isLastRowRelative ? (aptg.getLastRow() + row) % maxRows : aptg.getLastRow(), isFirstColRelative ? (aptg.getFirstColumn() + col) % maxCols : aptg.getFirstColumn(), isLastColRelative ? (aptg.getLastColumn() + col) % maxCols : aptg.getLastColumn(), isFirstRowRelative, isLastRowRelative, isFirstColRelative, isLastColRelative, aptg.getExternSheetIndex());
                aaptg=Some(Ptg::newArea3D(if isFirstRowRelative{aptg.getFirstRow().wrapping_add(row)%maxRows}else{aptg.getFirstRow()},if isLastRowRelative{aptg.getLastRow().wrapping_add(row)%maxRows}else{aptg.getLastRow()},if isFirstColRelative{aptg.getFirstColumn().wrapping_add(col)%maxCols}else{aptg.getFirstColumn()},if isLastColRelative{aptg.getLastColumn().wrapping_add(col)%maxCols}else{aptg.getLastColumn()},isFirstRowRelative,isLastRowRelative,isFirstColRelative,isLastColRelative,extern_index,false));
            }else{
                // java aaptg = new Area3D10Ptg(isFirstRowRelative ? (aptg.getFirstRow() + row) % maxRows : aptg.getFirstRow(), isLastRowRelative ? (aptg.getLastRow() + row) % maxRows : aptg.getLastRow(), isFirstColRelative ? (aptg.getFirstColumn() + col) % maxCols : aptg.getFirstColumn(), isLastColRelative ? (aptg.getLastColumn() + col) % maxCols : aptg.getLastColumn(), isFirstRowRelative, isLastRowRelative, isFirstColRelative, isLastColRelative, aptg.getExternSheetIndex());
                aaptg=Some(Ptg::newArea3D(if isFirstRowRelative{aptg.getFirstRow().wrapping_add(row)%maxRows}else{aptg.getFirstRow()},if isLastRowRelative{aptg.getLastRow().wrapping_add(row)%maxRows}else{aptg.getLastRow()},if isFirstColRelative{aptg.getFirstColumn().wrapping_add(col)%maxCols}else{aptg.getFirstColumn()},if isLastColRelative{aptg.getLastColumn().wrapping_add(col)%maxCols}else{aptg.getLastColumn()},isFirstRowRelative,isLastRowRelative,isFirstColRelative,isLastColRelative,extern_index,true));
            }
            // java aaptg.setClass(aptg.getPtgClass());
            aaptg.as_mut().unwrap().setClass(aptg.getPtgClass()as u8);
        }
        // java return aaptg == null ? aptg : aaptg;
        aaptg.map_or_else(||original.clone(),super::Ptg::ptgRef)
    }
}
