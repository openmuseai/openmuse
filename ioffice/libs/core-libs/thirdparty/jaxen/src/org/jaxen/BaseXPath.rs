use std::sync::Arc;
use crate::org::jaxen::{Object, List};
use crate::org::jaxen::expr::Expr::Expr;
use crate::org::jaxen::expr::XPathExpr::XPathExpr;
use crate::org::jaxen::function::BooleanFunction::BooleanFunction;
use crate::org::jaxen::function::NumberFunction::NumberFunction;
use crate::org::jaxen::function::StringFunction::StringFunction;
use crate::org::jaxen::saxpath::SAXPathException::SAXPathException;
use crate::org::jaxen::saxpath::XPathReader::XPathReader;
use crate::org::jaxen::saxpath::helpers::XPathReaderFactory::XPathReaderFactory;
use crate::org::jaxen::util::SingletonList::SingletonList;
use crate::org::jaxen::Context::Context;
use crate::org::jaxen::ContextSupport::ContextSupport;
use crate::org::jaxen::Navigator::Navigator;
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::NamespaceContext::NamespaceContext;
use crate::org::jaxen::FunctionContext::FunctionContext;
use crate::org::jaxen::VariableContext::VariableContext;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -1993731281300293168;

// Java: public class BaseXPath implements XPath, Serializable
pub struct BaseXPath {
    // Java field: String exprText
    pub(crate) exprText: String,
    // Java field: XPathExpr xpath
    pub(crate) xpath: Option<Box<dyn XPathExpr>>,
    // Java field: ContextSupport support
    support: Option<ContextSupport>,
    // Java field: Navigator navigator
    pub(crate) navigator: Option<Box<dyn Navigator>>,
}

// Java: public class BaseXPath implements XPath, Serializable
impl BaseXPath {
    // Java: BaseXPath(String xpathExpr)
    pub fn new(xpathExpr: String) -> Result<Self, JaxenException> {
        // Java: super();
        // Java: try
        // Java: XPathReader reader = XPathReaderFactory.createReader();
        // Java: JaxenHandler handler = new JaxenHandler();
        // Java: reader.setXPathHandler(handler);
        // Java: reader.parse(xpathExpr);
        // Java: this.xpath = handler.getXPathExpr();
        // Java: catch (org.jaxen.saxpath.XPathSyntaxException e)
        // Java: throw new XPathSyntaxException(e);
        // Java: catch (SAXPathException e)
        // Java: throw new JaxenException(e);
        // Java: this.exprText = xpathExpr;
        Ok(BaseXPath {
            exprText: xpathExpr,
            xpath: None,
            support: None,
            navigator: None,
        })
    }

    // Java: BaseXPath(String xpathExpr, Navigator navigator)
    pub fn new_with_navigator(xpathExpr: String, navigator: Box<dyn Navigator>) -> Result<Self, JaxenException> {
        // Java: this(xpathExpr);
        let mut base = Self::new(xpathExpr)?;
        // Java: this.navigator = navigator;
        base.navigator = Some(navigator);
        Ok(base)
    }

    // Java: Object evaluate(Object context)
    pub fn evaluate(&self, context: Object) -> Result<Object, JaxenException> {
        // Java: List answer = this.selectNodes(context);
        let answer = self.selectNodes(context)?;
        // Java: if (answer != null && answer.size() == 1)
        if answer.len() == 1 {
            // Java: Object first = answer.get(0);
            let first = &answer[0];
            // Java: if (first instanceof String || first instanceof Number || first instanceof Boolean)
            if first.downcast_ref::<String>().is_some()
                || first.downcast_ref::<f64>().is_some()
                || first.downcast_ref::<bool>().is_some()
            {
                // Java: return first;
                return Ok(Arc::clone(first));
            }
        }
        // Java: return answer;
        Ok(Arc::new(answer))
    }

    // Java: List selectNodes(Object node)
    pub fn selectNodes(&self, node: Object) -> Result<List, JaxenException> {
        // Java: Context context = this.getContext(node);
        let context = self.getContext(node);
        // Java: return this.selectNodesForContext(context);
        self.selectNodesForContext(&context)
    }

