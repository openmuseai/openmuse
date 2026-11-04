// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.jaxen.BaseXPath
use crate::org::jaxen::BaseXPath::BaseXPath;
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;

// Java field: long serialVersionUID

// Java: public class JavaBeanXPath extends BaseXPath
pub struct JavaBeanXPath {
    pub(crate) base: BaseXPath,
}

// Java: public class JavaBeanXPath extends BaseXPath
impl JavaBeanXPath {
    // Java: JavaBeanXPath(String xpathExpr)
    pub fn new(_xpathExpr: String) -> Result<Self, JaxenException> {
      // Java: super(xpathExpr, DocumentNavigator.getInstance());
      todo!("JavaBeanXPath::new")
    }

    // Java: Context getContext(Object node)
    pub fn getContext(&self, _node: &Object) -> Context {
      // Java: if (node instanceof Context)
      // Java: return (Context)node;
      // Java: else if (node instanceof Element)
      // Java: return super.getContext(node);
      // Java: else if (!(node instanceof List))
      // Java: return super.getContext(new Element((Element)null, "root", node));
      // Java: else
      // Java: List newList = new ArrayList();
      // Java: Iterator listIter = ((List)node).iterator();
      // Java: while(listIter.hasNext())
      // Java: newList.add(new Element((Element)null, "root", listIter.next()));
      // Java: return super.getContext(newList);
      todo!("JavaBeanXPath::getContext")
    }

    // Java: Object evaluate(Object node)
    pub fn evaluate(&self, _node: &Object) -> Result<Object, JaxenException> {
      // Java: Object result = super.evaluate(node);
      // Java: if (result instanceof Element)
      // Java: return ((Element)result).getObject();
      // Java: else if (result instanceof Collection)
      // Java: List newList = new ArrayList();
      // Java: for(Object member : (Collection)result)
      // Java: if (member instanceof Element)
      // Java: newList.add(((Element)member).getObject());
      // Java: else
      // Java: newList.add(member);
      // Java: return newList;
      // Java: else
      // Java: return result;
      todo!("JavaBeanXPath::evaluate")
    }
}
