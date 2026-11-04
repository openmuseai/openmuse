// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.dom4j.Branch
// use crate::org::dom4j::Branch;
// Java: import org.dom4j.Document
// use crate::org::dom4j::Document;
// Java: import org.dom4j.DocumentFactory
// use crate::org::dom4j::DocumentFactory;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.Namespace
// use crate::org::dom4j::Namespace;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;
// Java: import org.dom4j.tree.NamespaceStack
// use crate::org::dom4j::tree::NamespaceStack;
// Java: import org.w3c.dom.DocumentType
// use io_wps_io::cn::wps::io::dom::DocumentType;
// Java: import org.w3c.dom.NamedNodeMap
// use crate::org::w3c::dom::NamedNodeMap;
// Java: import org.w3c.dom.Node
// use io_wps_io::cn::wps::io::dom::Node;
// Java: import org.w3c.dom.NodeList
// use crate::org::w3c::dom::NodeList;

// Java: public class DOMReader
pub struct DOMReader {
    // Java field: DocumentFactory factory
    pub(crate) factory: i32,
    // Java field: NamespaceStack namespaceStack
    namespaceStack: i32,
}

// Java: public class DOMReader
impl DOMReader {
    // Java: DOMReader()
    pub fn DOMReader(&self) {
      // Java: super();
      // Java: this.factory = DocumentFactory.getInstance();
      // Java: this.namespaceStack = new NamespaceStack(this.factory);
      todo!()
    }

    // Java: DOMReader(DocumentFactory factory)
    pub fn DOMReader_with_factory(&self, _factory: i32) {
      // Java: super();
      // Java: this.factory = factory;
      // Java: this.namespaceStack = new NamespaceStack(factory);
      todo!()
    }

    // Java: DocumentFactory getDocumentFactory()
    pub fn getDocumentFactory(&self) -> i32 {
      // Java: return this.factory;
      todo!()
    }

    // Java: void setDocumentFactory(DocumentFactory docFactory)
    pub fn setDocumentFactory(&self, _docFactory: i32) {
      // Java: this.factory = docFactory;
      // Java: this.namespaceStack.setDocumentFactory(this.factory);
      todo!()
    }

    // Java: Document read(org.w3c.dom.Document domDocument)
    pub fn read(&self, _domDocument: i32) -> i32 {
      // Java: if (domDocument instanceof Document)
      {
         // Java: return (Document)domDocument;
         todo!()
      }
      // Java: else
      {
         // Java: Document document = this.createDocument();
         // Java: this.clearNamespaceStack();
         // Java: NodeList nodeList = domDocument.getChildNodes();
         // Java: int i = 0;
         // Java: for(int size = nodeList.getLength(); i < size; ++i)
         {
            // Java: this.readTree(nodeList.item(i), document);
            todo!()
         }
         // Java: return document;
         todo!()
      }
    }

    // Java: void readTree(Node node, Branch current)
    pub fn readTree(&self, _node: i32, _current: i32) {
      // Java: Element element = null;
      // Java: Document document = null;
      // Java: if (current instanceof Element)
      {
         // Java: element = (Element)current;
         todo!()
      }
      // Java: else
      {
         // Java: document = (Document)current;
         todo!()
      }
      // Java: switch (node.getNodeType())
      {
         // Java: case 1:
            // Java: this.readElement(node, current);
            // Java: break;
         // Java: case 2:
         // Java: case 9:
         // Java: default:
            // Java: System.out.println("WARNING: Unknown DOM node type: " + node.getNodeType());
            // Java: break;
         // Java: case 3:
            // Java: element.addText(node.getNodeValue());
            // Java: break;
         // Java: case 4:
            // Java: element.addCDATA(node.getNodeValue());
            // Java: break;
         // Java: case 5:
            // Java: Node firstChild = node.getFirstChild();
            // Java: if (firstChild != null)
            {
               // Java: element.addEntity(node.getNodeName(), firstChild.getNodeValue());
               todo!()
            }
            // Java: else
            {
               // Java: element.addEntity(node.getNodeName(), "");
               todo!()
            }
            // Java: break;
         // Java: case 6:
            // Java: element.addEntity(node.getNodeName(), node.getNodeValue());
            // Java: break;
         // Java: case 7:
            // Java: if (current instanceof Element)
            {
               // Java: Element currentEl = (Element)current;
               // Java: currentEl.addProcessingInstruction(node.getNodeName(), node.getNodeValue());
               todo!()
            }
            // Java: else
            {
               // Java: Document currentDoc = (Document)current;
               // Java: currentDoc.addProcessingInstruction(node.getNodeName(), node.getNodeValue());
               todo!()
            }
            // Java: break;
         // Java: case 8:
            // Java: if (current instanceof Element)
            {
               // Java: ((Element)current).addComment(node.getNodeValue());
               todo!()
            }
            // Java: else
            {
               // Java: ((Document)current).addComment(node.getNodeValue());
               todo!()
            }
            // Java: break;
         // Java: case 10:
            // Java: DocumentType domDocType = (DocumentType)node;
            // Java: document.addDocType(domDocType.getName(), domDocType.getPublicId(), domDocType.getSystemId());
            todo!()
      }
    }

