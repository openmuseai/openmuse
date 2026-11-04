// Java: import java.util.Iterator
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

// Java: public class ConcatFunction implements Function
pub struct ConcatFunction {
}

// Java: public class ConcatFunction implements Function
impl ConcatFunction {
    // Java: ConcatFunction()
    pub fn new() -> Self {
      // Java: super();
      ConcatFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() >= 2)
      if args.len() >= 2 {
         // Java: return evaluate(args, context.getNavigator());
         let result = Self::evaluate(args, context.getNavigator());
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("concat() requires at least two arguments");
         Err(FunctionCallException::new("concat() requires at least two arguments".to_string()))
      }
    }

    // Java: static String evaluate(List list, Navigator nav)
    pub fn evaluate(list: &List, nav: Option<&dyn Navigator>) -> String {
      // Java: StringBuffer result = new StringBuffer();
      let mut result = String::new();
      // Java: Iterator argIter = list.iterator();
      // Java: while(argIter.hasNext())
      for item in list.iter() {
         // Java: result.append(StringFunction.evaluate(argIter.next(), nav));
         result.push_str(&StringFunction::evaluate(item, nav));
      }
      // Java: return result.toString();
      result
    }
}

impl Function for ConcatFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
