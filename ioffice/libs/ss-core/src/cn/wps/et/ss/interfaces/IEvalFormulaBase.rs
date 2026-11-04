// java package cn.wps.et.ss.interfaces;

use crate::java_runtime::Rc;
use std::any::Any;

// java import cn.wps.et.ss.formula.ptg.Ptg;
use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;

// java public interface IEvalFormulaBase {
pub trait IEvalFormulaBase: Any + Send + Sync {
    // java int getSheetUid();
    fn getSheetUid(&self) -> i32;
    // java int getSheetIndex();
    fn getSheetIndex(&self) -> i32;
    // java int getFmlRow();
    fn getFmlRow(&self) -> i32;
    // java int getFmlCol();
    fn getFmlCol(&self) -> i32;
    // java Ptg[] getTokens();
    fn getTokens(&self) -> Vec<Ptg>;
    // java byte[] getEncodedTokens();
    fn getEncodedTokens(&self) -> Option<&[u8]>;
    // java int getEncodedTokensLen();
    fn getEncodedTokensLen(&self) -> i32;
    // java void setNeedRecalculationFlag(boolean var1);
    fn setNeedRecalculationFlag(&self, value: bool);
    // java boolean isNeedRecalculationFlagSet();
    fn isNeedRecalculationFlagSet(&self) -> bool;
    // java void setZeroResultIfBlank();
    fn setZeroResultIfBlank(&self);
    // java boolean equals(Object var1);
    fn equals(&self, other: &dyn IEvalFormulaBase) -> bool;
    // java int hashCode();
    fn hashCode(&self) -> i32;
    // java void lock();
    fn lock(&self);
    // java boolean islock();
    fn islock(&self) -> bool;
    // java void unlock();
    fn unlock(&self);
    // java boolean isCycled();
    fn isCycled(&self) -> bool;
    // java void setCycled(boolean var1);
    fn setCycled(&self, value: bool);
    // java void setRef(IEvalFormulaBase var1);
    fn setRef(&self, value: Option<Rc<dyn IEvalFormulaBase>>);
    // java IEvalFormulaBase getRef();
    fn getRef(&self) -> Option<Rc<dyn IEvalFormulaBase>>;

    // rust: Java `instanceof IEvalCellFormula` on an IEvalFormulaBase reference.
    fn as_cell_formula(&self) -> Option<&dyn super::IEvalCellFormula::IEvalCellFormula> {
        None
    }

    fn as_array_formula(&self) -> Option<&dyn super::IEvalArrayFormula::IEvalArrayFormula> {
        None
    }

    // Rust dynamic downcast for Java's instanceof in equals(Object).
    fn as_any(&self) -> &dyn Any;
}
