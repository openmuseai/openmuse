// java package cn.wps.et.ss.formula.eval;

use super::IValueEval;

// java public final class BlankEval implements IValueEval
pub struct BlankEval { _private: () }
static INSTANCE: BlankEval = BlankEval { _private: () };

impl BlankEval {
    // java public static final BlankEval instance = new BlankEval();
    pub const instance: &'static Self = &INSTANCE;
    // java public boolean equals(Object o) { return o == this; }
    pub fn equals(&self, other: &dyn IValueEval) -> bool {
        other.as_any().downcast_ref::<Self>().is_some_and(|other| std::ptr::eq(self, other))
    }
    // java public int hashCode() { return 17; }
    pub fn hashCode(&self) -> i32 { 17 }
    // java public String toString() { return "BlankEval"; }
    pub fn toString(&self) -> &'static str { "BlankEval" }
}

impl IValueEval for BlankEval {
    fn as_any(&self) -> &dyn std::any::Any { self }
}
