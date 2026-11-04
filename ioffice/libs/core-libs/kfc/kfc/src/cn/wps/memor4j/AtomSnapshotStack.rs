#![allow(non_snake_case)]
#![allow(dead_code)]

// Java: import java.util.LinkedList
use std::collections::VecDeque;
use super::AtomSnapshot::AtomSnapshot;

// Java: public class AtomSnapshotStack
pub struct AtomSnapshotStack {
    // Java field: LinkedList<AtomSnapshot> snapshots
    pub(crate) snapshots: VecDeque<AtomSnapshot>,
    // Java field: int top
    pub(crate) top: i32,
    // Java field: int capability
    pub(crate) capability: i32,
}

// Java: public class AtomSnapshotStack
impl AtomSnapshotStack {
    pub fn new() -> Self {
        AtomSnapshotStack {
            snapshots: VecDeque::new(),
            top: -1,
            capability: 20,
        }
    }

    // Java: int capability()
    pub fn capability(&self) -> i32 {
      // Java: return this.capability;
      self.capability
    }

    // Java: void setCapability(int capability)
    pub fn setCapability(&mut self, capability: i32) {
      // Java: this.capability = capability;
      self.capability = capability;
      // Java: this.recapability();
      self.recapability();
    }

    // Java: void forward()
    pub fn forward(&mut self) {
      // Java: if (this.snapshotsAhead() == 0)
      if self.snapshotsAhead() == 0
      {
         // Java: this.snapshots.add(new AtomSnapshot());
         self.snapshots.push_back(AtomSnapshot::new(0));
      }
      // Java: ++this.top;
      self.top += 1;
      // Java: this.recapability();
      self.recapability();
    }

    // Java: void forwardBySnap(AtomSnapshot snap)
    pub fn forwardBySnap(&mut self, mut snap: AtomSnapshot) {
      // Java: if (this.snapshotsAhead() == 0)
      if self.snapshotsAhead() == 0
      {
         // Java: snap.beforeCommit();
         snap.beforeCommit();
         // Java: this.snapshots.add(snap);
         self.snapshots.push_back(snap);
      }
      // Java: ++this.top;
      self.top += 1;
      // Java: this.recapability();
      self.recapability();
    }

    // Java: void backward()
    pub fn backward(&mut self) {
      // Java: if (this.top > -1)
      if self.top > -1
      {
         // Java: --this.top;
         self.top -= 1;
      }
    }

    // Java: int snapshotsAhead()
    pub fn snapshotsAhead(&self) -> i32 {
      // Java: return this.snapshots.size() - (this.top + 1);
      self.snapshots.len() as i32 - (self.top + 1)
    }

    // Java: void cleanupAhead()
    pub fn cleanupAhead(&mut self) {
      // Java: int size = this.snapshots.size();
      let size = self.snapshots.len() as i32;
      // Java: for(int i = this.top + 1; i < size; ++i)
      for _i in (self.top + 1)..size
      {
         // Java: this.snapshots.remove(this.snapshots.size() - 1);
         self.snapshots.pop_back();
      }
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.top == -1;
      self.top == -1
    }

    // Java: AtomSnapshot top()
    pub fn top(&mut self) -> &mut AtomSnapshot {
      // Java: if (this.snapshots.isEmpty())
      if self.snapshots.is_empty()
      {
         // Java: assert false;
         panic!("AtomSnapshotStack is empty");
         // Java: return new AtomSnapshot();
      }
      // Java: else
      else
      {
         // Java: return (AtomSnapshot)this.snapshots.get(this.top);
         &mut self.snapshots[self.top as usize]
      }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: for(int i = 0; i < this.snapshots.size(); ++i)
      for i in 0..self.snapshots.len()
      {
         // Java: ((AtomSnapshot)this.snapshots.get(i)).clear();
         self.snapshots[i].clear();
      }
      // Java: this.snapshots.clear();
      self.snapshots.clear();
      // Java: this.top = -1;
      self.top = -1;
      // Java: this.capability = 20;
      self.capability = 20;
    }

    // Java: void recapability()
    fn recapability(&mut self) {
      // Java: if (this.top >= this.capability)
      if self.top >= self.capability
      {
         // Java: int count = this.top - this.capability + 1;
         let count = self.top - self.capability + 1;
         // Java: for(int i = 0; i < count; ++i)
         for _i in 0..count
         {
            // Java: AtomSnapshot snapshot = (AtomSnapshot)this.snapshots.get(0);
            // Java: snapshot.clear();
            if let Some(mut snapshot) = self.snapshots.pop_front() {
                snapshot.clear();
            }
            // Java: this.snapshots.remove(0);
            // (handled by pop_front above)
            // Java: --this.top;
            self.top -= 1;
         }
      }
    }

    // Java: void removeTop()
    pub fn removeTop(&mut self) {
      // Java: if (!this.snapshots.isEmpty())
      if !self.snapshots.is_empty()
      {
         // Java: this.snapshots.remove(this.snapshots.size() - 1);
         self.snapshots.pop_back();
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.snapshots.size();
      self.snapshots.len() as i32
    }

    // Java: boolean hasTip(int tip)
    pub fn hasTip(&self, tip: i32) -> bool {
      // Java: for(AtomSnapshot snapshot : this.snapshots)
      for snapshot in &self.snapshots
      {
         // Java: if (tip == snapshot.tip())
         if tip == snapshot.tip()
         {
            // Java: return true;
            return true;
         }
      }
      // Java: return false;
      false
    }

}
