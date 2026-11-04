// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.Function
use crate::org::jaxen::Function::Function;
// Java: import org.jaxen.FunctionCallException
use crate::org::jaxen::FunctionCallException::FunctionCallException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.function.StringFunction
use crate::org::jaxen::function::StringFunction::StringFunction;
use crate::org::jaxen::{Object, List};

// Java: public class DocumentFunction implements Function
pub struct DocumentFunction {
}

// Java: public class DocumentFunction implements Function
impl DocumentFunction {
    // Java: DocumentFunction()
    pub fn new() -> Self {
      // Java: super();
      DocumentFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 1)
      if args.len() == 1 {
         // Java: Navigator nav = context.getNavigator();
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         // Java: String url = StringFunction.evaluate(args.get(0), nav);
         let url = StringFunction::evaluate(&args[0], Some(nav));
         // Java: return evaluate(url, nav);
         Self::evaluate(&url, nav)
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("document() requires one argument.");
         Err(FunctionCallException::new("document() requires one argument.".to_string()))
      }
    }

    // Java: static Object evaluate(String url, Navigator nav)
    pub fn evaluate(url: &str, nav: &dyn Navigator) -> Result<Object, FunctionCallException> {
      // Java: return nav.getDocument(url);
      nav.getDocument(url)
    }
}

impl Function for DocumentFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
