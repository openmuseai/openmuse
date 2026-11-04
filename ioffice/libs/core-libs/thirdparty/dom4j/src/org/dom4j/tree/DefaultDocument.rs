// Java: import cn.wps.base.assertion.Assert
// use util::cn::wps::core::runtime::Assert;
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.dom4j.Document
use crate::org::dom4j::Document;
// Java: import org.dom4j.DocumentFactory
use crate::org::dom4j::DocumentFactory;
// Java: import org.dom4j.DocumentType
use crate::org::dom4j::DocumentType;
// Java: import org.dom4j.Element
use crate::org::dom4j::Element;
// Java: import org.dom4j.IllegalAddException
use crate::org::dom4j::IllegalAddException;
// Java: import org.dom4j.Node
use crate::org::dom4j::Node;
// Java: import org.dom4j.NodeHelper
use crate::org::dom4j::NodeHelper;
// Java: import org.dom4j.ProcessingInstruction
use crate::org::dom4j::ProcessingInstruction;
// Java: import org.xml.sax.EntityResolver

// Java: public class DefaultDocument extends AbstractDocument
pub struct DefaultDocument {
    // Java field: String name
    pub(crate) name: i32,
    // Java field: Element rootElement
    pub(crate) rootElement: i32,
    // Java field: List<Node> content
    content: Vec<i32>,
    // Java field: DocumentType docType
    pub(crate) docType: i32,
    // Java field: DocumentFactory documentFactory
    documentFactory: i32,
    // Java field: EntityResolver entityResolver
    pub(crate) entityResolver: i32,
}

// Java: public class DefaultDocument extends AbstractDocument
impl DefaultDocument {
    // Java: DefaultDocument()
    pub fn new() -> Self { return Self { name: 0, rootElement: 0, content: Vec::new(), docType: 0, documentFactory: 0, entityResolver: 0 }; } pub fn _unused_ctor(&self) {
      // Java: this((String)null, (Element)null, (DocumentType)null);
    }

    // Java: DefaultDocument(String name)
    pub fn DefaultDocument_name(&self, _name: &str) {
      // Java: this(name, (Element)null, (DocumentType)null);
    }

    // Java: DefaultDocument(Element rootElement)
    pub fn DefaultDocument_root(&self, _rootElement: &str) {
      // Java: this((String)null, rootElement, (DocumentType)null);
    }

    // Java: DefaultDocument(DocumentType docType)
    pub fn DefaultDocument_doctype(&self, _docType: &str) {
      // Java: this((String)null, (Element)null, docType);
    }

    // Java: DefaultDocument(Element rootElement, DocumentType docType)
    pub fn DefaultDocument_root_doctype(&self, _rootElement: &str, _docType: &str) {
      // Java: this((String)null, rootElement, docType);
    }

    // Java: DefaultDocument(String name, Element rootElement, DocumentType docType)
    pub fn DefaultDocument_full(&self, _name: &str, _rootElement: &str, _docType: &str) {
      // Java: super();
      // Java: this.content = new ArrayList();
      // Java: this.documentFactory = DocumentFactory.getInstance();
      // Java: this.name = name;
      // Java: this.setRootElement(rootElement);
      // Java: this.docType = docType;
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
      // Java: return this.name;
     String::new() }

    // Java: void setName(String name)
    pub fn setName(&self, _name: &str) {
      // Java: this.name = name;
    }

    // Java: Element getRootElement()
    pub fn getRootElement(&self) {
      // Java: return this.rootElement;
    }

    // Java: DocumentType getDocType()
    pub fn getDocType(&self) {
      // Java: return this.docType;
    }

    // Java: void setDocType(DocumentType docType)
    pub fn setDocType(&self, _docType: &str) {
      // Java: this.docType = docType;
    }

    // Java: Document addDocType(String docTypeName, String publicId, String systemId)
    pub fn addDocType(&self, _docTypeName: &str, _publicId: &str, _systemId: &str) {
      // Java: this.setDocType(this.getDocumentFactory().createDocType(docTypeName, publicId, systemId));
      // Java: return this;
    }

    // Java: String getXMLEncoding()
    pub fn getXMLEncoding(&self) -> String {
      // Java: return this.encoding;
     String::new() }

    // Java: EntityResolver getEntityResolver()
    pub fn getEntityResolver(&self) {
      // Java: return this.entityResolver;
    }

    // Java: void setEntityResolver(EntityResolver entityResolver)
    pub fn setEntityResolver(&self, _entityResolver: &str) {
      // Java: this.entityResolver = entityResolver;
    }

    // Java: DefaultDocument clone()
    pub fn clone(&self) {
      // Java: DefaultDocument document = (DefaultDocument)super.clone();
      // Java: document.rootElement = null;
      // Java: CloneHelper.setFinalContent(DefaultDocument.class, document);
      // Java: document.appendContent(this);
      // Java: return document;
    }

