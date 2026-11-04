use std::any::Any;
use super::SingletonStrategy::SingletonStrategy;

// Java: public class SimpleSingleton implements SingletonStrategy
pub struct SimpleSingleton {
    // Java field: String singletonClassName
    singletonClassName: Option<String>,
    // Java field: Object singletonInstance
    pub(crate) singletonInstance: Option<Box<dyn Any>>,
}

// Java: public class SimpleSingleton implements SingletonStrategy
impl SimpleSingleton {
    // Java: SimpleSingleton()
    pub fn new() -> Self {
        // Java: super();
        SimpleSingleton {
            singletonClassName: None,
            singletonInstance: None,
        }
    }
}

impl SingletonStrategy for SimpleSingleton {
    // Java: Object instance()
    fn instance(&self) -> Box<dyn Any> {
        // Java: return this.singletonInstance;
        match &self.singletonInstance {
            Some(_inst) => {
                Box::new(())
            }
            None => Box::new(()),
        }
    }

    // Java: void reset()
    fn reset(&mut self) {
        // Java: if (this.singletonClassName != null)
        if self.singletonClassName.is_some()
        {
            // Java: Class<?> clazz = null;
            // Java: try
            // Java: clazz = Thread.currentThread().getContextClassLoader().loadClass(this.singletonClassName);
            // Java: this.singletonInstance = clazz.newInstance();
            // Java: catch (Exception var5)
            // Java: try
            // Java: clazz = Class.forName(this.singletonClassName);
            // Java: this.singletonInstance = clazz.newInstance();
            // Java: catch (Exception var4)
            self.singletonInstance = None;
        }
    }

    // Java: void setSingletonClassName(String singletonClassName)
    fn setSingletonClassName(&mut self, singletonClassName: &str) {
        // Java: this.singletonClassName = singletonClassName;
        self.singletonClassName = Some(singletonClassName.to_string());
        // Java: this.reset();
        self.reset();
    }
}
