// java package cn.wps.et.ss.interfaces;

// java import cn.wps.et.ss.formula.ptg.Ptg;
use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;
// java import cn.wps.et.ss.interfaces.IEvalNameBase;
use super::IEvalNameBase::IEvalNameBase;

// java public interface IEvalExtName extends IEvalNameBase
pub trait IEvalExtName: IEvalNameBase {
    // java Ptg[] getNameDefinition();
    fn getNameDefinition(&self) -> Option<Vec<Ptg>>;
}
