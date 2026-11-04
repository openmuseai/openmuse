// Java: import java.util.List
// Java: import org.dom4j.Attribute
// use crate::org::dom4j::Attribute;
// Java: import org.dom4j.CDATA
// use crate::org::dom4j::CDATA;
// Java: import org.dom4j.Comment
// use crate::org::dom4j::Comment;
// Java: import org.dom4j.DocumentException
// use crate::org::dom4j::DocumentException;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.Entity
// use crate::org::dom4j::Entity;
// Java: import org.dom4j.Namespace
// use crate::org::dom4j::Namespace;
// Java: import org.dom4j.ProcessingInstruction
// use crate::org::dom4j::ProcessingInstruction;
// Java: import org.dom4j.Text
// use crate::org::dom4j::Text;
// Java: import org.dom4j.tree.NamespaceStack
// use crate::org::dom4j::tree::NamespaceStack;
// Java: import org.w3c.dom.CDATASection
// use crate::org::w3c::dom::CDATASection;
// Java: import org.w3c.dom.DOMImplementation
// use crate::org::w3c::dom::DOMImplementation;
// Java: import org.w3c.dom.Document
// use io_wps_io::cn::wps::io::dom::Document;
// Java: import org.w3c.dom.DocumentType
// use io_wps_io::cn::wps::io::dom::DocumentType;
// Java: import org.w3c.dom.EntityReference
// use crate::org::w3c::dom::EntityReference;
// Java: import org.w3c.dom.Node
// use io_wps_io::cn::wps::io::dom::Node;

// Java field: boolean loggedWarning
static LOGGED_WARNING: std::sync::OnceLock<std::sync::Mutex<bool>> = std::sync::OnceLock::new();
// Java field: String[] DEFAULT_DOM_DOCUMENT_CLASSES
static DEFAULT_DOM_DOCUMENT_CLASSES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();

// Java: public class DOMWriter
pub struct DOMWriter {
    // Java field: Class domDocumentClass
    domDocumentClass: i32,
    // Java field: NamespaceStack namespaceStack
    namespaceStack: i32,
}

// Java: public class DOMWriter
impl DOMWriter {
    // Java: DOMWriter()
    pub fn DOMWriter(&self) {
      // Java: super();
      todo!()
    }

    // Java: DOMWriter(Class domDocumentClass)
    pub fn DOMWriter_with_class(&self, _domDocumentClass: i32) {
      // Java: super();
      // Java: this.domDocumentClass = domDocumentClass;
      todo!()
    }

