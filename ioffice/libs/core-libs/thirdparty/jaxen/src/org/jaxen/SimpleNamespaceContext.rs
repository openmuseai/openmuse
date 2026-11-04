// Java: import java.io.Serializable
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Iterator
// Java: import java.util.Map
use super::NamespaceContext::NamespaceContext;
use super::Navigator::Navigator;
use super::UnsupportedAxisException::UnsupportedAxisException;
use super::Object;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -808928409643497762;

// Java: public class SimpleNamespaceContext implements NamespaceContext, Serializable
pub struct SimpleNamespaceContext {
    // Java field: Map namespaces
    pub(crate) namespaces: HashMap<String, String>,
}

// Java: public class SimpleNamespaceContext implements NamespaceContext, Serializable
impl SimpleNamespaceContext {
    // Java: SimpleNamespaceContext()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.namespaces = new HashMap();
      SimpleNamespaceContext {
          namespaces: HashMap::new(),
      }
    }

    // Java: SimpleNamespaceContext(Map namespaces)
    pub fn new_with_map(namespaces: HashMap<String, String>) -> Self {
      // Java: super();
      // Java: for(Object obj : namespaces.entrySet())
      // Java: Map.Entry entry = (Map.Entry)obj;
      // Java: if (!(entry.getKey() instanceof String) || !(entry.getValue() instanceof String))
      // Java: throw new ClassCastException("Non-string namespace binding");
      // Java: this.namespaces = new HashMap(namespaces);
      SimpleNamespaceContext {
          namespaces,
      }
    }

    // Java: void addElementNamespaces(Navigator nav, Object element)
    pub fn addElementNamespaces(&mut self, nav: &dyn Navigator, element: &Object) -> Result<(), UnsupportedAxisException> {
      // Java: Iterator namespaceAxis = nav.getNamespaceAxisIterator(element);
      let mut namespace_axis = nav.getNamespaceAxisIterator(element)?;
      // Java: while(namespaceAxis.hasNext())
      while let Some(namespace) = namespace_axis.next() {
         // Java: Object namespace = namespaceAxis.next();
         // Java: String prefix = nav.getNamespacePrefix(namespace);
         let prefix = nav.getNamespacePrefix(&namespace);
         // Java: String uri = nav.getNamespaceStringValue(namespace);
         let uri = nav.getNamespaceStringValue(&namespace);
         // Java: if (this.translateNamespacePrefixToUri(prefix) == null)
         if self.translateNamespacePrefixToUri(&prefix).is_empty() {
            // Java: this.addNamespace(prefix, uri);
            self.addNamespace(&prefix, &uri);
         }
      }
      Ok(())
    }

    // Java: void addNamespace(String prefix, String URI)
    pub fn addNamespace(&mut self, prefix: &str, uri: &str) {
      // Java: this.namespaces.put(prefix, URI);
      self.namespaces.insert(prefix.to_string(), uri.to_string());
    }

    // Java: String translateNamespacePrefixToUri(String prefix)
    pub fn translateNamespacePrefixToUri_impl(&self, prefix: &str) -> String {
      // Java: return this.namespaces.containsKey(prefix) ? (String)this.namespaces.get(prefix) : null;
      self.namespaces.get(prefix).cloned().unwrap_or_default()
    }
}

impl NamespaceContext for SimpleNamespaceContext {
    fn translateNamespacePrefixToUri(&self, var1: &str) -> String {
        self.translateNamespacePrefixToUri_impl(var1)
    }
}
