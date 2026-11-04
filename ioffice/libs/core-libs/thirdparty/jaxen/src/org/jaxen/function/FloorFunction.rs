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
use super::NumberFunction::NumberFunction;
use std::sync::Arc;

// Java: public class FloorFunction implements Function
pub struct FloorFunction {
}

// Java: public class FloorFunction implements Function
impl FloorFunction {
    // Java: FloorFunction()
    pub fn new() -> Self {
      // Java: super();
      FloorFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 1)
      if args.len() == 1 {
         // Java: return evaluate(args.get(0), context.getNavigator());
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         Ok(Arc::new(Self::evaluate(&args[0], nav)))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("floor() requires one argument.");
         Err(FunctionCallException::new("floor() requires one argument.".to_string()))
      }
    }

    // Java: static Double evaluate(Object obj, Navigator nav)
    pub fn evaluate(obj: &Object, nav: &dyn Navigator) -> f64 {
      // Java: Double value = NumberFunction.evaluate(obj, nav);
      let value = NumberFunction::evaluate(obj, Some(nav));
      // Java: return new Double(Math.floor(value));
      value.floor()
    }
}

impl Function for FloorFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
