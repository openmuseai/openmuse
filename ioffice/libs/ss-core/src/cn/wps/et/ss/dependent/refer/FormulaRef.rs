use crate::java_runtime::Rc;

// java import cn.wps.et.ss.interfaces.IEvalFormulaBase;
use crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;

// rust: ownership adapter for Java ArrayList<IEvalFormulaBase>. Java list
// removal calls IEvalFormulaBase.equals(Object), while Rc compares identity.
#[derive(Clone)]
pub struct FormulaRef(pub Rc<dyn IEvalFormulaBase>);

impl PartialEq for FormulaRef {
    // java ArrayList.indexOf/remove(Object): o.equals(element).
    fn eq(&self, other: &Self) -> bool { self.0.equals(other.0.as_ref()) }
}

impl Eq for FormulaRef {}

// rust: Java's `catch (Throwable)` around selected registration branches.
// An unfinished Rust snapshot branch must remain visible to the caller.
pub(crate) fn catch_java_throwable(f: impl FnOnce()) {
    if let Err(payload) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        let deferred = payload.downcast_ref::<&str>().map(|s| s.contains("DEFERRED_NON_FUNCTION")).unwrap_or(false)
            || payload.downcast_ref::<String>().map(|s| s.contains("DEFERRED_NON_FUNCTION")).unwrap_or(false);
        if deferred { std::panic::resume_unwind(payload); }
        // java catch(Throwable e) { IOLog.d(TAG, "Throwable", e); }
    }
}
