// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.Function
use super::Function;
// Java: import org.jaxen.FunctionCallException
use crate::org::jaxen::FunctionCallException::FunctionCallException;
use crate::org::jaxen::{Object, List};
use std::sync::Arc;
use std::any::Any;

// Java: public class CountFunction implements Function
pub struct CountFunction {
}

// Java: public class CountFunction implements Function
impl CountFunction {
    // Java: CountFunction()
    pub fn new() -> Self {
      // Java: super();
      CountFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, _context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 1)
      if args.len() == 1 {
         // Java: return evaluate(args.get(0));
         let result = Self::evaluate(&args[0])?;
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("count() requires one argument.");
         Err(FunctionCallException::new("count() requires one argument.".to_string()))
      }
    }

    // Java: static Double evaluate(Object obj)
    pub fn evaluate(obj: &Object) -> Result<f64, FunctionCallException> {
      // Java: if (obj instanceof List)
      if let Some(list) = obj.downcast_ref::<List>() {
         // Java: return new Double((double)((List)obj).size());
         Ok(list.len() as f64)
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("count() function can only be used for node-sets");
         Err(FunctionCallException::new("count() function can only be used for node-sets".to_string()))
      }
    }
}

impl Function for CountFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
