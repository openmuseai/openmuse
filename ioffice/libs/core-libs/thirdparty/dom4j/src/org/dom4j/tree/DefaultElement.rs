// Java: import cn.wps.base.assertion.Assert
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.dom4j.Attribute
use crate::org::dom4j::Attribute;
// Java: import org.dom4j.Branch
use crate::org::dom4j::Branch;
// Java: import org.dom4j.Document
use crate::org::dom4j::Document;
// Java: import org.dom4j.DocumentFactory
use crate::org::dom4j::DocumentFactory;
// Java: import org.dom4j.Element
use crate::org::dom4j::Element;
// Java: import org.dom4j.IllegalAddException
use crate::org::dom4j::IllegalAddException;
// Java: import org.dom4j.Namespace
use crate::org::dom4j::Namespace;
// Java: import org.dom4j.Node
use crate::org::dom4j::Node;
// Java: import org.dom4j.NodeHelper
use crate::org::dom4j::NodeHelper;
// Java: import org.dom4j.NodeType
use crate::org::dom4j::NodeType;
// Java: import org.dom4j.ProcessingInstruction
use crate::org::dom4j::ProcessingInstruction;
// Java: import org.dom4j.QName
use crate::org::dom4j::QName;

// Java field: DocumentFactory DOCUMENT_FACTORY
// Java field: DocumentFactory DOCUMENT_FACTORY

// Java: public class DefaultElement extends AbstractElement
pub struct DefaultElement {
    // Java field: QName qname
    pub(crate) qname: i32,
    // Java field: Branch parentBranch
    parentBranch: i32,
    // Java field: List<Node> mContent
    pub(crate) mContent: Vec<i32>,
    // Java field: List<Attribute> mAttributes
    pub(crate) mAttributes: Vec<i32>,
}

// Java: public class DefaultElement extends AbstractElement
impl DefaultElement {
    pub fn new() -> Self {
      Self { qname: 0, parentBranch: 0, mContent: Vec::new(), mAttributes: Vec::new() }
    }

    // Java: DefaultElement(String name)
    pub fn DefaultElement(&self, _name: &str) {
      // Java: this(DOCUMENT_FACTORY.createQName(name));
    }

    // Java: DefaultElement(QName qname)
    pub fn DefaultElement_2(&self, _qname: &str) {
      // Java: this(qname, 0);
    }

    // Java: DefaultElement(QName qname, int attributeCount)
    pub fn DefaultElement_3(&self, _qname: &str, _attributeCount: &str) {
      // Java: super();
      // Java: this.mContent = new ArrayList();
      // Java: this.qname = qname;
      // Java: this.mAttributes = new ArrayList(attributeCount);
    }

    // Java: DefaultElement(String name, Namespace namespace)
    pub fn DefaultElement_4(&self, _name: &str, _namespace: &str) {
      // Java: this(DOCUMENT_FACTORY.createQName(name, namespace));
    }

    // Java: Element getParent()
    pub fn getParent(&self) {
      // Java: Element result = null;
      // Java: if (this.parentBranch instanceof Element)
      {
         // Java: result = (Element)this.parentBranch;
      }
      // Java: return result;
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&self, _parent: &str) {
      // Java: if (parent != null || this.parentBranch instanceof Element)
      {
         // Java: this.parentBranch = parent;
      }
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) {
      // Java: if (this.parentBranch == null)
      {
         // Java: return null;
      }
      // Java: else if (this.parentBranch instanceof Document)
      {
         // Java: return (Document)this.parentBranch;
      }
      // Java: else if (this.parentBranch instanceof Element)
      {
         // Java: Element parent = (Element)this.parentBranch;
         // Java: return parent.getDocument();
      }
      // Java: else
      {
         // Java: return null;
      }
    }

    // Java: void setDocument(Document document)
    pub fn setDocument(&self, _document: &str) {
      // Java: if (document != null || this.parentBranch instanceof Document)
      {
         // Java: this.parentBranch = document;
      }
    }

    // Java: boolean supportsParent()
    pub fn supportsParent(&self) -> bool {
      // Java: return true;
      true
    }

    // Java: QName getQName()
    pub fn getQName(&self) {
      // Java: return this.qname;
    }

