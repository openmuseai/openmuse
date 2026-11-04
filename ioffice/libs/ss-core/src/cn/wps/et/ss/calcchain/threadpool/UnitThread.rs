// java package cn.wps.et.ss.calcchain.threadpool;
use super::super::{
    listener::UnitThreadCallback::UnitThreadCallback,
    CalcChain::{CalcChain, Node},
};
use crate::java_runtime::{Cell, RefCell};
use std::sync::Arc;
// java public class UnitThread implements Runnable
pub struct UnitThread {
    pub(crate) calcChain: Arc<CalcChain>,
    pub start: RefCell<Option<Node>>,
    pub end: RefCell<Option<Node>>,
    pub meetInterrupt: Cell<bool>,
    shouldContinue: Cell<bool>,
    pub meetBreak: Cell<bool>,
    unitThreadCallback: RefCell<Option<Arc<dyn UnitThreadCallback>>>,
}
impl UnitThread {
    // java this.calcChain=calcChain; this.start=start; this.end=end;
    pub fn UnitThread(chain: Arc<CalcChain>, start: Option<Node>, end: Option<Node>) -> Self {
        Self {
            calcChain: chain,
            start: RefCell::new(start),
            end: RefCell::new(end),
            meetInterrupt: Cell::new(false),
            shouldContinue: Cell::new(false),
            meetBreak: Cell::new(false),
            unitThreadCallback: RefCell::new(None),
        }
    }
    // java synchronized(calcChain.threadPools) { threadPools.add(this); }
    fn addThread(self: &Arc<Self>) {
        self.calcChain
            .threadPools()
            .lock()
            .unwrap()
            .insert(Arc::as_ptr(self) as usize, self.clone());
    }
    // java synchronized(calcChain.threadPools) { threadPools.remove(this); }
    fn removeThread(self: &Arc<Self>) {
        self.calcChain
            .threadPools()
            .lock()
            .unwrap()
            .remove(&(Arc::as_ptr(self) as usize));
    }
    fn startCalculate(&self) {
        // java while(start!=null || end!=null) { calculateUnit(start,end,this); if(!meetInterrupt) break; }
        while self.start.borrow().is_some() || self.end.borrow().is_some() {
            let start = self.start.borrow().clone();
            let end = self.end.borrow().clone();
            self.calcChain.calculateUnit(start, end, self);
            if !self.meetInterrupt.get() {
                break;
            }
        }
    }
    fn searchNewWork(&self) {
        // java synchronized(calcChain.threadPools)
        let pools = self.calcChain.threadPools().lock().unwrap();
        if self.meetBreak.get() {
            self.shouldContinue.set(false);
            return;
        }
        let threads: Vec<_> = pools.values().cloned().collect();
        if threads.is_empty() {
            return;
        }
        let own_index = self.start.borrow().as_ref().map(|n| n.borrow().index);
        let mut index = 0;
        let mut maxDValue = 0;
        for (i, t) in threads.iter().enumerate() {
            if let (Some(s), Some(e)) = (&*t.start.borrow(), &*t.end.borrow()) {
                let si = s.borrow().index;
                let ei = e.borrow().index;
                if own_index.is_none_or(|own| own != si) {
                    let d = ei - si;
                    if maxDValue < d {
                        index = i;
                        maxDValue = d;
                    }
                }
            }
        }
        if maxDValue > 2 {
            let fork = &threads[index];
            let fork_start = fork.start.borrow().as_ref().unwrap().borrow().index;
            let half = (fork.end.borrow().as_ref().unwrap().borrow().index - fork_start) / 2;
            let mut startIndex = fork_start + half + 1;
            let mut start = self.calcChain.nodeAtIndex(startIndex);
            while start.is_none() && startIndex < self.calcChain.maxIndex() {
                startIndex += 1;
                start = self.calcChain.nodeAtIndex(startIndex);
            }
            *self.start.borrow_mut() = start;
            *self.end.borrow_mut() = fork.end.borrow().clone();
            let mut endIndex = fork_start + half;
            let mut end = self.calcChain.nodeAtIndex(endIndex);
            while end.is_none() && endIndex > 0 {
                endIndex -= 1;
                end = self.calcChain.nodeAtIndex(endIndex);
            }
            *fork.end.borrow_mut() = end;
            self.shouldContinue.set(true);
        }
    }
    // java private void start()
    fn start(self: &Arc<Self>) {
        self.shouldContinue.set(true);
        self.meetBreak.set(false);
        loop {
            self.shouldContinue.set(false);
            self.addThread();
            self.startCalculate();
            self.removeThread();
            self.searchNewWork();
            if !self.shouldContinue.get() {
                break;
            }
        }
    }
    // java if(unitThreadCallback!=null) unitThreadCallback.beforeRun(); start();
    pub fn run(self: &Arc<Self>) {
        if let Some(c) = &*self.unitThreadCallback.borrow() {
            c.beforeRun();
        }
        self.start();
    }
    pub fn setUnitThreadCallback(&self, callback: Option<Arc<dyn UnitThreadCallback>>) {
        *self.unitThreadCallback.borrow_mut() = callback;
    }
}
