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
use super::BooleanFunction::BooleanFunction;
use std::sync::Arc;

// Java: public class NotFunction implements Function
pub struct NotFunction {
}

// Java: public class NotFunction implements Function
impl NotFunction {
    // Java: NotFunction()
    pub fn new() -> Self {
      // Java: super();
      NotFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 1)
      if args.len() == 1 {
         // Java: return evaluate(args.get(0), context.getNavigator());
         let result = Self::evaluate(&args[0], context.getNavigator());
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("not() requires one argument.");
         Err(FunctionCallException::new("not() requires one argument.".to_string()))
      }
    }

    // Java: static Boolean evaluate(Object obj, Navigator nav)
    pub fn evaluate(obj: &Object, nav: Option<&dyn Navigator>) -> bool {
      // Java: return BooleanFunction.evaluate(obj, nav) ? Boolean.FALSE : Boolean.TRUE;
      !BooleanFunction::evaluate(obj, nav)
    }
}

impl Function for NotFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
