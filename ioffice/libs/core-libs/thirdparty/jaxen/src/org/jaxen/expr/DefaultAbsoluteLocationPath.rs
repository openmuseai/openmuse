// Java: import java.util.Collections
// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.util.SingletonList
use crate::org::jaxen::{Object, List};
use super::DefaultLocationPath::DefaultLocationPath;
use std::sync::Arc;

// Java field: long serialVersionUID

// Java: public class DefaultAbsoluteLocationPath extends DefaultLocationPath
pub struct DefaultAbsoluteLocationPath {
    pub(crate) base: DefaultLocationPath,
}

// Java: public class DefaultAbsoluteLocationPath extends DefaultLocationPath
impl DefaultAbsoluteLocationPath {
    // Java: DefaultAbsoluteLocationPath()
    pub fn new() -> Self {
      // Java: super();
      DefaultAbsoluteLocationPath {
         base: DefaultLocationPath::new(),
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultAbsoluteLocationPath): " + super.toString() + "]";
      format!("[(DefaultAbsoluteLocationPath): {}]", self.base.toString())
    }

    // Java: boolean isAbsolute()
    pub fn isAbsolute(&self) -> bool {
      // Java: return true;
      true
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return "/" + super.getText();
      format!("/{}", self.base.getText())
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: ContextSupport support = context.getContextSupport();
      // Java: Navigator nav = support.getNavigator();
      // Java: Context absContext = new Context(support);
      // Java: List contextNodes = context.getNodeSet();
      // Java: if (contextNodes.isEmpty())
      // Java: return Collections.EMPTY_LIST;
      // Java: else
      // Java: Object firstNode = contextNodes.get(0);
      // Java: Object docNode = nav.getDocumentNode(firstNode);
      // Java: if (docNode == null)
      // Java: return Collections.EMPTY_LIST;
      // Java: else
      // Java: List list = new SingletonList(docNode);
      // Java: absContext.setNodeSet(list);
      // Java: return super.evaluate(absContext);
      todo!("DefaultAbsoluteLocationPath::evaluate")
    }
}
