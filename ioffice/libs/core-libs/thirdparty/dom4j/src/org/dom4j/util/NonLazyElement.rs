// Java: import org.dom4j.Namespace
// use crate::org::dom4j::Namespace;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;
// Java: import org.dom4j.tree.BaseElement
// use crate::org::dom4j::tree::BaseElement;

// Java: public class NonLazyElement extends BaseElement
pub struct NonLazyElement {
}

// Java: public class NonLazyElement extends BaseElement
impl NonLazyElement {
    // Java: NonLazyElement(String name)
    pub fn new(_name: &str) -> Self {
      // Java: super(name);
      // Java: this.attributes = this.createAttributeList();
      // Java: this.content = this.createContentList();
      Self {}
    }

    // Java: NonLazyElement(QName qname)
    pub fn new_with_qname(_qname: i32) -> Self {
      // Java: super(qname);
      // Java: this.attributes = this.createAttributeList();
      // Java: this.content = this.createContentList();
      Self {}
    }

    // Java: NonLazyElement(String name, Namespace namespace)
    pub fn new_with_name_and_ns(_name: &str, _namespace: i32) -> Self {
      // Java: super(name, namespace);
      // Java: this.attributes = this.createAttributeList();
      // Java: this.content = this.createContentList();
      Self {}
    }

    // Java: NonLazyElement(QName qname, int attributeCount)
    pub fn new_with_qname_and_count(_qname: i32, _attributeCount: i32) -> Self {
      // Java: super(qname);
      // Java: this.attributes = this.createAttributeList(attributeCount);
      // Java: this.content = this.createContentList();
      Self {}
    }

}
