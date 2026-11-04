// Java: import java.io.Serializable
// Java: import org.dom4j.Document
// Java: import org.dom4j.Element
// Java: import org.dom4j.Namespace
// Java: import org.dom4j.Node
// Java: import org.jaxen.NamespaceContext
use core_libs_jaxen::org::jaxen::NamespaceContext::NamespaceContext;

// Java: public class DefaultNamespaceContext implements NamespaceContext, Serializable
pub struct DefaultNamespaceContext {
    // Java field: Element element
    pub(crate) prefix_uri_map: std::collections::HashMap<String, String>,
}

// Java: public class DefaultNamespaceContext implements NamespaceContext, Serializable
impl DefaultNamespaceContext {
    // Java: DefaultNamespaceContext(Element element)
    pub fn new() -> Self {
        // Java: super();
        // Java: this.element = element;
        DefaultNamespaceContext {
            prefix_uri_map: std::collections::HashMap::new(),
        }
    }

    // Java: static DefaultNamespaceContext create(Object node)
    pub fn create() -> Option<DefaultNamespaceContext> {
        // Java: Element element = null;
        // Java: if (node instanceof Element)
        {
            // Java: element = (Element)node;
        }
        // Java: else if (node instanceof Document)
        {
            // Java: Document doc = (Document)node;
            // Java: element = doc.getRootElement();
        }
        // Java: else if (node instanceof Node)
        {
            // Java: element = ((Node)node).getParent();
        }
        // Java: return element != null ? new DefaultNamespaceContext(element) : null;
        Some(DefaultNamespaceContext::new())
    }

    // Java: String translateNamespacePrefixToUri(String prefix)
    pub fn translate_prefix(&self, prefix: &str) -> Option<String> {
        // Java: if (prefix != null && prefix.length() > 0)
        if !prefix.is_empty()
        {
            // Java: Namespace ns = this.element.getNamespaceForPrefix(prefix);
            // Java: if (ns != null)
            if let Some(uri) = self.prefix_uri_map.get(prefix)
            {
                // Java: return ns.getURI();
                return Some(uri.clone());
            }
        }
        // Java: return null;
        None
    }
}

impl NamespaceContext for DefaultNamespaceContext {
    fn translateNamespacePrefixToUri(&self, prefix: &str) -> String {
        self.translate_prefix(prefix).unwrap_or_default()
    }
}
