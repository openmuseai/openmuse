#![allow(non_snake_case)]
#![allow(dead_code)]

// Java: import java.util.ArrayList
// Vec is in Rust prelude
use std::sync::{Arc, Mutex};

// rust: Java AtomImage.atom retains the original object reference. The Arc is
// the Rust ownership handle for that same object, not a cloned atom value.
pub type SharedAtom = Arc<Mutex<Box<dyn KmoAtom>>>;

pub trait KmoAtomData: Send {
    fn tip(&self) -> i32;
    fn copy(&self) -> Box<dyn KmoAtomData>;
    // rust: recover Java's concrete KmoAtomData subclass after type erasure.
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub trait KmoAtom: Send {
    fn willSwap(&mut self, isUndo: bool);
    fn swapData(&mut self, data: &mut Box<dyn KmoAtomData>);
    fn didFinishSwapping(&mut self, isUndo: bool);
    fn isNeedSupportNonSequence(&self) -> bool;
    fn cloneAtom(&self) -> SharedAtom;
}

// Java: public class AtomSnapshot
pub struct AtomSnapshot {
    // Java field: int tip
    pub(crate) tip: i32,
    // Java field: ArrayList<AtomImage> atomImages
    pub(crate) atomImages: Vec<AtomImage>,
}

// Java: public class AtomImage
pub struct AtomImage {
    // Java field: KmoAtom<? extends KmoAtomData> atom
    pub(crate) atom: SharedAtom,
    // Java field: KmoAtomData data
    pub(crate) data: Box<dyn KmoAtomData>,
}

// Java: public class AtomSnapshot
impl AtomSnapshot {
    pub fn new(tip: i32) -> Self {
        AtomSnapshot {
            tip,
            atomImages: Vec::new(),
        }
    }

    // Java: void append(KmoAtom<? extends KmoAtomData> atom, KmoAtomData data)
    pub fn append(&mut self, atom: SharedAtom, data: Box<dyn KmoAtomData>) {
      // Java: AtomImage image = new AtomImage();
      // Java: image.atom = atom;
      // Java: image.data = data;
      let image = AtomImage { atom, data };
      // Java: this.atomImages.add(image);
      self.atomImages.push(image);
    }

    // Java: void swap(boolean isUndo)
    pub fn swap(&mut self, isUndo: bool) {
      // Java: if (!isUndo)
      if !isUndo
      {
         // Java: for(AtomImage image : this.atomImages)
         for image in &mut self.atomImages
         {
            // Java: image.atom.willSwap(isUndo);
            image.atom.lock().unwrap().willSwap(isUndo);
            // Java: image.data = image.atom.swapData(image.data);
            image.atom.lock().unwrap().swapData(&mut image.data);
            // Java: image.atom.didFinishSwapping(isUndo);
            image.atom.lock().unwrap().didFinishSwapping(isUndo);
         }
      }
      // Java: else
      else
      {
         // Java: for(int i = this.atomImages.size() - 1; i >= 0; --i)
         for i in (0..self.atomImages.len()).rev()
         {
            // Java: AtomImage image = (AtomImage)this.atomImages.get(i);
            let image = &mut self.atomImages[i];
            // Java: image.atom.willSwap(isUndo);
            image.atom.lock().unwrap().willSwap(isUndo);
            // Java: image.data = image.atom.swapData(image.data);
            image.atom.lock().unwrap().swapData(&mut image.data);
            // Java: image.atom.didFinishSwapping(isUndo);
            image.atom.lock().unwrap().didFinishSwapping(isUndo);
         }
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.atomImages.clear();
      self.atomImages.clear();
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.atomImages.isEmpty();
      self.atomImages.is_empty()
    }

    // Java: int tip()
    pub fn tip(&self) -> i32 {
      // Java: return this.tip;
      self.tip
    }

    // Java: void beforeCommit()
    pub fn beforeCommit(&mut self) {
      // Java: try
      {
         // Java: for(int i = this.atomImages.size() - 1; i >= 0; --i)
         for i in (0..self.atomImages.len()).rev()
         {
            // Java: AtomImage atomImage = (AtomImage)this.atomImages.get(i);
            // Java: if (atomImage != null && atomImage.atom != null && atomImage.atom.isNeedSupportNonSequence())
            if self.atomImages[i].atom.lock().unwrap().isNeedSupportNonSequence()
            {
               // Java: atomImage.atom = (KmoAtom)atomImage.atom.clone();
               let cloned = self.atomImages[i].atom.lock().unwrap().cloneAtom();
               self.atomImages[i].atom = cloned;
            }
         }
      }
      // Java: catch (CloneNotSupportedException e)
      {
         // Java: e.printStackTrace();
      }
    }

}

// Java: public class AtomImage
impl AtomImage {
}
