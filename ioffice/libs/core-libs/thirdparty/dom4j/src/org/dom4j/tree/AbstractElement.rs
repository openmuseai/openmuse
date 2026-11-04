// Java: import cn.wps.base.assertion.Assert
// Java: import java.io.IOException
// Java: import java.io.StringWriter
// Java: import java.io.Writer
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.dom4j.Attribute
// Java: import org.dom4j.CDATA
// Java: import org.dom4j.CharacterData
// Java: import org.dom4j.Comment
// Java: import org.dom4j.Document
// Java: import org.dom4j.DocumentFactory
// Java: import org.dom4j.Element
// Java: import org.dom4j.Entity
// Java: import org.dom4j.IllegalAddException
// Java: import org.dom4j.Namespace
// Java: import org.dom4j.Node
// Java: import org.dom4j.NodeHelper
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.ProcessingInstruction
// Java: import org.dom4j.QName
// Java: import org.dom4j.Text
// Java: import org.dom4j.Visitor
// Java: import org.dom4j.io.MCEContext
// Java: import org.dom4j.io.OutputFormat
// Java: import org.dom4j.io.XMLWriter
// Java: import org.xml.sax.Attributes

// Java field: DocumentFactory DOCUMENT_FACTORY
// Java field: boolean VERBOSE_TOSTRING
pub(crate) const VERBOSE_TOSTRING: bool = false;
// Java field: boolean USE_STRINGVALUE_SEPARATOR
pub(crate) const USE_STRINGVALUE_SEPARATOR: bool = false;

// Java: public class AbstractElement extends AbstractBranch implements Element
pub struct AbstractElement {
}

// Java: public class AbstractElement extends AbstractBranch implements Element
impl AbstractElement {
    // Java: AbstractElement()
    pub fn new() -> Self {
      // Java: super();
        AbstractElement {}
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self)  {
      // Java: return NodeType.ELEMENT_NODE;
    }

    // Java: boolean isRootElement()
    pub fn isRootElement(&self) -> bool {
      // Java: Document document = this.getDocument();
      // Java: if (document != null)
      {
         // Java: Element root = document.getRootElement();
         // Java: if (root == this)
         {
            // Java: return true;
          false }
      }
      // Java: return false;
    }

    // Java: void setName(String name)
    pub fn setName(&self) {
      // Java: this.setQName(this.getDocumentFactory().createQName(name));
    }

    // Java: void setNamespace(Namespace namespace)
    pub fn setNamespace(&self) {
      // Java: this.setQName(this.getDocumentFactory().createQName(this.getName(), namespace));
    }

    // Java: String getXPathNameStep()
    pub fn getXPathNameStep(&self) -> String {
      // Java: String uri = this.getNamespaceURI();
      // Java: if (uri != null && uri.length() != 0)
      {
         // Java: String prefix = this.getNamespacePrefix();
         // Java: return prefix != null && prefix.length() != 0 ? this.getQualifiedName() : "*[name()='" + this.getName() + "']";
      }
      // Java: else
      {
         // Java: return this.getName();
      }
      String::new()
    }

    // Java: String getPath(Element context)
    pub fn getPath(&self) {
      // Java: if (this == context)
      {
         // Java: return ".";
      }
      // Java: else
      {
         // Java: Element parent = this.getParent();
         // Java: if (parent == null)
         {
            // Java: return "/" + this.getXPathNameStep();
         }
         // Java: else
         {
            // Java: return parent == context ? this.getXPathNameStep() : parent.getPath(context) + "/" + this.getXPathNameStep();
         }
      }
    }

    // Java: String getUniquePath(Element context)
    pub fn getUniquePath(&self) {
      // Java: Element parent = this.getParent();
      // Java: if (parent == null)
      {
         // Java: return "/" + this.getXPathNameStep();
      }
      // Java: else
      {
         // Java: StringBuffer buffer = new StringBuffer();
         // Java: if (parent != context)
         {
            // Java: buffer.append(parent.getUniquePath(context));
            // Java: buffer.append("/");
         }
         // Java: buffer.append(this.getXPathNameStep());
         // Java: List<Element> mySiblings = parent.elements(this.getQName());
         // Java: if (mySiblings.size() > 1)
         {
            // Java: int idx = mySiblings.indexOf(this);
            // Java: if (idx >= 0)
            {
               // Java: buffer.append("[");
               // Java: ++idx;
               // Java: buffer.append(Integer.toString(idx));
               // Java: buffer.append("]");
            }
         }
         // Java: return buffer.toString();
      }
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
      // Java: try
      {
         // Java: StringWriter out = new StringWriter();
         // Java: XMLWriter writer = new XMLWriter(out, new OutputFormat());
         // Java: writer.write((Element)this);
         // Java: writer.flush();
         // Java: return out.toString();
      }
      // Java: catch (IOException e)
      {
         // Java: throw new RuntimeException("IOException while generating textual representation: " + e.getMessage());
      }
      String::new()
    }

