// Java: import java.util.ArrayList
// Java: import java.util.Collections
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.WeakHashMap
// Java: import org.dom4j.DocumentFactory
use crate::org::dom4j::DocumentFactory::DocumentFactory;
// Java: import org.dom4j.Namespace
use crate::org::dom4j::Namespace::Namespace;
// Java: import org.dom4j.QName
use crate::org::dom4j::QName::QName;

// Java: public class QNameCache
pub struct QNameCache {
    // Java field: Map<String, QName> noNamespaceCache
    pub(crate) noNamespaceCache: HashMap<String, QName>,
    // Java field: Map<Namespace, Map<String, QName>> namespaceCache
    pub(crate) namespaceCache: HashMap<String, HashMap<String, QName>>,
    // Java field: DocumentFactory documentFactory
    documentFactory: Option<DocumentFactory>,
}

// Java: public class QNameCache
impl QNameCache {
    // Java: QNameCache()
    pub fn new() -> Self {
      // Java: super();
      QNameCache {
          noNamespaceCache: HashMap::new(),
          namespaceCache: HashMap::new(),
          documentFactory: None,
      }
    }

    // Java: QNameCache(DocumentFactory documentFactory)
    pub fn new_with_factory(documentFactory: DocumentFactory) -> Self {
      // Java: super();
      // Java: this.documentFactory = documentFactory;
      QNameCache {
          noNamespaceCache: HashMap::new(),
          namespaceCache: HashMap::new(),
          documentFactory: Some(documentFactory),
      }
    }

    // Java: List<QName> getQNames()
    pub fn getQNames(&self) -> Vec<QName> {
      // Java: List<QName> answer = new ArrayList();
      let mut answer = Vec::new();
      // Java: answer.addAll(this.noNamespaceCache.values());
      answer.extend(self.noNamespaceCache.values().cloned());
      // Java: for(Map<String, QName> map : this.namespaceCache.values())
      for map in self.namespaceCache.values()
      {
         // Java: answer.addAll(map.values());
         answer.extend(map.values().cloned());
      }
      // Java: return answer;
      answer
    }

    // Java: QName get(String name)
    pub fn get(&mut self, name: &str) -> QName {
      // Java: QName answer = null;
      let mut answer: Option<QName> = None;
      // Java: if (name != null)
      let name = if !name.is_empty()
      {
         // Java: answer = (QName)this.noNamespaceCache.get(name);
         answer = self.noNamespaceCache.get(name).cloned();
         name
      }
      // Java: else
      else
      {
         // Java: name = "";
         ""
      };
      // Java: if (answer == null)
      if answer.is_none()
      {
         // Java: answer = this.createQName(name);
         let mut q = self.createQName(name);
         // Java: answer.setDocumentFactory(this.documentFactory);
         q.setDocumentFactory(self.documentFactory.clone());
         // Java: this.noNamespaceCache.put(name, answer);
         self.noNamespaceCache.insert(name.to_string(), q.clone());
         answer = Some(q);
      }
      // Java: return answer;
      answer.unwrap()
    }

    // Java: QName get(String name, Namespace namespace)
    pub fn get_with_namespace(&mut self, name: &str, namespace: &Namespace) -> QName {
      // Java: Map<String, QName> cache = this.getNamespaceCache(namespace);
      let ns_uri = namespace.getURI().to_string();
      // Java: QName answer = null;
      let mut answer: Option<QName> = None;
      // Java: if (name != null)
      let name = if !name.is_empty()
      {
         // Java: answer = (QName)cache.get(name);
         answer = self.namespaceCache.get(&ns_uri).and_then(|c| c.get(name).cloned());
         name
      }
      // Java: else
      else
      {
         // Java: name = "";
         ""
      };
      // Java: if (answer == null)
      if answer.is_none()
      {
         // Java: answer = this.createQName(name, namespace);
         let mut q = self.createQName_with_namespace(name, namespace);
         // Java: answer.setDocumentFactory(this.documentFactory);
         q.setDocumentFactory(self.documentFactory.clone());
         // Java: cache.put(name, answer);
         self.namespaceCache.entry(ns_uri).or_insert_with(HashMap::new).insert(name.to_string(), q.clone());
         answer = Some(q);
      }
      // Java: return answer;
      answer.unwrap()
    }

