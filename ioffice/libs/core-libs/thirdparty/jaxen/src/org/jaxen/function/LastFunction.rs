// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.Function
use super::Function;
// Java: import org.jaxen.FunctionCallException
use crate::org::jaxen::FunctionCallException::FunctionCallException;
use crate::org::jaxen::{Object, List};
use std::sync::Arc;

// Java: public class LastFunction implements Function
pub struct LastFunction {
}

// Java: public class LastFunction implements Function
impl LastFunction {
    // Java: LastFunction()
    pub fn new() -> Self {
      // Java: super();
      LastFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 0)
      if args.is_empty() {
         // Java: return evaluate(context);
         Ok(Arc::new(Self::evaluate(context)))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("last() requires no arguments.");
         Err(FunctionCallException::new("last() requires no arguments.".to_string()))
      }
    }

    // Java: static Double evaluate(Context context)
    pub fn evaluate(context: &Context) -> f64 {
      // Java: return new Double((double)context.getSize());
      context.getSize() as f64
    }
}

impl Function for LastFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
