// Java: public class KmoPoint
pub struct KmoPoint {
    // Java field: int x
    pub(crate) x: i32,
    // Java field: int y
    pub(crate) y: i32,
}

// Java: public class KmoPoint
impl KmoPoint {
    // Java: KmoPoint()
    pub fn new() -> Self {
      Self { x: 0, y: 0 }
    }

    // Java: KmoPoint(int x, int y)
    pub fn new_from(x: i32, y: i32) -> Self {
      let mut inst = Self::new();
      inst.KmoPoint_init(x, y);
      inst
    }

    fn KmoPoint_init(&mut self, x: i32, y: i32) {
      // Java: this.copy(x, y);
      // LLM Rust Renamed: copy(int,int) -> copy_xy (overload disambiguation)
      self.copy_xy(x, y);
    }

    // Java: KmoPoint copy(KmoPoint pt)
    pub fn copy(&mut self, pt: &KmoPoint) -> &Self {
      // Java: this.x = pt.x;
      self.x = pt.x;
      // Java: this.y = pt.y;
      self.y = pt.y;
      // Java: return this;
      self
    }

    // Java: KmoPoint copy(int x, int y)
    pub fn copy_xy(&mut self, x: i32, y: i32) -> &Self {
      // Java: this.x = x;
      self.x = x;
      // Java: this.y = y;
      self.y = y;
      // Java: return this;
      self
    }

    // Java: boolean equals(Object rhs)
    pub fn equals(&self, rhs: Option<&KmoPoint>) -> bool {
      // Java: if (rhs == null)
      if rhs.is_none() {
         // Java: return false;
         return false;
      }
      // Java: else if (rhs == this)
      // LLM Rust No Need: reference equality not applicable in Rust
      else if false {
         // Java: return true;
         return true;
      }
      // Java: else if (!KmoPoint.class.isInstance(rhs))
      // LLM Rust No Need: type check not needed, parameter is already typed
      else if false {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: KmoPoint pt = (KmoPoint)rhs;
         let pt = rhs.unwrap();
         // Java: return pt.x == this.x && pt.y == this.y;
         return pt.x == self.x && pt.y == self.y;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.x + this.y;
      self.x + self.y
    }

}