    // Java: void write(Writer out)
    pub fn write(&self) {
      // Java: XMLWriter writer = new XMLWriter(out, new OutputFormat());
      // Java: writer.write((Element)this);
    }

    // Java: void accept(Visitor visitor)
    pub fn accept(&self) {
      // Java: visitor.visit((Element)this);
      // Java: int i = 0;
      // Java: for(int size = this.attributeCount(); i < size; ++i)
      {
         // Java: Attribute attribute = this.attribute(i);
         // Java: visitor.visit(attribute);
      }
      // Java: i = 0;
      // Java: for(int size = this.nodeCount(); i < size; ++i)
      {
         // Java: Node node = this.node(i);
         // Java: node.accept(visitor);
      }
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString(&self) {
      // Java: String uri = this.getNamespaceURI();
      // Java: super.toString(builder);
      // Java: builder.append(" [Element: <");
      // Java: builder.append(this.getQualifiedName());
      // Java: if (uri != null && uri.length() > 0)
      {
         // Java: builder.append(" uri: ");
         // Java: builder.append(uri);
      }
      // Java: builder.append(" attributes: ");
      // Java: builder.append(this.attributeList());
      // Java: builder.append("/>]");
    }

    // Java: Namespace getNamespace()
    pub fn getNamespace(&self)  {
      // Java: return this.getQName().getNamespace();
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
      // Java: return this.getQName().getName();
     String::new() }

    // Java: String getNamespacePrefix()
    pub fn getNamespacePrefix(&self) -> String {
      // Java: return this.getQName().getNamespacePrefix();
     String::new() }

    // Java: String getNamespaceURI()
    pub fn getNamespaceURI(&self) -> String {
      // Java: return this.getQName().getNamespaceURI();
     String::new() }

    // Java: String getQualifiedName()
    pub fn getQualifiedName(&self) -> String {
      // Java: return this.getQName().getQualifiedName();
     String::new() }

    // Java: Object getData()
    pub fn getData(&self)  {
      // Java: return this.getText();
    }

    // Java: void setData(Object data)
    pub fn setData(&self) {
    }

    // Java: Node node(int index)
    pub fn node(&self) {
      // Java: if (index >= 0)
      {
         // Java: List<Node> list = this.contentList();
         // Java: return index >= list.size() ? null : (Node)list.get(index);
      }
      // Java: else
      {
         // Java: return null;
      }
    }

    // Java: int indexOf(Node node)
    pub fn indexOf(&self) {
      // Java: return this.contentList().indexOf(node);
    }

    // Java: int nodeCount()
    pub fn nodeCount(&self) -> i32 {
      // Java: return this.contentList().size();
     0 }

    // Java: Iterator<Node> nodeIterator()
    pub fn nodeIterator(&self)  {
      // Java: return this.contentList().iterator();
    }

    // Java: Element element(String name)
    pub fn element_name(&self) {
      // Java: for(Node node : this.contentList())
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
    pub fn element_qname(&self) {
      // Java: for(Node node : this.contentList())
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
    pub fn element_ns(&self) {
      // Java: return this.element(this.getDocumentFactory().createQName(name, namespace));
    }

    // Java: Element lastElement(String name)
    pub fn lastElement_name(&self) {
      // Java: List<Node> content = this.contentList();
      // Java: int size = content.size();
      // Java: for(int i = size - 1; i >= 0; --i)
      {
         // Java: Node node = (Node)content.get(i);
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
    pub fn lastElement_qname(&self) {
      // Java: List<Node> content = this.contentList();
      // Java: int size = content.size();
      // Java: for(int i = size - 1; i >= 0; --i)
      {
         // Java: Node node = (Node)content.get(i);
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
    pub fn lastElement_ns(&self) {
      // Java: return this.lastElement(this.getDocumentFactory().createQName(name, namespace));
    }

    // Java: Element[] elementsRO()
    pub fn elementsRO(&self) -> Vec<String> {
      // Java: ArrayList<Element> answer = new ArrayList();
      // Java: List<Node> content = this.contentList();
      // Java: int size = content.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)content.get(i);
         // Java: if (node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
         {
            // Java: Assert.assertTrue("node instanceof Element should be true", node instanceof Element);
            // Java: answer.add((Element)node);
          Vec::new() }
      }
      // Java: return (Element[])answer.toArray(new Element[answer.size()]);
    }

    // Java: List<Element> elements()
    pub fn elements(&self) -> Vec<String> {
      // Java: ArrayList<Element> answer = new ArrayList();
      // Java: List<Node> content = this.contentList();
      // Java: int size = content.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)content.get(i);
         // Java: if (node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
         {
            // Java: Assert.assertTrue("node instanceof Element should be true", node instanceof Element);
            // Java: answer.add((Element)node);
          Vec::new() }
      }
      // Java: return answer;
    }

    // Java: Element[] elementsRO(String name)
    pub fn elementsRO_name(&self) {
      // Java: ArrayList<Element> answer = new ArrayList();
      // Java: List<Node> content = this.contentList();
      // Java: int size = content.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)content.get(i);
         // Java: if (name.equals(node.getName()) && node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
         {
            // Java: Assert.assertTrue("node instanceof Element should be true", node instanceof Element);
            // Java: answer.add((Element)node);
         }
      }
      // Java: return (Element[])answer.toArray(new Element[answer.size()]);
    }

    // Java: List<Element> elements(String name)
    pub fn elements_name(&self) -> Vec<String> {
      // Java: BackedList<Element> answer = this.createResultList();
      // Java: List<Node> content = this.contentList();
      // Java: int size = content.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)content.get(i);
         // Java: if (name.equals(node.getName()) && node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
         {
            // Java: Assert.assertTrue("node instanceof Element should be true", node instanceof Element);
            // Java: answer.addLocal((Element)node);
          Vec::new() }
      }
      // Java: return answer;
    }

    // Java: List<Element> elements(QName qName)
    pub fn elements_qname(&self) -> Vec<String> {
      // Java: BackedList<Element> answer = this.createResultList();
      // Java: for(Node node : this.contentList())
      {
         // Java: NodeHelper.appendElementQNamedLocal(node, answer, qName);
       Vec::new() }
      // Java: return answer;
    }

    // Java: List<Element> elements(String name, Namespace namespace)
    pub fn elements_ns(&self) -> Vec<String> {
      // Java: return this.elements(this.getDocumentFactory().createQName(name, namespace));
     Vec::new() }

    // Java: Iterator<Element> elementIterator()
    pub fn elementIterator(&self)  {
      // Java: return this.elements().iterator();
    }

    // Java: Iterator<Element> elementIterator(String name)
    pub fn elementIterator_name(&self) {
      // Java: return this.elements(name).iterator();
    }

    // Java: Iterator<Element> elementIterator(QName qName)
    pub fn elementIterator_qname(&self) {
      // Java: return this.elements(qName).iterator();
    }

    // Java: Iterator<Element> elementIterator(String name, Namespace ns)
    pub fn elementIterator_ns(&self) {
      // Java: return this.elementIterator(this.getDocumentFactory().createQName(name, ns));
    }

    // Java: List<Attribute> attributes()
    pub fn attributes(&self) -> Vec<String> {
      // Java: return new ContentListFacade<Attribute>(this, this.attributeList());
     Vec::new() }

    // Java: Iterator<Attribute> attributeIterator()
    pub fn attributeIterator(&self)  {
      // Java: return this.attributeList().iterator();
    }

    // Java: Attribute attribute(int index)
    pub fn attribute_index(&self) {
      // Java: return (Attribute)this.attributeList().get(index);
    }

    // Java: int attributeCount()
    pub fn attributeCount(&self) -> i32 {
      // Java: return this.attributeList().size();
     0 }

    // Java: Attribute attribute(String name)
    pub fn attribute_name(&self) {
      // Java: List<Attribute> attributes = this.attributeList();
      // Java: int size = attributes.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Attribute attribute = (Attribute)attributes.get(i);
         // Java: if (name.equals(attribute.getName()))
         {
            // Java: return attribute;
         }
      }
      // Java: return null;
    }

    // Java: Attribute attribute(QName qName)
    pub fn attribute_qname(&self) {
      // Java: for(Attribute attribute : this.attributeList())
      {
         // Java: if (qName.equals(attribute.getQName()))
         {
            // Java: return attribute;
         }
      }
      // Java: return null;
    }

    // Java: Attribute attribute(String name, Namespace namespace)
    pub fn attribute_ns(&self) {
      // Java: return this.attribute(this.getDocumentFactory().createQName(name, namespace));
    }

    // Java: void setAttributes(Attributes attributes, int size, NamespaceStack namespaceStack, boolean noNamespaceAttributes, MCEContext mceContext)
    pub fn setAttributes(&self) {
      // Java: if (size > 0)
      {
         // Java: DocumentFactory factory = this.getDocumentFactory();
         // Java: List<Attribute> list = this.attributeList(size);
         // Java: list.clear();
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: String attributeName = attributes.getQName(i);
            // Java: if (noNamespaceAttributes || !attributeName.startsWith("xmlns"))
            {
               // Java: String attributeURI = attributes.getURI(i);
               // Java: String attributeLocalName = attributes.getLocalName(i);
               // Java: String attributeValue = attributes.getValue(i);
               // Java: if (mceContext.mMceDisabled || !mceContext.isInMCEOrIgnoredNamespace(attributeURI))
               {
                  // Java: QName attributeQName = namespaceStack.getAttributeQName(attributeURI, attributeLocalName, attributeName);
                  // Java: Attribute attribute = factory.createAttribute(this, (QName)attributeQName, attributeValue);
                  // Java: list.add(attribute);
                  // Java: this.childAdded(attribute);
               }
            }
         }
      }
    }

    // Java: String attributeValue(String name)
    pub fn attributeValue_name(&self) {
      // Java: return NodeHelper.getAttributeValue(this.attribute(name), (String)null);
    }

    // Java: String attributeValue(QName qName)
    pub fn attributeValue_qname(&self) {
      // Java: return NodeHelper.getAttributeValue(this.attribute(qName), (String)null);
    }

    // Java: String attributeValue(String name, String defaultValue)
    pub fn attributeValue_default(&self) {
      // Java: return NodeHelper.getAttributeValue(this.attribute(name), defaultValue);
    }

    // Java: String attributeValue(QName qName, String defaultValue)
    pub fn attributeValue_qname_default(&self) {
      // Java: return NodeHelper.getAttributeValue(this.attribute(qName), defaultValue);
    }

    // Java: void add(Attribute attribute)
    pub fn add_attribute(&self) {
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
            // Java: this.attributeList().add(attribute);
            // Java: this.childAdded(attribute);
         }
      }
    }

    // Java: boolean remove(Attribute attribute)
    pub fn remove_attribute(&self) {
      // Java: List<Attribute> list = this.attributeList();
      // Java: boolean answer = list.remove(attribute);
      // Java: if (answer)
      {
         // Java: this.childRemoved(attribute);
      }
      // Java: else
      {
         // Java: Attribute copy = this.attribute(attribute.getQName());
         // Java: if (copy != null)
         {
            // Java: list.remove(copy);
            // Java: answer = true;
         }
      }
      // Java: return answer;
    }

    // Java: List<ProcessingInstruction> processingInstructions()
    pub fn processingInstructions(&self) -> Vec<String> {
      // Java: List<? extends Node> list = this.contentList();
      // Java: BackedList<ProcessingInstruction> answer = this.createResultList();
      // Java: int size = list.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)list.get(i);
         // Java: if (node instanceof ProcessingInstruction)
         {
            // Java: answer.addLocal((ProcessingInstruction)node);
          Vec::new() }
      }
      // Java: return answer;
    }

    // Java: List<ProcessingInstruction> processingInstructions(String target)
    pub fn processingInstructions_target(&self) {
      // Java: List<? extends Node> list = this.contentList();
      // Java: BackedList<ProcessingInstruction> answer = this.createResultList();
      // Java: int size = list.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)list.get(i);
         // Java: if (node instanceof ProcessingInstruction)
         {
            // Java: ProcessingInstruction pi = (ProcessingInstruction)node;
            // Java: if (target.equals(pi.getName()))
            {
               // Java: answer.addLocal(pi);
            }
         }
      }
      // Java: return answer;
    }

    // Java: ProcessingInstruction processingInstruction(String target)
    pub fn processingInstruction(&self) {
      // Java: List<? extends Node> list = this.contentList();
      // Java: int size = list.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)list.get(i);
         // Java: if (node instanceof ProcessingInstruction)
         {
            // Java: ProcessingInstruction pi = (ProcessingInstruction)node;
            // Java: if (target.equals(pi.getName()))
            {
               // Java: return pi;
            }
         }
      }
      // Java: return null;
    }

    // Java: boolean removeProcessingInstruction(String target)
    pub fn removeProcessingInstruction(&self) {
      // Java: List<? extends Node> list = this.contentList();
      // Java: Iterator<? extends Node> iter = list.iterator();
      // Java: while(iter.hasNext())
      {
         // Java: Node node = (Node)iter.next();
         // Java: if (node instanceof ProcessingInstruction)
         {
            // Java: ProcessingInstruction pi = (ProcessingInstruction)node;
            // Java: if (target.equals(pi.getName()))
            {
               // Java: iter.remove();
               // Java: return true;
            }
         }
      }
      // Java: return false;
    }

    // Java: Node getXPathResult(int index)
    pub fn getXPathResult(&self) {
      // Java: Node answer = this.node(index);
      // Java: return answer != null && !answer.supportsParent() ? answer.asXPathResult(this) : answer;
    }

    // Java: Element addAttribute(String name, String value)
    pub fn addAttribute_name(&self) {
      // Java: Attribute attribute = this.attribute(name);
      // Java: if (value != null)
      {
         // Java: if (attribute == null)
         {
            // Java: this.add(this.getDocumentFactory().createAttribute(this, (String)name, value));
         }
         // Java: else if (attribute.isReadOnly())
         {
            // Java: this.remove(attribute);
            // Java: this.add(this.getDocumentFactory().createAttribute(this, (String)name, value));
         }
         // Java: else
         {
            // Java: attribute.setValue(value);
         }
      }
      // Java: else if (attribute != null)
      {
         // Java: this.remove(attribute);
      }
      // Java: return this;
    }

    // Java: Element addAttribute(QName qName, String value)
    pub fn addAttribute_qname(&self) {
      // Java: Attribute attribute = this.attribute(qName);
      // Java: if (value != null)
      {
         // Java: if (attribute == null)
         {
            // Java: this.add(this.getDocumentFactory().createAttribute(this, (QName)qName, value));
         }
         // Java: else if (attribute.isReadOnly())
         {
            // Java: this.remove(attribute);
            // Java: this.add(this.getDocumentFactory().createAttribute(this, (QName)qName, value));
         }
         // Java: else
         {
            // Java: attribute.setValue(value);
         }
      }
      // Java: else if (attribute != null)
      {
         // Java: this.remove(attribute);
      }
      // Java: return this;
    }

    // Java: Element addCDATA(String cdata)
    pub fn addCDATA(&self) {
      // Java: CDATA node = this.getDocumentFactory().createCDATA(cdata);
      // Java: this.addNewNode(node);
      // Java: return this;
    }

    // Java: Element addComment(String comment)
    pub fn addComment(&self) {
      // Java: Comment node = this.getDocumentFactory().createComment(comment);
      // Java: this.addNewNode(node);
      // Java: return this;
    }

    // Java: Element addElement(String name)
    pub fn addElement(&self) {
      // Java: DocumentFactory factory = this.getDocumentFactory();
      // Java: int index = name.indexOf(":");
      // Java: String prefix = "";
      // Java: String localName = name;
      // Java: Namespace namespace = null;
      // Java: if (index > 0)
      {
         // Java: prefix = name.substring(0, index);
         // Java: localName = name.substring(index + 1);
         // Java: namespace = this.getNamespaceForPrefix(prefix);
         // Java: if (namespace == null)
         {
            // Java: throw new IllegalAddException("No such namespace prefix: " + prefix + " is in scope on: " + this + " so cannot add element: " + name);
         }
      }
      // Java: else
      {
         // Java: namespace = this.getNamespaceForPrefix("");
      }
      // Java: Element node;
      // Java: if (namespace != null)
      {
         // Java: QName qname = factory.createQName(localName, namespace);
         // Java: node = factory.createElement(qname);
      }
      // Java: else
      {
         // Java: node = factory.createElement(name);
      }
      // Java: this.addNewNode(node);
      // Java: return node;
    }

    // Java: Element addEntity(String name, String text)
    pub fn addEntity(&self) {
      // Java: Entity node = this.getDocumentFactory().createEntity(name, text);
      // Java: this.addNewNode(node);
      // Java: return this;
    }

    // Java: Element addNamespace(String prefix, String uri)
    pub fn addNamespace(&self) {
      // Java: Namespace node = this.getDocumentFactory().createNamespace(prefix, uri);
      // Java: this.addNewNode(node);
      // Java: return this;
    }

    // Java: Element addProcessingInstruction(String target, String data)
    pub fn addProcessingInstruction(&self) {
      // Java: ProcessingInstruction node = this.getDocumentFactory().createProcessingInstruction(target, data);
      // Java: this.addNewNode(node);
      // Java: return this;
    }

    // Java: Element addProcessingInstruction(String target, Map<String, String> data)
    pub fn addProcessingInstruction_map(&self) {
      // Java: ProcessingInstruction node = this.getDocumentFactory().createProcessingInstruction(target, data);
      // Java: this.addNewNode(node);
      // Java: return this;
    }

    // Java: Element addText(String text)
    pub fn addText(&self) {
      // Java: Text node = this.getDocumentFactory().createText(text);
      // Java: this.addNewNode(node);
      // Java: return this;
    }

    // Java: void add(Node node)
    pub fn add_node(&self) {
      // Java: switch (node.getNodeTypeEnum())
      {
         // Java: case ELEMENT_NODE:
            // Java: this.add((Element)node);
            // Java: break;
         // Java: case ATTRIBUTE_NODE:
            // Java: this.add((Attribute)node);
            // Java: break;
         // Java: case TEXT_NODE:
            // Java: this.add((Text)node);
            // Java: break;
         // Java: case CDATA_SECTION_NODE:
            // Java: this.add((CDATA)node);
            // Java: break;
         // Java: case ENTITY_REFERENCE_NODE:
            // Java: this.add((Entity)node);
            // Java: break;
         // Java: case PROCESSING_INSTRUCTION_NODE:
            // Java: this.add((ProcessingInstruction)node);
            // Java: break;
         // Java: case COMMENT_NODE:
            // Java: this.add((Comment)node);
            // Java: break;
         // Java: case NAMESPACE_NODE:
            // Java: this.add((Namespace)node);
            // Java: break;
         // Java: default:
            // Java: this.invalidNodeTypeAddException(node);
      }
    }

    // Java: boolean remove(Node node)
    pub fn remove_node(&self) {
      // Java: switch (node.getNodeTypeEnum())
      {
         // Java: case ELEMENT_NODE:
            // Java: return this.remove((Element)node);
         // Java: case ATTRIBUTE_NODE:
            // Java: return this.remove((Attribute)node);
         // Java: case TEXT_NODE:
            // Java: return this.remove((Text)node);
         // Java: case CDATA_SECTION_NODE:
            // Java: return this.remove((CDATA)node);
         // Java: case ENTITY_REFERENCE_NODE:
            // Java: return this.remove((Entity)node);
         // Java: case PROCESSING_INSTRUCTION_NODE:
            // Java: return this.remove((ProcessingInstruction)node);
         // Java: case COMMENT_NODE:
            // Java: return this.remove((Comment)node);
         // Java: case NAMESPACE_NODE:
            // Java: return this.remove((Namespace)node);
         // Java: default:
            // Java: return false;
      }
    }

    // Java: void add(CDATA cdata)
    pub fn add_cdata(&self) {
      // Java: this.addNode(cdata);
    }

    // Java: void add(Comment comment)
    pub fn add_comment(&self) {
      // Java: this.addNode(comment);
    }

    // Java: void add(Element element)
    pub fn add_element(&self) {
      // Java: this.addNode(element);
    }

    // Java: void add(Entity entity)
    pub fn add_entity(&self) {
      // Java: this.addNode(entity);
    }

    // Java: void add(Namespace namespace)
    pub fn add_namespace(&self) {
      // Java: this.addNode(namespace);
    }

    // Java: void add(ProcessingInstruction pi)
    pub fn add_pi(&self) {
      // Java: this.addNode(pi);
    }

    // Java: void add(Text text)
    pub fn add_text(&self) {
      // Java: this.addNode(text);
    }

    // Java: boolean remove(CDATA cdata)
    pub fn remove_cdata(&self) {
      // Java: return this.removeNode(cdata);
    }

    // Java: boolean remove(Comment comment)
    pub fn remove_comment(&self) {
      // Java: return this.removeNode(comment);
    }

    // Java: boolean remove(Element element)
    pub fn remove_element(&self) {
      // Java: return this.removeNode(element);
    }

    // Java: boolean remove(Entity entity)
    pub fn remove_entity(&self) {
      // Java: return this.removeNode(entity);
    }

    // Java: boolean remove(Namespace namespace)
    pub fn remove_namespace(&self) {
      // Java: return this.removeNode(namespace);
    }

    // Java: boolean remove(ProcessingInstruction pi)
    pub fn remove_pi(&self) {
      // Java: return this.removeNode(pi);
    }

    // Java: boolean remove(Text text)
    pub fn remove_text(&self) {
      // Java: return this.removeNode(text);
    }

    // Java: boolean hasMixedContent()
    pub fn hasMixedContent(&self) -> bool {
      // Java: List<? extends Node> content = this.contentList();
      // Java: if (content != null && !content.isEmpty() && content.size() >= 2)
      {
         // Java: Class<? extends Node> prevClass = null;
         // Java: for(Node node : content)
         {
            // Java: Class<? extends Node> newClass = node.getClass();
            // Java: if (newClass != prevClass)
            {
               // Java: if (prevClass != null)
               {
                  // Java: return true;
               }
               // Java: prevClass = newClass;
            }
         }
         // Java: return false;
      }
      // Java: else
      {
         // Java: return false;
      }
      false
    }

    // Java: boolean isTextOnly()
    pub fn isTextOnly(&self) -> bool {
      // Java: List<? extends Node> content = this.contentList();
      // Java: if (content != null && !content.isEmpty())
      {
         // Java: for(Node node : content)
         {
            // Java: if (!(node instanceof CharacterData))
            {
               // Java: return false;
            }
         }
         // Java: return true;
      }
      // Java: else
      {
         // Java: return true;
      }
      false
    }

    // Java: void setText(String text)
    pub fn setText(&self) {
      // Java: List<Node> allContent = this.contentList();
      // Java: if (allContent != null)
      {
         // Java: Iterator<Node> it = allContent.iterator();
         // Java: while(it.hasNext())
         {
            // Java: Node node = (Node)it.next();
            // Java: switch (node.getNodeTypeEnum())
            {
               // Java: case TEXT_NODE:
               // Java: case CDATA_SECTION_NODE:
               // Java: case ENTITY_REFERENCE_NODE:
                  // Java: it.remove();
            }
         }
      }
      // Java: this.addText(text);
    }

    // Java: String getStringValue()
    pub fn getStringValue(&self) -> String {
      // Java: List<Node> list = this.contentList();
      // Java: int size = list.size();
      // Java: if (size > 0)
      {
         // Java: if (size == 1)
         {
            // Java: return this.getContentAsStringValue((Node)list.get(0));
         }
         // Java: else
         {
            // Java: StringBuffer buffer = new StringBuffer();
            // Java: for(int i = 0; i < size; ++i)
            {
               // Java: Node node = (Node)list.get(i);
               // Java: String string = this.getContentAsStringValue(node);
               // Java: if (string.length() > 0)
               {
                  // Java: buffer.append(string);
               }
            }
            // Java: return buffer.toString();
         }
      }
      // Java: else
      {
         // Java: return "";
      }
      String::new()
    }

    // Java: void normalize()
    pub fn normalize(&self) {
      // Java: List<Node> content = this.contentList();
      // Java: Text previousText = null;
      // Java: int i = 0;
      // Java: while(i < content.size())
      {
         // Java: Node node = (Node)content.get(i);
         // Java: if (node instanceof Text)
         {
            // Java: Text text = (Text)node;
            // Java: if (previousText != null)
            {
               // Java: previousText.appendText(text.getText());
               // Java: this.remove(text);
            }
            // Java: else
            {
               // Java: String value = text.getText();
               // Java: if (value != null && value.length() > 0)
               {
                  // Java: previousText = text;
                  // Java: ++i;
               }
               // Java: else
               {
                  // Java: this.remove(text);
               }
            }
         }
         // Java: else
         {
            // Java: if (node instanceof Element)
            {
               // Java: Element element = (Element)node;
               // Java: element.normalize();
            }
            // Java: previousText = null;
            // Java: ++i;
         }
      }
    }

    // Java: String elementText(String name)
    pub fn elementText(&self) {
      // Java: Element element = this.element(name);
      // Java: return element != null ? element.getText() : null;
    }

    // Java: String elementText(QName qName)
    pub fn elementText_qname(&self) {
      // Java: Element element = this.element(qName);
      // Java: return element != null ? element.getText() : null;
    }

    // Java: String elementTextTrim(String name)
    pub fn elementTextTrim(&self) {
      // Java: Element element = this.element(name);
      // Java: return element != null ? element.getTextTrim() : null;
    }

    // Java: String elementTextTrim(QName qName)
    pub fn elementTextTrim_qname(&self) {
      // Java: Element element = this.element(qName);
      // Java: return element != null ? element.getTextTrim() : null;
    }

    // Java: void appendAttributes(Element element)
    pub fn appendAttributes(&self) {
      // Java: int i = 0;
      // Java: for(int size = element.attributeCount(); i < size; ++i)
      {
         // Java: Attribute attribute = element.attribute(i);
         // Java: if (attribute.supportsParent())
         {
            // Java: this.addAttribute(attribute.getQName(), attribute.getValue());
         }
         // Java: else
         {
            // Java: this.add(attribute);
         }
      }
    }

    // Java: Element createCopy()
    pub fn createCopy(&self)  {
      // Java: Element clone = this.createElement(this.getQName());
      // Java: clone.appendAttributes(this);
      // Java: clone.appendContent(this);
      // Java: return clone;
    }

    // Java: Element createCopy(String name)
    pub fn createCopy_name(&self) {
      // Java: Element clone = this.createElement(name);
      // Java: clone.appendAttributes(this);
      // Java: clone.appendContent(this);
      // Java: return clone;
    }

    // Java: Element createCopy(QName qName)
    pub fn createCopy_qname(&self) {
      // Java: Element clone = this.createElement(qName);
      // Java: clone.appendAttributes(this);
      // Java: clone.appendContent(this);
      // Java: return clone;
    }

    // Java: QName getQName(String qualifiedName)
    pub fn getQName(&self) {
      // Java: String prefix = "";
      // Java: String localName = qualifiedName;
      // Java: int index = qualifiedName.indexOf(":");
      // Java: if (index > 0)
      {
         // Java: prefix = qualifiedName.substring(0, index);
         // Java: localName = qualifiedName.substring(index + 1);
      }
      // Java: Namespace namespace = this.getNamespaceForPrefix(prefix);
      // Java: return namespace != null ? this.getDocumentFactory().createQName(localName, namespace) : this.getDocumentFactory().createQName(localName);
    }

    // Java: Namespace getNamespaceForPrefix(String prefix)
    pub fn getNamespaceForPrefix(&self) {
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
         // Java: List<Node> list = this.contentList();
         // Java: int size = list.size();
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: Object object = list.get(i);
            // Java: if (object instanceof Namespace)
            {
               // Java: Namespace namespace = (Namespace)object;
               // Java: if (prefix.equals(namespace.getPrefix()))
               {
                  // Java: return namespace;
               }
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
    pub fn getNamespaceForURI(&self) {
      // Java: if (uri != null && uri.length() > 0)
      {
         // Java: if (uri.equals(this.getNamespaceURI()))
         {
            // Java: return this.getNamespace();
         }
         // Java: else
         {
            // Java: List<Node> list = this.contentList();
            // Java: int size = list.size();
            // Java: for(int i = 0; i < size; ++i)
            {
               // Java: Object object = list.get(i);
               // Java: if (object instanceof Namespace)
               {
                  // Java: Namespace namespace = (Namespace)object;
                  // Java: if (uri.equals(namespace.getURI()))
                  {
                     // Java: return namespace;
                  }
               }
            }
            // Java: return null;
         }
      }
      // Java: else
      {
         // Java: return Namespace.NO_NAMESPACE;
      }
    }

    // Java: List<Namespace> getNamespacesForURI(String uri)
    pub fn getNamespacesForURI(&self) {
      // Java: BackedList<Namespace> answer = this.createResultList();
      // Java: List<? extends Node> list = this.contentList();
      // Java: int size = list.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)list.get(i);
         // Java: if (node instanceof Namespace && ((Namespace)node).getURI().equals(uri))
         {
            // Java: answer.addLocal((Namespace)node);
         }
      }
      // Java: return answer;
    }

    // Java: List<Namespace> declaredNamespaces()
    pub fn declaredNamespaces(&self) -> Vec<String> {
      // Java: BackedList<Namespace> answer = this.createResultList();
      // Java: List<? extends Node> list = this.contentList();
      // Java: int size = list.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)list.get(i);
         // Java: if (node instanceof Namespace)
         {
            // Java: answer.addLocal((Namespace)node);
          Vec::new() }
      }
      // Java: return answer;
    }

    // Java: List<Namespace> additionalNamespaces()
    pub fn additionalNamespaces(&self) -> Vec<String> {
      // Java: List<? extends Node> list = this.contentList();
      // Java: int size = list.size();
      // Java: BackedList<Namespace> answer = this.createResultList();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)list.get(i);
         // Java: if (node instanceof Namespace)
         {
            // Java: Namespace namespace = (Namespace)node;
            // Java: if (!namespace.equals(this.getNamespace()))
            {
               // Java: answer.addLocal(namespace);
             Vec::new() }
         }
      }
      // Java: return answer;
    }

    // Java: List<Namespace> additionalNamespaces(String defaultNamespaceURI)
    pub fn additionalNamespaces_uri(&self) {
      // Java: List<? extends Node> list = this.contentList();
      // Java: BackedList<Namespace> answer = this.createResultList();
      // Java: int size = list.size();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = (Node)list.get(i);
         // Java: if (node instanceof Namespace)
         {
            // Java: Namespace namespace = (Namespace)node;
            // Java: if (!defaultNamespaceURI.equals(namespace.getURI()))
            {
               // Java: answer.addLocal(namespace);
            }
         }
      }
      // Java: return answer;
    }

    // Java: void ensureAttributesCapacity(int minCapacity)
    pub fn ensureAttributesCapacity(&self) {
      // Java: if (minCapacity > 1)
      {
         // Java: List<Attribute> list = this.attributeList();
         // Java: if (list instanceof ArrayList)
         {
            // Java: ArrayList<Attribute> arrayList = (ArrayList)list;
            // Java: arrayList.ensureCapacity(minCapacity);
         }
      }
    }

    // Java: Element createElement(String name)
    pub fn createElement_name(&self) {
      // Java: return this.getDocumentFactory().createElement(name);
    }

    // Java: Element createElement(QName qName)
    pub fn createElement_qname(&self) {
      // Java: return this.getDocumentFactory().createElement(qName);
    }

    // Java: void addNode(Node node)
    pub fn addNode(&self) {
      // Java: if (node.getParent() != null)
      {
         // Java: String message = "The Node already has an existing parent of \"" + node.getParent().getQualifiedName() + "\"";
         // Java: throw new IllegalAddException(this, node, message);
      }
      // Java: else
      {
         // Java: this.addNewNode(node);
      }
    }

    // Java: void addNode(int index, Node node)
    pub fn addNode_index(&self) {
      // Java: if (node.getParent() != null)
      {
         // Java: String message = "The Node already has an existing parent of \"" + node.getParent().getQualifiedName() + "\"";
         // Java: throw new IllegalAddException(this, node, message);
      }
      // Java: else
      {
         // Java: this.addNewNode(index, node);
      }
    }

    // Java: void addNewNode(Node node)
    pub fn addNewNode(&self) {
      // Java: this.contentList().add(node);
      // Java: this.childAdded(node);
    }

    // Java: void addNewNode(int index, Node node)
    pub fn addNewNode_index(&self) {
      // Java: this.contentList().add(index, node);
      // Java: this.childAdded(node);
    }

    // Java: boolean removeNode(Node node)
    pub fn removeNode(&self) {
      // Java: boolean answer = this.contentList().remove(node);
      // Java: if (answer)
      {
         // Java: this.childRemoved(node);
      }
      // Java: return answer;
    }

    // Java: void childAdded(Node node)
    pub fn childAdded(&self) {
      // Java: Assert.assertNotNull("node should not be null", node);
      // Java: node.setParent(this);
    }

    // Java: void childRemoved(Node node)
    pub fn childRemoved(&self) {
      // Java: Assert.assertNotNull("node should not be null", node);
      // Java: node.setParent((Element)null);
      // Java: node.setDocument((Document)null);
    }

    // Java: abstract List<Attribute> attributeList()
    pub fn attributeList(&self) -> Vec<String> {
     Vec::new() }

    // Java: abstract List<Attribute> attributeList(int var1)
    pub fn attributeList_size(&self) {
    }

    // Java: DocumentFactory getDocumentFactory()
    pub fn getDocumentFactory(&self)  {
      // Java: QName qName = this.getQName();
      // Java: if (qName != null)
      {
         // Java: DocumentFactory factory = qName.getDocumentFactory();
         // Java: if (factory != null)
         {
            // Java: return factory;
         }
      }
      // Java: return DOCUMENT_FACTORY;
    }

    // Java: List<Attribute> createAttributeList()
    pub fn createAttributeList(&self) -> Vec<String> {
      // Java: return this.createAttributeList(5);
     Vec::new() }

    // Java: List<Attribute> createAttributeList(int size)
    pub fn createAttributeList_size(&self) {
      // Java: return new ArrayList(size);
    }

    // Java: protected <T> Iterator<T> createSingleIterator(T result)
    pub fn createSingleIterator(&self) {
      // Java: return new SingleIterator<T>(result);
    }
}
