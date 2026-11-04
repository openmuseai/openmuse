// Java: import org.dom4j.Element
use crate::org::dom4j::Element::Element;
// Java: import org.dom4j.Namespace
use crate::org::dom4j::Namespace::Namespace;

// Java: public class DefaultNamespace extends Namespace
pub struct DefaultNamespace {
    // Java field: Namespace base
    pub(crate) base: Namespace,
    // Java field: Element parent
    pub(crate) parent: Option<Box<dyn Element>>,
}

// Java: public class DefaultNamespace extends Namespace
impl DefaultNamespace {
    // Java: DefaultNamespace(String prefix, String uri)
    pub fn new(prefix: &str, uri: &str) -> Self {
      // Java: super(prefix, uri);
      DefaultNamespace {
          base: Namespace::new(prefix, uri),
          parent: None,
      }
    }

    // Java: DefaultNamespace(Element parent, String prefix, String uri)
    pub fn new_with_parent(parent: Box<dyn Element>, prefix: &str, uri: &str) -> Self {
      // Java: super(prefix, uri);
      // Java: this.parent = parent;
      DefaultNamespace {
          base: Namespace::new(prefix, uri),
          parent: Some(parent),
      }
    }

    // Java: int createHashCode()
    pub fn createHashCode(&self) -> i32 {
      // Java: int hashCode = super.createHashCode();
      let mut hashCode = self.base.hashCode;
      // Java: if (this.parent != null)
      if self.parent.is_some()
      {
         // Java: hashCode ^= this.parent.hashCode();
         hashCode ^= 0;
      }
      // Java: return hashCode;
      hashCode
    }

    // Java: boolean equals(Object object)
    pub fn equals(&self, _object: &DefaultNamespace) -> bool {
      // Java: if (object instanceof DefaultNamespace)
      // Java: DefaultNamespace that = (DefaultNamespace)object;
      // Java: if (that.parent == this.parent)
      // Java: return super.equals(object);
      // Java: return false;
      false
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      self.base.hashCode
    }

    // Java: Element getParent()
    pub fn getParent(&self) -> Option<&dyn Element> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&mut self, parent: Option<Box<dyn Element>>) {
      // Java: this.parent = parent;
      self.parent = parent;
    }

    // Java: boolean supportsParent()
    pub fn supportsParent(&self) -> bool {
      // Java: return true;
      true
    }

    // Java: boolean isReadOnly()
    pub fn isReadOnly(&self) -> bool {
      // Java: return false;
      false
    }

}