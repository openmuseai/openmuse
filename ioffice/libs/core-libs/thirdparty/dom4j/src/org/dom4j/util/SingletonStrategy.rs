use std::any::Any;

// Java: public interface SingletonStrategy
pub trait SingletonStrategy {
    // Java: abstract Object instance()
    fn instance(&self) -> Box<dyn Any>;

    // Java: abstract void reset()
    fn reset(&mut self);

    // Java: abstract void setSingletonClassName(String var1)
    fn setSingletonClassName(&mut self, var1: &str);
}
