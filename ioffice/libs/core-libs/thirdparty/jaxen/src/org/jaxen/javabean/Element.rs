use crate::org::jaxen::Object;

// Java: public class Element
pub struct Element {
    // Java field: Element parent
    pub(crate) parent: Option<Box<Element>>,
    // Java field: String name
    pub(crate) name: String,
    // Java field: Object object
    pub(crate) object: Object,
}

// Java: public class Element
impl Element {
    // Java: Element(Element parent, String name, Object object)
    pub fn new(parent: Option<Box<Element>>, name: String, object: Object) -> Self {
      // Java: super();
      // Java: this.parent = parent;
      // Java: this.name = name;
      // Java: this.object = object;
      Element { parent, name, object }
    }

    // Java: Element getParent()
    pub fn getParent(&self) -> Option<&Element> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this.name;
      &self.name
    }

    // Java: Object getObject()
    pub fn getObject(&self) -> &Object {
      // Java: return this.object;
      &self.object
    }
}
