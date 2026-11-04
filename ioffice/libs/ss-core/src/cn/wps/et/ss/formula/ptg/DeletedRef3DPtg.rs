// java public final class DeletedRef3DPtg extends OperandPtg implements IWorkbookDependentPtg {
use super::{Ptg::{Ptg,PtgKind},ExternSheetNameResolver::ExternSheetNameResolver};
use crate::{cn::wps::et::ss::{interfaces::IEvalWorkbook::IEvalWorkbook,formula::renderer::RendererOption::RendererOption},org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants};
use crate::org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::CellRangeAddressBase::CellRangeAddressBase};
impl Ptg{
    // java public String toFormulaString(IEvalWorkbook book,RendererOption opt){return ExternSheetNameResolver.prependSheetName(book,this.field_1_index_extern_sheet,ErrorConstants.getText(23),opt);}
    pub fn toDeletedRef3DFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>)->String{let PtgKind::DeletedRef3D{extern_index,..}=self.kind else{panic!("ClassCastException: DeletedRef3DPtg")};let text=ErrorConstants::getText(23);ExternSheetNameResolver::prependSheetName(book,extern_index as i32,Some(&text),opt)}
    // java public String toRCFormulaString(IEvalWorkbook book,RendererOption opt,CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress,SpreadsheetVersion version,boolean isName){return this.toFormulaString(book,opt);}
    pub fn toDeletedRef3DRCFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>,_formulaAddress:Option<&CellRangeAddressBase>,_originAddress:Option<&CellRangeAddressBase>,_version:SpreadsheetVersion,_isName:bool)->String{self.toDeletedRef3DFormulaString(book,opt)}
    // java public String toFormulaString(){throw new RuntimeException("3D references need a workbook to determine formula text");}
    pub fn toDeletedRef3DFormulaString__plain(&self)->String{panic!("RuntimeException: 3D references need a workbook to determine formula text")}
}
