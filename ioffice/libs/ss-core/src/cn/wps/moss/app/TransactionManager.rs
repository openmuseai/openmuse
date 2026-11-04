// java package cn.wps.moss.app;
use super::{
    KmoBook::KmoBook,
    KmoTransaction::{KmoSheetTransactionListener, KmoTransaction, KmoTransactionListener},
};
use crate::cn::wps::{et::ss::calcchain::RecalcLock, moss::util::KmoRect::KmoRect};
use crate::java_runtime::{Cell, Rc, RefCell, Weak};
use kfc::cn::wps::memor4j::KmoAtomStore::{KmoAtomStore, KmoAtomStoreTrait};
use std::{
    collections::HashMap,
    panic::{catch_unwind, AssertUnwindSafe},
};

// java class TransactionManager implements KmoTransaction {
pub struct TransactionManager {
    // java private List<Integer> sheetIndexs = new ArrayList();
    sheetIndexs: RefCell<Vec<i32>>,
    // java private List<KmoRect> selections = new ArrayList();
    selections: RefCell<Vec<KmoRect>>,
    // java private int sheetCursor = 0;
    sheetCursor: Cell<i32>,
    // java private boolean isBeginTrans = false;
    isBeginTrans: Cell<bool>,
    // java private HashMap<Integer, List<KmoTransaction.KmoSheetTransactionListener>> listenerMap = new HashMap();
    listenerMap:
        RefCell<HashMap<i32, Rc<RefCell<Vec<Option<Rc<dyn KmoSheetTransactionListener>>>>>>>,
    // java private boolean enable = true;
    enable: Cell<bool>,
    // java KmoBook book;
    book: Weak<KmoBook>,
    // java private List<KmoTransaction.KmoTransactionListener> listeners = new ArrayList();
    listeners: RefCell<Vec<Rc<dyn KmoTransactionListener>>>,
    // rust: Java synchronized reentrant monitor, independent of mutable field guards.
    monitor: RecalcLock,
}
pub(crate) fn withStore<R>(f: impl FnOnce(&mut dyn KmoAtomStoreTrait) -> R) -> R {
    let mut guard = KmoAtomStore::sharedStore();
    f(guard.as_mut().unwrap().as_mut())
}
impl TransactionManager {
    // java TransactionManager(KmoBook book) { this.book = book; }
    pub(crate) fn TransactionManager(book: Weak<KmoBook>) -> Self {
        Self {
            sheetIndexs: RefCell::new(vec![]),
            selections: RefCell::new(vec![]),
            sheetCursor: Cell::new(0),
            isBeginTrans: Cell::new(false),
            listenerMap: RefCell::new(HashMap::new()),
            enable: Cell::new(true),
            book,
            listeners: RefCell::new(vec![]),
            monitor: RecalcLock::new(),
        }
    }
    // java public void setEnable(boolean enable) { this.enable = enable; }
    pub fn setEnable(&self, enable: bool) {
        self.enable.set(enable);
    }
    // java public synchronized void start() {
    pub fn start(&self) {
        let _monitor = self.monitor.lock();
        // java if (this.enable) {
        if self.enable.get() {
            // java this.isBeginTrans = true;
            self.isBeginTrans.set(true);
            // java KmoAtomStore.sharedStore().start();
            withStore(|s| s.start());
            // java this.onTransStarted();
            self.onTransStarted();
        }
    }
    // java public synchronized void discard() {
    pub fn discard(&self) {
        let _monitor = self.monitor.lock();
        // java if (this.enable) { if (this.isBeginTrans) {
        if self.enable.get() && self.isBeginTrans.get() {
            // java KmoAtomStore.sharedStore().discard();
            withStore(|s| s.discard());
            // java this.isBeginTrans = false;
            self.isBeginTrans.set(false);
        }
    }
    // java public synchronized void commit() {
    pub fn commit(&self) {
        let _monitor = self.monitor.lock();
        // java if (this.enable) {
        if self.enable.get() {
            // java this.onSheetTransCommit();
            self.onSheetTransCommit();
            // java if (this.isBeginTrans) {
            if self.isBeginTrans.get() {
                // java if (!KmoAtomStore.sharedStore().commit()) { this.discard(); }
                if !withStore(|s| s.commit()) {
                    self.discard();
                } else {
                    let book = self.book.upgrade().expect("KmoBook");
                    // java int sheetIndex = this.book.whichSheetIsActivated();
                    let sheetIndex = book.whichSheetIsActivated();
                    // java KmoRect selection = new KmoRect(this.book.getActiveSheet().getSelectionRect());
                    let selection = book.getActiveSheet().borrow().getSelectionRect();
                    // java for(int i = this.sheetIndexs.size() - 1; i >= this.sheetCursor; --i) {
                    let size = self.sheetIndexs.borrow().len();
                    for i in (self.sheetCursor.get() as usize..size).rev() {
                        // java this.sheetIndexs.remove(i); this.selections.remove(i);
                        self.sheetIndexs.borrow_mut().remove(i);
                        self.selections.borrow_mut().remove(i);
                    }
                    // java this.sheetIndexs.add(sheetIndex); this.selections.add(selection);
                    self.sheetIndexs.borrow_mut().push(sheetIndex);
                    self.selections.borrow_mut().push(selection);
                    // java ++this.sheetCursor;
                    self.sheetCursor.set(self.sheetCursor.get().wrapping_add(1));
                    // java this.isBeginTrans = false;
                    self.isBeginTrans.set(false);
                    // java this.onTransCommited();
                    self.onTransCommited();
                }
            }
        }
    }
    // java public synchronized void reset() {
    pub fn reset(&self) {
        let _monitor = self.monitor.lock();
        // java this.sheetCursor = 0; this.sheetIndexs.clear(); this.selections.clear();
        self.sheetCursor.set(0);
        self.sheetIndexs.borrow_mut().clear();
        self.selections.borrow_mut().clear();
        // java KmoAtomStore.sharedStore().reset();
        withStore(|s| s.reset());
    }
    // java public int back() {
    pub fn back(&self) -> i32 {
        // java int sheetIndex = (Integer)this.sheetIndexs.get(this.sheetCursor - 1);
        let cursor = self.sheetCursor.get().wrapping_sub(1) as usize;
        let sheetIndex = self.sheetIndexs.borrow()[cursor];
        // java KmoRect selection = (KmoRect)this.selections.get(this.sheetCursor - 1);
        let selection = self.selections.borrow()[cursor].clone();
        // java KmoSheet sheet = this.book.sheetAt(sheetIndex);
        let sheet = self.book.upgrade().unwrap().sheetAt(sheetIndex);
        // java sheet.setSelection(selection, selection.tl.row, selection.tl.col);
        sheet
            .borrow()
            .setSelection__3(&selection, selection.tl.row, selection.tl.col);
        // java --this.sheetCursor;
        self.sheetCursor.set(self.sheetCursor.get().wrapping_sub(1));
        // java return sheetIndex;
        sheetIndex
    }
    // java public int forward() {
    pub fn forward(&self) -> i32 {
        // java int sheetIndex = (Integer)this.sheetIndexs.get(this.sheetCursor);
        let cursor = self.sheetCursor.get() as usize;
        let sheetIndex = self.sheetIndexs.borrow()[cursor];
        // java KmoRect selection = (KmoRect)this.selections.get(this.sheetCursor);
        let selection = self.selections.borrow()[cursor].clone();
        // java KmoSheet sheet = this.book.sheetAt(sheetIndex);
        let sheet = self.book.upgrade().unwrap().sheetAt(sheetIndex);
        // java sheet.setSelection(selection, selection.tl.row, selection.tl.col);
        sheet
            .borrow()
            .setSelection__3(&selection, selection.tl.row, selection.tl.col);
        // java ++this.sheetCursor; return sheetIndex;
        self.sheetCursor.set(self.sheetCursor.get().wrapping_add(1));
        sheetIndex
    }
    // java public boolean isTransStarted() { return this.isBeginTrans; }
    pub fn isTransStarted(&self) -> bool {
        self.isBeginTrans.get()
    }
    // java public void clearListeners() { this.listeners.clear(); }
    pub fn clearListeners(&self) {
        self.listeners.borrow_mut().clear();
    }
    // java public void addTransListener(KmoTransaction.KmoTransactionListener listener) {
    pub fn addTransListener(&self, listener: Option<Rc<dyn KmoTransactionListener>>) {
        // java if (listener != null && !this.listeners.contains(listener)) { this.listeners.add(listener); }
        if let Some(listener) = listener {
            if !self
                .listeners
                .borrow()
                .iter()
                .any(|v| Rc::ptr_eq(v, &listener))
            {
                self.listeners.borrow_mut().push(listener);
            }
        }
    }
    // java public void removeTransListener(KmoTransaction.KmoTransactionListener listener) { this.listeners.remove(listener); }
    pub fn removeTransListener(&self, listener: Option<&Rc<dyn KmoTransactionListener>>) {
        if let Some(listener) = listener {
            let i = self
                .listeners
                .borrow()
                .iter()
                .position(|v| Rc::ptr_eq(v, listener));
            if let Some(i) = i {
                self.listeners.borrow_mut().remove(i);
            }
        }
    }
    // java private void onTransStarted() {
    fn onTransStarted(&self) {
        // java try { for(int size = this.listeners.size(); i < size; ++i) { listeners.get(i).onStarted(); } } catch (Throwable var3) { }
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let size = self.listeners.borrow().len();
            for i in 0..size {
                let listener = self.listeners.borrow()[i].clone();
                listener.onStarted();
            }
        }));
    }
    // java private void onTransCommited() {
    fn onTransCommited(&self) {
        // java try { for(int size = this.listeners.size(); i < size; ++i) { listeners.get(i).onCommited(); } } catch (Throwable var3) { }
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let size = self.listeners.borrow().len();
            for i in 0..size {
                let listener = self.listeners.borrow()[i].clone();
                listener.onCommited();
            }
        }));
    }
    // java private void onSheetTransCommit() {
    fn onSheetTransCommit(&self) {
        // java List<KmoSheetTransactionListener> listeners = listenerMap.get(book.getActiveSheet().getSheetUid());
        let uid = self
            .book
            .upgrade()
            .unwrap()
            .getActiveSheet()
            .borrow()
            .getSheetUid();
        let listeners = self.listenerMap.borrow().get(&uid).cloned();
        // java if (listeners != null) { for(KmoSheetTransactionListener listener : listeners) { listener.onCommit(); } }
        if let Some(listeners) = listeners {
            let expected = listeners.borrow().len();
            let mut i = 0;
            while i != listeners.borrow().len() {
                assert_eq!(
                    expected,
                    listeners.borrow().len(),
                    "ConcurrentModificationException"
                );
                let listener = listeners.borrow()[i]
                    .clone()
                    .expect("NullPointerException transaction listener");
                listener.onCommit();
                i += 1;
            }
        }
    }
    // java public void addTransactionListener(int sheetUid, KmoTransaction.KmoSheetTransactionListener listener) {
    pub fn addTransactionListener(
        &self,
        sheetUid: i32,
        listener: Option<Rc<dyn KmoSheetTransactionListener>>,
    ) {
        // java List<KmoSheetTransactionListener> listeners = listenerMap.get(sheetUid); if (listeners == null) { listeners = new ArrayList(); }
        let listeners = self
            .listenerMap
            .borrow()
            .get(&sheetUid)
            .cloned()
            .unwrap_or_else(|| Rc::new(RefCell::new(vec![])));
        // java listeners.add(listener); this.listenerMap.put(sheetUid, listeners);
        listeners.borrow_mut().push(listener);
        self.listenerMap.borrow_mut().insert(sheetUid, listeners);
    }
}
impl KmoTransaction for TransactionManager {
    fn start(&self) {
        Self::start(self)
    }
    fn discard(&self) {
        Self::discard(self)
    }
    fn commit(&self) {
        Self::commit(self)
    }
    fn reset(&self) {
        Self::reset(self)
    }
    fn isTransStarted(&self) -> bool {
        Self::isTransStarted(self)
    }
    fn setEnable(&self, v: bool) {
        Self::setEnable(self, v)
    }
    fn addTransListener(&self, v: Option<Rc<dyn KmoTransactionListener>>) {
        Self::addTransListener(self, v)
    }
    fn removeTransListener(&self, v: Option<&Rc<dyn KmoTransactionListener>>) {
        Self::removeTransListener(self, v)
    }
    fn addTransactionListener(&self, id: i32, v: Option<Rc<dyn KmoSheetTransactionListener>>) {
        Self::addTransactionListener(self, id, v)
    }
}