    // Java: Class getDomDocumentClass()
    pub fn getDomDocumentClass(&self) -> i32 {
      // Java: Class result = this.domDocumentClass;
      // Java: if (result == null)
      {
         // Java: int size = DEFAULT_DOM_DOCUMENT_CLASSES.length;
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: try
            {
               // Java: String name = DEFAULT_DOM_DOCUMENT_CLASSES[i];
               // Java: result = Class.forName(name, true, DOMWriter.class.getClassLoader());
               // Java: if (result != null)
               {
                  // Java: break;
                  todo!()
               }
            }
            // Java: catch (Exception var5)
            {
            }
         }
      }
      // Java: return result;
      todo!()
    }

    // Java: void setDomDocumentClass(Class domDocumentClass)
    pub fn setDomDocumentClass(&self, _domDocumentClass: i32) {
      // Java: this.domDocumentClass = domDocumentClass;
      todo!()
    }

    // Java: void setDomDocumentClassName(String name)
    pub fn setDomDocumentClassName(&self, _name: &str) {
      // Java: try
      {
         // Java: this.domDocumentClass = Class.forName(name, true, DOMWriter.class.getClassLoader());
         todo!()
      }
      // Java: catch (Exception e)
      {
         // Java: throw new DocumentException("Could not load the DOM Document class: " + name, e);
         todo!()
      }
    }

    // Java: Document write(org.dom4j.Document document)
    pub fn write(&self, _document: i32) -> i32 {
      // Java: if (document instanceof Document)
      {
         // Java: return (Document)document;
         todo!()
      }
      // Java: else
      {
         // Java: this.resetNamespaceStack();
         // Java: Document domDocument = this.createDomDocument(document);
         // Java: this.appendDOMTree(domDocument, domDocument, (List)document.content());
         // Java: this.namespaceStack.clear();
         // Java: return domDocument;
         todo!()
      }
    }

    // Java: Document write(org.dom4j.Document document, DOMImplementation domImpl)
    pub fn write_with_impl(&self, _document: i32, _domImpl: i32) -> i32 {
      // Java: if (document instanceof Document)
      {
         // Java: return (Document)document;
         todo!()
      }
      // Java: else
      {
         // Java: this.resetNamespaceStack();
         // Java: Document domDocument = this.createDomDocument(document, domImpl);
         // Java: this.appendDOMTree(domDocument, domDocument, (List)document.content());
         // Java: this.namespaceStack.clear();
         // Java: return domDocument;
         todo!()
      }
    }

    // Java: void appendDOMTree(Document domDocument, Node domCurrent, List content)
    pub fn appendDOMTree(&self, _domDocument: i32, _domCurrent: i32, _content: i32) {
      // Java: int size = content.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Object object = content.get(i);
         // Java: if (object instanceof Element)
         {
            // Java: this.appendDOMTree(domDocument, domCurrent, (Element)object);
            todo!()
         }
         // Java: else if (object instanceof String)
         {
            // Java: this.appendDOMTree(domDocument, domCurrent, (String)object);
            todo!()
         }
         // Java: else if (object instanceof Text)
         {
            // Java: Text text = (Text)object;
            // Java: this.appendDOMTree(domDocument, domCurrent, text.getText());
            todo!()
         }
         // Java: else if (object instanceof CDATA)
         {
            // Java: this.appendDOMTree(domDocument, domCurrent, (CDATA)object);
            todo!()
         }
         // Java: else if (object instanceof Comment)
         {
            // Java: this.appendDOMTree(domDocument, domCurrent, (Comment)object);
            todo!()
         }
         // Java: else if (object instanceof Entity)
         {
            // Java: this.appendDOMTree(domDocument, domCurrent, (Entity)object);
            todo!()
         }
         // Java: else if (object instanceof ProcessingInstruction)
         {
            // Java: this.appendDOMTree(domDocument, domCurrent, (ProcessingInstruction)object);
            todo!()
         }
      }
    }

    // Java: void appendDOMTree(Document domDocument, Node domCurrent, Element element)
    pub fn appendDOMTree_element(&self, _domDocument: i32, _domCurrent: i32, _element: i32) {
      // Java: String elUri = element.getNamespaceURI();
      // Java: String elName = element.getQualifiedName();
      // Java: org.w3c.dom.Element domElement = domDocument.createElementNS(elUri, elName);
      // Java: int stackSize = this.namespaceStack.size();
      // Java: Namespace elementNamespace = element.getNamespace();
      // Java: if (this.isNamespaceDeclaration(elementNamespace))
      {
         // Java: this.namespaceStack.push(elementNamespace);
         // Java: this.writeNamespace(domElement, elementNamespace);
         todo!()
      }
      // Java: List declaredNamespaces = element.declaredNamespaces();
      // Java: int i = 0;
      // Java: for(int size = declaredNamespaces.size(); i < size; ++i)
      {
         // Java: Namespace namespace = (Namespace)declaredNamespaces.get(i);
         // Java: if (this.isNamespaceDeclaration(namespace))
         {
            // Java: this.namespaceStack.push(namespace);
            // Java: this.writeNamespace(domElement, namespace);
            todo!()
         }
      }
      // Java: i = 0;
      // Java: for(int size = element.attributeCount(); i < size; ++i)
      {
         // Java: Attribute attribute = element.attribute(i);
         // Java: String attUri = attribute.getNamespaceURI();
         // Java: String attName = attribute.getQualifiedName();
         // Java: String value = attribute.getValue();
         // Java: domElement.setAttributeNS(attUri, attName, value);
         todo!()
      }
      // Java: this.appendDOMTree(domDocument, domElement, (List)element.content());
      // Java: domCurrent.appendChild(domElement);
      // Java: while(this.namespaceStack.size() > stackSize)
      {
         // Java: this.namespaceStack.pop();
         todo!()
      }
    }

    // Java: void appendDOMTree(Document domDocument, Node domCurrent, CDATA cdata)
    pub fn appendDOMTree_cdata(&self, _domDocument: i32, _domCurrent: i32, _cdata: i32) {
      // Java: CDATASection domCDATA = domDocument.createCDATASection(cdata.getText());
      // Java: domCurrent.appendChild(domCDATA);
      todo!()
    }

    // Java: void appendDOMTree(Document domDocument, Node domCurrent, Comment comment)
    pub fn appendDOMTree_comment(&self, _domDocument: i32, _domCurrent: i32, _comment: i32) {
      // Java: org.w3c.dom.Comment domComment = domDocument.createComment(comment.getText());
      // Java: domCurrent.appendChild(domComment);
      todo!()
    }

    // Java: void appendDOMTree(Document domDocument, Node domCurrent, String text)
    pub fn appendDOMTree_text(&self, _domDocument: i32, _domCurrent: i32, _text: &str) {
      // Java: org.w3c.dom.Text domText = domDocument.createTextNode(text);
      // Java: domCurrent.appendChild(domText);
      todo!()
    }

    // Java: void appendDOMTree(Document domDocument, Node domCurrent, Entity entity)
    pub fn appendDOMTree_entity(&self, _domDocument: i32, _domCurrent: i32, _entity: i32) {
      // Java: EntityReference domEntity = domDocument.createEntityReference(entity.getName());
      // Java: domCurrent.appendChild(domEntity);
      todo!()
    }

    // Java: void appendDOMTree(Document domDoc, Node domCurrent, ProcessingInstruction pi)
    pub fn appendDOMTree_pi(&self, _domDoc: i32, _domCurrent: i32, _pi: i32) {
      // Java: org.w3c.dom.ProcessingInstruction domPI = domDoc.createProcessingInstruction(pi.getTarget(), pi.getText());
      // Java: domCurrent.appendChild(domPI);
      todo!()
    }

    // Java: void writeNamespace(org.w3c.dom.Element domElement, Namespace namespace)
    pub fn writeNamespace(&self, _domElement: i32, _namespace: i32) {
      // Java: String attributeName = this.attributeNameForNamespace(namespace);
      // Java: domElement.setAttribute(attributeName, namespace.getURI());
      todo!()
    }

    // Java: String attributeNameForNamespace(Namespace namespace)
    pub fn attributeNameForNamespace(&self, _namespace: i32) -> String {
      // Java: String xmlns = "xmlns";
      // Java: String prefix = namespace.getPrefix();
      // Java: return prefix.length() > 0 ? xmlns + ":" + prefix : xmlns;
      todo!()
    }

    // Java: Document createDomDocument(org.dom4j.Document document)
    pub fn createDomDocument(&self, _document: i32) -> i32 {
      // Java: Document result = null;
      // Java: if (this.domDocumentClass != null)
      {
         // Java: try
         {
            // Java: result = (Document)this.domDocumentClass.newInstance();
            todo!()
         }
         // Java: catch (Exception e)
         {
            // Java: throw new DocumentException("Could not instantiate an instance of DOM Document with class: " + this.domDocumentClass.getName(), e);
            todo!()
         }
      }
      // Java: else
      {
         // Java: result = this.createDomDocumentViaJAXP();
         // Java: if (result == null)
         {
            // Java: Class theClass = this.getDomDocumentClass();
            // Java: try
            {
               // Java: result = (Document)theClass.newInstance();
               todo!()
            }
            // Java: catch (Exception e)
            {
               // Java: throw new DocumentException("Could not instantiate an instance of DOM Document with class: " + theClass.getName(), e);
               todo!()
            }
         }
      }
      // Java: return result;
      todo!()
    }

    // Java: Document createDomDocumentViaJAXP()
    pub fn createDomDocumentViaJAXP(&self) -> i32 {
      // Java: try
      {
         // Java: return JAXPHelper.createDocument(false, true);
         todo!()
      }
      // Java: catch (Throwable e)
      {
         // Java: if (!loggedWarning)
         {
            // Java: loggedWarning = true;
            // Java: if (SAXHelper.isVerboseErrorReporting())
            {
               // Java: System.out.println("Warning: Caught exception attempting to use JAXP to create a W3C DOM document");
               // Java: System.out.println("Warning: Exception was: " + e);
               // Java: e.printStackTrace();
               todo!()
            }
            // Java: else
            {
               // Java: System.out.println("Warning: Error occurred using JAXP to create a DOM document.");
               todo!()
            }
         }
         // Java: return null;
         todo!()
      }
    }

    // Java: Document createDomDocument(org.dom4j.Document document, DOMImplementation domImpl)
    pub fn createDomDocument_with_impl(&self, _document: i32, _domImpl: i32) -> i32 {
      // Java: String namespaceURI = null;
      // Java: String qualifiedName = null;
      // Java: DocumentType docType = null;
      // Java: return domImpl.createDocument(namespaceURI, qualifiedName, docType);
      todo!()
    }

    // Java: boolean isNamespaceDeclaration(Namespace ns)
    pub fn isNamespaceDeclaration(&self, _ns: i32) -> bool {
      // Java: if (ns != null && ns != Namespace.NO_NAMESPACE && ns != Namespace.XML_NAMESPACE)
      {
         // Java: String uri = ns.getURI();
         // Java: if (uri != null && uri.length() > 0 && !this.namespaceStack.contains(ns))
         {
            // Java: return true;
            todo!()
         }
      }
      // Java: return false;
      todo!()
    }

    // Java: void resetNamespaceStack()
    pub fn resetNamespaceStack(&self) {
      // Java: this.namespaceStack.clear();
      // Java: this.namespaceStack.push(Namespace.XML_NAMESPACE);
      todo!()
    }

}
