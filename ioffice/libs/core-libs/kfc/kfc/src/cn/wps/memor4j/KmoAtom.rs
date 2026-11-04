#![allow(non_snake_case)]
#![allow(dead_code)]

use super::AtomStoreImpl::AtomStoreImpl;
use super::KmoAtomData::{KmoAtomData, KmoAtomDataSub};
use super::KmoAtomDataEvent::KmoAtomDataEvent;
use super::KmoAtomStore::is_disable;
use super::AtomSnapshot::{SharedAtom, KmoAtomData as SnapshotData};
use std::sync::{Mutex, Weak};
use std::sync::Arc;
use parking_lot::{RwLock, ArcRwLockReadGuard, ArcRwLockWriteGuard, RawRwLock};

// Native Java reference: atom and snapshot retain the same data if copy throws.
pub type AtomDataRef<T> = Arc<RwLock<T>>;
pub type AtomDataRead<T> = ArcRwLockReadGuard<RawRwLock,T>;
pub type AtomDataWrite<T> = ArcRwLockWriteGuard<RawRwLock,T>;

// Java: public abstract class KmoAtom<AtomRawDataType extends KmoAtomData> implements KmoAtomDataEvent
//
// ★门面对齐要点（旧版偏差，已修正）：
//   - Java 的 `KmoAtom` **不继承** `KmoAtomData`，它只持有 `private KmoAtomData rawData`；
//     旧版曾写成 `// Java: public class KmoAtom extends KmoAtomData` 并加了 `Deref<Target=KmoAtomData>`，
//     那是凭空造出的继承关系，已删除。
//   - Java 是泛型类（`AtomRawDataType extends KmoAtomData`），旧版丢失了泛型参数。
pub struct KmoAtom<AtomRawDataType: KmoAtomDataSub> {
    // Java field: KmoAtomData rawData
    pub(crate) rawData: AtomDataRef<AtomRawDataType>,
    // rust: weak reference to the actual derived object retained by AtomImage.
    snapshotOwner: Weak<Mutex<Box<dyn super::AtomSnapshot::KmoAtom>>>,
    // rust: install the Send-bound type erasure only when a shared owner is bound;
    // the Java beforeWrite facade remains callable for existing local subclasses.
    snapshotCopy: Option<fn(&mut AtomDataRef<AtomRawDataType>, &mut dyn super::KmoAtomStore::KmoAtomStoreTrait, SharedAtom)>,
}

// Java: public abstract class KmoAtom<AtomRawDataType extends KmoAtomData> implements KmoAtomDataEvent
impl<AtomRawDataType: KmoAtomDataSub> KmoAtom<AtomRawDataType> {
    // Java: protected KmoAtom(AtomRawDataType rhs)
    pub fn new(rhs: AtomRawDataType) -> Self {
      // Java: super();
      // Java: this.rawData = rhs;
      KmoAtom { rawData: Arc::new(RwLock::new(rhs)), snapshotOwner: Weak::new(), snapshotCopy: None }
    }

    // Java: protected final AtomRawDataType rawData()
    pub fn rawData(&self) -> AtomDataRead<AtomRawDataType> {
      // Java: return (AtomRawDataType)this.rawData;
      self.rawData.read_arc_recursive()
    }

    // rust-only: Java 侧 `rawData()` 返回的引用在子类方法里被直接写（如
    // `((ShowShapeAtomData)this.rawData()).parentGroup = ...`），Rust 需要 `&mut` 入口。
    pub fn rawData_mut(&mut self) -> AtomDataWrite<AtomRawDataType> {
        self.rawData.write_arc()
    }

