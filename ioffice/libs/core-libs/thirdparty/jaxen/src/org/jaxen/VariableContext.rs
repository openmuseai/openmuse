use super::Object;
use super::JaxenException::JaxenException;

// Java: public interface VariableContext
pub trait VariableContext {
    // Java: abstract Object getVariableValue(String var1, String var2, String var3)
    fn getVariableValue(&self, var1: &str, var2: &str, var3: &str) -> Result<Object, JaxenException>;
}
