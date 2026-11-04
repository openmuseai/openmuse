#![allow(non_snake_case, dead_code, unused_variables)]

use crate::runtime::ILogFileObserver::ILogFileObserver;

// Java: public class KLogFileObserverUtil
pub struct KLogFileObserverUtil;

// Java: public class KLogFileObserverUtil
impl KLogFileObserverUtil {
    // Java: KLogFileObserverUtil()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: ILogFileObserver newObserver()
    pub fn newObserver(&self) -> Option<Box<dyn ILogFileObserver>> {
        // Java: ILogFileObserver observer = null;
        let mut observer: Option<Box<dyn ILogFileObserver>> = None;
        // Java: ClassLoader cl = null;
        // Java: try
        {
            // Java: cl = this.getClass().getClassLoader();
            // Java: Class<?> mFileObserverClass = cl.loadClass("cn.wps.moffice.KLogFileObserver");
            // Java: Class<?>[] argTypes = new Class[0];
            // Java: Constructor<?> constructor = mFileObserverClass.getConstructor(argTypes);
            // Java: observer = (ILogFileObserver)constructor.newInstance();
            // Java: return observer;
            return observer;
        }
        // Java: catch (Throwable var6)
        // Java: return null;
    }
}
