// java public final class DeletedArea3DPtg extends OperandPtg implements IWorkbookDependentPtg {
use super::{Ptg::{Ptg,PtgKind},ExternSheetNameResolver::ExternSheetNameResolver};
use crate::{cn::wps::et::ss::{interfaces::IEvalWorkbook::IEvalWorkbook,formula::renderer::RendererOption::RendererOption},org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants};
use crate::org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::CellRangeAddressBase::CellRangeAddressBase};
impl Ptg{
    // java public String toFormulaString(IEvalWorkbook book,RendererOption opt){return opt.formulaType!=6&&opt.formulaType!=7?ExternSheetNameResolver.prependSheetName(book,this.field_1_index_extern_sheet,ErrorConstants.getText(23),opt):ErrorConstants.getText(23);}
    pub fn toDeletedArea3DFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>)->String{let PtgKind::DeletedArea3D{extern_index,..}=self.kind else{panic!("ClassCastException: DeletedArea3DPtg")};let opt=opt.expect("NullPointerException");if opt.formulaType!=6&&opt.formulaType!=7{let text=ErrorConstants::getText(23);ExternSheetNameResolver::prependSheetName(book,extern_index as i32,Some(&text),Some(opt))}else{ErrorConstants::getText(23)}}
    // java public String toRCFormulaString(IEvalWorkbook book,RendererOption opt,CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress,SpreadsheetVersion version,boolean isName){return this.toFormulaString(book,opt);}
    pub fn toDeletedArea3DRCFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>,_formulaAddress:Option<&CellRangeAddressBase>,_originAddress:Option<&CellRangeAddressBase>,_version:SpreadsheetVersion,_isName:bool)->String{self.toDeletedArea3DFormulaString(book,opt)}
    // java public String toFormulaString(){throw new RuntimeException("3D references need a workbook to determine formula text");}
    pub fn toDeletedArea3DFormulaString__plain(&self)->String{panic!("RuntimeException: 3D references need a workbook to determine formula text")}
}
