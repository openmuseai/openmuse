#![allow(non_snake_case)]
#![allow(dead_code)]

use super::AtomSnapshot::{AtomSnapshot, SharedAtom};
use super::AtomSnapshotStack::AtomSnapshotStack;
use super::KmoAtomStore::{is_disable, is_can_reset};

// Java: public class AtomStoreImpl extends KmoAtomStore
pub struct AtomStoreImpl {
    // Java field: AtomSnapshotStack snapshotStack
    pub(crate) snapshotStack: AtomSnapshotStack,
    // Java field: AtomSnapshot workingSnap
    pub(crate) workingSnap: Option<AtomSnapshot>,
    // Java field: int tip
    pub(crate) tip: i32,
    // Java field: int nextTip
    pub(crate) nextTip: i32,
}

// Java: public class AtomStoreImpl extends KmoAtomStore
impl AtomStoreImpl {
    pub fn new() -> Self {
        AtomStoreImpl {
            snapshotStack: AtomSnapshotStack::new(),
            workingSnap: None,
            tip: 0,
            nextTip: 1,
        }
    }

    pub fn new_boxed() -> Box<dyn super::KmoAtomStore::KmoAtomStoreTrait + Send> {
        Box::new(Self::new())
    }

    // Java: KmoTipStamp getCurrentTipStamp()
    pub fn getCurrentTipStamp(&self) -> Box<dyn super::KmoAtomStore::KmoTipStamp> {
      // Java: return new KmoTipStampImpl(this.tip());
      Box::new(KmoTipStampImpl { tip: self.tip() })
    }

    // Java: boolean isTheSameTipStamp(KmoTipStamp ts)
    pub fn isTheSameTipStamp(&self, ts: &dyn super::KmoAtomStore::KmoTipStamp) -> bool {
      // Java: return ts != null && ((KmoTipStampImpl)ts).tip == this.tip();
      ts.tip() == self.tip()
    }

    // Java: int tip()
    pub fn tip(&self) -> i32 {
      // Java: return this.tip;
      self.tip
    }

    // Java: int nextTip()
    pub fn nextTip(&self) -> i32 {
      // Java: return this.nextTip;
      self.nextTip
    }

    // Java: int maxSteps()
    pub fn maxSteps(&self) -> i32 {
      // Java: return this.snapshotStack.capability();
      self.snapshotStack.capability()
    }

    // Java: void setMaxSteps(int maxSteps)
    pub fn setMaxSteps(&mut self, maxSteps: i32) {
      // Java: if (maxSteps > 0)
      if maxSteps > 0
      {
         // Java: this.snapshotStack.setCapability(maxSteps);
         self.snapshotStack.setCapability(maxSteps);
      }
    }

    // Java: void start()
    pub fn start(&mut self) {
      // Java: if (!disable)
      if !is_disable()
      {
         // Java: this.tip = this.nextTip++;
         self.tip = self.nextTip;
         super::KmoAtomStore::publish_tip(self.tip);
         self.nextTip += 1;
         // Java: this.workingSnap = new AtomSnapshot();
         self.workingSnap = Some(AtomSnapshot::new(self.tip));
      }
    }

    // Java: void snapshot()
    pub fn snapshot(&mut self) {
      // Java: if (!disable)
      if !is_disable()
      {
         // Java: this.tip = this.nextTip++;
         self.tip = self.nextTip;
         super::KmoAtomStore::publish_tip(self.tip);
         self.nextTip += 1;
         // Java: if (this.canRedo())
         if self.canRedo()
         {
            // Java: this.snapshotStack.cleanupAhead();
            self.snapshotStack.cleanupAhead();
         }
         // Java: this.snapshotStack.forward();
         self.snapshotStack.forward();
      }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: if (!disable)
      if !is_disable()
      {
         // Java: if (canReset)
         if is_can_reset()
         {
            // Java: this.workingSnap = null;
            self.workingSnap = None;
            // Java: this.snapshotStack.reset();
            self.snapshotStack.reset();
         }
      }
    }