    // rust: bind Java's `this` to the shared handle for the most-derived object.
    // The derived object retains the strong handle, while its base keeps a weak
    // one so snapshots can retain the original object without a reference cycle.
    pub fn bindSnapshotOwner(&mut self, owner: &SharedAtom)
    where AtomRawDataType: Send + Sync + 'static {
        self.snapshotOwner = std::sync::Arc::downgrade(owner);
        self.snapshotCopy = Some(Self::copySnapshotData);
    }

    // Java: public final void beforeWrite()
    pub fn beforeWrite(&mut self) {
        // java this.rawData = AtomStoreImpl.sharedStoreImpl().needCopy(this, this.rawData);
        let mut guard = AtomStoreImpl::sharedStoreImpl();
        let store = guard.as_mut().expect("sharedStoreImpl");
        // java needCopy: if (disable) return data;
        if is_disable() { return; }
        // java else if (data.tip() < this.tip() && this.workingSnap != null)
        if self.rawData().as_base().tip() < store.tip() && store.isAtomStarted() {
            let owner = self.snapshotOwner.upgrade().unwrap_or_else(|| {
                panic!("DEFERRED_NON_FUNCTION: KmoAtom derived snapshot owner is not bound")
            });
            // java needCopy appends (this, original data), then invokes data.copy().
            // Native exception transport: Java's monitor is not poisoned when
            // needCopy throws. The assignment only occurs on normal return.
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.snapshotCopy.expect("bound snapshot data adapter")(&mut self.rawData, store.as_mut(), owner);
            }));
            drop(guard);
            if let Err(error) = result { std::panic::resume_unwind(error); }
        }
    }

    fn copySnapshotData(raw_data: &mut AtomDataRef<AtomRawDataType>, store: &mut dyn super::KmoAtomStore::KmoAtomStoreTrait, owner: SharedAtom)
    where AtomRawDataType: Send + Sync + 'static {
        // java this.rawData = needCopy(this, this.rawData);
        let mut next = store.needCopy(owner, Box::new(TypedAtomData { data: raw_data.clone() }));
        *raw_data = next.as_any_mut().downcast_mut::<TypedAtomData<AtomRawDataType>>()
            .expect("KmoAtomData subclass").data.clone();
    }

    // rust: type-erased entry used by AtomSnapshot.swap on the same derived object.
    pub fn swapSnapshotData(&mut self, data: &mut Box<dyn SnapshotData>)
    where AtomRawDataType: Send + Sync + 'static {
        let data = data.as_any_mut().downcast_mut::<TypedAtomData<AtomRawDataType>>()
            .expect("KmoAtomData subclass");
        // java KmoAtomData tmp = this.rawData; this.rawData = rhs; return tmp;
        std::mem::swap(&mut self.rawData, &mut data.data);
    }

    // Java: public final KmoAtomData swapData(KmoAtomData rhs)
    //
    // Java 签名用的是基类类型（`KmoAtomData`），调用方传入的是具体子类实例；
    // Rust 用泛型参数 `AtomRawDataType` 承载同一语义（Java 的 `KmoAtomData` 无子类型多态到具体类）。
    pub fn swapData(&mut self, rhs: AtomDataRef<AtomRawDataType>) -> AtomDataRef<AtomRawDataType> {
      // Java: KmoAtomData tmp = this.rawData;
      let tmp = std::mem::replace(&mut self.rawData, rhs);
      // Java: this.rawData = rhs;
      // included in std::mem::replace above
      let _ = ();
      // Java: return tmp;
      tmp
    }

    // Java: public void willSwap(boolean isUndo)
    pub fn willSwap(&mut self, _isUndo: bool) {
    }

    // Java: public void didFinishSwapping(boolean isUndo)
    pub fn didFinishSwapping(&mut self, _isUndo: bool) {
    }

    // Java: protected Object clone() throws CloneNotSupportedException
    pub fn clone(&self) -> Result<KmoAtom<AtomRawDataType>, String> {
      // Java: return super.clone();
      Ok(KmoAtom {
          rawData: self.rawData.clone(),
          snapshotOwner: Weak::new(),
          snapshotCopy: None,
      })
    }

    // Java: public boolean isNeedSupportNonSequence()
    pub fn isNeedSupportNonSequence(&self) -> bool {
      // Java: return false;
      false
    }

}

impl<AtomRawDataType: KmoAtomDataSub> KmoAtomDataEvent for KmoAtom<AtomRawDataType> {
    fn willSwap(&mut self, var1: bool) {
        self.willSwap(var1);
    }
    fn didFinishSwapping(&mut self, var1: bool) {
        self.didFinishSwapping(var1);
    }
}

// rust: preserve the concrete Java KmoAtomData subclass across the snapshot facade.
struct TypedAtomData<T> { data: AtomDataRef<T> }

impl<T: KmoAtomDataSub + Send + Sync + 'static> SnapshotData for TypedAtomData<T> {
    fn tip(&self) -> i32 { self.data.read_recursive().as_base().tip() }
    fn copy(&self) -> Box<dyn SnapshotData> {
        Box::new(Self { data: Arc::new(RwLock::new(self.data.read_recursive().copy())) })
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
