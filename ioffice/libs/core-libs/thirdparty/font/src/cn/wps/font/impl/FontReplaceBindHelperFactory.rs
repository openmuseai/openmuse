// Java: import cn.wps.moffice.util.KSLog
use std::sync::{Arc, OnceLock, Mutex};
use super::FontReplaceBindHelper::FontReplaceBindHelper;

// Java: public class FontReplaceBindHelperFactory
pub struct FontReplaceBindHelperFactory {
}

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: FontReplaceBindHelper impl
static IMPL: OnceLock<Mutex<Option<Arc<dyn FontReplaceBindHelper>>>> = OnceLock::new();

fn get_impl_lock() -> &'static Mutex<Option<Arc<dyn FontReplaceBindHelper>>> {
    IMPL.get_or_init(|| Mutex::new(None))
}

// Java: public class FontReplaceBindHelperFactory
impl FontReplaceBindHelperFactory {
    // Java: FontReplaceBindHelperFactory()
    fn new() -> Self {
        // Java: super();
        FontReplaceBindHelperFactory {}
    }

    // Java: static FontReplaceBindHelper getImpl()
    pub fn getImpl() -> Option<Arc<dyn FontReplaceBindHelper>> {
        // Java: if (impl == null)
        let guard = get_impl_lock().lock().unwrap();
        if guard.is_some() {
            return guard.clone();
        }
        drop(guard);
        // Java: synchronized(FontReplaceBindHelperFactory.class)
        {
            let mut guard = get_impl_lock().lock().unwrap();
            // Java: if (impl == null)
            if guard.is_none() {
                // Java: ClassLoader cl = FontReplaceBindHelperFactory.class.getClassLoader();
                // Java: try
                {
                    // Java: Class<?> clz = cl.loadClass("cn.wps.moffice.common.oldfont.guide.FontReplaceBindImpl");
                    // Java: impl = (FontReplaceBindHelper)clz.newInstance();
                }
                // Java: catch (ClassNotFoundException e)
                {
                    // Java: KSLog.e(TAG, "ClassNotFoundException", e);
                }
                // Java: catch (InstantiationException e)
                {
                    // Java: KSLog.e(TAG, "InstantiationException", e);
                }
                // Java: catch (IllegalAccessException e)
                {
                    // Java: KSLog.e(TAG, "IllegalAccessException", e);
                }
            }
            // Java: return impl;
            guard.clone()
        }
    }
}
