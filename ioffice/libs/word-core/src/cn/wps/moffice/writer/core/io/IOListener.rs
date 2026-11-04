// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public interface IOListener
pub trait IOListener {
    // java  void onFirstLock()
    fn onFirstLock(&mut self);

    // java  void onFirstUnLock()
    fn onFirstUnLock(&mut self);

    // java  void onLoadParas(int var1)
    fn onLoadParas(&mut self, var1: i32);

    // java  void onError(int var1, Object var2)
    fn onError(&mut self, var1: i32, var2: &dyn Object);

    // java  void beginLoadOnlineSecurityDoc()
    fn beginLoadOnlineSecurityDoc(&mut self);

    // java  void onFinish()
    fn onFinish(&mut self);

    // java  void onFinishDumpObjects()
    fn onFinishDumpObjects(&mut self);

    // java  void onHtmlOpenError()
    fn onHtmlOpenError(&mut self);

    // java  void onFinishVerifyWritePassword()
    fn onFinishVerifyWritePassword(&mut self);

}
