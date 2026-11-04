use crate::java_runtime::RefCell as JavaRefCell;
use kfc::cn::wps::kfc::numfmt::format::NumFmtResult::NumFmtResult;
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Weak,
    },
};
static NEXT: AtomicU64 = AtomicU64::new(1);
type ResultRef = Arc<JavaRefCell<NumFmtResult>>;
thread_local! {static RESULTS:RefCell<HashMap<u64,(Weak<()>,ResultRef)>>=RefCell::new(HashMap::new());}
// java class KmoSheetSharedObjects
pub struct KmoSheetSharedObjects {
    id: u64,
    owner: Arc<()>,
}
impl KmoSheetSharedObjects {
    pub fn new() -> Self {
        Self {
            id: NEXT.fetch_add(1, Ordering::Relaxed),
            owner: Arc::new(()),
        }
    }
    // java ThreadLocal<NumFmtResult> numFormatResult; initialValue(){return new NumFmtResult();}
    pub fn numFormatResult(&self) -> ResultRef {
        RESULTS.with(|results| {
            let mut results = results.borrow_mut();
            results.retain(|_, (owner, _)| owner.strong_count() > 0);
            results
                .entry(self.id)
                .or_insert_with(|| {
                    (
                        Arc::downgrade(&self.owner),
                        Arc::new(JavaRefCell::new(NumFmtResult::new())),
                    )
                })
                .1
                .clone()
        })
    }
}
