#![allow(non_snake_case, dead_code, unused_variables)]

use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: import java.util.ArrayList
// Java: import java.util.List

// Java: public class CoreColor implements Cloneable
pub struct CoreColor {
    // Java field: CoreColorData colorData
    pub(crate) colorData: Element,
    // Java field: List<CoreColorTransform> transforms
    pub(crate) transforms: Vec<Element>,
}

// Java: public class CoreColor implements Cloneable
impl CoreColor {
    // Java: CoreColor()
    pub fn new() -> Self {
      // Java: super();
      CoreColor {
          colorData: None,
          transforms: Vec::new(),
      }
    }

    // Java: CoreColorData getColorData()
    pub fn getColorData(&self) -> &Element {
      // Java: return this.colorData;
      &self.colorData
    }

    // Java: void setColorData(CoreColorData clrData)
    pub fn setColorData(&mut self, clrData: Element) {
      // Java: this.colorData = clrData;
      self.colorData = clrData;
    }

    // Java: void addTransform(CoreColorTransform trans)
    pub fn addTransform(&mut self, trans: Element) {
      // Java: this.transforms.add(trans);
      self.transforms.push(trans);
    }

    // Java: void removeTransform(int index)
    pub fn removeTransform(&mut self, index: i32) {
      // Java: this.transforms.remove(index);
      self.transforms.remove(index as usize);
    }

    // Java: CoreColorTransform getTrans(int index)
    pub fn getTrans(&self, index: i32) -> &Element {
      // Java: return (CoreColorTransform)this.transforms.get(index);
      &self.transforms[index as usize]
    }

    // Java: int transCount()
    pub fn transCount(&self) -> i32 {
      // Java: return this.transforms.size();
      self.transforms.len() as i32
    }

    // Java: CoreColor clone()
    pub fn clone(&self) -> CoreColor {
      // Java: CoreColor clone = new CoreColor();
      let mut clone = CoreColor::new();
      // Java: for(CoreColorTransform colorTransform : this.transforms)
      for colorTransform in &self.transforms
      {
         // Java: clone.addTransform(colorTransform.clone());
         clone.addTransform(colorTransform.clone());
      }
      // Java: return clone;
      clone
    }

}
