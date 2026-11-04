use super::{List, Object};
use super::Context::Context;
use super::FunctionCallException::FunctionCallException;

// Java: public interface Function
pub trait Function {
    // Java: abstract Object call(Context var1, List var2)
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException>;
}
