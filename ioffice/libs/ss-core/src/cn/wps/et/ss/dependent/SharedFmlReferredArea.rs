// java package cn.wps.et.ss.dependent;

use crate::java_runtime::Rc;
use crate::cn::wps::et::ss::interfaces::IEvalSharedFormula::IEvalSharedFormula;
use crate::cn::wps::et::ss::util::Area::Area;

// java public class SharedFmlReferredArea
pub struct SharedFmlReferredArea {
    // java public final IEvalSharedFormula fml;
    pub fml: Rc<dyn IEvalSharedFormula>,
    // java public final Area marked;
    pub marked: Area,
}

impl SharedFmlReferredArea {
    // java public SharedFmlReferredArea(IEvalSharedFormula fml, Area marked)
    pub fn SharedFmlReferredArea(fml: Rc<dyn IEvalSharedFormula>, marked: Area) -> Self {
        Self { fml, marked }
    }

    // java public boolean equals(Object o)
    pub fn equals(&self, other: &Self) -> bool {
        self.fml.equals(other.fml.as_ref()) && self.marked.equals(&other.marked)
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
        self.fml.hashCode().wrapping_mul(71).wrapping_add(self.marked.hashCode())
    }
}
