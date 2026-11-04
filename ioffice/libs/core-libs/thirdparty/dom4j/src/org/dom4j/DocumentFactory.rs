// Java: import java.io.IOException
// Java: import java.io.ObjectInputStream
// Java: import java.io.Serializable
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.dom4j.rule.Pattern
// Java: import org.dom4j.tree.AbstractDocument
// Java: import org.dom4j.tree.DefaultAttribute
// Java: import org.dom4j.tree.DefaultCDATA
// Java: import org.dom4j.tree.DefaultComment
// Java: import org.dom4j.tree.DefaultDocument
// Java: import org.dom4j.tree.DefaultDocumentType
// Java: import org.dom4j.tree.DefaultElement
// Java: import org.dom4j.tree.DefaultEntity
// Java: import org.dom4j.tree.DefaultProcessingInstruction
// Java: import org.dom4j.tree.DefaultText
// Java: import org.dom4j.tree.QNameCache
// Java: import org.dom4j.util.SimpleSingleton
// Java: import org.dom4j.util.SingletonStrategy
// Java: import org.dom4j.xpath.DefaultXPath
// Java: import org.dom4j.xpath.XPathPattern
// Java: import org.jaxen.VariableContext

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Java field: SingletonStrategy singleton

// Java: public class DocumentFactory implements Serializable
#[derive(Clone)]
pub struct DocumentFactory {
    // Java field: QNameCache cache
    pub(crate) cache: i32,
    // Java field: Map<String, String> xpathNamespaceURIs
    pub(crate) xpathNamespaceURIs: HashMap<String, String>,
}

// Java: public class DocumentFactory implements Serializable
impl DocumentFactory {
    // Java: static SingletonStrategy createSingleton()
    pub fn createSingleton_static() {
      // Java: SingletonStrategy result = null;
      // Java: String documentFactoryClassName;
      // Java: try
      {
         // Java: documentFactoryClassName = System.getProperty("org.dom4j.factory", "org.dom4j.DocumentFactory");
      }
      // Java: catch (Exception var5)
      {
         // Java: documentFactoryClassName = "org.dom4j.DocumentFactory";
      }
      // Java: try
      {
         // Java: String singletonClass = System.getProperty("org.dom4j.DocumentFactory.singleton.strategy", "org.dom4j.util.SimpleSingleton");
         // Java: Class<?> clazz = Class.forName(singletonClass);
         // Java: result = (SingletonStrategy)clazz.newInstance();
      }
      // Java: catch (Exception var4)
      {
         // Java: result = new SimpleSingleton();
      }
      // Java: result.setSingletonClassName(documentFactoryClassName);
      // Java: return result;
    }

