// Java: import java.util.List
// Java: import org.dom4j.Attribute
// use crate::org::dom4j::Attribute;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.Node
// use crate::org::dom4j::Node;
// Java: import org.dom4j.NodeHelper
// use crate::org::dom4j::NodeHelper;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;
// Java: import org.dom4j.tree.BackedList
// use crate::org::dom4j::tree::BackedList;
// Java: import org.dom4j.tree.DefaultElement
// use crate::org::dom4j::tree::DefaultElement;
// Java: import org.dom4j.tree.LazyList
// use crate::org::dom4j::tree::LazyList;

// Java: public class IndexedElement extends DefaultElement
pub struct IndexedElement {
    // Java field: DoubleNameMap<List<Element>> elementIndex
    elementIndex: i32,
    // Java field: DoubleNameMap<Attribute> attributeIndex
    attributeIndex: i32,
}

// Java: public class IndexedElement extends DefaultElement
impl IndexedElement {
    // Java: IndexedElement(String name)
    pub fn IndexedElement(&self, _name: &str) {
      // Java: super(name);
      todo!()
    }

    // Java: IndexedElement(QName qname)
    pub fn IndexedElement_qname(&self, _qname: i32) {
      // Java: super(qname);
      todo!()
    }

    // Java: IndexedElement(QName qname, int attributeCount)
    pub fn IndexedElement_qname_count(&self, _qname: i32, _attributeCount: i32) {
      // Java: super(qname, attributeCount);
      todo!()
    }

    // Java: Attribute attribute(String name)
    pub fn attribute(&self, _name: &str) -> i32 {
      // Java: return (Attribute)this.attributeIndex().get(name);
      todo!()
    }

    // Java: Attribute attribute(QName qName)
    pub fn attribute_qname(&self, _qName: i32) -> i32 {
      // Java: return (Attribute)this.attributeIndex().get(qName);
      todo!()
    }

    // Java: Element element(String name)
    pub fn element(&self, _name: &str) -> i32 {
      // Java: return firstElement((List)this.elementIndex().get(name));
      todo!()
    }

    // Java: Element element(QName qName)
    pub fn element_qname(&self, _qName: i32) -> i32 {
      // Java: return firstElement((List)this.elementIndex().get(qName));
      todo!()
    }

    // Java: List<Element> elements(String name)
    pub fn elements(&self, _name: &str) -> i32 {
      // Java: return this.asElementList((List)this.elementIndex().get(name));
      todo!()
    }

    // Java: List<Element> elements(QName qName)
    pub fn elements_qname(&self, _qName: i32) -> i32 {
      // Java: return this.asElementList((List)this.elementIndex().get(qName));
      todo!()
    }

    // Java: static Element firstElement(List<Element> list)
    pub fn firstElement(&self, _list: i32) -> i32 {
      // Java: return list.isEmpty() ? null : (Element)list.get(0);
      todo!()
    }

    // Java: List<Element> asElementList(List<Element> list)
    pub fn asElementList(&self, _list: i32) -> i32 {
      // Java: BackedList<Element> answer = this.createResultList();
      // Java: for(Element element : list)
      // Java: answer.addLocal(element);
      // Java: return answer;
      todo!()
    }

    // Java: void addNode(Node node)
    pub fn addNode(&self, _node: i32) {
      // Java: super.addNode(node);
      // Java: switch (node.getNodeTypeEnum())
      // Java: case ELEMENT_NODE:
      // Java: if (this.elementIndex != null)
      // Java: this.addToElementIndex((Element)node);
      // Java: break;
      // Java: case ATTRIBUTE_NODE:
      // Java: if (this.attributeIndex != null)
      // Java: this.addToAttributeIndex((Attribute)node);
      todo!()
    }

    // Java: boolean removeNode(Node node)
    pub fn removeNode(&self, _node: i32) -> bool {
      // Java: if (super.removeNode(node))
      // Java: switch (node.getNodeTypeEnum())
      // Java: case ELEMENT_NODE:
      // Java: if (this.elementIndex != null)
      // Java: this.removeFromElementIndex((Element)node);
      // Java: break;
      // Java: case ATTRIBUTE_NODE:
      // Java: if (this.attributeIndex != null)
      // Java: this.removeFromAttributeIndex((Attribute)node);
      // Java: return true;
      // Java: else
      // Java: return false;
      todo!()
    }

    // Java: DoubleNameMap<Attribute> attributeIndex()
    pub fn attributeIndex(&self) -> i32 {
      // Java: if (this.attributeIndex == null)
      // Java: this.attributeIndex = new DoubleNameMap<Attribute>();
      // Java: for(Attribute attribute : this.attributeList())
      // Java: this.addToAttributeIndex(attribute);
      // Java: return this.attributeIndex;
      todo!()
    }

    // Java: DoubleNameMap<List<Element>> elementIndex()
    pub fn elementIndex(&self) -> i32 {
      // Java: if (this.elementIndex == null)
      // Java: this.elementIndex = new DoubleNameMap<List<Element>>();
      // Java: for(Node node : this.contentList())
      // Java: Element element = NodeHelper.nodeAsElement(node);
      // Java: if (element != null)
      // Java: this.addToElementIndex(element);
      // Java: return this.elementIndex;
      todo!()
    }

    // Java: void addToElementIndex(Element element)
    pub fn addToElementIndex(&self, _element: i32) {
      // Java: QName qName = element.getQName();
      // Java: List<Element> list = (List)this.elementIndex.get(qName);
      // Java: if (list == null)
      // Java: list = new LazyList<Element>();
      // Java: this.elementIndex.put(qName, list);
      // Java: list.add(element);
      todo!()
    }

    // Java: void removeFromElementIndex(Element element)
    pub fn removeFromElementIndex(&self, _element: i32) {
      // Java: QName qName = element.getQName();
      // Java: List<Element> list = (List)this.elementIndex.get(qName);
      // Java: if (list != null)
      // Java: list.remove(element);
      // Java: if (list.isEmpty())
      // Java: this.elementIndex.remove(qName);
      todo!()
    }

    // Java: void addToAttributeIndex(Attribute attribute)
    pub fn addToAttributeIndex(&self, _attribute: i32) {
      // Java: QName qName = attribute.getQName();
      // Java: this.attributeIndex.put(qName, attribute);
      todo!()
    }

    // Java: void removeFromAttributeIndex(Attribute attribute)
    pub fn removeFromAttributeIndex(&self, _attribute: i32) {
      // Java: QName qName = attribute.getQName();
      // Java: this.attributeIndex.remove(qName);
      todo!()
    }

}
