// Java: import cn.wps.base.assertion.IOAssert
// use util::cn::wps::base::assertion::IOAssert;
// Java: import cn.wps.io.dom.Document
use crate::io::dom::Document;
// Java: import cn.wps.io.dom.DocumentType
use crate::io::dom::DocumentType;
// Java: import cn.wps.io.dom.Element
use crate::io::dom::Element;
// Java: import cn.wps.io.dom.IllegalAddException
use crate::io::dom::IllegalAddException;
// Java: import cn.wps.io.dom.Node
use crate::io::dom::Node;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.xml.sax.EntityResolver

// Java: public class DefaultDocument extends AbstractDocument
pub struct DefaultDocument {
    // Java field: String name
    pub(crate) name: Option<String>,
    // Java field: Element rootElement
    pub(crate) rootElement: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: List<Node> content
    content: Vec<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: DocumentType docType
    pub(crate) docType: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: EntityResolver entityResolver
    pub(crate) entityResolver: Option<Box<dyn std::any::Any + Send + Sync>>,
    pub(crate) base: super::AbstractDocument::AbstractDocument,
}

// Java: public class DefaultDocument extends AbstractDocument
// Java: 无对应物（Java 对象天然可跨线程共享引用）
// Rust: 模板内部容器是 `Box<dyn Any>`（无 Send/Sync 约束），而 SAX 通路要把文档以
//       `Box<dyn Any + Send + Sync>` 交回（`SAXReader::read_stream` 契约）⇒ 与 `PackageRelationship`
//       同法显式声明（登记 §1.7.1 ⑤）。文档只在单线程内构造与读取。
unsafe impl Send for DefaultDocument {}
unsafe impl Sync for DefaultDocument {}

impl DefaultDocument {
    // Java: void setRootElement(Element rootElement)
    pub fn setRootElement(&mut self, root_element: Option<Box<dyn std::any::Any + Send + Sync>>) {
        // Java: this.rootElement = rootElement;
        self.rootElement = root_element;
    }

    // Java: DefaultDocument()
    pub fn new() -> Self {
        // Java: this((String)null, (Element)null, (DocumentType)null);
        Self::new_with_all(None, None, None)
    }

    // Java: DefaultDocument(String name)
    pub fn new_with_name(name: Option<String>) -> Self {
        // Java: this(name, (Element)null, (DocumentType)null);
        Self::new_with_all(name, None, None)
    }

    // Java: DefaultDocument(Element rootElement)
    pub fn new_with_root(root_element: Option<Box<dyn std::any::Any + Send + Sync>>) -> Self {
        // Java: this((String)null, rootElement, (DocumentType)null);
        Self::new_with_all(None, root_element, None)
    }

    // Java: DefaultDocument(DocumentType docType)
    pub fn new_with_doc_type(doc_type: Option<Box<dyn std::any::Any + Send + Sync>>) -> Self {
        // Java: this((String)null, (Element)null, docType);
        Self::new_with_all(None, None, doc_type)
    }

    // Java: DefaultDocument(Element rootElement, DocumentType docType)
    pub fn new_with_root_and_doc_type(root_element: Option<Box<dyn std::any::Any + Send + Sync>>, doc_type: Option<Box<dyn std::any::Any + Send + Sync>>) -> Self {
        // Java: this((String)null, rootElement, docType);
        Self::new_with_all(None, root_element, doc_type)
    }

    // Java: DefaultDocument(String name, Element rootElement, DocumentType docType)
    pub fn new_with_all(name: Option<String>, root_element: Option<Box<dyn std::any::Any + Send + Sync>>, doc_type: Option<Box<dyn std::any::Any + Send + Sync>>) -> Self {
        // Java: super();
        let mut doc = Self {
            // Java: this.content = new ArrayList();
            content: Vec::new(),
            // Java: this.name = name;
            name,
            rootElement: None,
            docType: None,
            entityResolver: None,
            base: super::AbstractDocument::AbstractDocument::new(),
        };
        // Java: this.setRootElement(rootElement);
        if root_element.is_some() {
            doc.rootElement = root_element;
        }
        // Java: this.docType = docType;
        doc.docType = doc_type;
        doc
    }