    // Java: DocumentFactory()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.init();
      Self {
          cache: 0,
          xpathNamespaceURIs: HashMap::new(),
      }
    }

    // Java: static DocumentFactory getInstance()
    pub fn getInstance() -> DocumentFactory {
      // Java: if (singleton == null)
      {
         // Java: singleton = createSingleton();
      }
      // Java: return (DocumentFactory)singleton.instance();
      DocumentFactory::new()
    }

    // Java: Document createDocument()
    pub fn createDocument(&self) {
      // Java: DefaultDocument answer = new DefaultDocument();
      // Java: answer.setDocumentFactory(this);
      // Java: return answer;
    }

    // Java: Document createDocument(String encoding)
    pub fn createDocument_encoding(&self, _encoding: &str) {
      // Java: Document answer = this.createDocument();
      // Java: if (answer instanceof AbstractDocument)
      {
         // Java: ((AbstractDocument)answer).setXMLEncoding(encoding);
      }
      // Java: return answer;
    }

    // Java: Document createDocument(Element rootElement)
    pub fn createDocument_root(&self) {
      // Java: Document answer = this.createDocument();
      // Java: answer.setRootElement(rootElement);
      // Java: return answer;
    }

    // Java: DocumentType createDocType(String name, String publicId, String systemId)
    pub fn createDocType(&self, _name: &str, _public_id: &str, _system_id: &str) {
      // Java: return new DefaultDocumentType(name, publicId, systemId);
    }

    // Java: Element createElement(QName qname)
    pub fn createElement_qname(&self) {
      // Java: return new DefaultElement(qname);
    }

    // Java: Element createElement(String name)
    pub fn createElement_name(&self, _name: &str) {
      // Java: return this.createElement(this.createQName(name));
    }

    // Java: Element createElement(String qualifiedName, String namespaceURI)
    pub fn createElement_qualified(&self, _qualified_name: &str, _namespace_uri: &str) {
      // Java: return this.createElement(this.createQName(qualifiedName, namespaceURI));
    }

    // Java: Attribute createAttribute(Element owner, QName qname, String value)
    pub fn createAttribute_qname(&self, _value: &str) {
      // Java: return new DefaultAttribute(qname, value);
    }

    // Java: Attribute createAttribute(Element owner, String name, String value)
    pub fn createAttribute_name(&self, _name: &str, _value: &str) {
      // Java: return this.createAttribute(owner, this.createQName(name), value);
    }

    // Java: CDATA createCDATA(String text)
    pub fn createCDATA(&self, _text: &str) {
      // Java: return new DefaultCDATA(text);
    }

    // Java: Comment createComment(String text)
    pub fn createComment(&self, _text: &str) {
      // Java: return new DefaultComment(text);
    }

    // Java: Text createText(String text)
    pub fn createText(&self, _text: &str) {
      // Java: if (text == null)
      {
         // Java: String msg = "Adding text to an XML document must not be null";
         // Java: throw new IllegalArgumentException(msg);
      }
      // Java: else
      {
         // Java: return new DefaultText(text);
      }
    }

    // Java: Entity createEntity(String name, String text)
    pub fn createEntity(&self, _name: &str, _text: &str) {
      // Java: return new DefaultEntity(name, text);
    }

    // Java: Namespace createNamespace(String prefix, String uri)
    pub fn createNamespace(&self, _prefix: &str, _uri: &str) {
      // Java: return Namespace.get(prefix, uri);
    }

    // Java: ProcessingInstruction createProcessingInstruction(String target, String data)
    pub fn createProcessingInstruction_data(&self, _target: &str, _data: &str) {
      // Java: return new DefaultProcessingInstruction(target, data);
    }

    // Java: ProcessingInstruction createProcessingInstruction(String target, Map<String, String> data)
    pub fn createProcessingInstruction_map(&self, _target: &str, _data: &HashMap<String, String>) {
      // Java: return new DefaultProcessingInstruction(target, data);
    }

    // Java: QName createQName(String localName, Namespace namespace)
    pub fn createQName_namespace(&self, _local_name: &str) {
      // Java: return this.cache.get(localName, namespace);
    }

    // Java: QName createQName(String localName)
    pub fn createQName(&self, _local_name: &str) {
      // Java: return this.cache.get(localName);
    }

    // Java: QName createQName(String name, String prefix, String uri)
    pub fn createQName_prefix_uri(&self, _name: &str, _prefix: &str, _uri: &str) {
      // Java: return this.cache.get(name, Namespace.get(prefix, uri));
    }

    // Java: QName createQName(String qualifiedName, String uri)
    pub fn createQName_uri(&self, _qualified_name: &str, _uri: &str) {
      // Java: return this.cache.get(qualifiedName, uri);
    }

    // Java: XPath createXPath(String xpathExpression)
    pub fn createXPath(&self, _xpath_expression: &str) {
      // Java: DefaultXPath xpath = new DefaultXPath(xpathExpression);
      // Java: if (this.xpathNamespaceURIs != null)
      {
         // Java: xpath.setNamespaceURIs(this.xpathNamespaceURIs);
      }
      // Java: return xpath;
    }

    // Java: XPath createXPath(String xpathExpression, VariableContext variableContext)
    pub fn createXPath_variable(&self, _xpath_expression: &str) {
      // Java: XPath xpath = this.createXPath(xpathExpression);
      // Java: xpath.setVariableContext(variableContext);
      // Java: return xpath;
    }

    // Java: NodeFilter createXPathFilter(String xpathFilterExpression, VariableContext variableContext)
    pub fn createXPathFilter_variable(&self, _xpath_filter_expression: &str) {
      // Java: XPath answer = this.createXPath(xpathFilterExpression);
      // Java: answer.setVariableContext(variableContext);
      // Java: return answer;
    }

    // Java: NodeFilter createXPathFilter(String xpathFilterExpression)
    pub fn createXPathFilter(&self, _xpath_filter_expression: &str) {
      // Java: return this.createXPath(xpathFilterExpression);
    }

    // Java: Pattern createPattern(String xpathPattern)
    pub fn createPattern(&self, _xpath_pattern: &str) {
      // Java: return new XPathPattern(xpathPattern);
    }

    // Java: List<QName> getQNames()
    pub fn getQNames(&self) {
      // Java: return this.cache.getQNames();
    }

    // Java: public Map<String, String> getXPathNamespaceURIs()
    pub fn getXPathNamespaceURIs(&self) {
      // Java: return this.xpathNamespaceURIs;
    }

    // Java: void setXPathNamespaceURIs(Map<String, String> namespaceURIs)
    pub fn setXPathNamespaceURIs(&self, _namespace_uris: &HashMap<String, String>) {
      // Java: this.xpathNamespaceURIs = namespaceURIs;
    }

    // Java: static DocumentFactory createSingleton(String className)
    pub fn createSingleton_class(_class_name: &str) -> DocumentFactory {
      // Java: try
      {
         // Java: Class<?> theClass = Class.forName(className, true, DocumentFactory.class.getClassLoader());
         // Java: return (DocumentFactory)theClass.newInstance();
      }
      // Java: catch (Throwable var2)
      {
         // Java: System.out.println("WARNING: Cannot load DocumentFactory: " + className);
         // Java: return new DocumentFactory();
      }
      DocumentFactory::new()
    }

    // Java: QName intern(QName qname)
    pub fn intern(&self) {
      // Java: return this.cache.intern(qname);
    }

    // Java: QNameCache createQNameCache()
    pub fn createQNameCache(&self) {
      // Java: return new QNameCache(this);
    }

    // Java: void readObject(ObjectInputStream in)
    pub fn readObject(&self) {
      // Java: in.defaultReadObject();
      // Java: this.init();
    }

    // Java: void init()
    pub fn init(&self) {
      // Java: this.cache = this.createQNameCache();
    }

}
