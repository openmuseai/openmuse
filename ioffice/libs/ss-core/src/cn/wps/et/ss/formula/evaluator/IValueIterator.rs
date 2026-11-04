use super::EvalResult;use super::super::eval::ValueRef;
// java public interface IValueIterator
pub trait IValueIterator{fn hasNext(&mut self)->bool;fn next(&mut self);fn getSheetIndex(&self)->i32;fn getRowIndex(&self)->i32;fn getColIndex(&self)->i32;fn getValue(&self)->EvalResult<ValueRef>;}
