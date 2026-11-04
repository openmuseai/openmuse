// java package cn.wps.et.ss.calcchain;

use crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use crate::java_runtime::{Rc, RefCell};
use std::collections::HashMap;
use std::thread::{self, ThreadId};

// java public class ThreadCalcStack
pub struct ThreadCalcStack {
    // java private HashMap<Thread, Stack<IEvalFormulaBase>> stackMap = new HashMap();
    stackMap: RefCell<HashMap<ThreadId, Vec<Rc<dyn IEvalFormulaBase>>>>,
}

impl ThreadCalcStack {
    // java implicit public ThreadCalcStack()
    pub fn new() -> Self {
        Self {
            stackMap: RefCell::new(HashMap::new()),
        }
    }

    // java public void reSet()
    pub fn reSet(&self) {
        // java this.stackMap.clear();
        self.stackMap.borrow_mut().clear();
    }

    // java public boolean checkCycl(IEvalFormulaBase fml)
    pub fn checkCycl(&self, fml: &Rc<dyn IEvalFormulaBase>) -> bool {
        // java IEvalFormulaBase ref = fml.getRef(); if (ref == null) return false;
        let Some(mut reference) = fml.getRef() else {
            return false;
        };
        // java else if (ref.isCycled()) { this.cyclFml(fml); return true; }
        if reference.isCycled() {
            self.cyclFml(fml);
            return true;
        }
        // java while (ref != fml) { if ((ref = ref.getRef()) == null) return false; }
        while !Rc::ptr_eq(&reference, fml) {
            let Some(next) = reference.getRef() else {
                return false;
            };
            reference = next;
        }
        // java this.cyclFml(fml); ref = fml.getRef(); fml.setRef(null);
        self.cyclFml(fml);
        let mut reference = fml.getRef();
        fml.setRef(None);
        // java while (ref != null && ref != fml) { this.cyclFml(ref); ref = ref.getRef(); }
        while let Some(current) = reference {
            if Rc::ptr_eq(&current, fml) {
                break;
            }
            self.cyclFml(&current);
            reference = current.getRef();
        }
        // java return true;
        true
    }

    // java public void add(IEvalFormulaBase fml)
    pub fn add(&self, fml: &Rc<dyn IEvalFormulaBase>) {
        // java get(Thread.currentThread()), initialize under synchronized(ThreadCalcStack.class), push(fml).
        self.stackMap
            .borrow_mut()
            .entry(thread::current().id())
            .or_default()
            .push(Rc::clone(fml));
    }

    // java public void pop()
    pub fn pop(&self) {
        // java Stack stack = stackMap.get(Thread.currentThread()); if (stack != null && !stack.isEmpty()) stack.pop();
        if let Some(stack) = self.stackMap.borrow_mut().get_mut(&thread::current().id()) {
            stack.pop();
        }
    }

    // java public void addRef(IEvalFormulaBase fml)
    pub fn addRef(&self, fml: &Rc<dyn IEvalFormulaBase>) {
        self.addRef__nullable(Some(Rc::clone(fml)));
    }

    // rust: Java permits null in addRef; preserve stack.peek().setRef(null).
    pub fn addRef__nullable(&self, fml: Option<Rc<dyn IEvalFormulaBase>>) {
        // java if (stack != null && !stack.isEmpty()) stack.peek().setRef(fml);
        let current = self
            .stackMap
            .borrow()
            .get(&thread::current().id())
            .and_then(|stack| stack.last())
            .cloned();
        if let Some(current) = current {
            current.setRef(fml);
        }
    }

    // java public void cyclFml(IEvalFormulaBase fml)
    pub fn cyclFml(&self, fml: &Rc<dyn IEvalFormulaBase>) {
        // java fml.setZeroResultIfBlank(); fml.setNeedRecalculationFlag(false); fml.setCycled(true);
        fml.setZeroResultIfBlank();
        fml.setNeedRecalculationFlag(false);
        fml.setCycled(true);
    }
}