    // Java: void setQName(QName name)
    pub fn setQName(&self, _name: &str) {
      // Java: this.qname = name;
    }

    // Java: String getStringValue()
    pub fn getStringValue(&self) -> String {
      // Java: if (this.mContent.size() == 1)
      // Java: return this.getContentAsStringValue((Node)this.mContent.get(0));
      // Java: else if (this.mContent.size() == 0)
      // Java: return "";
      // Java: else
      // Java: StringBuilder builder = new StringBuilder();
      // Java: for(Node node : this.mContent)
      // Java: String string = this.getContentAsStringValue(node);
      // Java: if (string.length() > 0)
      // Java: builder.append(string);
      // Java: return builder.toString();
      todo!()
    }

    // Java: DefaultElement clone()
    pub fn clone(&self) {
      // Java: DefaultElement answer = (DefaultElement)super.clone();
      // Java: if (answer != this)
      {
         // Java: CloneHelper.setFinalLazyList(DefaultElement.class, answer, "attributes");
         // Java: CloneHelper.setFinalContent(DefaultElement.class, answer);
         // Java: answer.appendAttributes(this);
         // Java: answer.appendContent(this);
      }
      // Java: return answer;
    }

    // Java: Namespace getNamespaceForPrefix(String prefix)
    pub fn getNamespaceForPrefix(&self, _prefix: &str) {
      // Java: if (prefix == null)
      {
         // Java: prefix = "";
      }
      // Java: if (prefix.equals(this.getNamespacePrefix()))
      {
         // Java: return this.getNamespace();
      }
      // Java: else if (prefix.equals("xml"))
      {
         // Java: return Namespace.XML_NAMESPACE;
      }
      // Java: else
      {
         // Java: for(Node node : this.mContent)
         {
            // Java: Namespace namespace = NodeHelper.nodeAsNamespace(node);
            // Java: if (namespace != null && prefix.equals(namespace.getPrefix()))
            {
               // Java: return namespace;
            }
         }
         // Java: Element parent = this.getParent();
         // Java: if (parent != null)
         {
            // Java: Namespace answer = parent.getNamespaceForPrefix(prefix);
            // Java: if (answer != null)
            {
               // Java: return answer;
            }
         }
         // Java: if (prefix != null && prefix.length() > 0)
         {
            // Java: return null;
         }
         // Java: else
         {
            // Java: return Namespace.NO_NAMESPACE;
         }
      }
    }

    // Java: Namespace getNamespaceForURI(String uri)
    pub fn getNamespaceForURI(&self, _uri: &str) {
      // Java: if (uri != null && uri.length() > 0)
      {
         // Java: if (uri.equals(this.getNamespaceURI()))
         {
            // Java: return this.getNamespace();
         }
         // Java: else
         {
            // Java: for(Node node : this.mContent)
            {
               // Java: Namespace namespace = NodeHelper.nodeAsNamespace(node);
               // Java: if (namespace != null && uri.equals(namespace.getURI()))
               {
                  // Java: return namespace;
               }
            }
            // Java: Element parent = this.getParent();
            // Java: if (parent != null)
            {
               // Java: return parent.getNamespaceForURI(uri);
            }
            // Java: else
            {
               // Java: return null;
            }
         }
      }
      // Java: else
      {
         // Java: return Namespace.NO_NAMESPACE;
      }
    }

    // Java: List<Namespace> declaredNamespaces()
    pub fn declaredNamespaces(&self) {
      // Java: BackedList<Namespace> answer = this.createResultList();
      // Java: if (this.mContent != null)
      {
         // Java: for(Node node : this.mContent)
         {
            // Java: Namespace namespace = NodeHelper.nodeAsNamespace(node);
            // Java: if (namespace != null)
            {
               // Java: answer.addLocal(namespace);
            }
         }
      }
      // Java: return answer;
    }

    // Java: List<Namespace> additionalNamespaces()
    pub fn additionalNamespaces(&self) {
      // Java: BackedList<Namespace> answer = this.createResultList();
      // Java: for(Node node : this.mContent)
      {
         // Java: Namespace namespace = NodeHelper.nodeAsNamespace(node);
         // Java: if (namespace != null && !namespace.equals(this.getNamespace()))
         {
            // Java: answer.addLocal(namespace);
         }
      }
      // Java: return answer;
    }

