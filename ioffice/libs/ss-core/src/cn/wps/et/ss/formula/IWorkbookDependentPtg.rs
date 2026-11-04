// java package cn.wps.et.ss.formula;
use crate::{cn::wps::et::ss::{interfaces::IEvalWorkbook::IEvalWorkbook,formula::{renderer::RendererOption::RendererOption,ptg::Ptg::{Ptg,PtgKind}}},org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::CellRangeAddressBase::CellRangeAddressBase}};
// java public interface IWorkbookDependentPtg {
pub trait IWorkbookDependentPtg{
    // java String toFormulaString(IEvalWorkbook var1,RendererOption var2);
    fn toFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>)->String;
    // java String toRCFormulaString(IEvalWorkbook var1,RendererOption var2,CellRangeAddressBase var3,CellRangeAddressBase var4,SpreadsheetVersion var5,boolean var6);
    fn toRCFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>,formulaAddress:Option<&CellRangeAddressBase>,originAddress:Option<&CellRangeAddressBase>,version:SpreadsheetVersion,isName:bool)->String;
}
// rust: runtime interface dispatch for the retained token carrier. The Java
// instanceof guard below excludes external-name 3D classes despite equal sid.
impl IWorkbookDependentPtg for Ptg{
    fn toFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>)->String{match self.kind{PtgKind::Name{..}=>self.toNameFormulaString(book,opt),PtgKind::NameX{..}=>self.toNameXFormulaString(book,opt),PtgKind::Ref3D{..}=>self.toRef3DFormulaString(book,opt),PtgKind::Area3D{..}=>self.toArea3DFormulaString(book,opt),PtgKind::DeletedRef3D{..}=>self.toDeletedRef3DFormulaString(book,opt),PtgKind::DeletedArea3D{..}=>self.toDeletedArea3DFormulaString(book,opt),_=>panic!("ClassCastException: IWorkbookDependentPtg")}}
    fn toRCFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>,formulaAddress:Option<&CellRangeAddressBase>,originAddress:Option<&CellRangeAddressBase>,version:SpreadsheetVersion,isName:bool)->String{match self.kind{PtgKind::Ref3D{..}=>self.toRef3DRCFormulaString(book,opt,formulaAddress,originAddress,version,isName),PtgKind::Area3D{..}=>self.toArea3DRCFormulaString(book,opt,formulaAddress,originAddress,version,isName),PtgKind::Name{..}=>self.toNameRCFormulaString(book,opt,formulaAddress,originAddress,version,isName),PtgKind::NameX{..}=>self.toNameXRCFormulaString(book,opt,formulaAddress,originAddress,version,isName),PtgKind::DeletedRef3D{..}=>self.toDeletedRef3DRCFormulaString(book,opt,formulaAddress,originAddress,version,isName),PtgKind::DeletedArea3D{..}=>self.toDeletedArea3DRCFormulaString(book,opt,formulaAddress,originAddress,version,isName),_=>panic!("ClassCastException: IWorkbookDependentPtg")}}
}
impl Ptg{
    pub(crate) fn isWorkbookDependentPtg(&self)->bool{matches!(self.kind,PtgKind::Name{..}|PtgKind::NameX{..}|PtgKind::Ref3D{..}|PtgKind::Area3D{..}|PtgKind::DeletedRef3D{..}|PtgKind::DeletedArea3D{..})}
    pub(crate) fn isOperationPtg(&self)->bool{matches!(self.kind,PtgKind::Add|PtgKind::Subtract|PtgKind::Multiply|PtgKind::Divide|PtgKind::Power|PtgKind::Concat|PtgKind::Equal|PtgKind::LessThan|PtgKind::GreaterThan|PtgKind::LessEqual|PtgKind::GreaterEqual|PtgKind::NotEqual|PtgKind::UnaryMinus|PtgKind::UnaryPlus|PtgKind::Percent|PtgKind::Union|PtgKind::Intersection|PtgKind::Range|PtgKind::Func{..}|PtgKind::FuncVar{..}|PtgKind::FuncVarExt{..})}
}
