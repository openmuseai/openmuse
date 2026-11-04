// java package cn.wps.et.ss.formula.eval;

use super::IValueEval;

// java public final class MissingArgEval implements IValueEval
pub struct MissingArgEval { _private: () }
static INSTANCE: MissingArgEval = MissingArgEval { _private: () };

impl MissingArgEval {
    // java public static final MissingArgEval instance = new MissingArgEval();
    pub const instance: &'static Self = &INSTANCE;
}

impl IValueEval for MissingArgEval {
    fn as_any(&self) -> &dyn std::any::Any { self }
}
