use crate::{java_runtime::{Rc,Weak,RefCell,Cell},cn::wps::moss::app::{KmoSheet::KmoSheet,TransactionManager::TransactionManager,KmoTransaction::KmoSheetTransactionListener}};
use kfc::cn::wps::memor4j::{KmoAtom::KmoAtom,KmoAtomData::{KmoAtomData,KmoAtomDataSub},AtomSnapshot::SharedAtom};
use super::KmoRowRange::{KmoRowRange,KmoRowRangeIterator};
// java public class KmoHiddenRowsManager implements KmoTransaction.KmoSheetTransactionListener {
pub struct KmoHiddenRowsManager{
    // java private KmoHiddenRows hiddenRows = new KmoHiddenRows();
    hiddenRows:RefCell<Rc<KmoHiddenRows>>,
    // java private boolean hasWrite = false;
    hasWrite:Cell<bool>,
    // java private final KmoTransaction transaction;
    // Native owner back-reference; same transaction object retained by KmoBook.
    transaction:Weak<TransactionManager>,
}
impl KmoHiddenRowsManager{
    // java public KmoHiddenRowsManager(KmoSheet sheet) {
    pub fn new(sheet:&Rc<RefCell<KmoSheet>>)->Rc<Self>{
        // java this.transaction = sheet.getBook().transaction();
        let transaction=sheet.borrow().getBook().transactionReference();
        let ret=Rc::new(Self{hiddenRows:RefCell::new(Rc::new(KmoHiddenRows::new())),hasWrite:Cell::new(false),transaction:Rc::downgrade(&transaction)});
        // java this.transaction.addTransactionListener(sheet.getSheetUid(), this);
        transaction.addTransactionListener(sheet.borrow().getSheetUid(),Some(ret.clone()));ret
    }
    // java public KmoHiddenRowsManager copy(KmoHiddenRowsManager manager) {
    pub fn copy(&self,manager:Option<&Self>)->&Self{
        // java this.hiddenRows = manager.hiddenRows.copy(); return this;
        let copy=manager.expect("NullPointerException").hiddenRows.borrow().copy();*self.hiddenRows.borrow_mut()=Rc::new(copy);self
    }
    // java public void onCommit() { this.hasWrite = false; }
    pub fn onCommit(&self){self.hasWrite.set(false);}
    // java public boolean isHidden(int row) { return this.hiddenRows.isHidden(row); }
    pub fn isHidden(&self,row:i32)->bool{self.hiddenRows.borrow().isHidden(row)}
    // java public void setHidden(int row, boolean hidden) {
    pub fn setHidden(&self,row:i32,hidden:bool){
        // java if (this.transaction.isTransStarted() && !this.hasWrite) {
        if self.transaction.upgrade().expect("NullPointerException").isTransStarted()&&!self.hasWrite.get(){
            // java this.hiddenRows.beforeWrite(); this.hasWrite = true;
            self.hiddenRows.borrow().beforeWrite();self.hasWrite.set(true);
        }
        // java this.hiddenRows.setHiddenInner(row, hidden);
        self.hiddenRows.borrow().setHiddenInner(row,hidden);
    }
    // java public void mergeCopy(KmoSheet extSheet) {
    pub fn mergeCopy(&self,extSheet:Option<&KmoSheet>){
        // java for(KmoRowInfo rowInfo : extSheet.getRowInfos()) {
        let mut iter=extSheet.expect("NullPointerException").getRowInfos().iterator();while iter.hasNext(){let rowInfo=iter.next().expect("NullPointerException");
            // java this.hiddenRows.setHiddenInner(rowInfo.getRowIndex(), rowInfo.isHidden());
            self.hiddenRows.borrow().setHiddenInner(rowInfo.getRowIndex(),rowInfo.isHidden());
        }
    }
    // java public void mergeCopy(int offset, int ignoreNum, KmoRect extUsedRange, KmoSheet extSheet) {
    pub fn mergeCopy__range(&self,offset:i32,ignoreNum:i32,extUsedRange:Option<&crate::cn::wps::moss::util::KmoRect::KmoRect>,extSheet:Option<&KmoSheet>){
        // java KmoRowInfos infos = extSheet.getRowInfos(); int lst = infos.getLastRow(); if (lst >= ignoreNum) {
        let infos=extSheet.expect("NullPointerException").getRowInfos();let mut lst=infos.getLastRow();if lst>=ignoreNum{
            // java if (lst > extUsedRange.br.row) { lst = extUsedRange.br.row; }
            let extUsedRange=extUsedRange.expect("NullPointerException");if lst>extUsedRange.br.row{lst=extUsedRange.br.row;}
            // java int fst = infos.getFirstRow(); if (fst < ignoreNum) { fst = ignoreNum; }
            let mut fst=infos.getFirstRow();if fst<ignoreNum{fst=ignoreNum;}
            // java for(int i = fst; i <= lst; ++i) {
            let mut i=fst;while i<=lst{
                // java KmoRowInfo rowInfo = infos.getRowInfo(i); if (rowInfo != null) { this.hiddenRows.setHiddenInner(i + offset, rowInfo.isHidden()); }
                if let Some(rowInfo)=infos.getRowInfo(i){self.hiddenRows.borrow().setHiddenInner(i.wrapping_add(offset),rowInfo.isHidden());}i=i.wrapping_add(1);
            }
        }
    }
    // java public int getHiddenRowsCount(int rowIndex) { return this.hiddenRows.getHiddenCount(rowIndex); }
    pub fn getHiddenRowsCount(&self,rowIndex:i32)->i32{self.hiddenRows.borrow().getHiddenCount(rowIndex)}
    // java public KmoRowRange getRowRange() { return this.hiddenRows.getRowRange(); }
    pub fn getRowRange(&self)->Rc<KmoRowRange>{self.hiddenRows.borrow().getRowRange()}
    // java public Iterator<KmoPair<Integer, Integer>> iterator() { return this.hiddenRows.iterator(); }
    pub fn iterator(&self)->KmoRowRangeIterator{self.hiddenRows.borrow().iterator()}
    #[cfg(test)]pub(crate) fn reviewHasWrite(&self)->bool{self.hasWrite.get()}
}
impl KmoSheetTransactionListener for KmoHiddenRowsManager{fn onCommit(&self){Self::onCommit(self)}}
// java static class KmoHiddenRows extends KmoAtom<KmoHiddenRowsData> {
struct KmoHiddenRows{atom:Rc<RefCell<KmoAtom<KmoHiddenRowsData>>>,_owner:SharedAtom}
impl KmoHiddenRows{
    // java private KmoHiddenRows() { super(new KmoHiddenRowsData()); }
    fn new()->Self{let(atom,owner)=crate::cn::wps::moss::app::xf::snapshot::bind(KmoHiddenRowsData::new());Self{atom,_owner:owner}}
    // java private KmoHiddenRows copy() {
    fn copy(&self)->Self{
        // java KmoHiddenRows ret = new KmoHiddenRows();
        let ret=Self::new();
        // java ((KmoHiddenRowsData)ret.rawData()).hiddenRows = ((KmoHiddenRowsData)this.rawData()).hiddenRows.clone(); return ret;
        ret.atom.borrow_mut().rawData_mut().hiddenRows=Rc::new(self.atom.borrow().rawData().hiddenRows.as_ref().clone());ret
    }
    // java private boolean isHidden(int row) { return ((KmoHiddenRowsData)this.rawData()).hiddenRows.containsRow(row); }
    fn isHidden(&self,row:i32)->bool{self.atom.borrow().rawData().hiddenRows.containsRow(row)}
    // java private void setHiddenInner(int row, boolean hidden) {
    fn setHiddenInner(&self,row:i32,hidden:bool){
        // java if (hidden) { ((KmoHiddenRowsData)this.rawData()).hiddenRows.addRow(row); }
        if hidden{self.atom.borrow().rawData().hiddenRows.addRow(row);}
        // java else { ((KmoHiddenRowsData)this.rawData()).hiddenRows.removeRow(row); }
        else{self.atom.borrow().rawData().hiddenRows.removeRow(row);}
    }
    // java public int getHiddenCount(int rowIndex) { return ((KmoHiddenRowsData)this.rawData()).hiddenRows.getRowCount(rowIndex); }
    fn getHiddenCount(&self,rowIndex:i32)->i32{self.atom.borrow().rawData().hiddenRows.getRowCount(rowIndex)}
    // java public KmoRowRange getRowRange() { return ((KmoHiddenRowsData)this.rawData()).hiddenRows; }
    fn getRowRange(&self)->Rc<KmoRowRange>{self.atom.borrow().rawData().hiddenRows.clone()}
    // java public Iterator<KmoPair<Integer, Integer>> iterator() { return ((KmoHiddenRowsData)this.rawData()).hiddenRows.iterator(); }
    fn iterator(&self)->KmoRowRangeIterator{self.atom.borrow().rawData().hiddenRows.iterator()}
    // Native inherited KmoAtom dispatch.
    fn beforeWrite(&self){self.atom.borrow_mut().beforeWrite();}
}
// java static class KmoHiddenRowsData extends KmoAtomData {
struct KmoHiddenRowsData{base:KmoAtomData,
    // java private KmoRowRange hiddenRows = new KmoRowRange();
    hiddenRows:Rc<KmoRowRange>,
}
impl KmoHiddenRowsData{
    fn new()->Self{Self{base:KmoAtomData::new(),hiddenRows:Rc::new(KmoRowRange::new())}}
    // java public KmoAtomData copy() {
    fn copy(&self)->Self{
        // java KmoHiddenRowsData data = new KmoHiddenRowsData();
        let mut data=Self::new();
        // java data.hiddenRows = this.hiddenRows.clone(); return data;
        data.hiddenRows=Rc::new(self.hiddenRows.as_ref().clone());data
    }
}
impl KmoAtomDataSub for KmoHiddenRowsData{fn as_base(&self)->&KmoAtomData{&self.base}fn as_base_mut(&mut self)->&mut KmoAtomData{&mut self.base}fn copy(&self)->Self{KmoHiddenRowsData::copy(self)}}
