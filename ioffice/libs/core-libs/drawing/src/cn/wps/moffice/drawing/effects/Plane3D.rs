#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports)]

use super::super::PropBase::PropBase;
use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class Plane3D extends PropBase
pub struct Plane3D {
    pub(crate) base: PropBase,
}

// Java: public class Plane3D extends PropBase
impl Plane3D {
    // Java: Plane3D()
    pub fn new() -> Self {
      // Java: super();
      Plane3D { base: PropBase::new() }
    }

    // Java: void setAnchor(Point3D point)
    pub fn setAnchor(&mut self, point: Element) {
      // Java: this.mProperty.setValue(398, point);
      self.base.setValue(398, point);
    }

    // Java: Point3D getAnchor()
    pub fn getAnchor(&mut self) -> Element {
      // Java: Object val = this.mProperty.getValue(398);
      let val = self.base.getValue(398);
      // Java: return val != null ? (Point3D)val : null;
      val
    }

    // Java: void setUp(Vector3D vec)
    pub fn setUp(&mut self, vec: Element) {
      // Java: this.mProperty.setValue(396, vec);
      self.base.setValue(396, vec);
    }

    // Java: Vector3D getUp()
    pub fn getUp(&mut self) -> Element {
      // Java: Object val = this.mProperty.getValue(396);
      let val = self.base.getValue(396);
      // Java: return val != null ? (Vector3D)val : null;
      val
    }

    // Java: void setNormal(Vector3D vec)
    pub fn setNormal(&mut self, vec: Element) {
      // Java: this.mProperty.setValue(397, vec);
      self.base.setValue(397, vec);
    }

    // Java: Vector3D getNormal()
    pub fn getNormal(&mut self) -> Element {
      // Java: Object val = this.mProperty.getValue(397);
      let val = self.base.getValue(397);
      // Java: return val != null ? (Vector3D)val : null;
      val
    }

}