    // Java: String getName()
    pub fn getName(&self) -> Option<&String> {
        // Java: return this.name;
        self.name.as_ref()
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: Option<String>) {
        // Java: this.name = name;
        self.name = name;
    }

    // Java: Element getRootElement()
    pub fn getRootElement(&self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.rootElement;
        self.rootElement.as_ref()
    }

    // Java: DocumentType getDocType()
    pub fn getDocType(&self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.docType;
        self.docType.as_ref()
    }

    // Java: void setDocType(DocumentType docType)
    pub fn setDocType(&mut self, doc_type: Option<Box<dyn std::any::Any + Send + Sync>>) {
        // Java: this.docType = docType;
        self.docType = doc_type;
    }

    // Java: Document addDocType(String docTypeName, String publicId, String systemId)
    pub fn addDocType(&mut self, _doc_type_name: &str, _public_id: &str, _system_id: &str) -> &mut Self {
        // Java: this.setDocType(new DefaultDocumentType(this.name, publicId, systemId));
        todo!();
        // Java: return this;
    }

    // Java: String getXMLEncoding()
    pub fn getXMLEncoding(&self) -> Option<&Box<dyn std::any::Any>> {
        // Java: return this.encoding;
        self.base.encoding.as_ref()
    }

    // Java: EntityResolver getEntityResolver()
    pub fn getEntityResolver(&self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.entityResolver;
        self.entityResolver.as_ref()
    }

    // Java: void setEntityResolver(EntityResolver entityResolver)
    pub fn setEntityResolver(&mut self, entity_resolver: Option<Box<dyn std::any::Any + Send + Sync>>) {
        // Java: this.entityResolver = entityResolver;
        self.entityResolver = entity_resolver;
    }

    // Java: void clearContent()
    pub fn clearContent(&mut self) {
        // Java: this.contentRemoved();
        self.contentRemoved();
        // Java: this.contentList().clear();
        self.content.clear();
        // Java: this.rootElement = null;
        self.rootElement = None;
    }

    // Java: List<Node> contentList()
    pub fn contentList(&mut self) -> &mut Vec<Box<dyn std::any::Any + Send + Sync>> {
        // Java: IOAssert.assertNotNull("this.content should not be null", this.content);
        // Java: return this.content;
        &mut self.content
    }

    // Java: void addNode(Node node)
    pub fn addNode(&mut self, node: Option<Box<dyn std::any::Any + Send + Sync>>) {
        // Java: if (node != null)
        if let Some(node) = node {
            // Java: Document document = node.getDocument();
            // Java: if (document != null && document != this)
            {
                // Java: String message = "The Node already has an existing document: " + document;
                // Java: throw new IllegalAddException(this, node, message);
            }
            // Java: this.contentList().add(node);
            self.content.push(node);
            // Java: this.childAdded(node);
        }
    }

    // Java: void addNode(int index, Node node)
    pub fn addNode_at(&mut self, index: usize, node: Option<Box<dyn std::any::Any + Send + Sync>>) {
        // Java: if (node != null)
        if let Some(node) = node {
            // Java: Document document = node.getDocument();
            // Java: IOAssert.assertTrue("The Node already has an existing document.", document == null || document == this);
            // Java: this.content.add(index, node);
            if index <= self.content.len() {
                self.content.insert(index, node);
            }
            // Java: node.setDocument(this);
        }
    }

    // Java: boolean removeNode(Node node)
    pub fn removeNode(&mut self, _node: &dyn std::any::Any) -> bool {
        // Java: if (node == this.rootElement)
        {
            // Java: this.rootElement = null;
            self.rootElement = None;
        }
        // Java: if (this.contentList().remove(node))
        {
            // Java: this.childRemoved(node);
            // Java: return true;
        }
        // Java: else
        {
            // Java: return false;
            return false;
        }
    }

    // Java: void rootElementAdded(Element element)
    pub fn rootElementAdded(&mut self, element: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.rootElement = element;
        self.rootElement = Some(element);
        // Java: element.setDocument(this);
    }

    fn contentRemoved(&mut self) {
    }
}
