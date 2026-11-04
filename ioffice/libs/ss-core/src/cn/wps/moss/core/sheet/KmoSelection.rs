use crate::java_runtime::{Rc,RefCell};use kfc::cn::wps::memor4j::{KmoAtom::KmoAtom,AtomSnapshot::SharedAtom,KmoAtomData::KmoAtomDataSub};use super::KmoSelectionData::KmoSelectionData;use crate::cn::wps::moss::{util::KmoRect::KmoRect,filefmt::biff8::record::SelectionRecord::SelectionRecord};
// java public class KmoSelection extends KmoAtom<KmoSelectionData>
pub struct KmoSelection{atom:Rc<RefCell<KmoAtom<KmoSelectionData>>>,_owner:SharedAtom}
impl KmoSelection{
// java public KmoSelection() {
// java super(new KmoSelectionData());
// java }
pub fn new()->Self{Self::fromData(KmoSelectionData::new())}fn fromData(data:KmoSelectionData)->Self{let(atom,owner)=crate::cn::wps::moss::app::xf::snapshot::bind(data);Self{atom,_owner:owner}}
// java public static KmoSelection fromRecord(SelectionRecord rec) {
// java KmoSelection ret = new KmoSelection(new KmoSelectionData());
// java ((KmoSelectionData)ret.rawData()).pane = rec.getPane();
// java ((KmoSelectionData)ret.rawData()).activeRow = rec.getActiveCellRow();
// java ((KmoSelectionData)ret.rawData()).activeCol = rec.getActiveCellCol();
// java ((KmoSelectionData)ret.rawData()).activeRefIndex = rec.getActiveCellRef();
// java
// java for(int i = 0; i < rec.getSelectSize(); ++i) {
// java CellRangeAddress rg = new CellRangeAddress(0, 0, 0, 0);
// java rec.getCellRangeAddress(i, rg);
// java ((KmoSelectionData)ret.rawData()).refs.add(rg);
// java if (rg.isInRange(rec.getActiveCellRow(), rec.getActiveCellCol())) {
// java ((KmoSelectionData)ret.rawData()).selection.copy(rg.getFirstRow(), rg.getFirstColumn(), rg.getLastRow(), rg.getLastColumn());
// java }
// java }
// java
// java return ret;
// java }
pub fn fromRecord(rec:&SelectionRecord)->Self{let mut d=KmoSelectionData::new();d.pane=rec.getPane();d.activeRow=rec.getActiveCellRow();d.activeCol=rec.getActiveCellCol();d.activeRefIndex=rec.getActiveCellRef();for range in rec.refs(){d.refs.push(range.clone());if range.isInRange(rec.getActiveCellRow(),rec.getActiveCellCol()){d.selection=KmoRect::KmoRect__bounds(range.getFirstRow(),range.getFirstColumn(),range.getLastRow(),range.getLastColumn());}}Self::fromData(d)}
// java public byte pane() {
// java return ((KmoSelectionData)this.rawData()).pane;
// java }
pub fn pane(&self)->i8{self.atom.borrow().rawData().pane}
// java public void setPane(byte pane) {
// java this.beforeWrite();
// java ((KmoSelectionData)this.rawData()).pane = pane;
// java }
pub fn setPane(&self,v:i8){let mut a=self.atom.borrow_mut();a.beforeWrite();a.rawData_mut().pane=v;}
// java public int activeRow() {
// java return ((KmoSelectionData)this.rawData()).activeRow;
// java }
pub fn activeRow(&self)->i32{self.atom.borrow().rawData().activeRow}
// java public void setActiveRow(int row) {
// java this.beforeWrite();
// java ((KmoSelectionData)this.rawData()).activeRow = row;
// java }
pub fn setActiveRow(&self,v:i32){let mut a=self.atom.borrow_mut();a.beforeWrite();a.rawData_mut().activeRow=v;}
// java public int activeCol() {
// java return ((KmoSelectionData)this.rawData()).activeCol;
// java }
pub fn activeCol(&self)->i32{self.atom.borrow().rawData().activeCol}
// java public void setActiveCol(int col) {
// java this.beforeWrite();
// java ((KmoSelectionData)this.rawData()).activeCol = col;
// java }
pub fn setActiveCol(&self,v:i32){let mut a=self.atom.borrow_mut();a.beforeWrite();a.rawData_mut().activeCol=v;}
// java public KmoRect selection() {
// java return ((KmoSelectionData)this.rawData()).selection;
// java }
pub fn selection(&self)->KmoRect{self.atom.borrow().rawData().selection.clone()}
// java public void setSelection(KmoRect rect, int row, int col) {
pub fn setSelection(&self,rect:&KmoRect,row:i32,col:i32){
    // java this.beforeWrite();
    let mut atom=self.atom.borrow_mut();atom.beforeWrite();let mut data=atom.rawData_mut();
    // java ((KmoSelectionData)this.rawData()).selection.copy(rect);
    data.selection=rect.clone();
    // java ((KmoSelectionData)this.rawData()).activeRow = row;
    data.activeRow=row;
    // java ((KmoSelectionData)this.rawData()).activeCol = col;
    data.activeCol=col;
    // java ((KmoSelectionData)this.rawData()).refs.clear();
    data.refs.clear();
    // java ((KmoSelectionData)this.rawData()).refs.add(ExpandHelper.rect2range(rect));
    data.refs.push(crate::cn::wps::moss::filefmt::biff8::util::CellRangeAddress::CellRangeAddress::new(rect.tl.row,rect.br.row,rect.tl.col,rect.br.col));
}
}
impl Clone for KmoSelection{fn clone(&self)->Self{Self::fromData(self.atom.borrow().rawData().copy())}}