    // Java: List<ProcessingInstruction> processingInstructions()
    pub fn processingInstructions(&self) {
      // Java: List<ProcessingInstruction> answer = this.createResultList();
      // Java: for(Node node : this.contentList())
      {
         // Java: ProcessingInstruction pi = NodeHelper.nodeAsProcessingInstruction(node);
         // Java: if (pi != null)
         {
            // Java: answer.add(pi);
         }
      }
      // Java: return answer;
    }

    // Java: List<ProcessingInstruction> processingInstructions(String target)
    pub fn processingInstructions_target(&self, _target: &str) {
      // Java: List<ProcessingInstruction> answer = this.createResultList();
      // Java: for(Node node : this.contentList())
      {
         // Java: ProcessingInstruction pi = NodeHelper.nodeAsProcessingInstruction(node);
         // Java: if (pi != null && target.equals(pi.getName()))
         {
            // Java: answer.add(pi);
         }
      }
      // Java: return answer;
    }

    // Java: ProcessingInstruction processingInstruction(String target)
    pub fn processingInstruction(&self, _target: &str) {
      // Java: for(Node node : this.contentList())
      {
         // Java: ProcessingInstruction pi = NodeHelper.nodeAsProcessingInstruction(node);
         // Java: if (pi != null && target.equals(pi.getName()))
         {
            // Java: return pi;
         }
      }
      // Java: return null;
    }

    // Java: boolean removeProcessingInstruction(String target)
    pub fn removeProcessingInstruction(&self, _target: &str) -> bool {
      // Java: Iterator<Node> iter = this.contentList().iterator();
      // Java: while(iter.hasNext())
      {
         // Java: Node node = (Node)iter.next();
         // Java: ProcessingInstruction pi = NodeHelper.nodeAsProcessingInstruction(node);
         // Java: if (pi != null && target.equals(pi.getName()))
         {
            // Java: iter.remove();
            // Java: return true;
          false }
      }
      // Java: return false;
    }

    // Java: void setContent(List<Node> content)
    pub fn setContent(&self, _content: &str) {
      // Java: this.rootElement = null;
      // Java: this.contentRemoved();
      // Java: if (content instanceof ContentListFacade)
      {
         // Java: content = ((ContentListFacade<Node>)content).getBackingList();
      }
      // Java: this.contentList().clear();
      // Java: if (content != null)
      {
         // Java: List<Node> newContent = this.createContentList();
         // Java: for(Node node : content)
         {
            // Java: Document doc = node.getDocument();
            // Java: if (doc != null && doc != this)
            {
               // Java: node = (Node)node.clone();
            }
            // Java: Element element = NodeHelper.nodeAsElement(node);
            // Java: if (element != null)
            {
               // Java: if (this.rootElement != null)
               {
                  // Java: throw new IllegalAddException("A document may only contain one root element: " + content);
               }
               // Java: this.rootElement = element;
            }
            // Java: newContent.add(node);
            // Java: this.childAdded(node);
         }
         // Java: this.contentList().addAll(newContent);
      }
    }

    // Java: void clearContent()
    pub fn clearContent(&self) {
      // Java: this.contentRemoved();
      // Java: this.contentList().clear();
      // Java: this.rootElement = null;
    }

    // Java: void setDocumentFactory(DocumentFactory documentFactory)
    pub fn setDocumentFactory(&self, _documentFactory: &str) {
      // Java: this.documentFactory = documentFactory;
    }

    // Java: List<Node> contentList()
    pub fn contentList(&self) {
      // Java: Assert.assertNotNull("this.content should not be null", this.content);
      // Java: return this.content;
    }

    // Java: void addNode(Node node)
    pub fn addNode(&self, _node: &str) {
      // Java: if (node != null)
      {
         // Java: Document document = node.getDocument();
         // Java: if (document != null && document != this)
         {
            // Java: String message = "The Node already has an existing document: " + document;
            // Java: throw new IllegalAddException(this, node, message);
         }
         // Java: this.contentList().add(node);
         // Java: this.childAdded(node);
      }
    }

    // Java: void addNode(int index, Node node)
    pub fn addNode_index(&self, _index: &str, _node: &str) {
      // Java: if (node != null)
      {
         // Java: Document document = node.getDocument();
         // Java: if (document != null && document != this)
         {
            // Java: String message = "The Node already has an existing document: " + document;
            // Java: throw new IllegalAddException(this, node, message);
         }
         // Java: this.contentList().add(index, node);
         // Java: this.childAdded(node);
      }
    }

    // Java: boolean removeNode(Node node)
    pub fn removeNode(&self, _node: &str) -> bool {
      // Java: if (node == this.rootElement)
      {
         // Java: this.rootElement = null;
      }
      // Java: if (this.contentList().remove(node))
      {
         // Java: this.childRemoved(node);
         // Java: return true;
      }
      // Java: else
      {
         // Java: return false;
      }
      false
    }

    // Java: void rootElementAdded(Element element)
    pub fn rootElementAdded(&self, _element: &str) {
      // Java: this.rootElement = element;
      // Java: element.setDocument(this);
    }

    // Java: DocumentFactory getDocumentFactory()
    pub fn getDocumentFactory(&self) {
      // Java: return this.documentFactory;
    }

}
