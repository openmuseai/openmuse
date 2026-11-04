// java package cn.wps.moss.app;
use crate::java_runtime::Rc;

// java public interface KmoTransaction {
pub trait KmoTransaction: Send + Sync {
    // java void start();
    fn start(&self);
    // java void discard();
    fn discard(&self);
    // java void commit();
    fn commit(&self);
    // java void reset();
    fn reset(&self);
    // java boolean isTransStarted();
    fn isTransStarted(&self) -> bool;
    // java void setEnable(boolean var1);
    fn setEnable(&self, enable: bool);
    // java void addTransListener(KmoTransactionListener var1);
    fn addTransListener(&self, listener: Option<Rc<dyn KmoTransactionListener>>);
    // java void removeTransListener(KmoTransactionListener var1);
    fn removeTransListener(&self, listener: Option<&Rc<dyn KmoTransactionListener>>);
    // java void addTransactionListener(int var1, KmoSheetTransactionListener var2);
    fn addTransactionListener(
        &self,
        sheetUid: i32,
        listener: Option<Rc<dyn KmoSheetTransactionListener>>,
    );
}
// java public interface KmoSheetTransactionListener { void onCommit(); }
pub trait KmoSheetTransactionListener: Send + Sync {
    fn onCommit(&self);
}
// java public interface KmoTransactionListener { void onStarted(); void onCommited(); }
pub trait KmoTransactionListener: Send + Sync {
    fn onStarted(&self);
    fn onCommited(&self);
}
