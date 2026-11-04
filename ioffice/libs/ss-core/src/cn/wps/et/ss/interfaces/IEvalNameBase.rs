// java package cn.wps.et.ss.interfaces;

use std::any::Any;

// java public interface IEvalNameBase
pub trait IEvalNameBase: Any {
    // java String getNameText();
    fn getNameText(&self) -> String;
    // java int getSheetIndexOneBased();
    fn getSheetIndexOneBased(&self) -> i32;
    // rust: Java's `instanceof IEvalName` needs concrete-type inspection.
    fn as_any(&self) -> &dyn Any;
    // rust: Java interface-cast metadata, distinct from IEvalNameBase methods.
    fn as_ext_name(&self)->Option<&dyn super::IEvalExtName::IEvalExtName>{None}
}
