// Java: import java.util.HashSet
use std::collections::HashSet;
use crate::org::jaxen::Object;

// Java: public class IdentitySet
pub struct IdentitySet {
    // Java field: HashSet contents
    pub(crate) contents: HashSet<usize>,
}

// Java: public class IdentityWrapper
pub struct IdentityWrapper {
    // Java field: Object object
    pub(crate) object: Object,
    pub(crate) identity: usize,
}

// Java: public class IdentitySet
impl IdentitySet {
    // Java: IdentitySet()
    pub fn new() -> Self {
      // Java: super();
      IdentitySet {
         contents: HashSet::new(),
      }
    }

    // Java: void add(Object object)
    pub fn add(&mut self, object: &Object) {
      // Java: IdentityWrapper wrapper = new IdentityWrapper(object);
      let ptr = &**object as *const dyn std::any::Any as *const () as usize;
      // Java: this.contents.add(wrapper);
      self.contents.insert(ptr);
    }

    // Java: boolean contains(Object object)
    pub fn contains(&self, object: &Object) -> bool {
      // Java: IdentityWrapper wrapper = new IdentityWrapper(object);
      let ptr = &**object as *const dyn std::any::Any as *const () as usize;
      // Java: return this.contents.contains(wrapper);
      self.contents.contains(&ptr)
    }
}

// Java: public class IdentityWrapper
impl IdentityWrapper {
    // Java: IdentityWrapper(Object object)
    pub fn new(object: Object) -> Self {
      // Java: super();
      let identity = &*object as *const dyn std::any::Any as *const () as usize;
      // Java: this.object = object;
      IdentityWrapper { object, identity }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, other: &IdentityWrapper) -> bool {
      // Java: if (!(o instanceof IdentityWrapper))
      // Java: return false;
      // Java: else
      // Java: IdentityWrapper w = (IdentityWrapper)o;
      // Java: return this.object == w.object;
      self.identity == other.identity
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> usize {
      // Java: return System.identityHashCode(this.object);
      self.identity
    }
}
