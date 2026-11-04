// Java: import cn.wps.util.KeepNotProguard
use util::util::KeepNotProguard::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java @KeepNotProguard public interface IDebugger
pub trait IDebugger {
    // java  void onDebug(String var1, Object... var2)
    fn onDebug(&mut self, var1: &str);

}
