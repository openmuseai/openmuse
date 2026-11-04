use crate::{java_runtime::{Rc,Weak,RefCell},cn::wps::{moss::app::KmoSheet::KmoSheet,et::ss::util::indexmap::{SparseIndexMap::SparseIndexMap,IndexMapIterator::IndexMapIterator}}};
use super::KmoRowInfo::KmoRowInfo;
#[cfg(test)]
#[path="KmoRowInfos.review.rs"]
mod review;
#[cfg(test)]
#[path="KmoRowInfos.merge.review.rs"]
mod merge_review;
// java public class KmoRowInfos extends SparseIndexMap<KmoRowInfo> {
pub struct KmoRowInfos{
    pub(crate) base:Rc<RefCell<SparseIndexMap<Rc<KmoRowInfo>>>>,
    // java private KmoSheet sheet;
    sheet:Weak<RefCell<KmoSheet>>,
}
impl KmoRowInfos{
    // java public KmoRowInfos copy(KmoRowInfos infos) {
    pub fn copy(&self,infos:Option<&Self>)->&Self{
        // java this.firstIndex = infos.firstIndex; this.lastIndex = infos.lastIndex;
        let infos=infos.expect("NullPointerException");let first=infos.base.borrow().firstIndex;self.base.borrow_mut().firstIndex=first;let last=infos.base.borrow().lastIndex;self.base.borrow_mut().lastIndex=last;
        // java for(int i = 0; i < infos.blocks.length; ++i) {
        let mut i=0;while i<infos.base.borrow().blocks.len(){
            // java Object[] srcRows = infos.blocks[i]; if (srcRows != null) {
            // Native retained references to the original block elements; no row/data copy.
            let srcRows=infos.base.borrow().blocks[i].clone();if let Some(srcRows)=srcRows{
                // java this.blocks[i] = new KmoRowInfo[srcRows.length];
                let mut rows=vec![];rows.resize_with(srcRows.len(),||None);*self.base.borrow_mut().blocks.get_mut(i).expect("ArrayIndexOutOfBoundsException")=Some(rows);
                // java for(int j = 0; j < srcRows.length; ++j) { if (srcRows[j] != null) {
                for j in 0..srcRows.len(){if let Some(src)=&srcRows[j]{
                    // java KmoRowInfo rowInfo = ((KmoRowInfo)srcRows[j]).copy();
                    let rowInfo=Rc::new(src.copy());
                    // java rowInfo.setHiddenRowsManager(this.sheet);
                    rowInfo.setHiddenRowsManager(Some(self.sheet.upgrade().expect("NullPointerException")));
                    // java this.blocks[i][j] = rowInfo;
                    self.base.borrow_mut().blocks[i].as_mut().unwrap()[j]=Some(rowInfo);
                }}
            }i+=1;
        }
        // java return this;
        self
    }
    // java public void mergeCopy(KmoSheet extSheet, KmoBook curBook) {
    pub fn mergeCopy(&self,extSheet:Option<&KmoSheet>,curBook:Option<&crate::cn::wps::moss::app::KmoBook::KmoBook>){
        // java KmoRowInfos infos = extSheet.getRowInfos();
        let extSheet=extSheet.expect("NullPointerException");let infos=extSheet.getRowInfos();
        // java this.firstIndex = infos.firstIndex; this.lastIndex = infos.lastIndex;
        let first=infos.base.borrow().firstIndex;self.base.borrow_mut().firstIndex=first;let last=infos.base.borrow().lastIndex;self.base.borrow_mut().lastIndex=last;
        // java for(int i = 0; i < infos.blocks.length; ++i) {
        let mut i=0;while i<infos.base.borrow().blocks.len(){
            // java Object[] srcRows = infos.blocks[i]; if (srcRows != null) {
            let srcRows=infos.base.borrow().blocks[i].clone();if let Some(srcRows)=srcRows{
                // java if (i >= this.blocks.length) { break; }
                if i>=self.base.borrow().blocks.len(){break;}
                // java this.blocks[i] = new KmoRowInfo[this.BLOCK_SIZE]; int len = Math.min(this.BLOCK_SIZE, srcRows.length);
                let blockSize=self.base.borrow().BLOCK_SIZE as usize;let mut rows=vec![];rows.resize_with(blockSize,||None);self.base.borrow_mut().blocks[i]=Some(rows);let len=std::cmp::min(blockSize,srcRows.len());
                // java for(int j = 0; j < len; ++j) { if (srcRows[j] != null) {
                for j in 0..len{if let Some(src)=&srcRows[j]{
                    // java KmoRowInfo rowInfo = ((KmoRowInfo)srcRows[j]).mergeCopy(extSheet, curBook);
                    let rowInfo=Rc::new(src.mergeCopy(Some(extSheet),curBook));
                    // java rowInfo.setHiddenRowsManager(this.sheet);
                    rowInfo.setHiddenRowsManager(Some(self.sheet.upgrade().expect("NullPointerException")));
                    // java this.blocks[i][j] = rowInfo;
                    self.base.borrow_mut().blocks[i].as_mut().unwrap()[j]=Some(rowInfo);
                }}
            }i+=1;
        }
    }
    // java public void mergeCopy(int offset, int ignoreNum, KmoRect extUsedRange, KmoSheet extSheet, KmoBook curBook) {
    pub fn mergeCopy__range(&self,offset:i32,ignoreNum:i32,extUsedRange:Option<&crate::cn::wps::moss::util::KmoRect::KmoRect>,extSheet:Option<&KmoSheet>,curBook:Option<&crate::cn::wps::moss::app::KmoBook::KmoBook>){
        // java KmoRowInfos infos = extSheet.getRowInfos(); int lst = infos.getLastRow(); if (lst >= ignoreNum) {
        let extSheet=extSheet.expect("NullPointerException");let infos=extSheet.getRowInfos();let mut lst=infos.getLastRow();if lst>=ignoreNum{
            // java if (lst > extUsedRange.br.row) { lst = extUsedRange.br.row; }
            let extUsedRange=extUsedRange.expect("NullPointerException");if lst>extUsedRange.br.row{lst=extUsedRange.br.row;}
            // java int fst = infos.getFirstRow(); if (fst < ignoreNum) { fst = ignoreNum; }
            let mut fst=infos.getFirstRow();if fst<ignoreNum{fst=ignoreNum;}
            // java for(int i = fst; i <= lst; ++i) {
            let mut i=fst;while i<=lst{
                // java KmoRowInfo rowInfo = infos.getRowInfo(i); if (rowInfo != null) {
                if let Some(rowInfo)=infos.getRowInfo(i){
                    // java KmoRowInfo newRowInfo = rowInfo.mergeCopy(i + offset, extSheet, curBook); this.setRowInfo(newRowInfo);
                    let newRowInfo=Rc::new(rowInfo.mergeCopy__row(i.wrapping_add(offset),Some(extSheet),curBook));self.setRowInfo(Some(newRowInfo));
                }i=i.wrapping_add(1);
            }
            // java this.lastIndex += lst - fst + 1;
            let mut base=self.base.borrow_mut();base.lastIndex=base.lastIndex.wrapping_add(lst.wrapping_sub(fst).wrapping_add(1));
        }
    }
    // java public KmoRowInfos(KmoSheet sheet) { super(sheet.getMaxRows()); this.sheet = sheet; }
    // Native constructor-this binding: maxRows evaluates the original sheet.getMaxRows before Arc publication.
    pub fn new(sheet:Weak<RefCell<KmoSheet>>,maxRows:i32)->Self{Self{base:Rc::new(RefCell::new(SparseIndexMap::SparseIndexMap(maxRows))),sheet}}
    // java public KmoRowInfo getRowInfo(int rowIndex) { return rowIndex < 0 ? null : (KmoRowInfo)this.get(rowIndex); }
    pub fn getRowInfo(&self,rowIndex:i32)->Option<Rc<KmoRowInfo>>{if rowIndex<0{None}else{self.base.borrow().get(rowIndex).cloned()}}
    // java public void setRowInfo(KmoRowInfo rowInfo) {
    pub fn setRowInfo(&self,rowInfo:Option<Rc<KmoRowInfo>>){
        // java rowInfo.setHiddenRowsManager(this.sheet);
        let rowInfo=rowInfo.expect("NullPointerException");rowInfo.setHiddenRowsManager(Some(self.sheet.upgrade().expect("NullPointerException")));
        // java this.put(rowInfo.getRowIndex(), rowInfo);
        let index=rowInfo.getRowIndex();self.base.borrow_mut().put(index,rowInfo);
    }
    // java public int getFirstRow() { return this.getFirstIndex(); }
    pub fn getFirstRow(&self)->i32{self.base.borrow().getFirstIndex()}
    // java public int getLastRow() { return this.getLastIndex(); }
    pub fn getLastRow(&self)->i32{self.base.borrow().getLastIndex()}
    // Native inherited SparseIndexMap iterator receiver transport.
    // java public IndexMapIterator<T> iterator() { return this.iterator(this.firstIndex, this.lastIndex); }
    pub fn iterator(&self)->IndexMapIterator<Rc<KmoRowInfo>>{let base=self.base.borrow();let(first,last)=(base.firstIndex,base.lastIndex);drop(base);self.iterator__range(first,last)}
    // java public IndexMapIterator<T> iterator(int first, int last) { return new IndexMapIterator<T>(this, first, last); }
    pub fn iterator__range(&self,first:i32,last:i32)->IndexMapIterator<Rc<KmoRowInfo>>{IndexMapIterator::new(Some(self.base.clone()),first,last)}
    // Native inherited SparseIndexMap methods use the same base object.
    pub fn put(&self,index:i32,rowInfo:Option<Rc<KmoRowInfo>>)->Option<Rc<KmoRowInfo>>{self.base.borrow_mut().putNullable(index,rowInfo)}
    pub fn remove(&self,index:i32)->Option<Rc<KmoRowInfo>>{self.base.borrow_mut().remove(index)}
}
