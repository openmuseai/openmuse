// java package cn.wps.moss.engine.areaevent;

use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

// java import cn.wps.moss.app.KmoSheet;
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
// java import cn.wps.moss.util.KmoRect;
use crate::cn::wps::moss::util::KmoRect::KmoRect;

// java public class AreaWatcher {
pub struct AreaWatcher {
    // java private final KmoUndoableRTree<Entry> listeners = new KmoUndoableRTree<Entry>();
    // Rust's content-event lookup traverses entries; the intersection predicate is the same.
    listeners: Vec<Entry>,
    // java private final ArrayList<IAreaEventSendListener> eventSendListeners = new ArrayList();
    eventSendListeners: Vec<Rc<RefCell<dyn IAreaEventSendListener>>>,
    // java private volatile int nested = 0;
    nested: i32,
}

// java public interface INameChangeListener {
pub trait INameChangeListener: Send + Sync {
    // java void onSheetNameChanged(KmoSheet var1);
    fn onSheetNameChanged(&mut self, sheet: &KmoSheet);
}

// java public interface IAreaChangeListener extends INameChangeListener {
pub trait IAreaChangeListener: INameChangeListener {
    // java void onContentChanged();
    fn onContentChanged(&mut self);
    // java void onAreaMoved(KmoSheet var1, KmoRect var2);
    fn onAreaMoved(&mut self, sheet: &KmoSheet, rect: &KmoRect);
    // java void onAreaDeleted();
    fn onAreaDeleted(&mut self);
}

// java public interface IAreaEventSendListener {
pub trait IAreaEventSendListener: Send + Sync {
    // java void onSendEventFinished();
    fn onSendEventFinished(&mut self);
}

// java static class Entry extends KmoAtom<EntryData> {
struct Entry {
    sheetUid: i32,
    rect: KmoRect,
    listener: Rc<RefCell<dyn IAreaChangeListener>>,
    // java EventState state;
    content: bool,
}

impl AreaWatcher {
    // java private AreaWatcher areaWatcher = new AreaWatcher();
    pub fn new() -> Self {
        Self { listeners: Vec::new(), eventSendListeners: Vec::new(), nested: 0 }
    }

    // java public synchronized void registerListener(KmoSheet sheet, KmoRect rect, IAreaChangeListener l)
    pub fn registerListener(&mut self, sheet: &KmoSheet, rect: &KmoRect, listener: Rc<RefCell<dyn IAreaChangeListener>>) {
        // java Entry e = new Entry(sheet, rect, l); this.listeners.insert(key, e);
        self.listeners.push(Entry { sheetUid: sheet.getSheetUid(), rect: rect.clone(), listener, content: false });
    }

    // java public synchronized boolean unregisterListener(KmoSheet sheet, KmoRect rect, IAreaChangeListener l)
    pub fn unregisterListener(&mut self, sheet: &KmoSheet, rect: &KmoRect, listener: &Rc<RefCell<dyn IAreaChangeListener>>) -> bool {
        // java find entry by sheet, rect and listener identity, then remove it.
        let at = self.listeners.iter().position(|e| e.sheetUid == sheet.getSheetUid() && e.rect == *rect && Rc::ptr_eq(&e.listener, listener));
        if let Some(at) = at { self.listeners.remove(at); true } else { false }
    }

    // java public synchronized void addSendEventListener(IAreaEventSendListener listener)
    pub fn addSendEventListener(&mut self, listener: Rc<RefCell<dyn IAreaEventSendListener>>) {
        // java if (!this.eventSendListeners.contains(listener)) this.eventSendListeners.add(listener);
        if !self.eventSendListeners.iter().any(|item| Rc::ptr_eq(item, &listener)) { self.eventSendListeners.push(listener); }
    }

    // java public synchronized void removeSendEventListener(IAreaEventSendListener listener)
    pub fn removeSendEventListener(&mut self, listener: &Rc<RefCell<dyn IAreaEventSendListener>>) {
        // java this.eventSendListeners.remove(listener);
        self.eventSendListeners.retain(|item| !Rc::ptr_eq(item, listener));
    }

    // java public synchronized void destroy()
    pub fn destroy(&mut self) {
        // java eventSendListeners.clear(); listeners.clear(); deletedEntries.clear(); nested = 0;
        self.eventSendListeners.clear();
        self.listeners.clear();
        self.nested = 0;
    }

    // java public synchronized void startMergeEvents()
    pub fn startMergeEvents(&mut self) {
        // java if (++this.nested < 1) this.nested = 1;
        self.nested = (self.nested + 1).max(1);
    }

    // java public synchronized void finishMergeEvents()
    pub fn finishMergeEvents(&mut self) {
        // java if (--this.nested < 0) this.nested = 0; this.sendAllEvents();
        self.nested = (self.nested - 1).max(0);
        self.sendAllEvents();
    }

    // java public void postContentChangedEvent(KmoSheet sheet, int row, int col)
    pub fn postContentChangedEvent(&mut self, sheet: &KmoSheet, row: i32, col: i32) {
        // java this.postContentChangedEvent(sheet, new KmoRect(row, col, row, col));
        self.postContentChangedEvent__rect(sheet, &KmoRect::KmoRect__bounds(row, col, row, col));
    }

    // java public synchronized void postContentChangedEvent(KmoSheet sheet, KmoRect rect)
    pub fn postContentChangedEvent__rect(&mut self, sheet: &KmoSheet, rect: &KmoRect) {
        // java this.listeners.lookup(makeKey(sheet.getSheetUid(), rect, sv), out);
        for e in &mut self.listeners {
            if e.sheetUid == sheet.getSheetUid() && e.rect.intersects(rect) {
                // java if (e.getState() == EventState.NONE) e.setState(EventState.CONTENT);
                e.content = true;
            }
        }
        // java this.sendAllEvents();
        self.sendAllEvents();
    }

    // java private void sendAllEvents()
    fn sendAllEvents(&mut self) {
        // java if (this.nested == 0)
        if self.nested == 0 {
            for e in &mut self.listeners {
                // java case CONTENT: e.getListener().onContentChanged();
                if e.content { e.listener.borrow_mut().onContentChanged(); }
                // java e.setState(EventState.NONE);
                e.content = false;
            }
            // java this.onEventSend();
            for listener in &self.eventSendListeners { listener.borrow_mut().onSendEventFinished(); }
        }
    }
}
