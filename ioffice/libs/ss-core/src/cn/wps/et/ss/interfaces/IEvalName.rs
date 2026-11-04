// java package cn.wps.et.ss.interfaces;

// java import cn.wps.et.ss.formula.ptg.Ptg;
use crate::cn::wps::et::ss::formula::ptg::Ptg::{Ptg,PtgArray};
// java import cn.wps.et.ss.interfaces.IEvalNameBase;
use super::IEvalNameBase::IEvalNameBase;

// java public interface IEvalName extends IEvalNameBase
pub trait IEvalName: IEvalNameBase {
    // java boolean isFunctionName();
    fn isFunctionName(&self) -> bool;
    // java boolean hasFormula();
    fn hasFormula(&self) -> bool;
    // java Ptg[] getNameDefinition();
    fn getNameDefinition(&self) -> PtgArray;
    // java NamePtg createNamePtg();
    fn createNamePtg(&self) -> Ptg;
    // java NameXPtg createNameXPtg(int externSheetRef);
    fn createNameXPtg(&self, extern_sheet_ref: i32) -> Ptg;
}
