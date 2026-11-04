// java package cn.wps.et.ss.calcchain.marktype;
use super::super::CalcChain::CalcChain;
use std::{any::Any, sync::Arc};
// java public abstract class MarkType
pub trait MarkType: Any + Send + Sync {
    fn reMark(&self, chain: &CalcChain);
    fn equals(&self, other: Option<&dyn MarkType>) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_area(&self) -> Option<&super::MarkTypeArea::MarkTypeArea> {
        None
    }
}
// java public static MarkType getInstance(int type)
pub fn getInstance(kind: i32) -> Option<Arc<dyn MarkType>> {
    Some(match kind {
        1 => Arc::new(super::MarkTypeName::MarkTypeName::new()),
        2 => Arc::new(super::MarkTypeSingle::MarkTypeSingle::new()),
        3 => Arc::new(super::MarkTypeArea::MarkTypeArea::new()),
        4 => Arc::new(super::MarkTypeAreaParam::MarkTypeAreaParam::new()),
        5 => Arc::new(super::MarkTypeAreaSub::MarkTypeAreaSub::new()),
        6 => Arc::new(super::MarkTypeAll::MarkTypeAll),
        _ => return None,
    })
}
