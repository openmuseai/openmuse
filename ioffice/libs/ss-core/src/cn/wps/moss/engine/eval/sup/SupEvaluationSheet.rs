use crate::{java_runtime::{Rc,Weak},cn::wps::{moss::{engine::linktb::LinkTable::CRNBlock,filefmt::biff8::util::CellRangeAddress::CellRangeAddress},et::ss::{interfaces::{IEvalCell::IEvalCell,IEvalWorksheet::IEvalWorksheet,IEvalFormulaBase::IEvalFormulaBase},util::Area::Area}}};
use super::SupEvaluationCell::SupEvaluationCell;
// java public final class SupEvaluationSheet implements IEvalWorksheet {
pub struct SupEvaluationSheet{
    // java int sheetIndex;
    sheetIndex:i32,
    // java LinkTable.CRNBlock crnblock;
    crnblock:Option<Rc<CRNBlock>>,
    // Native Java outer-reference adapter; iterators retain the sheet object.
    selfRef:Weak<SupEvaluationSheet>,
}
impl SupEvaluationSheet{
    // java public SupEvaluationSheet(int sheetIndex,LinkTable.CRNBlock crnblock){this.sheetIndex=sheetIndex;this.crnblock=crnblock;}
    pub fn SupEvaluationSheet(sheetIndex:i32,crnblock:Option<Rc<CRNBlock>>)->Rc<Self>{Rc::new_cyclic(|this|Self{sheetIndex,crnblock,selfRef:this.clone()})}
    // java public IEvalCell getCell(int rowIndex,int columnIndex){CRNRecord[] records=this.crnblock.getCrns();int i=0;for(int len=records.length;i<len;++i){CRNRecord r=records[i];if(r.getRowIndex()==rowIndex&&r.getFirstColumn()<=columnIndex&&columnIndex<=r.getLastColumn()){return new SupEvaluationCell(this.sheetIndex,rowIndex,columnIndex,r.getConstantValue(columnIndex));}}return new SupEvaluationCell(this.sheetIndex,rowIndex,columnIndex,"");}
    pub fn getCell(&self,rowIndex:i32,columnIndex:i32)->Rc<SupEvaluationCell>{let records=self.crnblock.as_ref().expect("NullPointerException").getCrns();let mut i=0;let len=records.len();while i<len{let r=records[i].as_ref().expect("NullPointerException");if r.getRowIndex()==rowIndex&&r.getFirstColumn()<=columnIndex&&columnIndex<=r.getLastColumn(){return Rc::new(SupEvaluationCell::SupEvaluationCell(self.sheetIndex,rowIndex,columnIndex,r.getConstantValue(columnIndex)));}i+=1;}Rc::new(SupEvaluationCell::SupEvaluationCell(self.sheetIndex,rowIndex,columnIndex,Some(Rc::new(String::new()))))}
    // java public Iterator<IEvalCell> getCellIterator(int firstRow,int lastRow,int firstCol,int lastCol){return new CellIter(firstRow,lastRow,firstCol,lastCol);}
    pub fn getCellIterator(&self,firstRow:i32,lastRow:i32,firstCol:i32,lastCol:i32)->CellIter{CellIter::CellIter(self.selfRef.upgrade().unwrap(),firstRow,lastRow,firstCol,lastCol)}
    // java public Iterator<IEvalCell> getSubtotalIterator(int firstRow,int lastRow,int firstCol,int lastCol,boolean ignoreHiddenRow){return new CellIter(firstRow,lastRow,firstCol,lastCol);}
    pub fn getSubtotalIterator(&self,firstRow:i32,lastRow:i32,firstCol:i32,lastCol:i32,_ignoreHiddenRow:bool)->CellIter{CellIter::CellIter(self.selfRef.upgrade().unwrap(),firstRow,lastRow,firstCol,lastCol)}
    // java public boolean isRowHidden(int row){return false;}
    pub fn isRowHidden(&self,_row:i32)->bool{false}
    // java public boolean isRowHiddenByFilter(int row){return false;}
    pub fn isRowHiddenByFilter(&self,_row:i32)->bool{false}
    // java public int getCellAligenment(int row,int col){return 0;}
    pub fn getCellAligenment(&self,_row:i32,_col:i32)->i32{0}
    // java public String getCellNumfmt(int row,int col){return "";}
    pub fn getCellNumfmt(&self,_row:i32,_col:i32)->String{String::new()}
    // java public int getColWidth(int col){return 0;}
    pub fn getColWidth(&self,_col:i32)->i32{0}
    // java public boolean isCellLocked(int row,int col){return true;}
    pub fn isCellLocked(&self,_row:i32,_col:i32)->bool{true}
    // java public int getLeftTopCol(){return 0;}
    pub fn getLeftTopCol(&self)->i32{0}
    // java public CellRangeAddressBase getUseRange() {
    pub fn getUseRange(&self)->Option<CellRangeAddress>{
        // java if(this.crnblock==null){return null;}else if(this.crnblock.getCrns()==null){return null;}else{
        let crnblock=self.crnblock.as_ref()?;
        // Native CRNBlock.getCrns clones its always initialized array. A null
        // array would throw inside the original clone before that null guard.
        let _=crnblock.getCrns();
        // java CRNRecord[] crnRecords=this.crnblock.getCrns();int fr=0;int lr=0;int fc=0;int lc=0;
        let crnRecords=crnblock.getCrns();let mut fr=0;let mut lr=0;let mut fc=0;let mut lc=0;
        // java for(CRNRecord record:crnRecords){int rrIndex=record.getRowIndex();int rfc=record.getFirstColumn();int rlc=record.getLastColumn();if(fr>rrIndex){fr=rrIndex;}if(lr<rrIndex){lr=rrIndex;}if(fc>rfc){fc=rfc;}if(lc<rlc){lc=rlc;}}
        for record in crnRecords{let record=record.expect("NullPointerException");let rrIndex=record.getRowIndex();let rfc=record.getFirstColumn();let rlc=record.getLastColumn();if fr>rrIndex{fr=rrIndex;}if lr<rrIndex{lr=rrIndex;}if fc>rfc{fc=rfc;}if lc<rlc{lc=rlc;}}
        // java return new CellRangeAddress(fr,lr,fc,lc);
        Some(CellRangeAddress::new(fr,lr,fc,lc))
    }
    // java public int getLeftTopRow(){return 0;}
    pub fn getLeftTopRow(&self)->i32{0}
    // java public int getUid(){throw new RuntimeException("unimpl");}
    pub fn getUid(&self)->i32{panic!("RuntimeException unimpl")}
    // java public Iterator<IEvalFormulaBase> getFormulaIterator(Area area){return null;}
    pub fn getFormulaIterator(&self,_area:Option<&Area>)->Option<Box<dyn Iterator<Item=Option<Rc<dyn IEvalFormulaBase>>>>>{None}
}
// java private class CellIter implements Iterator<IEvalCell> {
pub struct CellIter{
    // java final int firstRow;final int lastRow;final int firstCol;final int lastCol;int currentRow;int currentCol;
    firstRow:i32,lastRow:i32,firstCol:i32,lastCol:i32,currentRow:i32,currentCol:i32,
    // Native reference for SupEvaluationSheet.this.
    owner:Rc<SupEvaluationSheet>,
}
impl CellIter{
    // java public CellIter(int fr,int lr,int fc,int lc){this.firstRow=fr;this.lastRow=lr;this.firstCol=fc;this.lastCol=lc;this.currentRow=this.firstRow;this.currentCol=this.firstCol;}
    pub fn CellIter(owner:Rc<SupEvaluationSheet>,fr:i32,lr:i32,fc:i32,lc:i32)->Self{Self{firstRow:fr,lastRow:lr,firstCol:fc,lastCol:lc,currentRow:fr,currentCol:fc,owner}}
    // java public boolean hasNext(){return this.currentRow<=this.lastRow;}
    pub fn hasNext(&self)->bool{self.currentRow<=self.lastRow}
    // java public IEvalCell next(){IEvalCell ret=SupEvaluationSheet.this.getCell(this.currentRow,this.currentCol);++this.currentCol;if(this.currentCol>this.lastCol){++this.currentRow;this.currentCol=this.firstCol;}return ret;}
    pub fn next(&mut self)->Rc<SupEvaluationCell>{let ret=self.owner.getCell(self.currentRow,self.currentCol);self.currentCol=self.currentCol.wrapping_add(1);if self.currentCol>self.lastCol{self.currentRow=self.currentRow.wrapping_add(1);self.currentCol=self.firstCol;}ret}
    // java public void remove(){throw new RuntimeException("operation not allowed in this iterator");}
    pub fn remove(&self){panic!("RuntimeException operation not allowed in this iterator")}
}
impl Iterator for CellIter{type Item=Rc<dyn IEvalCell>;fn next(&mut self)->Option<Self::Item>{if self.hasNext(){Some(CellIter::next(self))}else{None}}}
impl IEvalWorksheet for SupEvaluationSheet{fn getUid(&self)->i32{Self::getUid(self)}fn getCell(&self,r:i32,c:i32)->Rc<dyn IEvalCell>{Self::getCell(self,r,c)}fn getCellIterator(&self,fr:i32,lr:i32,fc:i32,lc:i32)->Box<dyn Iterator<Item=Rc<dyn IEvalCell>>>{Box::new(Self::getCellIterator(self,fr,lr,fc,lc))}fn getFormulaIterator(&self,a:&Area)->Option<Box<dyn Iterator<Item=Option<Rc<dyn IEvalFormulaBase>>>>>{Self::getFormulaIterator(self,Some(a))}}
