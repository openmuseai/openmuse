use kfc::cn::wps::kfc::numfmt::{INumFormatter::INumFormatterTrait, NumFmtFactory::NumFmtFactory};
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, OnceLock, Weak,
    },
};
static NEXT_APP: AtomicU64 = AtomicU64::new(1);
pub(super) static APP: OnceLock<Arc<KmoApp>> = OnceLock::new();
// Rust adapter for each original LocalNumFormatter ThreadLocal object.
thread_local! {static FORMATTERS:RefCell<HashMap<u64,(Weak<()>,u64,Box<dyn INumFormatterTrait>)>>=RefCell::new(HashMap::new());}
// java public class KmoApp
pub struct KmoApp {
    // java protected KmoBooks books;
    books:crate::java_runtime::RefCell<Option<Arc<super::KmoBooksApi::KmoBooksApi>>>,
    owner: Arc<()>,
    id: u64,
    numberFormatter: AtomicU64,
}
impl KmoApp {
    pub(super) fn new() -> Self {
        Self {
            books:crate::java_runtime::RefCell::new(None),
            owner: Arc::new(()),
            id: NEXT_APP.fetch_add(1, Ordering::Relaxed),
            numberFormatter: AtomicU64::new(0),
        }
    }
    // java public KmoBooks books(){return this.books;}
    pub fn books(&self)->Option<Arc<super::KmoBooksApi::KmoBooksApi>>{self.books.borrow().clone()}
    // java public void setBooks(KmoBooks books){this.books=books;}
    pub fn setBooks(&self,books:Option<Arc<super::KmoBooksApi::KmoBooksApi>>){*self.books.borrow_mut()=books;}
    // java startup(): this.numberFormatter=new LocalNumFormatter();this.numFormatter();KmoSheet.resetConstRecognLang();
    pub fn startup(&self) {
        self.numberFormatter.fetch_add(1, Ordering::SeqCst);
        self.numFormatter(|_| ());
        super::KmoSheet::KmoSheet::resetConstRecognLang();
    }
    // java public INumFormatter numFormatter(): return numberFormatter!=null?numberFormatter.get():null.
    // Rust callback keeps a thread-local Java reference inside its owning thread.
    pub fn numFormatter<R>(&self, f: impl FnOnce(Option<&mut dyn INumFormatterTrait>) -> R) -> R {
        let epoch = self.numberFormatter.load(Ordering::SeqCst);
        if epoch == 0 {
            return f(None);
        }
        FORMATTERS.with(|formatters| {
            let mut formatters = formatters.borrow_mut();
            formatters.retain(|_, (owner, _, _)| owner.strong_count() != 0);
            if formatters.get(&self.id).is_none_or(|(_, e, _)| *e != epoch) {
                // java LocalNumFormatter.initialValue():return NumFmtFactory.createNumFormatter((short)0);
                formatters.insert(
                    self.id,
                    (
                        Arc::downgrade(&self.owner),
                        epoch,
                        NumFmtFactory::createNumFormatter(0),
                    ),
                );
            }
            f(Some(formatters.get_mut(&self.id).unwrap().2.as_mut()))
        })
    }
    // java if(app==null)throw new RuntimeException("You should use KmoAppApi.sharedApp() at the begin of your app.");return app;
    pub fn sharedApp() -> Arc<Self> {
        APP.get()
            .expect("You should use KmoAppApi.sharedApp() at the begin of your app.")
            .clone()
    }
}
