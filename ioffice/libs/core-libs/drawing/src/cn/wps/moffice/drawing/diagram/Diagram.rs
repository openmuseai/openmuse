#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.PropBase
use super::super::PropBase::PropBase;

use std::sync::Arc;
use std::any::Any;
use std::ops::Deref;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class Diagram extends PropBase implements Cloneable
pub struct Diagram {
    pub base: PropBase,
}

pub trait Cloneable {}
impl Cloneable for Diagram {}

impl Deref for Diagram {
    type Target = PropBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Diagram extends PropBase implements Cloneable
impl Diagram {
    // Java: Diagram()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent PropBase, merged into struct init
      Diagram { base: PropBase::new() }
    }

    // Java: Diagram clone()
    pub fn clone(&mut self) -> Result<Diagram, String> {
      // Java: return (Diagram)super.clone();
      Ok(Diagram { base: self.base.clone() })
    }

    // Java: void setDiagramType(int type)
    pub fn setDiagramType(&mut self, r#type: i32) {
      // Java: this.mProperty.setInt(735, type);
      self.base.mProperty.as_mut().unwrap().setInt(735, r#type);
    }

    // Java: int getDiagramType()
    pub fn getDiagramType(&self) -> i32 {
      // Java: return this.mProperty.getInt(735, 4095);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(735, 4095)
    }

    // Java: Integer getDiagramTypeObj()
    pub fn getDiagramTypeObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(735);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(735);
      // Java: return object == null ? null : (Integer)object;
      // LLM JavaLine Aggregation: cast merged into downcast_ref
      let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
      object
    }

}
