use crate::java_runtime::Cell;
pub struct CfBasedValue{priority:Cell<i32>}impl CfBasedValue{pub fn new()->Self{Self{priority:Cell::new(0)}}
// java public int getPriority() {
// java return this.priority;
// java }
pub fn getPriority(&self)->i32 {
self.priority.get()
}
// java public void setPriority(int priority) {
// java this.priority = priority;
// java }
pub fn setPriority(&self,v:i32) {
self.priority.set(v);
}
}
