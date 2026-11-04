// Java: import java.io.Serializable
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map
use super::VariableContext::VariableContext;
use super::JaxenException::JaxenException;
use super::QualifiedName::QualifiedName;
use super::Object;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 961322093794516518;

// Java: public class SimpleVariableContext implements VariableContext, Serializable
pub struct SimpleVariableContext {
    // Java field: Map variables
    pub(crate) variables: HashMap<QualifiedName, Object>,
}

// Java: public class SimpleVariableContext implements VariableContext, Serializable
impl SimpleVariableContext {
    // Java: SimpleVariableContext()
    pub fn new() -> Self {
      // Java: super();
      SimpleVariableContext {
          variables: HashMap::new(),
      }
    }

    // Java: void setVariableValue(String namespaceURI, String localName, Object value)
    pub fn setVariableValue(&mut self, namespaceURI: &str, localName: &str, value: Object) {
      // Java: this.variables.put(new QualifiedName(namespaceURI, localName), value);
      self.variables.insert(QualifiedName::new(Some(namespaceURI), localName), value);
    }

    // Java: void setVariableValue(String localName, Object value)
    pub fn setVariableValue_simple(&mut self, localName: &str, value: Object) {
      // Java: this.variables.put(new QualifiedName((String)null, localName), value);
      self.variables.insert(QualifiedName::new(None, localName), value);
    }

    // Java: Object getVariableValue(String namespaceURI, String prefix, String localName)
    pub fn getVariableValue_impl(&self, namespaceURI: &str, _prefix: &str, localName: &str) -> Result<Object, JaxenException> {
      // Java: QualifiedName key = new QualifiedName(namespaceURI, localName);
      let key = QualifiedName::new(Some(namespaceURI), localName);
      // Java: if (this.variables.containsKey(key))
      if let Some(value) = self.variables.get(&key) {
         // Java: return this.variables.get(key);
         Ok(value.clone())
      }
      // Java: else
      else {
         // Java: throw new UnresolvableException("Variable " + key.getClarkForm());
         Err(JaxenException::new(format!("Variable {}", key.getClarkForm())))
      }
    }
}

impl VariableContext for SimpleVariableContext {
    fn getVariableValue(&self, var1: &str, var2: &str, var3: &str) -> Result<Object, JaxenException> {
        self.getVariableValue_impl(var1, var2, var3)
    }
}
