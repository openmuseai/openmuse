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
use super::StringFunction::StringFunction;
use std::sync::Arc;

// Java: public class StartsWithFunction implements Function
pub struct StartsWithFunction {
}

// Java: public class StartsWithFunction implements Function
impl StartsWithFunction {
    // Java: StartsWithFunction()
    pub fn new() -> Self {
      // Java: super();
      StartsWithFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 2)
      if args.len() == 2 {
         // Java: return evaluate(args.get(0), args.get(1), context.getNavigator());
         let result = Self::evaluate(&args[0], &args[1], context.getNavigator());
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("starts-with() requires two arguments.");
         Err(FunctionCallException::new("starts-with() requires two arguments.".to_string()))
      }
    }

    // Java: static Boolean evaluate(Object strArg, Object matchArg, Navigator nav)
    pub fn evaluate(strArg: &Object, matchArg: &Object, nav: Option<&dyn Navigator>) -> bool {
      // Java: String str = StringFunction.evaluate(strArg, nav);
      let str_val = StringFunction::evaluate(strArg, nav);
      // Java: String match = StringFunction.evaluate(matchArg, nav);
      let match_val = StringFunction::evaluate(matchArg, nav);
      // Java: return str.startsWith(match) ? Boolean.TRUE : Boolean.FALSE;
      str_val.starts_with(&match_val)
    }
}

impl Function for StartsWithFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
