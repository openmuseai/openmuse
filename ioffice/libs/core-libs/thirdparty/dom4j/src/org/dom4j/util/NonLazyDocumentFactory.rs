// Java: import org.dom4j.DocumentFactory
// use crate::org::dom4j::DocumentFactory;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;

// Java field: NonLazyDocumentFactory singleton
static SINGLETON: std::sync::OnceLock<i32> = std::sync::OnceLock::new();

// Java: public class NonLazyDocumentFactory extends DocumentFactory
pub struct NonLazyDocumentFactory {
}

// Java: public class NonLazyDocumentFactory extends DocumentFactory
impl NonLazyDocumentFactory {
    // Java: NonLazyDocumentFactory()
    pub fn new() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static DocumentFactory getInstance()
    pub fn getInstance() -> i32 {
      // Java: return singleton;
      todo!()
    }

    // Java: Element createElement(QName qname)
    pub fn createElement(&self, _qname: i32) -> i32 {
      // Java: return new NonLazyElement(qname);
      todo!()
    }

}
