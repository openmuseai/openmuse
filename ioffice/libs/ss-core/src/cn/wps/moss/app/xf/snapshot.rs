// Rust identity adapter for the Java KmoAtom<T> superclass. Every snapshot
// swaps data on the same atom used by KmoXF/KmoFont's public methods.
use crate::java_runtime::{Rc, RefCell};
use kfc::cn::wps::memor4j::{KmoAtom::KmoAtom, KmoAtomData::KmoAtomDataSub, AtomSnapshot::{KmoAtom as SnapshotAtom, KmoAtomData as SnapshotData, SharedAtom}};
struct AtomOwner<T: KmoAtomDataSub + Send + Sync + 'static>(Rc<RefCell<KmoAtom<T>>>);
impl<T: KmoAtomDataSub + Send + Sync + 'static> SnapshotAtom for AtomOwner<T> {
    fn willSwap(&mut self, undo: bool) {self.0.borrow_mut().willSwap(undo);}
    fn swapData(&mut self, data: &mut Box<dyn SnapshotData>) {self.0.borrow_mut().swapSnapshotData(data);}
    fn didFinishSwapping(&mut self, undo: bool) {self.0.borrow_mut().didFinishSwapping(undo);}
    fn isNeedSupportNonSequence(&self)->bool {self.0.borrow().isNeedSupportNonSequence()}
    fn cloneAtom(&self)->SharedAtom {bind(self.0.borrow().rawData().copy()).1}
}
pub(crate) fn bind<T: KmoAtomDataSub + Send + Sync + 'static>(data: T) -> (Rc<RefCell<KmoAtom<T>>>, SharedAtom) {
    let atom=Rc::new(RefCell::new(KmoAtom::new(data)));
    let owner: SharedAtom = std::sync::Arc::new(std::sync::Mutex::new(Box::new(AtomOwner(Rc::clone(&atom)))));
    atom.borrow_mut().bindSnapshotOwner(&owner);
    (atom,owner)
}