    // Java: List<Namespace> additionalNamespaces(String defaultNamespaceURI)
    pub fn additionalNamespaces_2(&self, _defaultNamespaceURI: &str) {
      // Java: BackedList<Namespace> answer = this.createResultList();
      // Java: for(Node node : this.mContent)
      {
         // Java: Namespace namespace = NodeHelper.nodeAsNamespace(node);
         // Java: if (namespace != null && !defaultNamespaceURI.equals(namespace.getURI()))
         {
            // Java: answer.addLocal(namespace);
         }
      }
      // Java: return answer;
    }

    // Java: List<ProcessingInstruction> processingInstructions()
    pub fn processingInstructions(&self) {
      // Java: BackedList<ProcessingInstruction> answer = this.createResultList();
      // Java: for(Node node : this.mContent)
      {
         // Java: ProcessingInstruction pi = NodeHelper.nodeAsProcessingInstruction(node);
         // Java: if (pi != null)
         {
            // Java: answer.addLocal(pi);
         }
      }
      // Java: return answer;
    }

    // Java: List<ProcessingInstruction> processingInstructions(String target)
    pub fn processingInstructions_2(&self, _target: &str) {
      // Java: BackedList<ProcessingInstruction> answer = this.createResultList();
      // Java: for(Node node : this.mContent)
      {
         // Java: ProcessingInstruction pi = NodeHelper.nodeAsProcessingInstruction(node);
         // Java: if (pi != null && target.equals(pi.getName()))
         {
            // Java: answer.addLocal(pi);
         }
      }
      // Java: return answer;
    }

    // Java: ProcessingInstruction processingInstruction(String target)
    pub fn processingInstruction(&self, _target: &str) {
      // Java: for(Node node : this.mContent)
      // Java: ProcessingInstruction pi = NodeHelper.nodeAsProcessingInstruction(node);
      // Java: if (pi != null && target.equals(pi.getName()))
      // Java: return pi;
      // Java: return null;
      todo!()
    }

    // Java: boolean removeProcessingInstruction(String target)
    pub fn removeProcessingInstruction(&self, _target: &str) -> bool {
      // Java: Iterator<? extends Node> iterator = this.mContent.iterator();
      // Java: while(iterator.hasNext())
      // Java: ProcessingInstruction pi = NodeHelper.nodeAsProcessingInstruction((Node)iterator.next());
      // Java: if (pi != null && target.equals(pi.getName()))
      // Java: iterator.remove();
      // Java: return true;
      // Java: return false;
      todo!()
    }

    // Java: Element element(String name)
    pub fn element(&self, _name: &str) {
      // Java: for(Node node : this.mContent)
      {
         // Java: Element element = NodeHelper.nodeAsElement(node);
         // Java: if (element != null && name.equals(element.getName()))
         {
            // Java: return element;
         }
      }
      // Java: return null;
    }

    // Java: Element element(QName qName)
    pub fn element_2(&self, _qName: &str) {
      // Java: for(Node node : this.mContent)
      {
         // Java: Element element = NodeHelper.nodeAsElement(node);
         // Java: if (element != null && qName.equals(element.getQName()))
         {
            // Java: return element;
         }
      }
      // Java: return null;
    }

    // Java: Element element(String name, Namespace namespace)
    pub fn element_3(&self, _name: &str, _namespace: &str) {
      // Java: return this.element(this.getDocumentFactory().createQName(name, namespace));
    }

    // Java: Element lastElement(String name)
    pub fn lastElement(&self, _name: &str) {
      // Java: int size = this.mContent.size();
      // Java: for(int i = size - 1; i >= 0; --i)
      {
         // Java: Node node = (Node)this.mContent.get(i);
         // Java: if (name.equals(node.getName()))
         {
            // Java: if (node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
            {
               // Java: Assert.assertTrue("node instanceof Element should be true", node instanceof Element);
               // Java: return (Element)node;
            }
            // Java: return null;
         }
      }
      // Java: return null;
    }

