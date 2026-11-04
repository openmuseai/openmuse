// java package cn.wps.et.ss.calcchain.listener;
use super::ICalculationListener::ICalculationListener;
use crate::java_runtime::Cell;
use std::sync::Arc;
// java public final class ListenerAdapter
pub struct ListenerAdapter {
    listener: Arc<dyn ICalculationListener>,
    started: Cell<bool>,
    total: Cell<i32>,
    current: Cell<i32>,
}
impl ListenerAdapter {
    // java this.listener=l;
    pub fn ListenerAdapter(l: Arc<dyn ICalculationListener>) -> Self {
        Self {
            listener: l,
            started: Cell::new(false),
            total: Cell::new(0),
            current: Cell::new(0),
        }
    }
    // java if(!started){listener.notifyBegin(100);started=true;} total=totalCells;current=0;
    pub fn beginCalculation(&self, total: i32) {
        if !self.started.get() {
            self.listener.notifyBegin(100);
            self.started.set(true);
        }
        self.total.set(total);
        self.current.set(0);
    }
    // java ++current; listener.notifyProcess((int)(20.0+80.0*current/total));
    pub fn cellCalculated(&self) {
        let n = self.current.get().wrapping_add(1);
        self.current.set(n);
        self.listener
            .notifyProcess((20.0 + 80.0 * n as f64 / self.total.get() as f64) as i32);
    }
    // java listener.notifyFinished();
    pub fn finishCalculation(&self) {
        self.listener.notifyFinished();
    }
}
