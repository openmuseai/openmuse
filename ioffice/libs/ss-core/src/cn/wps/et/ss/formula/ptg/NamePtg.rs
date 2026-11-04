// java public final class NamePtg extends OperandPtg implements IWorkbookDependentPtg {
// rust: the existing Ptg carrier retains original name-index data and token identity.
use super::Ptg::{Ptg,PtgKind};
use crate::cn::wps::et::ss::{interfaces::IEvalWorkbook::IEvalWorkbook,formula::renderer::RendererOption::RendererOption};
use crate::org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::CellRangeAddressBase::CellRangeAddressBase};
impl Ptg{
    // java public String toFormulaString(IEvalWorkbook book,RendererOption opt){
    pub fn toNameFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>)->String{
        // java IEvalName name=book.getNameByNamePtg(this.getIndex());String ret=name.getNameText();
        let PtgKind::Name{index}=self.kind else{panic!("ClassCastException: NamePtg")};let name=book.expect("NullPointerException").getNameByNamePtg(index);let name=name.as_ref().expect("NullPointerException");let ret=name.getNameText();
        // java return name.isFunctionName()&&opt.formulaType!=6&&opt.formulaType!=7&&ret.length()>6&&ret.indexOf(".")>0&&ret.startsWith("_")?ret.substring(6):ret;
        if name.isFunctionName(){let opt=opt.expect("NullPointerException");if opt.formulaType!=6&&opt.formulaType!=7&&ret.encode_utf16().count()>6&&ret.find('.').is_some_and(|p|p>0)&&ret.starts_with('_'){let units:Vec<_>=ret.encode_utf16().collect();return String::from_utf16(&units[6..]).expect("native UTF16 String output boundary: isolated surrogate");}}ret
    }
    // java public String toRCFormulaString(IEvalWorkbook book,RendererOption opt,CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress,SpreadsheetVersion version,boolean isName){return this.toFormulaString(book,opt);}
    pub fn toNameRCFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>,_formulaAddress:Option<&CellRangeAddressBase>,_originAddress:Option<&CellRangeAddressBase>,_version:SpreadsheetVersion,_isName:bool)->String{self.toNameFormulaString(book,opt)}
    // java public String toFormulaString(){throw new RuntimeException("3D references need a workbook to determine formula text");}
    pub fn toNameFormulaString__plain(&self)->String{panic!("RuntimeException: 3D references need a workbook to determine formula text")}
}
