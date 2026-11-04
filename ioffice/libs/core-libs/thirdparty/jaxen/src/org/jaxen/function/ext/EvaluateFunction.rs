// Java: import java.util.Collections
// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.Function
use crate::org::jaxen::Function::Function;
// Java: import org.jaxen.FunctionCallException
use crate::org::jaxen::FunctionCallException::FunctionCallException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.XPath
use crate::org::jaxen::XPath::XPath;
// Java: import org.jaxen.function.StringFunction
use crate::org::jaxen::function::StringFunction::StringFunction;
// Java: import org.jaxen.saxpath.SAXPathException
use crate::org::jaxen::saxpath::SAXPathException::SAXPathException;
use crate::org::jaxen::{Object, List};
use std::sync::Arc;
use std::any::Any;

// Java: public class EvaluateFunction implements Function
pub struct EvaluateFunction {
}

// Java: public class EvaluateFunction implements Function
impl EvaluateFunction {
    // Java: EvaluateFunction()
    pub fn new() -> Self {
      // Java: super();
      EvaluateFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 1)
      if args.len() == 1 {
         // Java: return evaluate(context, args.get(0));
         let result = Self::evaluate(context, &args[0])?;
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("evaluate() requires one argument");
         Err(FunctionCallException::new("evaluate() requires one argument".to_string()))
      }
    }

    // Java: static List evaluate(Context context, Object arg)
    pub fn evaluate(context: &Context, arg: &Object) -> Result<List, FunctionCallException> {
      // Java: List contextNodes = context.getNodeSet();
      let contextNodes = context.getNodeSet();
      // Java: if (contextNodes.size() == 0)
      if contextNodes.is_empty() {
         // Java: return Collections.EMPTY_LIST;
         return Ok(Vec::new());
      }
      // Java: else
      {
         // Java: Navigator nav = context.getNavigator();
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         // Java: String xpathString;
         // Java: if (arg instanceof String)
         let _xpathString = if let Some(s) = arg.downcast_ref::<String>() {
            // Java: xpathString = (String)arg;
            s.clone()
         }
         // Java: else
         else {
            // Java: xpathString = StringFunction.evaluate(arg, nav);
            StringFunction::evaluate(arg, Some(nav))
         };

         // Java: try
         // Java: XPath xpath = nav.parseXPath(xpathString);
         // Java: ContextSupport support = context.getContextSupport();
         // Java: xpath.setVariableContext(support.getVariableContext());
         // Java: xpath.setFunctionContext(support.getFunctionContext());
         // Java: xpath.setNamespaceContext(support.getNamespaceContext());
         // Java: return xpath.selectNodes(context.duplicate());
         // Java: catch (SAXPathException e)
         // Java: throw new FunctionCallException(e.toString());
         todo!("EvaluateFunction::evaluate - requires XPath parsing support")
      }
    }
}

impl Function for EvaluateFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