    // Java: Object selectSingleNode(Object node)
    pub fn selectSingleNode(&self, node: Object) -> Result<Option<Object>, JaxenException> {
        // Java: List results = this.selectNodes(node);
        let results = self.selectNodes(node)?;
        // Java: return results.isEmpty() ? null : results.get(0);
        if results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Arc::clone(&results[0])))
        }
    }

    // Java: String valueOf(Object node)
    pub fn valueOf(&self, node: Object) -> Result<String, JaxenException> {
        // Java: return this.stringValueOf(node);
        self.stringValueOf(node)
    }

    // Java: String stringValueOf(Object node)
    pub fn stringValueOf(&self, node: Object) -> Result<String, JaxenException> {
        // Java: Context context = this.getContext(node);
        let context = self.getContext(node);
        // Java: Object result = this.selectSingleNodeForContext(context);
        let result = self.selectSingleNodeForContext(&context)?;
        // Java: return result == null ? "" : StringFunction.evaluate(result, context.getNavigator());
        match result {
            None => Ok(String::new()),
            Some(r) => Ok(StringFunction::evaluate(&r, context.getNavigator())),
        }
    }

    // Java: boolean booleanValueOf(Object node)
    pub fn booleanValueOf(&self, node: Object) -> Result<bool, JaxenException> {
        // Java: Context context = this.getContext(node);
        let context = self.getContext(node);
        // Java: List result = this.selectNodesForContext(context);
        let result = self.selectNodesForContext(&context)?;
        // Java: return result == null ? false : BooleanFunction.evaluate(result, context.getNavigator());
        let result_obj: Object = Arc::new(result);
        Ok(BooleanFunction::evaluate(&result_obj, context.getNavigator()))
    }

    // Java: Number numberValueOf(Object node)
    pub fn numberValueOf(&self, node: Object) -> Result<f64, JaxenException> {
        // Java: Context context = this.getContext(node);
        let context = self.getContext(node);
        // Java: Object result = this.selectSingleNodeForContext(context);
        let result = self.selectSingleNodeForContext(&context)?;
        // Java: return NumberFunction.evaluate(result, context.getNavigator());
        match result {
            Some(r) => Ok(NumberFunction::evaluate(&r, context.getNavigator())),
            None => Ok(f64::NAN),
        }
    }

    // Java: void addNamespace(String prefix, String uri)
    pub fn addNamespace(&mut self, _prefix: &str, _uri: &str) -> Result<(), JaxenException> {
        // Java: NamespaceContext nsContext = this.getNamespaceContext();
        // Java: if (nsContext instanceof SimpleNamespaceContext)
        // Java: ((SimpleNamespaceContext)nsContext).addNamespace(prefix, uri);
        // Java: else
        // Java: throw new JaxenException("Operation not permitted while using a non-simple namespace context.");
        Err(JaxenException::new("Operation not permitted while using a non-simple namespace context.".to_string()))
    }

    // Java: void setNamespaceContext(NamespaceContext namespaceContext)
    pub fn setNamespaceContext(&mut self, namespaceContext: Box<dyn NamespaceContext>) {
        // Java: this.getContextSupport().setNamespaceContext(namespaceContext);
        self.get_or_create_support().setNamespaceContext(Some(namespaceContext));
    }

    // Java: void setFunctionContext(FunctionContext functionContext)
    pub fn setFunctionContext(&mut self, functionContext: Box<dyn FunctionContext>) {
        // Java: this.getContextSupport().setFunctionContext(functionContext);
        self.get_or_create_support().setFunctionContext(Some(functionContext));
    }

    // Java: void setVariableContext(VariableContext variableContext)
    pub fn setVariableContext(&mut self, variableContext: Box<dyn VariableContext>) {
        // Java: this.getContextSupport().setVariableContext(variableContext);
        self.get_or_create_support().setVariableContext(Some(variableContext));
    }

    // Java: NamespaceContext getNamespaceContext()
    pub fn getNamespaceContext(&self) -> Option<&dyn NamespaceContext> {
        // Java: return this.getContextSupport().getNamespaceContext();
        self.support.as_ref().and_then(|s| s.getNamespaceContext())
    }

    // Java: FunctionContext getFunctionContext()
    pub fn getFunctionContext(&self) -> Option<&dyn FunctionContext> {
        // Java: return this.getContextSupport().getFunctionContext();
        self.support.as_ref().and_then(|s| s.getFunctionContext())
    }

    // Java: VariableContext getVariableContext()
    pub fn getVariableContext(&self) -> Option<&dyn VariableContext> {
        // Java: return this.getContextSupport().getVariableContext();
        self.support.as_ref().and_then(|s| s.getVariableContext())
    }

    // Java: Expr getRootExpr()
    pub fn getRootExpr(&self) -> Option<&dyn Expr> {
        // Java: return this.xpath.getRootExpr();
        self.xpath.as_ref().map(|x| x.getRootExpr())
    }

    // Java: String toString()
    pub fn toString(&self) -> &str {
        // Java: return this.exprText;
        &self.exprText
    }

    // Java: String debug()
    pub fn debug(&self) -> String {
        // Java: return this.xpath.toString();
        self.exprText.clone()
    }

    // Java: Context getContext(Object node)
    pub fn getContext(&self, node: Object) -> Context {
        // Java: if (node instanceof Context)
        // Java: return (Context)node;
        // Java: else
        // Java: Context fullContext = new Context(this.getContextSupport());
        let mut full_context = Context::new(ContextSupport::new());
        // Java: if (node instanceof List)
        if let Some(list) = node.downcast_ref::<Vec<Object>>() {
            // Java: fullContext.setNodeSet((List)node);
            full_context.setNodeSet(list.clone());
        }
        // Java: else
        else {
            // Java: List list = new SingletonList(node);
            // Java: fullContext.setNodeSet(list);
            let list = vec![node];
            full_context.setNodeSet(list);
        }
        // Java: return fullContext;
        full_context
    }

    // Java: ContextSupport getContextSupport()
    fn get_or_create_support(&mut self) -> &mut ContextSupport {
        // Java: if (this.support == null)
        if self.support.is_none() {
            // Java: this.support = new ContextSupport(this.createNamespaceContext(), this.createFunctionContext(), this.createVariableContext(), this.getNavigator());
            self.support = Some(ContextSupport::new());
        }
        // Java: return this.support;
        self.support.as_mut().unwrap()
    }

    // Java: Navigator getNavigator()
    pub fn getNavigator(&self) -> Option<&dyn Navigator> {
        // Java: return this.navigator;
        self.navigator.as_deref()
    }

    // Java: FunctionContext createFunctionContext()
    pub fn createFunctionContext(&self) -> Option<Box<dyn FunctionContext>> {
        // Java: return XPathFunctionContext.getInstance();
        None
    }

    // Java: NamespaceContext createNamespaceContext()
    pub fn createNamespaceContext(&self) -> Option<Box<dyn NamespaceContext>> {
        // Java: return new SimpleNamespaceContext();
        None
    }

    // Java: VariableContext createVariableContext()
    pub fn createVariableContext(&self) -> Option<Box<dyn VariableContext>> {
        // Java: return new SimpleVariableContext();
        None
    }

    // Java: List selectNodesForContext(Context context)
    pub fn selectNodesForContext(&self, context: &Context) -> Result<List, JaxenException> {
        // Java: List list = this.xpath.asList(context);
        if let Some(ref xpath) = self.xpath {
            // Java: return list;
            xpath.asList(context)
        } else {
            Ok(Vec::new())
        }
    }

    // Java: Object selectSingleNodeForContext(Context context)
    pub fn selectSingleNodeForContext(&self, context: &Context) -> Result<Option<Object>, JaxenException> {
        // Java: List results = this.selectNodesForContext(context);
        let results = self.selectNodesForContext(context)?;
        // Java: return results.isEmpty() ? null : results.get(0);
        if results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Arc::clone(&results[0])))
        }
    }
}
