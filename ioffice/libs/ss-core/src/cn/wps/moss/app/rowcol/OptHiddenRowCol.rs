use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::core::sheet::KmoRowRange::{KmoRowRange,KmoRowRangeIterator}};
use kfc::cn::wps::memor4j::{KmoAtom::KmoAtom,KmoAtomData::{KmoAtomData,KmoAtomDataSub},AtomSnapshot::SharedAtom};
// java public class OptHiddenRowCol extends KmoAtom<OptHiddenRowCol.OptHiddenRowColData> {
pub struct OptHiddenRowCol{atom:Rc<RefCell<KmoAtom<OptHiddenRowColData>>>,_owner:SharedAtom}
impl OptHiddenRowCol{
    // java protected OptHiddenRowCol() { super(new OptHiddenRowColData()); }
    pub fn new()->Self{let(atom,owner)=crate::cn::wps::moss::app::xf::snapshot::bind(OptHiddenRowColData::new());Self{atom,_owner:owner}}
    // java OptHiddenRowCol copy() {
    pub fn copy(&self)->Self{
        // java OptHiddenRowCol ret = new OptHiddenRowCol();
        let ret=Self::new();
        // java ((OptHiddenRowColData)ret.rawData()).optHiddenRange = ((OptHiddenRowColData)this.rawData()).optHiddenRange.clone(); return ret;
        ret.atom.borrow_mut().rawData_mut().optHiddenRange=Rc::new(self.atom.borrow().rawData().optHiddenRange.as_ref().clone());ret
    }
    // java boolean isOperable(int rc) { return ((OptHiddenRowColData)this.rawData()).optHiddenRange.containsRow(rc); }
    pub fn isOperable(&self,rc:i32)->bool{self.atom.borrow().rawData().optHiddenRange.containsRow(rc)}
    // java void setOperable(int rc, boolean operable) {
    pub fn setOperable(&self,rc:i32,operable:bool){
        // java if (operable) { ((OptHiddenRowColData)this.rawData()).optHiddenRange.addRow(rc); }
        if operable{self.atom.borrow().rawData().optHiddenRange.addRow(rc);}
        // java else { ((OptHiddenRowColData)this.rawData()).optHiddenRange.removeRow(rc); }
        else{self.atom.borrow().rawData().optHiddenRange.removeRow(rc);}
    }
    // java Iterator<KmoPair<Integer, Integer>> iterator() { return ((OptHiddenRowColData)this.rawData()).optHiddenRange.iterator(); }
    pub fn iterator(&self)->KmoRowRangeIterator{self.atom.borrow().rawData().optHiddenRange.iterator()}
    // java int getRangeSize() { return ((OptHiddenRowColData)this.rawData()).optHiddenRange.getRangeSize(); }
    pub fn getRangeSize(&self)->i32{self.atom.borrow().rawData().optHiddenRange.getRangeSize()}
    // Native inherited KmoAtom dispatch.
    pub fn beforeWrite(&self){self.atom.borrow_mut().beforeWrite();}
}
// java static class OptHiddenRowColData extends KmoAtomData {
struct OptHiddenRowColData{base:KmoAtomData,
    // java private KmoRowRange optHiddenRange = new KmoRowRange();
    optHiddenRange:Rc<KmoRowRange>,
}
impl OptHiddenRowColData{
    fn new()->Self{Self{base:KmoAtomData::new(),optHiddenRange:Rc::new(KmoRowRange::new())}}
    // java public KmoAtomData copy() {
    fn copy(&self)->Self{
        // java OptHiddenRowColData data = new OptHiddenRowColData();
        let mut data=Self::new();
        // java data.optHiddenRange = this.optHiddenRange.clone(); return data;
        data.optHiddenRange=Rc::new(self.optHiddenRange.as_ref().clone());data
    }
}
impl KmoAtomDataSub for OptHiddenRowColData{fn as_base(&self)->&KmoAtomData{&self.base}fn as_base_mut(&mut self)->&mut KmoAtomData{&mut self.base}fn copy(&self)->Self{OptHiddenRowColData::copy(self)}}
