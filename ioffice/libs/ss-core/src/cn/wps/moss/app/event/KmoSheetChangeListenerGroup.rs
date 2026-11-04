// java package cn.wps.moss.app.event;
use crate::cn::wps::et::ss::calcchain::RecalcLock;
use crate::java_runtime::RefCell;
use std::{collections::HashMap, sync::Arc, thread::ThreadId};
// java KmoSheetChangeEventsListener.sheetContentsDidFinishChanging()
pub trait KmoSheetChangeEventsListener: Send + Sync {
    fn sheetDidFinishInitializing(&self){}
    fn sheetContentsDidFinishChanging(&self);
    fn sheetSettingsDidFinishChanging(&self){}
    fn sheetSelectionChanged(&self){}
    fn shapeContentChanged(&self,_id:i32){}
}
struct LocalVar {
    nestedCount: i32,
    sheetDidFinishInitializing:bool,
    sheetContentsDidFinishChanging: bool,
    sheetSettingsDidFinishChanging: bool,
    sheetSelectionChanged:bool,
    sheetShapeContentChanged:i32,
}
impl Default for LocalVar{fn default()->Self{Self{nestedCount:0,sheetDidFinishInitializing:false,sheetContentsDidFinishChanging:false,sheetSettingsDidFinishChanging:false,sheetSelectionChanged:false,sheetShapeContentChanged:-1}}}
// java public final class KmoSheetChangeListenerGroup
pub struct KmoSheetChangeListenerGroup {
    listeners: RefCell<Vec<Arc<dyn KmoSheetChangeEventsListener>>>,
    localVar: RefCell<HashMap<ThreadId, LocalVar>>,
    monitor: RecalcLock,
}
impl KmoSheetChangeListenerGroup {
    pub fn new() -> Self {
        Self {
            listeners: RefCell::new(Vec::new()),
            localVar: RefCell::new(HashMap::new()),
            monitor: RecalcLock::new(),
        }
    }
    // java synchronized add(): if (!listeners.contains(rhs)) listeners.add(rhs);
    pub fn add(&self, rhs: Arc<dyn KmoSheetChangeEventsListener>) {
        let _m = self.monitor.lock();
        let mut l = self.listeners.borrow_mut();
        if !l.iter().any(|v| Arc::ptr_eq(v, &rhs)) {
            l.push(rhs);
        }
    }
    // java synchronized remove(): listeners.remove(rhs);
    pub fn remove(&self, rhs: &Arc<dyn KmoSheetChangeEventsListener>) {
        let _m = self.monitor.lock();
        let mut l = self.listeners.borrow_mut();
        if let Some(i) = l.iter().position(|v| Arc::ptr_eq(v, rhs)) {
            l.remove(i);
        }
    }
    pub fn clear(&self) {
        let _m = self.monitor.lock();
        self.listeners.borrow_mut().clear();
    }
    // java public void sheetDidFinishInitializing(){LocalVar lv=this.localVar.get();if(lv.nestedCount>0){lv.sheetDidFinishInitializing=true;}else{this.sheetDidFinishInitializingNonBlockable();}}
    pub fn sheetDidFinishInitializing(&self){let blocked={let mut vars=self.localVar.borrow_mut();let lv=vars.entry(std::thread::current().id()).or_default();if lv.nestedCount>0{lv.sheetDidFinishInitializing=true;true}else{false}};if !blocked{self.sheetDidFinishInitializingNonBlockable();}}
    // java public synchronized void sheetDidFinishInitializingNonBlockable(){ArrayList<KmoSheetChangeEventsListener> backupListeners=new ArrayList(this.listeners.size());int i=0;for(int size=this.listeners.size();i<size;++i){backupListeners.add(this.listeners.get(i));}i=0;for(int size=backupListeners.size();i<size;++i){this.listeners.get(i).sheetDidFinishInitializing();}}
    pub fn sheetDidFinishInitializingNonBlockable(&self){let _m=self.monitor.lock();let mut backupListeners=Vec::with_capacity(self.listeners.borrow().len());let mut i=0;let size=self.listeners.borrow().len();while i<size{backupListeners.push(self.listeners.borrow()[i].clone());i+=1;}i=0;let size=backupListeners.len();while i<size{let l=self.listeners.borrow().get(i).unwrap_or_else(||panic!("IndexOutOfBoundsException")).clone();l.sheetDidFinishInitializing();i+=1;}}
    // java LocalVar lv = localVar.get(); if (lv.nestedCount > 0) lv.sheetContentsDidFinishChanging=true; else sheetContentsDidFinishChangingNonBlockable();
    pub fn sheetContentsDidFinishChanging(&self) {
        let blocked = {
            let mut vars = self.localVar.borrow_mut();
            let lv = vars.entry(std::thread::current().id()).or_default();
            if lv.nestedCount > 0 {
                lv.sheetContentsDidFinishChanging = true;
                true
            } else {
                false
            }
        };
        if !blocked {
            self.sheetContentsDidFinishChangingNonBlockable();
        }
    }
    // java synchronized for(int i=0,size=listeners.size();i<size;++i) listeners.get(i).sheetContentsDidFinishChanging();
    pub fn sheetContentsDidFinishChangingNonBlockable(&self) {
        let _m = self.monitor.lock();
        let size = self.listeners.borrow().len();
        for i in 0..size {
            let l = self.listeners.borrow()[i].clone();
            l.sheetContentsDidFinishChanging();
        }
    }
// java public void sheetSettingsDidFinishChanging() {
// java LocalVar lv = (LocalVar)this.localVar.get();
// java if (lv.nestedCount > 0) {
// java lv.sheetSettingsDidFinishChanging = true;
// java } else {
// java this.sheetSettingsDidFinishChangingNonBlockable();
// java }
// java }
pub fn sheetSettingsDidFinishChanging(&self){let blocked={let mut vars=self.localVar.borrow_mut();let lv=vars.entry(std::thread::current().id()).or_default();if lv.nestedCount>0{lv.sheetSettingsDidFinishChanging=true;true}else{false}};if !blocked{self.sheetSettingsDidFinishChangingNonBlockable();}}
// java public synchronized void sheetSettingsDidFinishChangingNonBlockable() {
// java int i = 0;
// java
// java for(int size = this.listeners.size(); i < size; ++i) {
// java ((KmoSheetChangeEventsListener)this.listeners.get(i)).sheetSettingsDidFinishChanging();
// java }
// java
// java }
pub fn sheetSettingsDidFinishChangingNonBlockable(&self){let _m=self.monitor.lock();let size=self.listeners.borrow().len();for i in 0..size{let l=self.listeners.borrow()[i].clone();l.sheetSettingsDidFinishChanging();}}
// java public void sheetSelectionChanged() {
// java LocalVar lv = (LocalVar)this.localVar.get();
// java if (lv.nestedCount > 0) {
// java lv.sheetSelectionChanged = true;
// java } else {
// java this.sheetSelectionChangedNonBlockable();
// java }
// java }
pub fn sheetSelectionChanged(&self){let blocked={let mut vars=self.localVar.borrow_mut();let lv=vars.entry(std::thread::current().id()).or_default();if lv.nestedCount>0{lv.sheetSelectionChanged=true;true}else{false}};if !blocked{self.sheetSelectionChangedNonBlockable();}}
// java public synchronized void sheetSelectionChangedNonBlockable() {
// java int i = 0;
// java
// java for(int size = this.listeners.size(); i < size; ++i) {
// java ((KmoSheetChangeEventsListener)this.listeners.get(i)).sheetSelectionChanged();
// java }
// java
// java }
pub fn sheetSelectionChangedNonBlockable(&self){let _m=self.monitor.lock();let size=self.listeners.borrow().len();for i in 0..size{let l=self.listeners.borrow()[i].clone();l.sheetSelectionChanged();}}
    // java public void chartContentChanged(int id){LocalVar lv=this.localVar.get();if(lv.nestedCount>0){lv.sheetShapeContentChanged=id;}else{this.shapeChangedNonBlockable(id);}}
    pub fn chartContentChanged(&self,id:i32){let blocked={let mut vars=self.localVar.borrow_mut();let lv=vars.entry(std::thread::current().id()).or_default();if lv.nestedCount>0{lv.sheetShapeContentChanged=id;true}else{false}};if !blocked{self.shapeChangedNonBlockable(id);}}
    // java public synchronized void shapeChangedNonBlockable(int id){int i=0;for(int size=this.listeners.size();i<size;++i){this.listeners.get(i).shapeContentChanged(id);}}
    pub fn shapeChangedNonBlockable(&self,id:i32){let _m=self.monitor.lock();let mut i=0;let size=self.listeners.borrow().len();while i<size{let l=self.listeners.borrow().get(i).unwrap_or_else(||panic!("IndexOutOfBoundsException")).clone();l.shapeContentChanged(id);i+=1;}}
    // java public void startMergeEvents(){LocalVar lv=this.localVar.get();if(lv.nestedCount<0){lv.nestedCount=1;}else{++lv.nestedCount;}}
    pub fn startMergeEvents(&self){let mut vars=self.localVar.borrow_mut();let lv=vars.entry(std::thread::current().id()).or_default();if lv.nestedCount<0{lv.nestedCount=1;}else{lv.nestedCount=lv.nestedCount.wrapping_add(1);}}
    // java public void finishMergeEvents(){
    pub fn finishMergeEvents(&self){
        let thread=std::thread::current().id();
        // java LocalVar lv=this.localVar.get();if(lv.nestedCount<=0){lv.nestedCount=0;}else{--lv.nestedCount;}
        let nested={let mut vars=self.localVar.borrow_mut();let lv=vars.entry(thread).or_default();if lv.nestedCount<=0{lv.nestedCount=0;}else{lv.nestedCount=lv.nestedCount.wrapping_sub(1);}lv.nestedCount};
        // java if(0==lv.nestedCount){
        if nested==0{
            // java if(lv.sheetDidFinishInitializing){this.sheetDidFinishInitializingNonBlockable();lv.sheetDidFinishInitializing=false;}
            if self.localVar.borrow()[&thread].sheetDidFinishInitializing{self.sheetDidFinishInitializingNonBlockable();self.localVar.borrow_mut().get_mut(&thread).unwrap().sheetDidFinishInitializing=false;}
            // java if(lv.sheetContentsDidFinishChanging){this.sheetContentsDidFinishChangingNonBlockable();lv.sheetContentsDidFinishChanging=false;}
            if self.localVar.borrow()[&thread].sheetContentsDidFinishChanging{self.sheetContentsDidFinishChangingNonBlockable();self.localVar.borrow_mut().get_mut(&thread).unwrap().sheetContentsDidFinishChanging=false;}
            // java if(lv.sheetSettingsDidFinishChanging){this.sheetSettingsDidFinishChangingNonBlockable();lv.sheetSettingsDidFinishChanging=false;}
            if self.localVar.borrow()[&thread].sheetSettingsDidFinishChanging{self.sheetSettingsDidFinishChangingNonBlockable();self.localVar.borrow_mut().get_mut(&thread).unwrap().sheetSettingsDidFinishChanging=false;}
            // java if(lv.sheetSelectionChanged){this.sheetSelectionChangedNonBlockable();lv.sheetSelectionChanged=false;}
            if self.localVar.borrow()[&thread].sheetSelectionChanged{self.sheetSelectionChangedNonBlockable();self.localVar.borrow_mut().get_mut(&thread).unwrap().sheetSelectionChanged=false;}
            // java if(lv.sheetShapeContentChanged>-1){this.shapeChangedNonBlockable(lv.sheetShapeContentChanged);lv.sheetShapeContentChanged=-1;return;}
            let id=self.localVar.borrow()[&thread].sheetShapeContentChanged;if id> -1{self.shapeChangedNonBlockable(id);self.localVar.borrow_mut().get_mut(&thread).unwrap().sheetShapeContentChanged=-1;return;}
        }
    }
}