    // Java: boolean canUndo()
    pub fn canUndo(&self) -> bool {
      // Java: return !this.snapshotStack.isEmpty();
      !self.snapshotStack.isEmpty()
    }

    // Java: void undo()
    pub fn undo(&mut self) {
      // Java: if (!disable)
      if !is_disable()
      {
         // Java: if (this.canUndo())
         if self.canUndo()
         {
            // Java: this.snapshotStack.top().swap(true);
            self.snapshotStack.top().swap(true);
            // Java: this.snapshotStack.backward();
            self.snapshotStack.backward();
            // Java: this.tip = this.snapshotStack.isEmpty() ? 0 : this.snapshotStack.top().tip();
            self.tip = if self.snapshotStack.isEmpty() { 0 } else { self.snapshotStack.top().tip() };
            super::KmoAtomStore::publish_tip(self.tip);
         }
      }
    }

    // Java: void undo(int nextTip)
    pub fn undo_at(&mut self, nextTip: i32) {
      // Java: if (!disable)
      if !is_disable()
      {
         // Java: if (this.canUndo() && this.snapshotStack.hasTip(nextTip))
         if self.canUndo() && self.snapshotStack.hasTip(nextTip)
         {
            // Java: while(this.snapshotStack.top().tip() != nextTip)
            while self.snapshotStack.top().tip() != nextTip
            {
               // Java: this.snapshotStack.backward();
               self.snapshotStack.backward();
               // Java: this.snapshotStack.removeTop();
               self.snapshotStack.removeTop();
            }
            // Java: this.undo();
            self.undo();
            // Java: this.snapshotStack.removeTop();
            self.snapshotStack.removeTop();
         }
      }
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: if (!disable)
      if !is_disable()
      {
         // Java: if (this.workingSnap == null)
         if self.workingSnap.is_none()
         {
            // Java: assert false;
            debug_assert!(false, "workingSnap should not be None in discard");
         }
         // Java: else
         else
         {
            // Java: this.workingSnap.swap(true);
            self.workingSnap.as_mut().unwrap().swap(true);
            // Java: this.workingSnap = null;
            self.workingSnap = None;
            // Java: this.tip = this.snapshotStack.isEmpty() ? 0 : this.snapshotStack.top().tip();
            self.tip = if self.snapshotStack.isEmpty() { 0 } else { self.snapshotStack.top().tip() };
            super::KmoAtomStore::publish_tip(self.tip);
         }
      }
    }

    // Java: boolean commit()
    pub fn commit(&mut self) -> bool {
      // Java: System.out.println("transaction disable = " + disable);
      println!("transaction disable = {}", is_disable());
      // Java: if (disable)
      if is_disable()
      {
         // Java: return false;
         return false;
      }
      // Java: else if (this.workingSnap == null)
      else if self.workingSnap.is_none()
      {
         // Java: assert false;
         debug_assert!(false, "workingSnap should not be None in commit");
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: assert this.workingSnap != null;
         debug_assert!(self.workingSnap.is_some());
         // Java: if (this.workingSnap.isEmpty())
         if self.workingSnap.as_ref().unwrap().isEmpty()
         {
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: if (this.canRedo())
            if self.canRedo()
            {
               // Java: this.snapshotStack.cleanupAhead();
               self.snapshotStack.cleanupAhead();
            }
            // Java: this.snapshotStack.forwardBySnap(this.workingSnap);
            let snap = self.workingSnap.take().unwrap();
            self.snapshotStack.forwardBySnap(snap);
            // Java: this.workingSnap = null;
            // (already taken above)
            // Java: return true;
            return true;
         }
      }
    }

    // Java: boolean canRedo()
    pub fn canRedo(&self) -> bool {
      // Java: return this.snapshotStack.snapshotsAhead() > 0;
      self.snapshotStack.snapshotsAhead() > 0
    }