    // Java: Element lastElement(QName qName)
    pub fn lastElement_2(&self, _qName: &str) {
      // Java: int size = this.mContent.size();
      // Java: for(int i = size - 1; i >= 0; --i)
      {
         // Java: Node node = (Node)this.mContent.get(i);
         // Java: if (node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
         {
            // Java: Assert.assertTrue("node instanceof Element should be true", node instanceof Element);
            // Java: Element element = (Element)node;
            // Java: if (qName.equals(element.getQName()))
            {
               // Java: return element;
            }
         }
      }
      // Java: return null;
    }

    // Java: Element lastElement(String name, Namespace namespace)
    pub fn lastElement_3(&self, _name: &str, _namespace: &str) {
      // Java: return this.lastElement(this.getDocumentFactory().createQName(name, namespace));
    }

    // Java: List<Element> elements()
    pub fn elements(&self) {
      // Java: BackedList<Element> answer = this.createResultList();
      // Java: int size = this.mContent.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)this.mContent.get(i);
         // Java: if (node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
         {
            // Java: Assert.assertTrue("node instanceof Element should be true", node instanceof Element);
            // Java: answer.addLocal((Element)node);
         }
      }
      // Java: return answer;
    }

    // Java: List<Element> elements(String name)
    pub fn elements_2(&self, _name: &str) {
      // Java: ArrayList<Element> answer = new ArrayList();
      // Java: int size = this.mContent.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)this.mContent.get(i);
         // Java: if (name.equals(node.getName()) && node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
         {
            // Java: Assert.assertTrue("node instanceof Element should be true", node instanceof Element);
            // Java: answer.add((Element)node);
         }
      }
      // Java: return answer;
    }

    // Java: List<Element> elements(QName qName)
    pub fn elements_3(&self, _qName: &str) {
      // Java: BackedList<Element> answer = this.createResultList();
      // Java: for(Node node : this.mContent)
      {
         // Java: NodeHelper.appendElementQNamedLocal(node, answer, qName);
      }
      // Java: return answer;
    }

    // Java: void setContent(List<Node> content)
    pub fn setContent(&self, _content: &str) {
      // Java: this.contentRemoved();
      // Java: if (content instanceof ContentListFacade)
      {
         // Java: content = ((ContentListFacade<Node>)content).getBackingList();
      }
      // Java: List<Node> newContent = this.createContentList();
      // Java: if (content != null)
      {
         // Java: for(Node node : content)
         {
            // Java: Element parent = node.getParent();
            // Java: if (parent != null && parent != this)
            {
               // Java: node = (Node)node.clone();
            }
            // Java: newContent.add(node);
            // Java: this.childAdded(node);
         }
      }
      // Java: this.mContent = newContent;
    }

    // Java: void clearContent()
    pub fn clearContent(&self) {
      // Java: Assert.assertNotNull("mContent should not be null", this.mContent);
      // Java: this.contentRemoved();
      // Java: this.mContent.clear();
    }

    // Java: Node node(int index)
    pub fn node(&self, _index: &str) {
      // Java: return index >= 0 && index < this.mContent.size() ? (Node)this.mContent.get(index) : null;
    }

    // Java: int indexOf(Node node)
    pub fn indexOf(&self, _node: &str) -> i32 {
      // Java: return this.mContent.indexOf(node);
      todo!()
    }

    // Java: int nodeCount()
    pub fn nodeCount(&self) -> i32 {
      // Java: return this.mContent.size();
      todo!()
    }

    // Java: Iterator<Node> nodeIterator()
    pub fn nodeIterator(&self) {
      // Java: return this.mContent.iterator();
      todo!()
    }

    // Java: List<Attribute> attributes()
    pub fn attributes(&self) {
      // Java: return new ContentListFacade<Attribute>(this, this.mAttributes);
    }

    // Java: void setAttributes(List<Attribute> attributes)
    pub fn setAttributes(&self, _attributes: &str) {
      // Java: if (attributes instanceof ContentListFacade)
      {
         // Java: attributes = ((ContentListFacade<Attribute>)attributes).getBackingList();
      }
      // Java: this.mAttributes.clear();
      // Java: if (attributes != null)
      {
         // Java: this.mAttributes.addAll(attributes);
      }
    }

    // Java: Iterator<Attribute> attributeIterator()
    pub fn attributeIterator(&self) {
      // Java: return this.mAttributes.iterator();
      todo!()
    }

    // Java: Attribute attribute(int index)
    pub fn attribute(&self, _index: &str) {
      // Java: return index >= 0 && index < this.mAttributes.size() ? (Attribute)this.mAttributes.get(index) : null;
    }

    // Java: int attributeCount()
    pub fn attributeCount(&self) -> i32 {
      // Java: return this.mAttributes.size();
      todo!()
    }

    // Java: Attribute attribute(String name)
    pub fn attribute_2(&self, _name: &str) {
      // Java: int size = this.mAttributes.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Attribute attribute = (Attribute)this.mAttributes.get(i);
         // Java: if (name.equals(attribute.getName()))
         {
            // Java: return attribute;
         }
      }
      // Java: return null;
    }

    // Java: Attribute attribute(QName qName)
    pub fn attribute_3(&self, _qName: &str) {
      // Java: for(Attribute attribute : this.mAttributes)
      {
         // Java: if (qName.equals(attribute.getQName()))
         {
            // Java: return attribute;
         }
      }
      // Java: return null;
    }

    // Java: Attribute attribute(String name, Namespace namespace)
    pub fn attribute_4(&self, _name: &str, _namespace: &str) {
      // Java: return this.attribute(this.getDocumentFactory().createQName(name, namespace));
    }

    // Java: void add(Attribute attribute)
    pub fn add(&self, _attribute: &str) {
      // Java: if (attribute.getParent() != null)
      {
         // Java: String message = "The Attribute already has an existing parent \"" + attribute.getParent().getQualifiedName() + "\"";
         // Java: throw new IllegalAddException(this, attribute, message);
      }
      // Java: else
      {
         // Java: if (attribute.getValue() == null)
         {
            // Java: Attribute oldAttribute = this.attribute(attribute.getQName());
            // Java: if (oldAttribute != null)
            {
               // Java: this.remove(oldAttribute);
            }
         }
         // Java: else
         {
            // Java: this.mAttributes.add(attribute);
            // Java: this.childAdded(attribute);
         }
      }
    }

    // Java: boolean remove(Attribute attribute)
    pub fn remove(&self, _attribute: &str) -> bool {
      // Java: boolean answer = this.mAttributes.remove(attribute);
      // Java: if (!answer)
      // Java: Attribute copy = this.attribute(attribute.getQName());
      // Java: if (copy != null)
      // Java: answer = this.mAttributes.remove(copy);
      // Java: if (answer)
      // Java: this.childRemoved(attribute);
      // Java: return answer;
      todo!()
    }

    // Java: void addNewNode(Node node)
    pub fn addNewNode(&self, _node: &str) {
      // Java: this.mContent.add(node);
      // Java: this.childAdded(node);
    }

    // Java: boolean removeNode(Node node)
    pub fn removeNode(&self, _node: &str) -> bool {
      // Java: boolean answer = this.mContent.remove(node);
      // Java: if (answer)
      // Java: this.childRemoved(node);
      // Java: return answer;
      todo!()
    }

    // Java: List<Node> contentList()
    pub fn contentList(&self) {
      // Java: Assert.assertNotNull("mContent should not be null", this.mContent);
      // Java: return this.mContent;
    }

    // Java: void setAttributeList(List<Attribute> attributeList)
    pub fn setAttributeList(&self, _attributeList: &str) {
      // Java: this.mAttributes.clear();
      // Java: this.mAttributes.addAll(attributeList);
    }

    // Java: DocumentFactory getDocumentFactory()
    pub fn getDocumentFactory(&self) {
      // Java: DocumentFactory factory = this.qname.getDocumentFactory();
      // Java: return factory != null ? factory : DOCUMENT_FACTORY;
    }

    // Java: List<Attribute> attributeList()
    pub fn attributeList(&self) {
      // Java: Assert.assertNotNull("mAttributes should not be null", this.mAttributes);
      // Java: return this.mAttributes;
    }

    // Java: List<Attribute> attributeList(int attributeCount)
    pub fn attributeList_2(&self, _attributeCount: &str) {
      // Java: Assert.assertNotNull("mAttributes should not be null", this.mAttributes);
      // Java: return this.mAttributes;
    }

}
