// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;

// java public interface INotifyTextRope
pub trait INotifyTextRope: ITextRope {
    // java  Listener getListener()
    fn getListener(&self) -> Option<&dyn Listener>; /* soft: nullable Listener */

    // java  void setListener(Listener var1)
    fn setListener(&mut self, var1: Option<Box<dyn Listener>>); /* soft: nullable Listener */

}

// java public interface Listener
pub trait Listener {
    // java  void beforeInsertText(int var1, int var2, int var3)
    fn beforeInsertText(&mut self, var1: i32, var2: i32, var3: i32);

    // java  void afterInsertText(int var1, int var2, int var3)
    fn afterInsertText(&mut self, var1: i32, var2: i32, var3: i32);

    // java  void beforeRemoveText(int var1, int var2)
    fn beforeRemoveText(&mut self, var1: i32, var2: i32);

    // java  void afterRemoveText(int var1, int var2)
    fn afterRemoveText(&mut self, var1: i32, var2: i32);

    // Java: TextRopeListeners.add(Listener)
    fn addListener(&mut self, _listener: Box<dyn Listener>) {}
}
