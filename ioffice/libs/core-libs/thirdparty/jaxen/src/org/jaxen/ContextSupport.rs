use super::Object;
use super::FunctionContext::FunctionContext;
use super::NamespaceContext::NamespaceContext;
use super::VariableContext::VariableContext;
use super::Navigator::Navigator;
use super::function::Function;
use super::JaxenException::JaxenException;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 4494082174713652559;

// Java: public class ContextSupport implements Serializable
pub struct ContextSupport {
    // Java field: FunctionContext functionContext
    pub(crate) functionContext: Option<Box<dyn FunctionContext>>,
    // Java field: NamespaceContext namespaceContext
    pub(crate) namespaceContext: Option<Box<dyn NamespaceContext>>,
    // Java field: VariableContext variableContext
    pub(crate) variableContext: Option<Box<dyn VariableContext>>,
    // Java field: Navigator navigator
    pub(crate) navigator: Option<Box<dyn Navigator>>,
}

// Java: public class ContextSupport implements Serializable
impl ContextSupport {
    // Java: ContextSupport()
    pub fn new() -> Self {
        // Java: super();
        ContextSupport {
            functionContext: None,
            namespaceContext: None,
            variableContext: None,
            navigator: None,
        }
    }

    // Java: ContextSupport(NamespaceContext namespaceContext, FunctionContext functionContext, VariableContext variableContext, Navigator navigator)
    pub fn new_with_contexts(
        namespaceContext: Option<Box<dyn NamespaceContext>>,
        functionContext: Option<Box<dyn FunctionContext>>,
        variableContext: Option<Box<dyn VariableContext>>,
        navigator: Option<Box<dyn Navigator>>,
    ) -> Self {
        // Java: super();
        // Java: this.setNamespaceContext(namespaceContext);
        // Java: this.setFunctionContext(functionContext);
        // Java: this.setVariableContext(variableContext);
        // Java: this.navigator = navigator;
        ContextSupport {
            functionContext,
            namespaceContext,
            variableContext,
            navigator,
        }
    }

    // Java: void setNamespaceContext(NamespaceContext namespaceContext)
    pub fn setNamespaceContext(&mut self, namespaceContext: Option<Box<dyn NamespaceContext>>) {
        // Java: this.namespaceContext = namespaceContext;
        self.namespaceContext = namespaceContext;
    }

    // Java: NamespaceContext getNamespaceContext()
    pub fn getNamespaceContext(&self) -> Option<&dyn NamespaceContext> {
        // Java: return this.namespaceContext;
        self.namespaceContext.as_deref()
    }

    // Java: void setFunctionContext(FunctionContext functionContext)
    pub fn setFunctionContext(&mut self, functionContext: Option<Box<dyn FunctionContext>>) {
        // Java: this.functionContext = functionContext;
        self.functionContext = functionContext;
    }

    // Java: FunctionContext getFunctionContext()
    pub fn getFunctionContext(&self) -> Option<&dyn FunctionContext> {
        // Java: return this.functionContext;
        self.functionContext.as_deref()
    }

    // Java: void setVariableContext(VariableContext variableContext)
    pub fn setVariableContext(&mut self, variableContext: Option<Box<dyn VariableContext>>) {
        // Java: this.variableContext = variableContext;
        self.variableContext = variableContext;
    }

    // Java: VariableContext getVariableContext()
    pub fn getVariableContext(&self) -> Option<&dyn VariableContext> {
        // Java: return this.variableContext;
        self.variableContext.as_deref()
    }

    // Java: Navigator getNavigator()
    pub fn getNavigator(&self) -> Option<&dyn Navigator> {
        // Java: return this.navigator;
        self.navigator.as_deref()
    }

    // Java: String translateNamespacePrefixToUri(String prefix)
    pub fn translateNamespacePrefixToUri(&self, prefix: &str) -> Option<String> {
        // Java: if ("xml".equals(prefix))
        if prefix == "xml" {
            // Java: return "http://www.w3.org/XML/1998/namespace";
            return Some("http://www.w3.org/XML/1998/namespace".to_string());
        }
        // Java: else
        else {
            // Java: NamespaceContext context = this.getNamespaceContext();
            let context = self.getNamespaceContext();
            // Java: return context != null ? context.translateNamespacePrefixToUri(prefix) : null;
            context.map(|c| c.translateNamespacePrefixToUri(prefix))
        }
    }

    // Java: Object getVariableValue(String namespaceURI, String prefix, String localName)
    pub fn getVariableValue(&self, namespaceURI: &str, prefix: &str, localName: &str) -> Result<Object, JaxenException> {
        // Java: VariableContext context = this.getVariableContext();
        let context = self.getVariableContext();
        // Java: if (context != null)
        if let Some(context) = context {
            // Java: return context.getVariableValue(namespaceURI, prefix, localName);
            context.getVariableValue(namespaceURI, prefix, localName)
        }
        // Java: else
        else {
            // Java: throw new UnresolvableException("No variable context installed");
            Err(JaxenException::new("No variable context installed".to_string()))
        }
    }

    // Java: Function getFunction(String namespaceURI, String prefix, String localName)
    pub fn getFunction(&self, namespaceURI: &str, prefix: &str, localName: &str) -> Result<Box<dyn Function>, JaxenException> {
        // Java: FunctionContext context = this.getFunctionContext();
        let context = self.getFunctionContext();
        // Java: if (context != null)
        if let Some(context) = context {
            // Java: return context.getFunction(namespaceURI, prefix, localName);
            context.getFunction(namespaceURI, prefix, localName)
        }
        // Java: else
        else {
            // Java: throw new UnresolvableException("No function context installed");
            Err(JaxenException::new("No function context installed".to_string()))
        }
    }
}