    // Java: void redo()
    pub fn redo(&mut self) {
      // Java: if (!disable)
      if !is_disable()
      {
         // Java: if (this.canRedo())
         if self.canRedo()
         {
            // Java: this.snapshotStack.forward();
            self.snapshotStack.forward();
            // Java: this.snapshotStack.top().swap(false);
            self.snapshotStack.top().swap(false);
            // Java: this.tip = this.snapshotStack.top().tip();
            self.tip = self.snapshotStack.top().tip();
            super::KmoAtomStore::publish_tip(self.tip);
         }
      }
    }

    // Java: KmoAtomData needCopy(KmoAtom<? extends KmoAtomData> atom, KmoAtomData data)
    pub fn needCopy(&mut self, atom: SharedAtom, data: Box<dyn super::AtomSnapshot::KmoAtomData>) -> Box<dyn super::AtomSnapshot::KmoAtomData> {
      // Java: if (disable)
      if is_disable()
      {
         // Java: return data;
         return data;
      }
      // Java: else if (data.tip() < this.tip() && this.workingSnap != null)
      else if data.tip() < self.tip() && self.workingSnap.is_some()
      {
         // Java: this.workingSnap.append(atom, data);
         let snap = self.workingSnap.as_mut().unwrap();
         snap.append(atom, data);
         // Java: data = data.copy();
         // Java: return data;
         let last_data = &snap.atomImages.last().unwrap().data;
         last_data.copy()
      }
      // Java: else
      else
      {
         // Java: return data;
         data
      }
    }

    // Java: static AtomStoreImpl sharedStoreImpl()
    pub fn sharedStoreImpl() -> std::sync::MutexGuard<'static, Option<Box<dyn super::KmoAtomStore::KmoAtomStoreTrait + Send>>> {
      // Java: return (AtomStoreImpl)sharedStore();
      super::KmoAtomStore::KmoAtomStore::sharedStore()
    }

    // Java: boolean isAtomStarted()
    pub fn isAtomStarted(&self) -> bool {
      // Java: return this.workingSnap != null;
      self.workingSnap.is_some()
    }

}

struct KmoTipStampImpl {
    tip: i32,
}

impl super::KmoAtomStore::KmoTipStamp for KmoTipStampImpl {
    fn tip(&self) -> i32 {
        self.tip
    }
}

impl super::KmoAtomStore::KmoAtomStoreTrait for AtomStoreImpl {
    fn reset(&mut self) { self.reset(); }
    fn maxSteps(&self) -> i32 { self.maxSteps() }
    fn setMaxSteps(&mut self, var1: i32) { self.setMaxSteps(var1); }
    fn snapshot(&mut self) { self.snapshot(); }
    fn canUndo(&self) -> bool { self.canUndo() }
    fn undo(&mut self) { self.undo(); }
    fn start(&mut self) { self.start(); }
    fn discard(&mut self) { self.discard(); }
    fn isAtomStarted(&self) -> bool { self.isAtomStarted() }
    fn getCurrentTipStamp(&self) -> Box<dyn super::KmoAtomStore::KmoTipStamp> { self.getCurrentTipStamp() }
    fn isTheSameTipStamp(&self, var1: &dyn super::KmoAtomStore::KmoTipStamp) -> bool { self.isTheSameTipStamp(var1) }
    fn commit(&mut self) -> bool { self.commit() }
    fn canRedo(&self) -> bool { self.canRedo() }
    fn redo(&mut self) { self.redo(); }
    fn undo_at(&mut self, var1: i32) { self.undo_at(var1); }
    fn nextTip(&self) -> i32 { self.nextTip() }
    fn tip(&self) -> i32 { self.tip() }
    fn needCopy(&mut self, atom: SharedAtom, data: Box<dyn super::AtomSnapshot::KmoAtomData>) -> Box<dyn super::AtomSnapshot::KmoAtomData> {
        self.needCopy(atom, data)
    }
}
