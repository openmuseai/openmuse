pub mod DefaultNamespaceContext;
pub mod XPathPattern;

// Java: import java.util.List
// Java: import java.util.Map
use std::collections::HashMap;
use std::any::Any;
use std::sync::Arc;
// Java: import org.jaxen.FunctionContext
use core_libs_jaxen::org::jaxen::FunctionContext::FunctionContext;
// Java: import org.jaxen.NamespaceContext
use core_libs_jaxen::org::jaxen::NamespaceContext::NamespaceContext;
// Java: import org.jaxen.VariableContext
use core_libs_jaxen::org::jaxen::VariableContext::VariableContext;

type Object = Arc<dyn Any + Send + Sync>;
type List = Vec<Object>;

// Java: public interface XPath extends NodeFilter
pub trait XPath {
    // Java: abstract String getText()
    fn getText(&self) -> String;

    // Java: abstract boolean matches(Node var1)
    fn matches_node(&self, var1: Object) -> bool;

    // Java: abstract Object evaluate(Object var1)
    fn evaluate(&self, var1: Object) -> Option<Object>;

    // Java: abstract Object selectObject(Object var1)
    fn selectObject(&self, var1: Object) -> Option<Object>;

    // Java: abstract List<? extends Node> selectNodes(Object var1)
    fn selectNodes(&self, var1: Object) -> List;

    // Java: abstract List<? extends Node> selectNodes(Object var1, XPath var2)
    fn selectNodes_sorted(&self, var1: Object, var2: &dyn XPath) -> List;

    // Java: abstract List<? extends Node> selectNodes(Object var1, XPath var2, boolean var3)
    fn selectNodes_sorted_distinct(&self, var1: Object, var2: &dyn XPath, var3: bool) -> List;

    // Java: abstract Node selectSingleNode(Object var1)
    fn selectSingleNode(&self, var1: Object) -> Option<Object>;

    // Java: abstract String valueOf(Object var1)
    fn valueOf(&self, var1: Object) -> String;

    // Java: abstract Number numberValueOf(Object var1)
    fn numberValueOf(&self, var1: Object) -> Option<f64>;

    // Java: abstract boolean booleanValueOf(Object var1)
    fn booleanValueOf(&self, var1: Object) -> bool;

    // Java: abstract void sort(List<? extends Node> var1)
    fn sort(&self, var1: &mut List);

    // Java: abstract void sort(List<? extends Node> var1, boolean var2)
    fn sort_distinct(&self, var1: &mut List, var2: bool);

    // Java: abstract FunctionContext getFunctionContext()
    fn getFunctionContext(&self) -> Option<&dyn FunctionContext>;

    // Java: abstract void setFunctionContext(FunctionContext var1)
    fn setFunctionContext(&mut self, var1: Box<dyn FunctionContext>);

    // Java: abstract NamespaceContext getNamespaceContext()
    fn getNamespaceContext(&self) -> Option<&dyn NamespaceContext>;

    // Java: abstract void setNamespaceContext(NamespaceContext var1)
    fn setNamespaceContext(&mut self, var1: Box<dyn NamespaceContext>);

    // Java: abstract void setNamespaceURIs(Map<String, String> var1)
    fn setNamespaceURIs(&mut self, var1: HashMap<String, String>);

    // Java: abstract VariableContext getVariableContext()
    fn getVariableContext(&self) -> Option<&dyn VariableContext>;

    // Java: abstract void setVariableContext(VariableContext var1)
    fn setVariableContext(&mut self, var1: Box<dyn VariableContext>);
}
