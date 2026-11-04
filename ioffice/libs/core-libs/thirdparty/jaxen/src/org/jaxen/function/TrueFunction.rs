// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.Function
use super::Function;
// Java: import org.jaxen.FunctionCallException
use crate::org::jaxen::FunctionCallException::FunctionCallException;
use crate::org::jaxen::{Object, List};
use std::sync::Arc;

// Java: public class TrueFunction implements Function
pub struct TrueFunction {
}

// Java: public class TrueFunction implements Function
impl TrueFunction {
    // Java: TrueFunction()
    pub fn new() -> Self {
      // Java: super();
      TrueFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, _context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 0)
      if args.is_empty() {
         // Java: return evaluate();
         Ok(Arc::new(Self::evaluate()))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("true() requires no arguments.");
         Err(FunctionCallException::new("true() requires no arguments.".to_string()))
      }
    }

    // Java: static Boolean evaluate()
    pub fn evaluate() -> bool {
      // Java: return Boolean.TRUE;
      true
    }
}

impl Function for TrueFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
