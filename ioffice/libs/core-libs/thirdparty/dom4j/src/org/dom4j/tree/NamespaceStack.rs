// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.dom4j.DocumentFactory
// Java: import org.dom4j.Namespace
// Java: import org.dom4j.QName

use std::collections::HashMap;

// Java: public class NamespaceStack
pub struct NamespaceStack {
    // Java field: DocumentFactory documentFactory
    pub(crate) documentFactory: i32,
    // Java field: ArrayList<Namespace> namespaceStack
    pub(crate) namespaceStack: Vec<i32>,
    // Java field: ArrayList<Map<String, QName>> namespaceCacheList
    namespaceCacheList: Vec<HashMap<String, i32>>,
    // Java field: Map<String, QName> currentNamespaceCache
    currentNamespaceCache: HashMap<String, i32>,
    // Java field: Map<String, QName> rootNamespaceCache
    rootNamespaceCache: HashMap<String, i32>,
    // Java field: Namespace defaultNamespace
    pub(crate) defaultNamespace: i32,
}

// Java: public class NamespaceStack
impl NamespaceStack {
    // Java: NamespaceStack()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.documentFactory = DocumentFactory.getInstance();
      Self {
          documentFactory: 0,
          namespaceStack: Vec::new(),
          namespaceCacheList: Vec::new(),
          currentNamespaceCache: HashMap::new(),
          rootNamespaceCache: HashMap::new(),
          defaultNamespace: 0,
      }
    }

    // Java: NamespaceStack(DocumentFactory documentFactory)
    pub fn new_with_factory() -> Self {
      // Java: super();
      // Java: this.documentFactory = documentFactory;
      Self {
          documentFactory: 0,
          namespaceStack: Vec::new(),
          namespaceCacheList: Vec::new(),
          currentNamespaceCache: HashMap::new(),
          rootNamespaceCache: HashMap::new(),
          defaultNamespace: 0,
      }
    }

    // Java: void push(Namespace namespace)
    pub fn push(&self) {
      // Java: this.namespaceStack.add(namespace);
      // Java: this.namespaceCacheList.add(null);
      // Java: this.currentNamespaceCache = null;
      // Java: String prefix = namespace.getPrefix();
      // Java: if (prefix == null || prefix.length() == 0)
      {
         // Java: this.defaultNamespace = namespace;
      }
    }

    // Java: Namespace pop()
    pub fn pop(&self) {
      // Java: return this.remove(this.namespaceStack.size() - 1);
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.namespaceStack.size();
      0
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: this.namespaceStack.clear();
      // Java: this.namespaceCacheList.clear();
      // Java: this.rootNamespaceCache.clear();
      // Java: this.currentNamespaceCache = null;
    }

    // Java: Namespace getNamespace(int index)
    pub fn getNamespace(&self, _index: i32) {
      // Java: return (Namespace)this.namespaceStack.get(index);
    }

    // Java: Namespace getNamespaceForPrefix(String prefix)
    pub fn getNamespaceForPrefix(&self, _prefix: &str) {
      // Java: if (prefix == null)
      {
         // Java: prefix = "";
      }
      // Java: for(int i = this.namespaceStack.size() - 1; i >= 0; --i)
      {
         // Java: Namespace namespace = (Namespace)this.namespaceStack.get(i);
         // Java: if (prefix.equals(namespace.getPrefix()))
         {
            // Java: return namespace;
         }
      }
      // Java: return null;
    }

    // Java: String getURI(String prefix)
    pub fn getURI(&self, _prefix: &str) -> String {
      // Java: Namespace namespace = this.getNamespaceForPrefix(prefix);
      // Java: return namespace != null ? namespace.getURI() : null;
      String::new()
    }

    // Java: boolean contains(Namespace namespace)
    pub fn contains(&self) -> bool {
      // Java: String prefix = namespace.getPrefix();
      // Java: Namespace current = null;
      // Java: if (prefix != null && prefix.length() != 0)
      {
         // Java: current = this.getNamespaceForPrefix(prefix);
      }
      // Java: else
      {
         // Java: current = this.getDefaultNamespace();
      }
      // Java: if (current == null)
      {
         // Java: return false;
      }
      // Java: else
      {
         // Java: return current == namespace ? true : namespace.getURI().equals(current.getURI());
      }
      false
    }

    // Java: QName getQName(String namespaceURI, String localName, String qualifiedName)
    pub fn getQName(&self, _namespace_uri: &str, _local_name: &str, _qualified_name: &str) {
      // Java: if (localName == null)
      {
         // Java: localName = qualifiedName;
      }
      // Java: else if (qualifiedName == null)
      {
         // Java: qualifiedName = localName;
      }
      // Java: if (namespaceURI == null)
      {
         // Java: namespaceURI = "";
      }
      // Java: String prefix = "";
      // Java: int index = qualifiedName.indexOf(58);
      // Java: if (index > 0)
      {
         // Java: prefix = qualifiedName.substring(0, index);
      }
      // Java: Namespace namespace = this.createNamespace(prefix, namespaceURI);
      // Java: return this.pushQName(localName, qualifiedName, namespace, prefix);
    }

    // Java: QName getAttributeQName(String namespaceURI, String localName, String qualifiedName)
    pub fn getAttributeQName(&self, _namespace_uri: &str, _local_name: &str, _qualified_name: &str) {
      // Java: if (qualifiedName == null || qualifiedName.equals(""))
      {
         // Java: qualifiedName = localName;
      }
      // Java: Map<String, QName> map = this.getNamespaceCache();
      // Java: QName answer = (QName)map.get(qualifiedName);
      // Java: if (answer != null)
      {
         // Java: return answer;
      }
      // Java: else
      {
         // Java: if (localName == null)
         {
            // Java: localName = qualifiedName;
         }
         // Java: if (namespaceURI == null)
         {
            // Java: namespaceURI = "";
         }
         // Java: Namespace namespace = null;
         // Java: String prefix = "";
         // Java: int index = qualifiedName.indexOf(58);
         // Java: if (index > 0)
         {
            // Java: prefix = qualifiedName.substring(0, index);
            // Java: namespace = this.createNamespace(prefix, namespaceURI);
            // Java: if (localName.trim().length() == 0)
            {
               // Java: localName = qualifiedName.substring(index + 1);
            }
         }
         // Java: else
         {
            // Java: namespace = Namespace.NO_NAMESPACE;
            // Java: if (localName.trim().length() == 0)
            {
               // Java: localName = qualifiedName;
            }
         }
         // Java: answer = this.pushQName(localName, qualifiedName, namespace, prefix);
         // Java: map.put(qualifiedName, answer);
         // Java: return answer;
      }
    }

    // Java: void push(String prefix, String uri)
    pub fn push_prefix_uri(&self, _prefix: &str, _uri: &str) {
      // Java: if (uri == null)
      {
         // Java: uri = "";
      }
      // Java: this.push(this.createNamespace(prefix, uri));
    }

    // Java: Namespace addNamespace(String prefix, String uri)
    pub fn addNamespace(&self, _prefix: &str, _uri: &str) {
      // Java: Namespace namespace = this.createNamespace(prefix, uri);
      // Java: this.push(namespace);
      // Java: return namespace;
    }

    // Java: Namespace pop(String prefix)
    pub fn pop_prefix(&self, _prefix: &str) {
      // Java: if (prefix == null)
      {
         // Java: prefix = "";
      }
      // Java: Namespace namespace = null;
      // Java: for(int i = this.namespaceStack.size() - 1; i >= 0; --i)
      {
         // Java: Namespace ns = (Namespace)this.namespaceStack.get(i);
         // Java: if (prefix.equals(ns.getPrefix()))
         {
            // Java: this.remove(i);
            // Java: namespace = ns;
            // Java: break;
         }
      }
      // Java: if (namespace == null)
      {
         // Java: System.out.println("Warning: missing namespace prefix ignored: " + prefix);
      }
      // Java: return namespace;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return super.toString() + " Stack: " + this.namespaceStack.toString();
      String::new()
    }

    // Java: DocumentFactory getDocumentFactory()
    pub fn getDocumentFactory(&self) {
      // Java: return this.documentFactory;
    }

    // Java: void setDocumentFactory(DocumentFactory documentFactory)
    pub fn setDocumentFactory(&self) {
      // Java: this.documentFactory = documentFactory;
    }

    // Java: Namespace getDefaultNamespace()
    pub fn getDefaultNamespace(&self) {
      // Java: if (this.defaultNamespace == null)
      {
         // Java: this.defaultNamespace = this.findDefaultNamespace();
      }
      // Java: return this.defaultNamespace;
    }

    // Java: QName pushQName(String localName, String qualifiedName, Namespace namespace, String prefix)
    pub fn pushQName(&self, _local_name: &str, _qualified_name: &str) {
      // Java: if (prefix == null || prefix.length() == 0)
      {
         // Java: this.defaultNamespace = null;
      }
      // Java: return this.createQName(localName, qualifiedName, namespace);
    }

    // Java: QName createQName(String localName, String qualifiedName, Namespace namespace)
    pub fn createQName(&self, _local_name: &str, _qualified_name: &str) {
      // Java: return this.documentFactory.createQName(localName, namespace);
    }

    // Java: Namespace createNamespace(String prefix, String namespaceURI)
    pub fn createNamespace(&self, _prefix: &str, _namespace_uri: &str) {
      // Java: return Namespace.CACHE.get(prefix, namespaceURI);
    }

    // Java: Namespace findDefaultNamespace()
    pub fn findDefaultNamespace(&self) {
      // Java: for(int i = this.namespaceStack.size() - 1; i >= 0; --i)
      {
         // Java: Namespace namespace = (Namespace)this.namespaceStack.get(i);
         // Java: if (namespace != null)
         {
            // Java: String prefix = namespace.getPrefix();
            // Java: if (prefix == null || namespace.getPrefix().length() == 0)
            {
               // Java: return namespace;
            }
         }
      }
      // Java: return null;
    }

    // Java: Namespace remove(int index)
    pub fn remove(&self, _index: i32) {
      // Java: Namespace namespace = (Namespace)this.namespaceStack.remove(index);
      // Java: this.namespaceCacheList.remove(index);
      // Java: this.defaultNamespace = null;
      // Java: this.currentNamespaceCache = null;
      // Java: return namespace;
    }

    // Java: protected Map<String, QName> getNamespaceCache()
    pub fn getNamespaceCache(&self) {
      // Java: if (this.currentNamespaceCache == null)
      {
         // Java: int index = this.namespaceStack.size() - 1;
         // Java: if (index < 0)
         {
            // Java: this.currentNamespaceCache = this.rootNamespaceCache;
         }
         // Java: else
         {
            // Java: this.currentNamespaceCache = (Map)this.namespaceCacheList.get(index);
            // Java: if (this.currentNamespaceCache == null)
            {
               // Java: this.currentNamespaceCache = new HashMap();
               // Java: this.namespaceCacheList.set(index, this.currentNamespaceCache);
            }
         }
      }
      // Java: return this.currentNamespaceCache;
    }

}
