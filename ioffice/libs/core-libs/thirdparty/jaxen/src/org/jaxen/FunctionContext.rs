use super::function::Function;
use super::JaxenException::JaxenException;

// Java: public interface FunctionContext
pub trait FunctionContext {
    // Java: abstract Function getFunction(String var1, String var2, String var3)
    fn getFunction(&self, var1: &str, var2: &str, var3: &str) -> Result<Box<dyn Function>, JaxenException>;
}
