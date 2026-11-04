#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

// Java field: int DEFAULT_MAX_UNDO_STEPS
pub(crate) const DEFAULT_MAX_UNDO_STEPS: i32 = 20;
// Java field: KmoAtomStore inst
static inst: Mutex<Option<Box<dyn KmoAtomStoreTrait + Send>>> = Mutex::new(None);
// Java field: boolean disable
static disable: AtomicBool = AtomicBool::new(false);
// Java field: boolean canReset
static canReset: AtomicBool = AtomicBool::new(true);
// rust: mirror AtomStoreImpl.tip so KmoAtomData construction during needCopy
// can read the Java singleton's tip without recursively locking inst.
static currentTip: AtomicI32 = AtomicI32::new(0);

pub trait KmoTipStamp {
    fn tip(&self) -> i32;
}

pub trait KmoAtomStoreTrait: Send {
    fn reset(&mut self);
    fn maxSteps(&self) -> i32;
    fn setMaxSteps(&mut self, var1: i32);
    fn snapshot(&mut self);
    fn canUndo(&self) -> bool;
    fn undo(&mut self);
    fn start(&mut self);
    fn discard(&mut self);
    fn isAtomStarted(&self) -> bool;
    fn getCurrentTipStamp(&self) -> Box<dyn KmoTipStamp>;
    fn isTheSameTipStamp(&self, var1: &dyn KmoTipStamp) -> bool;
    fn commit(&mut self) -> bool;
    fn canRedo(&self) -> bool;
    fn redo(&mut self);
    fn undo_at(&mut self, var1: i32);
    fn nextTip(&self) -> i32;
    fn tip(&self) -> i32;
    fn needCopy(&mut self, atom: super::AtomSnapshot::SharedAtom, data: Box<dyn super::AtomSnapshot::KmoAtomData>) -> Box<dyn super::AtomSnapshot::KmoAtomData>;
}

// Java: public class KmoAtomStore
pub struct KmoAtomStore {
}

// Java: public class KmoAtomStore
impl KmoAtomStore {
    // Java: KmoAtomStore()
    pub fn new() -> Self {
      // Java: super();
      KmoAtomStore {}
    }

    // Java: abstract void reset()
    pub fn reset(&self) {
    todo!()
    }

    // Java: abstract int maxSteps()
    pub fn maxSteps(&self) -> i32 {
    todo!()
    }

    // Java: abstract void setMaxSteps(int var1)
    pub fn setMaxSteps(&self, _var1: i32) {
    todo!()
    }

    // Java: abstract void snapshot()
    pub fn snapshot(&self) {
    todo!()
    }

    // Java: abstract boolean canUndo()
    pub fn canUndo(&self) -> bool {
    todo!()
    }

    // Java: abstract void undo()
    pub fn undo(&self) {
    todo!()
    }

    // Java: abstract void start()
    pub fn start(&self) {
    todo!()
    }

    // Java: abstract void discard()
    pub fn discard(&self) {
    todo!()
    }

    // Java: abstract boolean isAtomStarted()
    pub fn isAtomStarted(&self) -> bool {
    todo!()
    }

    // Java: abstract KmoTipStamp getCurrentTipStamp()
    pub fn getCurrentTipStamp(&self) -> Box<dyn KmoTipStamp> {
    todo!()
    }

    // Java: abstract boolean isTheSameTipStamp(KmoTipStamp var1)
    pub fn isTheSameTipStamp(&self, _var1: &dyn KmoTipStamp) -> bool {
    todo!()
    }

    // Java: abstract boolean commit()
    pub fn commit(&self) -> bool {
    todo!()
    }

    // Java: abstract boolean canRedo()
    pub fn canRedo(&self) -> bool {
    todo!()
    }

    // Java: abstract void redo()
    pub fn redo(&self) {
    todo!()
    }

    // Java: abstract void undo(int var1)
    pub fn undo_at(&self, _var1: i32) {
    todo!()
    }

    // Java: abstract int nextTip()
    pub fn nextTip(&self) -> i32 {
    todo!()
    }

    // Java: static KmoAtomStore sharedStore()
    pub fn sharedStore() -> std::sync::MutexGuard<'static, Option<Box<dyn KmoAtomStoreTrait + Send>>> {
      let mut guard = inst.lock().unwrap();
      // Java: if (inst == null)
      if guard.is_none() {
         // Java: inst = new AtomStoreImpl();
         *guard = Some(super::AtomStoreImpl::AtomStoreImpl::new_boxed());
         currentTip.store(0, Ordering::SeqCst);
      }
      // Java: return inst;
      guard
    }

    // Java: static void clearUp()
    pub fn clearUp() {
      // Java: if (!disable)
      if !disable.load(Ordering::SeqCst) {
         // Java: inst = null;
         *inst.lock().unwrap() = None;
         currentTip.store(0, Ordering::SeqCst);
      }
    }

    // Java: static void disable()
    pub fn disable() {
      // Java: disable = true;
      disable.store(true, Ordering::SeqCst);
    }

    // Java: static void enable()
    pub fn enable() {
      // Java: disable = false;
      disable.store(false, Ordering::SeqCst);
    }

    // Java: static void setCanReset(boolean reset)
    pub fn setCanReset(reset: bool) {
      // Java: canReset = reset;
      canReset.store(reset, Ordering::SeqCst);
    }
}

pub fn is_disable() -> bool {
    disable.load(Ordering::SeqCst)
}

pub fn is_can_reset() -> bool {
    canReset.load(Ordering::SeqCst)
}

pub fn set_shared_store(store: Box<dyn KmoAtomStoreTrait + Send>) {
    currentTip.store(store.tip(), Ordering::SeqCst);
    *inst.lock().unwrap() = Some(store);
}

pub(crate) fn tip_without_relocking_store() -> i32 {
    // java KmoAtomData constructor calls sharedStoreImpl().tip(); when no
    // store operation is active, preserve sharedStore's lazy initialization.
    match inst.try_lock() {
        Ok(mut guard) => {
            if guard.is_none() {
                *guard = Some(super::AtomStoreImpl::AtomStoreImpl::new_boxed());
            }
            let tip = guard.as_ref().unwrap().tip();
            currentTip.store(tip, Ordering::SeqCst);
            tip
        }
        // rust: data.copy() may construct KmoAtomData while needCopy owns inst.
        // The atomic mirror contains the tip published by that same store.
        Err(std::sync::TryLockError::WouldBlock) => currentTip.load(Ordering::SeqCst),
        Err(std::sync::TryLockError::Poisoned(error)) => panic!("KmoAtomStore poisoned: {error}"),
    }
}

pub(crate) fn publish_tip(tip: i32) {
    currentTip.store(tip, Ordering::SeqCst);
}
