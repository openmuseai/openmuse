// java package cn.wps.et.ss.interfaces;

// java import cn.wps.et.ss.formula.ptg.Ptg;
use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;
use std::any::Any;

// java public interface IEvalSharedFormula
pub trait IEvalSharedFormula: Any + Send + Sync {
    // java int getSheetUid();
    fn getSheetUid(&self) -> i32;
    // java int getSheetIndex();
    fn getSheetIndex(&self) -> i32;
    // java int getFmlRow();
    fn getFmlRow(&self) -> i32;
    // java int getFmlCol();
    fn getFmlCol(&self) -> i32;
    // java int getFmlLastRow();
    fn getFmlLastRow(&self) -> i32;
    // java int getFmlLastCol();
    fn getFmlLastCol(&self) -> i32;
    // java Ptg[] getTokens();
    fn getTokens(&self) -> Vec<Ptg>;
    // java boolean equals(Object var1);
    fn equals(&self, other: &dyn IEvalSharedFormula) -> bool;
    // java int hashCode();
    fn hashCode(&self) -> i32;
    // Rust dynamic downcast for Java equals(Object).
    fn as_any(&self) -> &dyn Any;
}
