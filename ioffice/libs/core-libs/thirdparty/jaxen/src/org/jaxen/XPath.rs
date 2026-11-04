use super::{List, Object};
use super::JaxenException::JaxenException;
use super::NamespaceContext::NamespaceContext;
use super::FunctionContext::FunctionContext;
use super::VariableContext::VariableContext;
use super::Navigator::Navigator;

// Java: public interface XPath
pub trait XPath {
    // Java: abstract Object evaluate(Object var1)
    fn evaluate(&self, var1: Object) -> Result<Object, JaxenException>;

    // Java: abstract String valueOf(Object var1)
    fn valueOf(&self, var1: Object) -> Result<String, JaxenException>;

    // Java: abstract String stringValueOf(Object var1)
    fn stringValueOf(&self, var1: Object) -> Result<String, JaxenException>;

    // Java: abstract boolean booleanValueOf(Object var1)
    fn booleanValueOf(&self, var1: Object) -> Result<bool, JaxenException>;

    // Java: abstract Number numberValueOf(Object var1)
    fn numberValueOf(&self, var1: Object) -> Result<f64, JaxenException>;

    // Java: abstract List selectNodes(Object var1)
    fn selectNodes(&self, var1: Object) -> Result<List, JaxenException>;

    // Java: abstract Object selectSingleNode(Object var1)
    fn selectSingleNode(&self, var1: Object) -> Result<Object, JaxenException>;

    // Java: abstract void addNamespace(String var1, String var2)
    fn addNamespace(&mut self, var1: &str, var2: &str) -> Result<(), JaxenException>;

    // Java: abstract void setNamespaceContext(NamespaceContext var1)
    fn setNamespaceContext(&mut self, var1: Box<dyn NamespaceContext>);

    // Java: abstract void setFunctionContext(FunctionContext var1)
    fn setFunctionContext(&mut self, var1: Box<dyn FunctionContext>);

    // Java: abstract void setVariableContext(VariableContext var1)
    fn setVariableContext(&mut self, var1: Box<dyn VariableContext>);

    // Java: abstract NamespaceContext getNamespaceContext()
    fn getNamespaceContext(&self) -> &dyn NamespaceContext;

    // Java: abstract FunctionContext getFunctionContext()
    fn getFunctionContext(&self) -> &dyn FunctionContext;

    // Java: abstract VariableContext getVariableContext()
    fn getVariableContext(&self) -> &dyn VariableContext;

    // Java: abstract Navigator getNavigator()
    fn getNavigator(&self) -> &dyn Navigator;
}
