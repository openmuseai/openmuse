// Java: import java.util.List
// Java: import org.dom4j.Attribute
// use crate::org::dom4j::Attribute;
// Java: import org.dom4j.Branch
// use crate::org::dom4j::Branch;
// Java: import org.dom4j.Document
// use crate::org::dom4j::Document;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.Namespace
// use crate::org::dom4j::Namespace;
// Java: import org.dom4j.Node
// use crate::org::dom4j::Node;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;

// Java: public class BaseElement extends AbstractElement
pub struct BaseElement {
    // Java field: QName qname
    pub(crate) qname: i32,
    // Java field: Branch parentBranch
    parentBranch: i32,
    // Java field: List<Node> content
    pub(crate) content: i32,
    // Java field: List<Attribute> attributes
    pub(crate) attributes: i32,
}

// Java: public class BaseElement extends AbstractElement
impl BaseElement {
    // Java: BaseElement(String name)
    pub fn BaseElement(&self, _name: &str) {
      // Java: super();
      // Java: this.qname = this.getDocumentFactory().createQName(name);
      todo!()
    }

    // Java: BaseElement(QName qname)
    pub fn BaseElement_qname(&self, _qname: i32) {
      // Java: super();
      // Java: this.qname = qname;
      todo!()
    }

    // Java: BaseElement(String name, Namespace namespace)
    pub fn BaseElement_name_ns(&self, _name: &str, _namespace: i32) {
      // Java: super();
      // Java: this.qname = this.getDocumentFactory().createQName(name, namespace);
      todo!()
    }

    // Java: Element getParent()
    pub fn getParent(&self) -> i32 {
      // Java: Element result = null;
      // Java: if (this.parentBranch instanceof Element)
      // Java: result = (Element)this.parentBranch;
      // Java: return result;
      todo!()
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&self, _parent: i32) {
      // Java: if (this.parentBranch instanceof Element || parent != null)
      // Java: this.parentBranch = parent;
      todo!()
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) -> i32 {
      // Java: if (this.parentBranch instanceof Document)
      // Java: return (Document)this.parentBranch;
      // Java: else if (this.parentBranch instanceof Element)
      // Java: Element parent = (Element)this.parentBranch;
      // Java: return parent.getDocument();
      // Java: else
      // Java: return null;
      todo!()
    }

    // Java: void setDocument(Document document)
    pub fn setDocument(&self, _document: i32) {
      // Java: if (this.parentBranch instanceof Document || document != null)
      // Java: this.parentBranch = document;
      todo!()
    }

    // Java: boolean supportsParent()
    pub fn supportsParent(&self) -> bool {
      // Java: return true;
      todo!()
    }

    // Java: QName getQName()
    pub fn getQName(&self) -> i32 {
      // Java: return this.qname;
      todo!()
    }

    // Java: void setQName(QName name)
    pub fn setQName(&self, _name: i32) {
      // Java: this.qname = name;
      todo!()
    }

    // Java: void clearContent()
    pub fn clearContent(&self) {
      // Java: this.contentList().clear();
      todo!()
    }

    // Java: void setContent(List<Node> content)
    pub fn setContent(&self, _content: i32) {
      // Java: this.content = content;
      // Java: if (content instanceof ContentListFacade)
      // Java: this.content = ((ContentListFacade<Node>)content).getBackingList();
      todo!()
    }

    // Java: void setAttributes(List<Attribute> attributes)
    pub fn setAttributes(&self, _attributes: i32) {
      // Java: this.attributes = attributes;
      // Java: if (attributes instanceof ContentListFacade)
      // Java: this.attributes = ((ContentListFacade<Attribute>)attributes).getBackingList();
      todo!()
    }

    // Java: List<Node> contentList()
    pub fn contentList(&self) -> i32 {
      // Java: if (this.content == null)
      // Java: this.content = this.createContentList();
      // Java: return this.content;
      todo!()
    }

    // Java: List<Attribute> attributeList()
    pub fn attributeList(&self) -> i32 {
      // Java: if (this.attributes == null)
      // Java: this.attributes = this.createAttributeList();
      // Java: return this.attributes;
      todo!()
    }

    // Java: List<Attribute> attributeList(int size)
    pub fn attributeList_size(&self, _size: i32) -> i32 {
      // Java: if (this.attributes == null)
      // Java: this.attributes = this.createAttributeList(size);
      // Java: return this.attributes;
      todo!()
    }

    // Java: void setAttributeList(List<Attribute> attributeList)
    pub fn setAttributeList(&self, _attributeList: i32) {
      // Java: this.attributes = attributeList;
      todo!()
    }

}
