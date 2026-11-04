use crate::java_runtime::{Rc,RefCell};use kfc::cn::wps::memor4j::{KmoAtom::KmoAtom,KmoAtomData::{KmoAtomData,KmoAtomDataSub},AtomSnapshot::SharedAtom};
// java class BackUp extends KmoAtomData {protected int count=0;public KmoAtomData copy(){BackUp backUp=new BackUp();backUp.count=this.count;return backUp;}}
struct BackUp{base:KmoAtomData,count:i32}
impl KmoAtomDataSub for BackUp{fn as_base(&self)->&KmoAtomData{&self.base}fn as_base_mut(&mut self)->&mut KmoAtomData{&mut self.base}fn copy(&self)->Self{Self{base:KmoAtomData::new(),count:self.count}}}
// java class CfRuleCounter extends KmoAtom<BackUp>
pub struct CfRuleCounter{atom:Rc<RefCell<KmoAtom<BackUp>>>,_owner:SharedAtom}
impl CfRuleCounter{
// java public CfRuleCounter() {
// java super(new BackUp());
// java }
pub fn new()->Self {
let(atom,owner)=super::super::xf::snapshot::bind(BackUp{base:KmoAtomData::new(),count:0});Self{atom,_owner:owner}
}
// java public int count() {
// java return ((BackUp)this.rawData()).count;
// java }
pub fn count(&self)->i32 {
self.atom.borrow().rawData().count
}
// java public void plus() {
// java this.beforeWrite();
// java ++((BackUp)this.rawData()).count;
// java }
pub fn plus(&self) {
let mut a=self.atom.borrow_mut();a.beforeWrite();let mut data=a.rawData_mut();data.count=data.count.wrapping_add(1);
}
// java public void minus() {
// java this.beforeWrite();
// java --((BackUp)this.rawData()).count;
// java }
pub fn minus(&self) {
let mut a=self.atom.borrow_mut();a.beforeWrite();let mut data=a.rawData_mut();data.count=data.count.wrapping_sub(1);
}
}