    // Java: void readElement(Node node, Branch current)
    pub fn readElement(&self, _node: i32, _current: i32) {
      // Java: int previouslyDeclaredNamespaces = this.namespaceStack.size();
      // Java: String namespaceUri = node.getNamespaceURI();
      // Java: String elementPrefix = node.getPrefix();
      // Java: if (elementPrefix == null)
      {
         // Java: elementPrefix = "";
         todo!()
      }
      // Java: NamedNodeMap attributeList = node.getAttributes();
      // Java: if (attributeList != null && namespaceUri == null)
      {
         // Java: Node attribute = attributeList.getNamedItem("xmlns");
         // Java: if (attribute != null)
         {
            // Java: namespaceUri = attribute.getNodeValue();
            // Java: elementPrefix = "";
            todo!()
         }
      }
      // Java: QName qName = this.namespaceStack.getQName(namespaceUri, node.getLocalName(), node.getNodeName());
      // Java: Element element = current.addElement(qName);
      // Java: if (attributeList != null)
      {
         // Java: int size = attributeList.getLength();
         // Java: List attributes = new ArrayList(size);
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: Node attribute = attributeList.item(i);
            // Java: String name = attribute.getNodeName();
            // Java: if (name.startsWith("xmlns"))
            {
               // Java: String prefix = this.getPrefix(name);
               // Java: String uri = attribute.getNodeValue();
               // Java: Namespace namespace = this.namespaceStack.addNamespace(prefix, uri);
               // Java: element.add(namespace);
               todo!()
            }
            // Java: else
            {
               // Java: attributes.add(attribute);
               todo!()
            }
         }
         // Java: size = attributes.size();
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: Node attribute = (Node)attributes.get(i);
            // Java: QName attributeQName = this.namespaceStack.getQName(attribute.getNamespaceURI(), attribute.getLocalName(), attribute.getNodeName());
            // Java: element.addAttribute(attributeQName, attribute.getNodeValue());
            todo!()
         }
      }
      // Java: NodeList children = node.getChildNodes();
      // Java: int i = 0;
      // Java: for(int size = children.getLength(); i < size; ++i)
      {
         // Java: Node child = children.item(i);
         // Java: this.readTree(child, element);
         todo!()
      }
      // Java: while(this.namespaceStack.size() > previouslyDeclaredNamespaces)
      {
         // Java: this.namespaceStack.pop();
         todo!()
      }
    }

    // Java: Namespace getNamespace(String prefix, String uri)
    pub fn getNamespace(&self, _prefix: &str, _uri: &str) -> i32 {
      // Java: return this.getDocumentFactory().createNamespace(prefix, uri);
      todo!()
    }

    // Java: Document createDocument()
    pub fn createDocument(&self) -> i32 {
      // Java: return this.getDocumentFactory().createDocument();
      todo!()
    }

    // Java: void clearNamespaceStack()
    pub fn clearNamespaceStack(&self) {
      // Java: this.namespaceStack.clear();
      // Java: if (!this.namespaceStack.contains(Namespace.XML_NAMESPACE))
      {
         // Java: this.namespaceStack.push(Namespace.XML_NAMESPACE);
         todo!()
      }
    }

    // Java: String getPrefix(String xmlnsDecl)
    pub fn getPrefix(&self, _xmlnsDecl: &str) -> String {
      // Java: int index = xmlnsDecl.indexOf(58, 5);
      // Java: return index != -1 ? xmlnsDecl.substring(index + 1) : "";
      todo!()
    }

}
