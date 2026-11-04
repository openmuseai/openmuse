// java package cn.wps.et.ss.formula.evaluator;

use std::sync::{Mutex, atomic::{AtomicI32, Ordering}};
use crate::java_runtime::{Cell, Rc, RefCell};

// java public final class NotCalculatedException extends RuntimeException
pub struct NotCalculatedException {
    // java private int sheetUid;
    sheetUid: Cell<i32>,
    // java private int row;
    row: Cell<i32>,
    // java private int col;
    col: Cell<i32>,
    // java private NotCalculatedException next = null;
    next: RefCell<Option<Rc<NotCalculatedException>>>,
}

// java private static volatile int recycleCount = 0;
static recycleCount: AtomicI32 = AtomicI32::new(0);
// java private static volatile NotCalculatedException recycleList = null;
// The mutex is the shared NotCalculatedException.class monitor.
static recycleList: Mutex<Option<Rc<NotCalculatedException>>> = Mutex::new(None);

impl NotCalculatedException {
    // java private NotCalculatedException(int sheetUid, int row, int col)
    fn new(sheet_uid: i32, row: i32, col: i32) -> Self {
        Self { sheetUid: Cell::new(sheet_uid), row: Cell::new(row), col: Cell::new(col), next: RefCell::new(None) }
    }

    // java public int getSheetUid() { return this.sheetUid; }
    pub fn getSheetUid(&self) -> i32 { self.sheetUid.get() }
    // java public int getRow() { return this.row; }
    pub fn getRow(&self) -> i32 { self.row.get() }
    // java public int getCol() { return this.col; }
    pub fn getCol(&self) -> i32 { self.col.get() }

    // java public static synchronized NotCalculatedException obtain(int sheetUid, int row, int col)
    pub fn obtain(sheet_uid: i32, row: i32, col: i32) -> Rc<Self> {
        let mut list = recycleList.lock().unwrap_or_else(|error| error.into_inner());
        // java if (recycleList != null)
        if let Some(ret) = list.clone() {
            // java NotCalculatedException ret = recycleList; recycleList = ret.next; --recycleCount;
            *list = ret.next.borrow().clone();
            recycleCount.fetch_sub(1, Ordering::SeqCst);
            // java ret.sheetUid = sheetUid; ret.row = row; ret.col = col;
            ret.sheetUid.set(sheet_uid);
            ret.row.set(row);
            ret.col.set(col);
            // java return ret; (ret.next is deliberately not cleared.)
            ret
        } else {
            // java return new NotCalculatedException(sheetUid, row, col);
            Rc::new(Self::new(sheet_uid, row, col))
        }
    }

    // java public void recycle()
    pub fn recycle(self: &Rc<Self>) {
        // java if (recycleCount <= 10), checked before entering the class monitor.
        if recycleCount.load(Ordering::SeqCst) <= 10 {
            // java synchronized(NotCalculatedException.class)
            let mut list = recycleList.lock().unwrap_or_else(|error| error.into_inner());
            // java this.next = recycleList; recycleList = this; ++recycleCount;
            *self.next.borrow_mut() = list.clone();
            *list = Some(Rc::clone(self));
            recycleCount.fetch_add(1, Ordering::SeqCst);
        }
    }
}
