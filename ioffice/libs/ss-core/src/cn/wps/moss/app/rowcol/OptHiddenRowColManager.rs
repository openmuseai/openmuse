use crate::{java_runtime::{Rc,Weak,RefCell,Cell},cn::wps::moss::{app::{KmoSheet::KmoSheet,TransactionManager::TransactionManager,KmoTransaction::KmoSheetTransactionListener},core::sheet::KmoRowRange::KmoRowRangeIterator}};
use super::OptHiddenRowCol::OptHiddenRowCol;
// java public class OptHiddenRowColManager implements KmoTransaction.KmoSheetTransactionListener {
pub struct OptHiddenRowColManager{
    // java private final KmoSheet kmoSheet;
    kmoSheet:Weak<RefCell<KmoSheet>>,
    // java private OptHiddenRowCol optHiddenRows;
    optHiddenRows:RefCell<Rc<OptHiddenRowCol>>,
    // java private OptHiddenRowCol optHiddenCols;
    optHiddenCols:RefCell<Rc<OptHiddenRowCol>>,
    // java private final KmoTransaction transaction;
    transaction:Weak<TransactionManager>,
    // java private boolean hasRowWrite = false;
    hasRowWrite:Cell<bool>,
    // java private boolean hasColWrite = false;
    hasColWrite:Cell<bool>,
    // java private boolean isInit = false;
    isInit:Cell<bool>,
}
impl OptHiddenRowColManager{
    // java public OptHiddenRowColManager(KmoSheet sheet) {
    pub fn new(sheet:&Rc<RefCell<KmoSheet>>)->Rc<Self>{
        // java this.kmoSheet = sheet; this.optHiddenRows = new OptHiddenRowCol(); this.optHiddenCols = new OptHiddenRowCol();
        let rows=Rc::new(OptHiddenRowCol::new());let cols=Rc::new(OptHiddenRowCol::new());
        // java this.transaction = sheet.getBook().transaction();
        let transaction=sheet.borrow().getBook().transactionReference();let ret=Rc::new(Self{kmoSheet:Rc::downgrade(sheet),optHiddenRows:RefCell::new(rows),optHiddenCols:RefCell::new(cols),transaction:Rc::downgrade(&transaction),hasRowWrite:Cell::new(false),hasColWrite:Cell::new(false),isInit:Cell::new(false)});
        // java this.transaction.addTransactionListener(sheet.getSheetUid(), this);
        transaction.addTransactionListener(sheet.borrow().getSheetUid(),Some(ret.clone()));ret
    }
    // java public OptHiddenRowColManager copy(OptHiddenRowColManager manager) {
    pub fn copy(&self,manager:Option<&Self>)->&Self{
        // java this.optHiddenRows = manager.optHiddenRows.copy();
        let rows=manager.expect("NullPointerException").optHiddenRows.borrow().copy();*self.optHiddenRows.borrow_mut()=Rc::new(rows);
        // java this.optHiddenCols = manager.optHiddenCols.copy(); return this;
        let cols=manager.unwrap().optHiddenCols.borrow().copy();*self.optHiddenCols.borrow_mut()=Rc::new(cols);self
    }
    // java public void onCommit() { this.hasRowWrite = false; this.hasColWrite = false; }
    pub fn onCommit(&self){self.hasRowWrite.set(false);self.hasColWrite.set(false);}
    // java public void setRowOperable(int row, boolean operable) {
    pub fn setRowOperable(&self,row:i32,operable:bool){
        // java if (this.transaction.isTransStarted() && !this.hasRowWrite) {
        if self.transaction.upgrade().expect("NullPointerException").isTransStarted()&&!self.hasRowWrite.get(){
            // java this.optHiddenRows.beforeWrite(); this.hasRowWrite = true;
            self.optHiddenRows.borrow().beforeWrite();self.hasRowWrite.set(true);
        }
        // java this.optHiddenRows.setOperable(row, operable);
        self.optHiddenRows.borrow().setOperable(row,operable);
    }
    // java public void setColOperable(int col, boolean operable) {
    pub fn setColOperable(&self,col:i32,operable:bool){
        // java if (this.transaction.isTransStarted() && !this.hasColWrite) {
        if self.transaction.upgrade().expect("NullPointerException").isTransStarted()&&!self.hasColWrite.get(){
            // java this.optHiddenCols.beforeWrite(); this.hasColWrite = true;
            self.optHiddenCols.borrow().beforeWrite();self.hasColWrite.set(true);
        }
        // java this.optHiddenCols.setOperable(col, operable);
        self.optHiddenCols.borrow().setOperable(col,operable);
    }
    // java public boolean isRowOperable(int row) { return this.optHiddenRows.isOperable(row); }
    pub fn isRowOperable(&self,row:i32)->bool{self.optHiddenRows.borrow().isOperable(row)}
    // java public boolean isColOperable(int col) { return this.optHiddenCols.isOperable(col); }
    pub fn isColOperable(&self,col:i32)->bool{self.optHiddenCols.borrow().isOperable(col)}
    // java public Iterator<KmoPair<Integer, Integer>> rowIterator() { return this.optHiddenRows.iterator(); }
    pub fn rowIterator(&self)->KmoRowRangeIterator{self.optHiddenRows.borrow().iterator()}
    // java public Iterator<KmoPair<Integer, Integer>> colIterator() { return this.optHiddenCols.iterator(); }
    pub fn colIterator(&self)->KmoRowRangeIterator{self.optHiddenCols.borrow().iterator()}
    // java public int getRowRangeSize() { return this.optHiddenRows.getRangeSize(); }
    pub fn getRowRangeSize(&self)->i32{self.optHiddenRows.borrow().getRangeSize()}
    // java public int getColRangeSize() { return this.optHiddenCols.getRangeSize(); }
    pub fn getColRangeSize(&self)->i32{self.optHiddenCols.borrow().getRangeSize()}
    // java public void reset() {
    pub fn reset(&self){
        // java this.optHiddenRows = new OptHiddenRowCol(); this.optHiddenCols = new OptHiddenRowCol(); this.isInit = false;
        *self.optHiddenRows.borrow_mut()=Rc::new(OptHiddenRowCol::new());*self.optHiddenCols.borrow_mut()=Rc::new(OptHiddenRowCol::new());self.isInit.set(false);
    }
    // java public boolean isInit() { return this.isInit; }
    pub fn isInit(&self)->bool{self.isInit.get()}
    // java public void setInit(boolean init) { this.isInit = init; }
    pub fn setInit(&self,init:bool){self.isInit.set(init);}
    #[cfg(test)]pub(crate) fn reviewWriteFlags(&self)->(bool,bool){(self.hasRowWrite.get(),self.hasColWrite.get())}
}
impl KmoSheetTransactionListener for OptHiddenRowColManager{fn onCommit(&self){Self::onCommit(self)}}
