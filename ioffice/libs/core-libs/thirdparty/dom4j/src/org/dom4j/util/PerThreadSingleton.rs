// Java: import java.lang.ref.WeakReference
use crate::org::dom4j::util::SingletonStrategy::SingletonStrategy;

use std::any::Any;
use std::sync::Arc;

type Object = Arc<dyn Any + Send + Sync>;

// Java: public class PerThreadSingleton implements SingletonStrategy
pub struct PerThreadSingleton {
    // Java field: String singletonClassName
    pub(crate) singletonClassName: Option<String>,
    // Java field: ThreadLocal perThreadCache
    pub(crate) cached: Option<Object>,
}

// Java: public class PerThreadSingleton implements SingletonStrategy
impl PerThreadSingleton {
    // Java: PerThreadSingleton()
    pub fn new() -> Self {
        // Java: super();
        PerThreadSingleton {
            singletonClassName: None,
            cached: None,
        }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.perThreadCache = new ThreadLocal();
        self.cached = None;
    }

    // Java: Object instance()
    pub fn instance(&self) -> Option<&Object> {
        // Java: Object singletonInstancePerThread = null;
        // Java: WeakReference ref = (WeakReference)this.perThreadCache.get();
        // Java: if (ref != null && ref.get() != null)
        {
            // Java: singletonInstancePerThread = ref.get();
        }
        // Java: else
        {
            // Java: Class clazz = null;
            // Java: try
            {
                // Java: clazz = Thread.currentThread().getContextClassLoader().loadClass(this.singletonClassName);
                // Java: singletonInstancePerThread = clazz.newInstance();
            }
            // Java: catch (Exception var7)
            {
                // Java: try
                {
                    // Java: clazz = Class.forName(this.singletonClassName);
                    // Java: singletonInstancePerThread = clazz.newInstance();
                }
                // Java: catch (Exception var6)
                {
                }
            }
            // Java: this.perThreadCache.set(new WeakReference(singletonInstancePerThread));
        }
        // Java: return singletonInstancePerThread;
        self.cached.as_ref()
    }

    // Java: void setSingletonClassName(String singletonClassName)
    pub fn setSingletonClassName(&mut self, class_name: String) {
        // Java: this.singletonClassName = singletonClassName;
        self.singletonClassName = Some(class_name);
    }
}

impl SingletonStrategy for PerThreadSingleton {
    fn instance(&self) -> Box<dyn Any> {
        Box::new(())
    }

    fn reset(&mut self) {
        self.cached = None;
    }

    fn setSingletonClassName(&mut self, var1: &str) {
        self.singletonClassName = Some(var1.to_string());
    }
}
