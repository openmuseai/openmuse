use crate::java_runtime::{Rc,RefCell};
use super::{KmoRect::KmoRect,KmoTreeSpatialIndex::KmoTreeSpatialIndex};
use kfc::cn::wps::memor4j::{KmoAtom::KmoAtom,KmoAtomData::{KmoAtomData,KmoAtomDataSub},KmoAtomStore::KmoAtomStore,AtomSnapshot::{SharedAtom,KmoAtom as SnapshotAtom,KmoAtomData as SnapshotData}};
// java class BackUp<T> extends KmoAtomData
struct BackUp<T>{base:KmoAtomData,range:Option<KmoRect>,data:Option<Rc<T>>,opType:i32}
impl<T> BackUp<T>{fn new()->Self{Self{base:KmoAtomData::new(),range:None,data:None,opType:0}}}
impl<T> KmoAtomDataSub for BackUp<T>{fn as_base(&self)->&KmoAtomData{&self.base}fn as_base_mut(&mut self)->&mut KmoAtomData{&mut self.base}
// java public KmoAtomData copy(){BackUp<T> ret=new BackUp(this.range,this.data,this.opType);ret.__forceOldest();return ret;}
fn copy(&self)->Self{let mut base=KmoAtomData::new();base.__forceOldest();Self{base,range:self.range.clone(),data:self.data.clone(),opType:self.opType}}}
struct Owner<T>{atom:Rc<RefCell<KmoAtom<BackUp<T>>>>,tree:Rc<KmoTreeSpatialIndex<T>>}
impl<T:Send+Sync+'static> Owner<T>{fn reverse(&self,undo:bool){let _lock=crate::cn::wps::et::ss::calcchain::recalcLock.lock();let a=self.atom.borrow();let d=a.rawData();if d.opType!=0{if (d.opType==1&&undo)||(d.opType==2&&!undo){self.tree.remove(d.range.as_ref().unwrap(),d.data.as_ref().unwrap());}else if (d.opType==2&&undo)||(d.opType==1&&!undo){self.tree.insert(d.range.as_ref().unwrap(),d.data.as_ref().unwrap().clone());}else{panic!("IllegalArgumentException");}}}}
impl<T:Send+Sync+'static> SnapshotAtom for Owner<T>{fn willSwap(&mut self,undo:bool){if undo{self.reverse(undo);}}fn swapData(&mut self,d:&mut Box<dyn SnapshotData>){self.atom.borrow_mut().swapSnapshotData(d);}fn didFinishSwapping(&mut self,undo:bool){if !undo{self.reverse(undo);}}fn isNeedSupportNonSequence(&self)->bool{false}fn cloneAtom(&self)->SharedAtom{let atom=Rc::new(RefCell::new(KmoAtom::new(self.atom.borrow().rawData().copy())));std::sync::Arc::new(std::sync::Mutex::new(Box::new(Owner{atom,tree:self.tree.clone()})))}}
// java public class KmoUndoableRTree<T> extends KmoAtom<BackUp<T>>
pub struct KmoUndoableRTree<T>{atom:Rc<RefCell<KmoAtom<BackUp<T>>>>,tree:Rc<KmoTreeSpatialIndex<T>>,_owner:SharedAtom}
impl<T:Send+Sync+'static> KmoUndoableRTree<T>{
// java public KmoUndoableRTree() {
// java super(new BackUp());
// java }
pub fn new()->Self {
let atom=Rc::new(RefCell::new(KmoAtom::new(BackUp::new())));let tree=Rc::new(KmoTreeSpatialIndex::new());let owner:SharedAtom=std::sync::Arc::new(std::sync::Mutex::new(Box::new(Owner{atom:atom.clone(),tree:tree.clone()})));atom.borrow_mut().bindSnapshotOwner(&owner);Self{atom,tree,_owner:owner}
}
// java public void insert(KmoRect rect, T r) {
// java if (KmoAtomStore.sharedStore().isAtomStarted()) {
// java this.beforeWrite();
// java ((BackUp)this.rawData()).data = r;
// java ((BackUp)this.rawData()).opType = 1;
// java ((BackUp)this.rawData()).range = new KmoRect(rect);
// java }
// java
// java this.tree.insert(rect, r);
// java }
pub fn insert(&self,rect:&KmoRect,data:Rc<T>) {
let started=KmoAtomStore::sharedStore().as_ref().unwrap().isAtomStarted();if started{let mut a=self.atom.borrow_mut();a.beforeWrite();let mut d=a.rawData_mut();d.data=Some(data.clone());d.opType=1;d.range=Some(rect.clone());}self.tree.insert(rect,data);
}
// java public void remove(KmoRect rect, T r) {
// java if (KmoAtomStore.sharedStore().isAtomStarted()) {
// java this.beforeWrite();
// java ((BackUp)this.rawData()).data = r;
// java ((BackUp)this.rawData()).opType = 2;
// java ((BackUp)this.rawData()).range = new KmoRect(rect);
// java }
// java
// java this.tree.remove(rect, r);
// java }
pub fn remove(&self,rect:&KmoRect,data:Rc<T>) {
let started=KmoAtomStore::sharedStore().as_ref().unwrap().isAtomStarted();if started{let mut a=self.atom.borrow_mut();a.beforeWrite();let mut d=a.rawData_mut();d.data=Some(data.clone());d.opType=2;d.range=Some(rect.clone());}self.tree.remove(rect,&data);
}
// java public void lookup(KmoRect rect, Collection<T> out) {
// java this.tree.lookup(rect, out);
// java }
pub fn lookup(&self,rect:&KmoRect,out:&mut Vec<Rc<T>>) {
self.tree.lookup(rect,out);
}
// java public T get(KmoRect rect) {
// java List<T> t = new ArrayList(1);
// java this.tree.lookup((KmoRect)rect, t);
// java return (T)(t.size() == 0 ? null : t.get(0));
// java }
pub fn get(&self,rect:&KmoRect)->Option<Rc<T>> {
self.tree.lookup_first(rect)
}
// java public void clear() {
// java this.tree.clear();
// java }
pub fn clear(&self) {
self.tree.clear();
}
}
