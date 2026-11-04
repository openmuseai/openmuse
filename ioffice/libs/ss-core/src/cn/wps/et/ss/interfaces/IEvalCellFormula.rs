// java package cn.wps.et.ss.interfaces;

use super::IEvalFormulaBase::IEvalFormulaBase;
use super::IEvalSharedFormula::IEvalSharedFormula;

// java public interface IEvalCellFormula extends IEvalFormulaBase
pub trait IEvalCellFormula: IEvalFormulaBase {
    // java void setStringResult(String var1);
    fn setStringResult(&self, value: &str);
    // java void setNumberResult(double var1);
    fn setNumberResult(&self, value: f64);
    // java void setErrorResult(byte var1);
    fn setErrorResult(&self, value: i8);
    // java void setBoolResult(boolean var1);
    fn setBoolResult(&self, value: bool);
    // java boolean isSharedFormula();
    fn isSharedFormula(&self) -> bool;
    // java boolean isMasterSharedFormula();
    fn isMasterSharedFormula(&self) -> bool;
    // java IEvalSharedFormula getSharedFormula();
    fn getSharedFormula(&self) -> Option<Box<dyn IEvalSharedFormula>>;
}
