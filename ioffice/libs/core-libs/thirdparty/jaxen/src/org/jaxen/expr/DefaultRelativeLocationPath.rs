use super::DefaultLocationPath::DefaultLocationPath;

// Java field: long serialVersionUID

// Java: public class DefaultRelativeLocationPath extends DefaultLocationPath
pub struct DefaultRelativeLocationPath {
    pub(crate) base: DefaultLocationPath,
}

// Java: public class DefaultRelativeLocationPath extends DefaultLocationPath
impl DefaultRelativeLocationPath {
    // Java: DefaultRelativeLocationPath()
    pub fn new() -> Self {
      // Java: super();
      DefaultRelativeLocationPath {
         base: DefaultLocationPath::new(),
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultRelativeLocationPath): " + super.toString() + "]";
      format!("[(DefaultRelativeLocationPath): {}]", self.base.toString())
    }
}
