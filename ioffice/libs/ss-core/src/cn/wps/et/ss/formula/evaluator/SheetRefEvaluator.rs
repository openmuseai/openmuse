// java package cn.wps.et.ss.formula.evaluator;
use super::super::eval::ValueRef;
use super::{EvalResult, WorkbookEvaluator::WorkbookEvaluator};
use crate::cn::wps::et::ss::interfaces::{
    IEvalWorkbook::IEvalWorkbook, IEvalWorksheet::IEvalWorksheet,
};
use crate::java_runtime::RefCell;
use std::sync::Arc;
// java public final class SheetRefEvaluator
pub struct SheetRefEvaluator {
    _workbook: Arc<dyn IEvalWorkbook>,
    _sheetIndex: i32,
    _sheet: RefCell<Option<Arc<dyn IEvalWorksheet>>>,
}
impl SheetRefEvaluator {
    pub fn SheetRefEvaluator(book: Arc<dyn IEvalWorkbook>, sheet: i32) -> Self {
        assert!(sheet >= 0, "Invalid sheetIndex: {sheet}.");
        Self {
            _workbook: book,
            _sheetIndex: sheet,
            _sheet: RefCell::new(None),
        }
    }
    // java lazy getSheet(): _sheet=workbook.getSheet(sheetIndex);
    fn getSheet(&self) -> Arc<dyn IEvalWorksheet> {
        let mut sheet = self._sheet.borrow_mut();
        sheet
            .get_or_insert_with(|| self._workbook.getSheet(self._sheetIndex))
            .clone()
    }
    // java cell=getSheet().getCell(rowIndex,columnIndex); return WorkbookEvaluator.getCellCachedValue(cell,_workbook);
    pub fn getEvalForCell(&self, row: i32, col: i32) -> EvalResult<Option<ValueRef>> {
        WorkbookEvaluator::getCellCachedValue(
            Some(self.getSheet().getCell(row, col)),
            Some(&self._workbook),
        )
    }
    pub fn getSheetIndex(&self) -> i32 {
        self._sheetIndex
    }
// java public IEvalWorkbook getBook(){return this._workbook;}
pub fn getBook(&self)->Arc<dyn IEvalWorkbook>{self._workbook.clone()}
// java return new ValueIterator(this._sheetIndex,this.getSheet().getCellIterator(firstRow,lastRow,firstCol,lastCol));
pub fn getValueIterator(&self,fr:i32,lr:i32,fc:i32,lc:i32)->Box<dyn super::IValueIterator::IValueIterator>{Box::new(ValueIterator{sheetIndex:self._sheetIndex,workbook:self._workbook.clone(),iter:self.getSheet().getCellIterator(fr,lr,fc,lc).peekable(),currentCell:None})}
}

struct ValueIterator{sheetIndex:i32,workbook:Arc<dyn IEvalWorkbook>,iter:std::iter::Peekable<Box<dyn Iterator<Item=Arc<dyn crate::cn::wps::et::ss::interfaces::IEvalCell::IEvalCell>>>>,currentCell:Option<Arc<dyn crate::cn::wps::et::ss::interfaces::IEvalCell::IEvalCell>>}
impl super::IValueIterator::IValueIterator for ValueIterator{fn hasNext(&mut self)->bool{self.iter.peek().is_some()}fn next(&mut self){self.currentCell=Some(self.iter.next().expect("NoSuchElementException"));}fn getSheetIndex(&self)->i32{self.sheetIndex}fn getRowIndex(&self)->i32{self.currentCell.as_ref().expect("null currentCell").getRowIndex()}fn getColIndex(&self)->i32{self.currentCell.as_ref().expect("null currentCell").getColIndex()}fn getValue(&self)->EvalResult<ValueRef>{WorkbookEvaluator::getCellCachedValue(self.currentCell.clone(),Some(&self.workbook))?.ok_or(super::EvalFailure::Runtime("null interrupted eval"))}}
