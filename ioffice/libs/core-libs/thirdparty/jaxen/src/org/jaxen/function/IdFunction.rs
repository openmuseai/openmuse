// Java: import java.util.ArrayList
// Java: import java.util.Collections
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.StringTokenizer
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.Function
use super::Function;
// Java: import org.jaxen.FunctionCallException
use crate::org::jaxen::FunctionCallException::FunctionCallException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
use crate::org::jaxen::{Object, List};
use super::StringFunction::StringFunction;
use std::sync::Arc;
use std::any::Any;

// Java: public class IdFunction implements Function
pub struct IdFunction {
}

// Java: public class IdFunction implements Function
impl IdFunction {
    // Java: IdFunction()
    pub fn new() -> Self {
      // Java: super();
      IdFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 1)
      if args.len() == 1 {
         // Java: return evaluate(context.getNodeSet(), args.get(0), context.getNavigator());
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         let result = Self::evaluate(context.getNodeSet(), &args[0], nav);
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("id() requires one argument");
         Err(FunctionCallException::new("id() requires one argument".to_string()))
      }
    }

    // Java: static List evaluate(List contextNodes, Object arg, Navigator nav)
    pub fn evaluate(contextNodes: &List, arg: &Object, nav: &dyn Navigator) -> List {
      // Java: if (contextNodes.size() == 0)
      if contextNodes.is_empty() {
         // Java: return Collections.EMPTY_LIST;
         return Vec::new();
      }
      // Java: else
      {
         // Java: List nodes = new ArrayList();
         let mut nodes: List = Vec::new();
         // Java: Object contextNode = contextNodes.get(0);
         let contextNode = &contextNodes[0];
         // Java: if (arg instanceof List)
         if let Some(list) = arg.downcast_ref::<List>() {
            // Java: Iterator iter = ((List)arg).iterator();
            // Java: while(iter.hasNext())
            for item in list.iter() {
               // Java: String id = StringFunction.evaluate(iter.next(), nav);
               let id = StringFunction::evaluate(item, Some(nav));
               // Java: nodes.addAll(evaluate(contextNodes, id, nav));
               let id_obj: Object = Arc::new(id);
               let mut results = Self::evaluate(contextNodes, &id_obj, nav);
               nodes.append(&mut results);
            }
         }
         // Java: else
         else {
            // Java: String ids = StringFunction.evaluate(arg, nav);
            let ids = StringFunction::evaluate(arg, Some(nav));
            // Java: StringTokenizer tok = new StringTokenizer(ids, " \t\n\r");
            // Java: while(tok.hasMoreTokens())
            for token in ids.split(&[' ', '\t', '\n', '\r'][..]) {
               // Java: String id = tok.nextToken();
               let id = token;
               if id.is_empty() { continue; }
               // Java: Object node = nav.getElementById(contextNode, id);
               let node = nav.getElementById(contextNode, id);
               // Java: if (node != null)
               {
                  // Java: nodes.add(node);
                  nodes.push(node);
               }
            }
         }
         // Java: return nodes;
         nodes
      }
    }
}

impl Function for IdFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
