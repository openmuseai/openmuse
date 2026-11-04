// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.Function
use super::Function;
// Java: import org.jaxen.FunctionCallException
use crate::org::jaxen::FunctionCallException::FunctionCallException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
use crate::org::jaxen::{Object, List};
use std::sync::Arc;
use std::any::Any;

// Java: public class NameFunction implements Function
pub struct NameFunction {
}

// Java: public class NameFunction implements Function
impl NameFunction {
    // Java: NameFunction()
    pub fn new() -> Self {
      // Java: super();
      NameFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 0)
      if args.is_empty() {
         // Java: return evaluate(context.getNodeSet(), context.getNavigator());
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         let result = Self::evaluate(context.getNodeSet(), nav)?;
         Ok(Arc::new(result))
      }
      // Java: else if (args.size() == 1)
      else if args.len() == 1 {
         // Java: return evaluate(args, context.getNavigator());
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         let result = Self::evaluate(args, nav)?;
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("name() requires zero or one argument.");
         Err(FunctionCallException::new("name() requires zero or one argument.".to_string()))
      }
    }

    // Java: static String evaluate(List list, Navigator nav)
    pub fn evaluate(list: &List, nav: &dyn Navigator) -> Result<String, FunctionCallException> {
      // Java: if (!list.isEmpty())
      if !list.is_empty() {
         // Java: Object first = list.get(0);
         let first = &list[0];
         // Java: if (first instanceof List)
         if let Some(inner_list) = first.downcast_ref::<List>() {
            // Java: return evaluate((List)first, nav);
            return Self::evaluate(inner_list, nav);
         }
         // Java: else if (nav.isElement(first))
         else if nav.isElement(first) {
            // Java: return nav.getElementQName(first);
            return Ok(nav.getElementQName(first));
         }
         // Java: else if (nav.isAttribute(first))
         else if nav.isAttribute(first) {
            // Java: return nav.getAttributeQName(first);
            return Ok(nav.getAttributeQName(first));
         }
         // Java: else if (nav.isProcessingInstruction(first))
         else if nav.isProcessingInstruction(first) {
            // Java: return nav.getProcessingInstructionTarget(first);
            return Ok(nav.getProcessingInstructionTarget(first));
         }
         // Java: else if (nav.isNamespace(first))
         else if nav.isNamespace(first) {
            // Java: return nav.getNamespacePrefix(first);
            return Ok(nav.getNamespacePrefix(first));
         }
         // Java: else if (nav.isDocument(first))
         else if nav.isDocument(first) {
            // Java: return "";
            return Ok(String::new());
         }
         // Java: else if (nav.isComment(first))
         else if nav.isComment(first) {
            // Java: return "";
            return Ok(String::new());
         }
         // Java: else if (nav.isText(first))
         else if nav.isText(first) {
            // Java: return "";
            return Ok(String::new());
         }
         // Java: else
         else {
            // Java: throw new FunctionCallException("The argument to the name function must be a node-set");
            return Err(FunctionCallException::new("The argument to the name function must be a node-set".to_string()));
         }
      }
      // Java: else
      else {
         // Java: return "";
         Ok(String::new())
      }
    }
}

impl Function for NameFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
