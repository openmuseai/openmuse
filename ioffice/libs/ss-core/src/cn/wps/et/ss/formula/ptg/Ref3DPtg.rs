// java public abstract class Ref3DPtg extends RefPtgBase implements IWorkbookDependentPtg {
use super::{Ptg::{Ptg,PtgKind},ExternSheetNameResolver::ExternSheetNameResolver};
use crate::{cn::wps::et::ss::{interfaces::IEvalWorkbook::IEvalWorkbook,formula::renderer::RendererOption::RendererOption},org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::CellRangeAddressBase::CellRangeAddressBase}};
impl Ptg{
    // java public Ref3DPtg(int externSheet, int row, int col, boolean rowRelative, boolean colRelative) {
    // Native wide metadata selects the original Ref3D03/Ref3D10 constructor.
    pub(crate) fn newRef3D(externSheet:i32,row:i32,col:i32,rowRelative:bool,colRelative:bool,wide:bool)->Self{
        // java this.field_1_index_extern_sheet = externSheet;
        let mut ret=Self::base(PtgKind::Ref3D{extern_index:externSheet,row:0,col_raw:0,wide});
        // java this.setRow(row); this.setColumn(col); this.setRowRelative(rowRelative); this.setColRelative(colRelative);
        ret.setRow(row);ret.setColumn(col);ret.setRowRelative(rowRelative);ret.setColRelative(colRelative);ret
    }
    // java public String toString() {
    pub fn toRef3DDebugString(&self)->String{
        // java StringBuffer sb = new StringBuffer(); sb.append(this.getClass().getName()); sb.append(" [");
        let mut sb=String::new();sb.push_str(&self.javaClassName());sb.push_str(" [");
        // java sb.append("sheetIx=").append(this.getExternSheetIndex());
        let PtgKind::Ref3D{extern_index,..}=self.kind else{panic!("ClassCastException: Ref3DPtg")};sb.push_str(&format!("sheetIx={extern_index}"));
        // java sb.append(" ! "); sb.append(this.formatReferenceAsString());
        sb.push_str(" ! ");sb.push_str(&self.formatReferenceAsString());
        // java sb.append("]"); return sb.toString();
        sb.push(']');sb
    }
    // java public String toFormulaString(IEvalWorkbook book,RendererOption opt){return ExternSheetNameResolver.prependSheetName(book,this.field_1_index_extern_sheet,this.formatReferenceAsString(),opt);}
    pub fn toRef3DFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>)->String{let PtgKind::Ref3D{extern_index,..}=self.kind else{panic!("ClassCastException: Ref3DPtg")};let text=self.formatReferenceAsString();ExternSheetNameResolver::prependSheetName(book,extern_index,Some(&text),opt)}
    // java public String toRCFormulaString(IEvalWorkbook book,RendererOption opt,CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress,SpreadsheetVersion version,boolean isName){return ExternSheetNameResolver.prependSheetName(book,this.field_1_index_extern_sheet,this.formatRCReferenceAsString(formulaAddress,originAddress,version,isName),opt);}
    pub fn toRef3DRCFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>,formulaAddress:Option<&CellRangeAddressBase>,originAddress:Option<&CellRangeAddressBase>,version:SpreadsheetVersion,isName:bool)->String{let PtgKind::Ref3D{extern_index,..}=self.kind else{panic!("ClassCastException: Ref3DPtg")};let text=self.formatRCReferenceAsString(formulaAddress,originAddress,version,isName);ExternSheetNameResolver::prependSheetName(book,extern_index,Some(&text),opt)}
    // java public String toFormulaString(){return this.formatReferenceAsString();}
    pub fn toRef3DFormulaString__plain(&self)->String{self.formatReferenceAsString()}
}
// java public abstract class Ref3DPtg extends RefPtgBase ...
pub enum Ref3DPtg{}
impl Ref3DPtg{
    // java public static Ref3DPtg createFromNptg(Ref3DPtg rptg, int row, int col, SpreadsheetVersion sv) {
    pub fn createFromNptg(rptg:Option<&super::Ptg::PtgRef>,row:i32,col:i32,sv:Option<SpreadsheetVersion>)->super::Ptg::PtgRef{
        // java int maxRows = sv.getMaxRows(); int maxCols = sv.getMaxColumns(); Ref3DPtg nRptg = null;
        let sv=sv.expect("NullPointerException");let maxRows=sv.getMaxRows();let maxCols=sv.getMaxColumns();let mut nRptg=None;
        // java boolean isRowRelative = rptg.isRowRelative(); boolean isColRelative = rptg.isColRelative();
        let original=rptg.expect("NullPointerException");let rptg=original.borrow();let isRowRelative=rptg.isRowRelative();let isColRelative=rptg.isColRelative();
        // java if (isRowRelative || isColRelative) {
        if isRowRelative||isColRelative{
            let PtgKind::Ref3D{extern_index,..}=rptg.kind else{panic!("ClassCastException: Ref3DPtg")};
            // java if (sv == SpreadsheetVersion.EXCEL97) {
            if sv==SpreadsheetVersion::EXCEL97{
                // java nRptg = new Ref3D03Ptg(rptg.getExternSheetIndex(), isRowRelative ? (rptg.getRow() + row) % maxRows : rptg.getRow(), isColRelative ? (rptg.getColumn() + col) % maxCols : rptg.getColumn(), isRowRelative, isColRelative);
                nRptg=Some(Ptg::newRef3D(extern_index,if isRowRelative{rptg.getRow().wrapping_add(row)%maxRows}else{rptg.getRow()},if isColRelative{rptg.getColumn().wrapping_add(col)%maxCols}else{rptg.getColumn()},isRowRelative,isColRelative,false));
            }else{
                // java nRptg = new Ref3D10Ptg(rptg.getExternSheetIndex(), isRowRelative ? (rptg.getRow() + row) % maxRows : rptg.getRow(), isColRelative ? (rptg.getColumn() + col) % maxCols : rptg.getColumn(), isRowRelative, isColRelative);
                nRptg=Some(Ptg::newRef3D(extern_index,if isRowRelative{rptg.getRow().wrapping_add(row)%maxRows}else{rptg.getRow()},if isColRelative{rptg.getColumn().wrapping_add(col)%maxCols}else{rptg.getColumn()},isRowRelative,isColRelative,true));
            }
            // java nRptg.setClass(rptg.getPtgClass());
            nRptg.as_mut().unwrap().setClass(rptg.getPtgClass()as u8);
        }
        // java return nRptg == null ? rptg : nRptg;
        nRptg.map_or_else(||original.clone(),super::Ptg::ptgRef)
    }
}
