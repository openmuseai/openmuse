// Java: import cn.wps.core.runtime.IClassLoaderManager
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.util.ArtMode
// Java: import cn.wps.moffice.util.KSLog

use std::sync::{OnceLock, Mutex};

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;
// Java field: IOnlineSecurityAPI sAPIImpl
static S_API_IMPL: OnceLock<Mutex<Option<Box<dyn std::any::Any + Send>>>> = OnceLock::new();
// Java field: IOnlineSecurityChecker sChecker
static S_CHECKER: OnceLock<Mutex<Option<Box<dyn std::any::Any + Send>>>> = OnceLock::new();

fn get_sChecker() -> &'static Mutex<Option<Box<dyn std::any::Any + Send>>> {
    S_CHECKER.get_or_init(|| Mutex::new(None))
}

fn get_sAPIImpl() -> &'static Mutex<Option<Box<dyn std::any::Any + Send>>> {
    S_API_IMPL.get_or_init(|| Mutex::new(None))
}

// Java: public class OnlineSecurityAPIHelper
pub struct OnlineSecurityAPIHelper {
}

// Java: public interface IOnlineSecurityChecker
pub trait IOnlineSecurityChecker {
    // Java: abstract boolean checkIfCanUseOnlineSecurity()
    fn checkIfCanUseOnlineSecurity(&self) -> bool;

    // Java: abstract boolean checkIfCanShowOnlineSecurity()
    fn checkIfCanShowOnlineSecurity(&self) -> bool;
}

// Java: public class OnlineSecurityAPIHelper
impl OnlineSecurityAPIHelper {
    // Java: OnlineSecurityAPIHelper()
    pub fn new() -> Self {
      // Java: super();
      OnlineSecurityAPIHelper {}
    }

    // Java: static void setOnlineSecurityChecker(IOnlineSecurityChecker checker)
    pub fn setOnlineSecurityChecker(_checker: Box<dyn std::any::Any + Send>) {
      // Java: sChecker = checker;
      let mut guard = get_sChecker().lock().unwrap();
      *guard = Some(_checker);
    }

    // Java: static boolean checkIfCanUseOnlineSecurity()
    pub fn checkIfCanUseOnlineSecurity() -> bool {
      // Java: return null == sChecker ? false : sChecker.checkIfCanUseOnlineSecurity();
      todo!()
    }

    // Java: static boolean checkIfCanShowOnlineSecurity()
    pub fn checkIfCanShowOnlineSecurity() -> bool {
      // Java: return null == sChecker ? false : sChecker.checkIfCanShowOnlineSecurity();
      todo!()
    }

    // Java: static IOnlineSecurityAPI getImpl()
    pub fn getImpl() -> Option<Box<dyn std::any::Any>> {
      // Java: if (null == sAPIImpl)
      // Java: synchronized(OnlineSecurityAPIHelper.class)
      // Java: if (null == sAPIImpl)
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      // Java: cl = IClassLoaderManager.getInstance().getSecureDocClassLoader();
      // Java: else
      // Java: cl = OnlineSecurityAPIHelper.class.getClassLoader();
      // Java: try
      // Java: Class<?> clz = cl.loadClass("cn.wps.moffice.online.security.impl.OnlineSecurityAPIImpl");
      // Java: sAPIImpl = (IOnlineSecurityAPI)clz.newInstance();
      // Java: catch (ClassNotFoundException e)
      // Java: KSLog.e(TAG, "ClassNotFoundException", e);
      // Java: catch (InstantiationException e)
      // Java: KSLog.e(TAG, "InstantiationException", e);
      // Java: catch (IllegalAccessException e)
      // Java: KSLog.e(TAG, "IllegalAccessException", e);
      // Java: return sAPIImpl;
      todo!()
    }

}