    // Java: QName get(String localName, Namespace namespace, String qName)
    pub fn get_with_namespace_qname(&mut self, localName: &str, namespace: &Namespace, qName: &str) -> QName {
      // Java: Map<String, QName> cache = this.getNamespaceCache(namespace);
      let ns_uri = namespace.getURI().to_string();
      // Java: QName answer = null;
      let mut answer: Option<QName> = None;
      // Java: if (localName != null)
      let localName = if !localName.is_empty()
      {
         // Java: answer = (QName)cache.get(localName);
         answer = self.namespaceCache.get(&ns_uri).and_then(|c| c.get(localName).cloned());
         localName
      }
      // Java: else
      else
      {
         // Java: localName = "";
         ""
      };
      // Java: if (answer == null)
      if answer.is_none()
      {
         // Java: answer = this.createQName(localName, namespace, qName);
         let mut q = self.createQName_with_namespace_qualified(localName, namespace, qName);
         // Java: answer.setDocumentFactory(this.documentFactory);
         q.setDocumentFactory(self.documentFactory.clone());
         // Java: cache.put(localName, answer);
         self.namespaceCache.entry(ns_uri).or_insert_with(HashMap::new).insert(localName.to_string(), q.clone());
         answer = Some(q);
      }
      // Java: return answer;
      answer.unwrap()
    }

    // Java: QName get(String qualifiedName, String uri)
    pub fn get_by_qualified_name(&mut self, qualifiedName: &str, uri: &str) -> QName {
      // Java: int index = qualifiedName.indexOf(58);
      let index = qualifiedName.find(':');
      // Java: if (index < 0)
      if index.is_none()
      {
         // Java: return this.get(qualifiedName, Namespace.get(uri));
         let ns = Namespace::get("", uri);
         return self.get_with_namespace(qualifiedName, &ns);
      }
      // Java: else
      else
      {
         // Java: String name = qualifiedName.substring(index + 1);
         let idx = index.unwrap();
         let name = &qualifiedName[idx + 1..];
         // Java: String prefix = qualifiedName.substring(0, index);
         let prefix = &qualifiedName[..idx];
         // Java: return this.get(name, Namespace.get(prefix, uri));
         let ns = Namespace::get(prefix, uri);
         return self.get_with_namespace(name, &ns);
      }
    }

    // Java: QName intern(QName qname)
    pub fn intern(&mut self, qname: &QName) -> QName {
      // Java: return this.get(qname.getName(), qname.getNamespace(), qname.getQualifiedName());
      self.get_with_namespace_qname(qname.getName(), qname.getNamespace(), qname.getQualifiedName())
    }

    // Java: protected Map<String, QName> getNamespaceCache(Namespace namespace)
    fn getNamespaceCache(&mut self, namespace: &Namespace) -> &mut HashMap<String, QName> {
      // Java: if (namespace == Namespace.NO_NAMESPACE)
      if namespace.getURI().is_empty() && namespace.getPrefix().is_empty()
      {
         // Java: return this.noNamespaceCache;
         todo!()
      }
      // Java: else
      else
      {
         // Java: Map<String, QName> answer = null;
         let uri = namespace.getURI().to_string();
         // Java: if (namespace != null)
         // Java: answer = (Map)this.namespaceCache.get(namespace);
         // Java: if (answer == null)
         // Java: answer = this.createMap();
         // Java: this.namespaceCache.put(namespace, answer);
         // Java: return answer;
         self.namespaceCache.entry(uri).or_insert_with(HashMap::new)
      }
    }

    // Java: protected Map<String, QName> createMap()
    fn createMap(&self) -> HashMap<String, QName> {
      // Java: return Collections.synchronizedMap(new HashMap());
      HashMap::new()
    }

    // Java: QName createQName(String name)
    pub fn createQName(&self, name: &str) -> QName {
      // Java: return new QName(name);
      QName::new(name)
    }

    // Java: QName createQName(String name, Namespace namespace)
    pub fn createQName_with_namespace(&self, name: &str, namespace: &Namespace) -> QName {
      // Java: return new QName(name, namespace);
      QName::new_with_namespace(name, namespace.clone())
    }

    // Java: QName createQName(String name, Namespace namespace, String qualifiedName)
    pub fn createQName_with_namespace_qualified(&self, name: &str, namespace: &Namespace, qualifiedName: &str) -> QName {
      // Java: return new QName(name, namespace, qualifiedName);
      QName::new_with_namespace_qualified(name, namespace.clone(), qualifiedName)
    }

}