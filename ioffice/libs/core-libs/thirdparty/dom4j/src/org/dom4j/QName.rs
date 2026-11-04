// Java: import java.io.IOException
// Java: import java.io.ObjectInputStream
// Java: import java.io.ObjectOutputStream
// Java: import java.io.Serializable
// Java: import org.dom4j.tree.QNameCache
// Java: import org.dom4j.util.SingletonStrategy

use super::Namespace::Namespace;
use super::DocumentFactory::DocumentFactory;

// Java field: SingletonStrategy singleton

// Java: public class QName implements Serializable
#[derive(Clone)]
pub struct QName {
    // Java field: String name
    pub(crate) name: String,
    // Java field: String qualifiedName
    pub(crate) qualifiedName: String,
    // Java field: Namespace namespace
    pub(crate) namespace: Namespace,
    // Java field: int hashCode
    pub(crate) hashCode: i32,
    // Java field: DocumentFactory documentFactory
    pub(crate) documentFactory: Option<DocumentFactory>,
}

// Java: public class QName implements Serializable
impl QName {
    // Java: QName(String name)
    pub fn new(name: &str) -> Self {
      // Java: this(name, Namespace.NO_NAMESPACE);
      Self {
          name: name.to_string(),
          qualifiedName: name.to_string(),
          namespace: Namespace::new("", ""),
          hashCode: 0,
          documentFactory: None,
      }
    }

    // Java: QName(String name, Namespace namespace)
    pub fn new_with_namespace(name: &str, namespace: Namespace) -> Self {
      // Java: super();
      // Java: this.name = name == null ? "" : name;
      // Java: this.namespace = namespace == null ? Namespace.NO_NAMESPACE : namespace;
      let qualified = if namespace.getPrefix().is_empty() {
          name.to_string()
      } else {
          format!("{}:{}", namespace.getPrefix(), name)
      };
      Self {
          name: name.to_string(),
          qualifiedName: qualified,
          namespace,
          hashCode: 0,
          documentFactory: None,
      }
    }

    // Java: QName(String name, Namespace namespace, String qualifiedName)
    pub fn new_with_namespace_qualified(name: &str, namespace: Namespace, qualifiedName: &str) -> Self {
      // Java: super();
      // Java: this.name = name == null ? "" : name;
      // Java: this.qualifiedName = qualifiedName;
      // Java: this.namespace = namespace == null ? Namespace.NO_NAMESPACE : namespace;
      Self {
          name: name.to_string(),
          qualifiedName: qualifiedName.to_string(),
          namespace,
          hashCode: 0,
          documentFactory: None,
      }
    }

    // Java: static QName get(String name)
    pub fn get_by_name(_name: &str) -> QName {
      // Java: return getCache().get(name);
      todo!()
    }

    // Java: static QName get(String name, Namespace namespace)
    pub fn get_by_name_namespace(_name: &str, _namespace: &Namespace) -> QName {
      // Java: return getCache().get(name, namespace);
      todo!()
    }

    // Java: static QName get(String name, String prefix, String uri)
    pub fn get_by_name_prefix_uri(_name: &str, _prefix: &str, _uri: &str) -> QName {
      // Java: if ((prefix == null || prefix.length() == 0) && uri == null)
      // Java: return get(name);
      // Java: else if (prefix != null && prefix.length() != 0)
      // Java: return uri == null ? get(name) : getCache().get(name, Namespace.get(prefix, uri));
      // Java: else
      // Java: return getCache().get(name, Namespace.get(uri));
      todo!()
    }

    // Java: static QName get(String qualifiedName, String uri)
    pub fn get_by_qualified_uri(_qualifiedName: &str, _uri: &str) -> QName {
      // Java: return uri == null ? getCache().get(qualifiedName) : getCache().get(qualifiedName, uri);
      todo!()
    }

    // Java: static QName get(String localName, Namespace namespace, String qualifiedName)
    pub fn get_by_local_namespace_qualified(_localName: &str, _namespace: &Namespace, _qualifiedName: &str) -> QName {
      // Java: return getCache().get(localName, namespace, qualifiedName);
      todo!()
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this.name;
      &self.name
    }

    // Java: String getQualifiedName()
    pub fn getQualifiedName(&self) -> &str {
      // Java: if (this.qualifiedName == null)
      // Java: String prefix = this.getNamespacePrefix();
      // Java: if (prefix != null && prefix.length() > 0)
      // Java: this.qualifiedName = prefix + ":" + this.name;
      // Java: else
      // Java: this.qualifiedName = this.name;
      // Java: return this.qualifiedName;
      &self.qualifiedName
    }

    // Java: Namespace getNamespace()
    pub fn getNamespace(&self) -> &Namespace {
      // Java: return this.namespace;
      &self.namespace
    }

    // Java: String getNamespacePrefix()
    pub fn getNamespacePrefix(&self) -> &str {
      // Java: return this.namespace == null ? "" : this.namespace.getPrefix();
      self.namespace.getPrefix()
    }

    // Java: String getNamespaceURI()
    pub fn getNamespaceURI(&self) -> &str {
      // Java: return this.namespace == null ? "" : this.namespace.getURI();
      self.namespace.getURI()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: if (this.hashCode == 0)
      // Java: this.hashCode = this.getName().hashCode() ^ this.getNamespaceURI().hashCode();
      // Java: if (this.hashCode == 0)
      // Java: this.hashCode = 47806;
      // Java: return this.hashCode;
      self.hashCode
    }

    // Java: boolean equals(Object object)
    pub fn equals(&self, other: &QName) -> bool {
      // Java: if (this == object)
      // Java: return true;
      // Java: else
      // Java: if (object instanceof QName)
      // Java: QName that = (QName)object;
      // Java: if (this.hashCode() == that.hashCode())
      // Java: return this.getName().equals(that.getName()) && this.getNamespaceURI().equals(that.getNamespaceURI());
      // Java: return false;
      self.name == other.name && self.namespace.getURI() == other.namespace.getURI()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return super.toString() + " [name: " + this.getName() + " namespace: \"" + this.getNamespace() + "\"]";
      format!("QName [name: {} namespace: \"{}\"]", self.name, self.namespace.getURI())
    }

    // Java: DocumentFactory getDocumentFactory()
    pub fn getDocumentFactory(&self) -> &Option<DocumentFactory> {
      // Java: return this.documentFactory;
      &self.documentFactory
    }

    // Java: void setDocumentFactory(DocumentFactory documentFactory)
    pub fn setDocumentFactory(&mut self, documentFactory: Option<DocumentFactory>) {
      // Java: this.documentFactory = documentFactory;
      self.documentFactory = documentFactory;
    }

    // Java: void writeObject(ObjectOutputStream out)
    pub fn writeObject(&self) {
      // Java: out.writeObject(this.namespace.getPrefix());
      // Java: out.writeObject(this.namespace.getURI());
      // Java: out.defaultWriteObject();
      todo!()
    }

    // Java: void readObject(ObjectInputStream in)
    pub fn readObject(&mut self) {
      // Java: String prefix = (String)in.readObject();
      // Java: String uri = (String)in.readObject();
      // Java: in.defaultReadObject();
      // Java: this.namespace = Namespace.get(prefix, uri);
      todo!()
    }

    // Java: static QNameCache getCache()
    pub fn getCache() -> i32 {
      // Java: QNameCache cache = (QNameCache)singleton.instance();
      // Java: return cache;
      todo!()
    }

}