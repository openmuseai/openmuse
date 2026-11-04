#![allow(non_snake_case)]

// Java: import cn.wps.moffice.drawing.PropBase
use crate::cn::wps::moffice::drawing::PropBase::PropBase;

use std::ops::Deref;

// Java: public class SoftEdge extends PropBase
pub struct SoftEdge {
    pub base: PropBase,
}

impl Deref for SoftEdge {
    type Target = PropBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class SoftEdge extends PropBase
impl SoftEdge {
    // Java: SoftEdge()
    pub fn new() -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: super() creates parent PropBase, merged into struct init
        SoftEdge {
            base: PropBase::new(),
        }
    }

    // Java: SoftEdge clone()
    pub fn clone(&mut self) -> Result<SoftEdge, String> {
        // Java: return (SoftEdge)super.clone();
        Ok(SoftEdge {
            base: self.base.clone(),
        })
    }

    // Java: void setRad(int size)
    pub fn setRad(&mut self, size: i32) {
        // Java: this.mProperty.setInt(283, size);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setInt(283, size);
        }
    }

    // Java: int getRad()
    pub fn getRad(&self) -> i32 {
        // Java: return this.mProperty.getInt(283, 0);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getInt_default(283, 0)
        } else {
            0
        }
    }
}
