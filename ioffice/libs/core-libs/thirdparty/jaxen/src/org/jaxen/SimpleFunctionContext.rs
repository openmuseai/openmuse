// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::Arc;
use super::FunctionContext::FunctionContext;
use super::function::Function;
use super::JaxenException::JaxenException;
use super::QualifiedName::QualifiedName;

// Java: public class SimpleFunctionContext implements FunctionContext
pub struct SimpleFunctionContext {
    // Java field: HashMap functions
    functions: HashMap<QualifiedName, Arc<dyn Function + Send + Sync>>,
}

// Java: public class SimpleFunctionContext implements FunctionContext
impl SimpleFunctionContext {
    // Java: SimpleFunctionContext()
    pub fn new() -> Self {
      // Java: super();
      SimpleFunctionContext {
          functions: HashMap::new(),
      }
    }

    // Java: void registerFunction(String namespaceURI, String localName, Function function)
    pub fn registerFunction(&mut self, namespaceURI: &str, localName: &str, function: Arc<dyn Function + Send + Sync>) {
      // Java: this.functions.put(new QualifiedName(namespaceURI, localName), function);
      self.functions.insert(QualifiedName::new(Some(namespaceURI), localName), function);
    }

    // Java: Function getFunction(String namespaceURI, String prefix, String localName)
    pub fn getFunction_impl(&self, namespaceURI: &str, _prefix: &str, localName: &str) -> Result<Box<dyn Function>, JaxenException> {
      // Java: QualifiedName key = new QualifiedName(namespaceURI, localName);
      let key = QualifiedName::new(Some(namespaceURI), localName);
      // Java: if (this.functions.containsKey(key))
      if let Some(func) = self.functions.get(&key) {
         // Java: return (Function)this.functions.get(key);
         Ok(Box::new(ArcFunction(Arc::clone(func))))
      }
      // Java: else
      else {
         // Java: throw new UnresolvableException("No Such Function " + key.getClarkForm());
         Err(JaxenException::new(format!("No Such Function {}", key.getClarkForm())))
      }
    }
}

struct ArcFunction(Arc<dyn Function + Send + Sync>);

impl Function for ArcFunction {
    fn call(&self, var1: &super::Context::Context, var2: &super::List) -> Result<super::Object, super::FunctionCallException::FunctionCallException> {
        self.0.call(var1, var2)
    }
}

impl FunctionContext for SimpleFunctionContext {
    fn getFunction(&self, var1: &str, var2: &str, var3: &str) -> Result<Box<dyn Function>, JaxenException> {
        self.getFunction_impl(var1, var2, var3)
    }
}
