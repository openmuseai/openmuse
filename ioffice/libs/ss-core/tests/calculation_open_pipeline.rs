use ss_core::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use ss_core::cn::wps::moss::{
    app::{KmoBook::KmoBook, KmoBookApi::KmoBookApi, KmoBooksApi::KmoBooksApi},
    engine::eval::EvaluationPresent::EvaluationPresent,
};
#[test]
fn real_open_initializes_chain_and_async_division_writes_error_seven() {
    let api = KmoBookApi::KmoBookApi();
    let path = format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/formula-cache.xlsx",
        env!("CARGO_MANIFEST_DIR")
    );
    assert_eq!(api.open(&path, None, true), 1);
    let book = api.book();
    let chain = book.getCalculationChain().unwrap();
    assert!(book.isCalcChainInitialized());
    assert!(!book.getReadOnly());
    assert_eq!(chain.getFormulaSize(), 3);
    let sheet = book.sheetAt(0);
    let uid = sheet.borrow().getSheetUid();
    let f = chain.calcChain.getFml(uid, 0, 2).unwrap();
    let token = f.getEncodedTokens().unwrap().to_vec();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let ready = {
            let s = sheet.borrow();
            s.getCellValueType__2(0, 2) == 6
        };
        if ready && !chain.isInCalulation() {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "async open did not write C1"
        );
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    let s = sheet.borrow();
    let id = s.getCellValueId(0, 2);
    assert_eq!(s.getCellValueStore().borrow().errorValue(id), 7);
    assert!(!EvaluationPresent::isNeedRecalculationFlagSet(&s, 0, 2));
    assert_eq!(f.getEncodedTokens().unwrap(), token);
}

use ss_core::cn::wps::moss::util::KmoRect::KmoRect;
use ss_core::cn::wps::moss::{
    app::{
        event::{
            KmoBookOpenEventListener::KmoBookOpenEventListener,
            KmoSheetChangeListenerGroup::KmoSheetChangeEventsListener,
        },
        KmoSheet::KmoSheet,
    },
    engine::areaevent::AreaWatcher::{IAreaChangeListener, INameChangeListener},
};
use ss_core::java_runtime::{Rc, RefCell, Weak};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
};
struct ContentObserver(Rc<AtomicUsize>);
impl INameChangeListener for ContentObserver {
    fn onSheetNameChanged(&mut self, _: &KmoSheet) {}
}
impl IAreaChangeListener for ContentObserver {
    fn onContentChanged(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
    fn onAreaMoved(&mut self, _: &KmoSheet, _: &KmoRect) {}
    fn onAreaDeleted(&mut self) {}
}
struct FinishObserver {
    book: Weak<KmoBook>,
    dirty: Rc<Mutex<Vec<bool>>>,
}
impl KmoSheetChangeEventsListener for FinishObserver {
    fn sheetContentsDidFinishChanging(&self) {
        self.dirty
            .lock()
            .unwrap()
            .push(self.book.upgrade().unwrap().isDirty());
    }
}
struct ReaderObserver {
    book: Weak<KmoBook>,
    dirty: Rc<Mutex<Vec<bool>>>,
    contents: Rc<AtomicUsize>,
    read: Rc<AtomicUsize>,
}
impl KmoBookOpenEventListener for ReaderObserver {
    fn bookDidFinishReadingEverything(&self) {
        let book = self.book.upgrade().unwrap();
        assert!(book.getReadOnly());
        assert!(!book.isCompletelyReaded());
        let sheet = book.sheetAt(0);
        let s = sheet.borrow();
        assert_eq!(s.getCellValueType__2(0, 2), 0);
        s.changeListenerGroup().add(Rc::new(FinishObserver {
            book: self.book.clone(),
            dirty: self.dirty.clone(),
        }));
        book.areaWatcher().borrow_mut().registerListener(
            &s,
            &KmoRect::KmoRect__bounds(0, 2, 0, 2),
            Rc::new(RefCell::new(ContentObserver(self.contents.clone()))),
        );
        self.read.fetch_add(1, Ordering::SeqCst);
    }
}
#[test]
fn reader_callback_precedes_chain_init_then_writeback_merges_events_and_marks_dirty() {
    let book = KmoBook::KmoBook();
    book.init(false);
    let dirty = Rc::new(Mutex::new(Vec::new()));
    let contents = Rc::new(AtomicUsize::new(0));
    let read = Rc::new(AtomicUsize::new(0));
    let listener = Rc::new(ReaderObserver {
        book: Rc::downgrade(&book),
        dirty: dirty.clone(),
        contents: contents.clone(),
        read: read.clone(),
    });
    let path = format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/formula-cache.xlsx",
        env!("CARGO_MANIFEST_DIR")
    );
    KmoBooksApi::openBook__book_listener(&book, &path, Some(listener)).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while dirty.lock().unwrap().len() < 2 {
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    assert_eq!(read.load(Ordering::SeqCst), 1);
    assert_eq!(*dirty.lock().unwrap(), [false, true]);
    assert_eq!(contents.load(Ordering::SeqCst), 1);
    assert!(book.isDirty());
    assert!(!book.getReadOnly());
}
