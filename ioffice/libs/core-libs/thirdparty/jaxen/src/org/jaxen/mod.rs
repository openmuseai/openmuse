use std::any::Any;
use std::sync::Arc;

pub type Object = Arc<dyn Any + Send + Sync>;
pub type List = Vec<Object>;
pub type NodeIterator = Box<dyn Iterator<Item = Object>>;

pub mod FunctionContext;
pub mod NamespaceContext;
pub mod VariableContext;
pub mod Navigator;
pub mod JaxenException;
pub mod FunctionCallException;
pub mod UnsupportedAxisException;
pub mod JaxenRuntimeException;
pub mod Context;
pub mod ContextSupport;
pub mod expr;
pub mod function;
pub mod saxpath;
pub mod util;
pub mod XPath;
pub mod BaseXPath;

pub mod JaxenConstants;
pub mod DefaultNavigator;
pub mod NamedAccessNavigator;
pub mod JaxenHandler;
pub mod QualifiedName;
pub mod SimpleFunctionContext;
pub mod SimpleNamespaceContext;
pub mod SimpleVariableContext;
pub mod UnresolvableException;
pub mod XPathFunctionContext;
pub mod XPathSyntaxException;
pub mod javabean;
pub mod pattern;
#[path = "FunctionTrait.rs"]
pub mod Function;
