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
use super::NumberFunction::NumberFunction;
use std::sync::Arc;
use std::any::Any;

// Java: public class SumFunction implements Function
pub struct SumFunction {
}

// Java: public class SumFunction implements Function
impl SumFunction {
    // Java: SumFunction()
    pub fn new() -> Self {
      // Java: super();
      SumFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 1)
      if args.len() == 1 {
         // Java: return evaluate(args.get(0), context.getNavigator());
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         let result = Self::evaluate(&args[0], nav)?;
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("sum() requires one argument.");
         Err(FunctionCallException::new("sum() requires one argument.".to_string()))
      }
    }

    // Java: static Double evaluate(Object obj, Navigator nav)
    pub fn evaluate(obj: &Object, nav: &dyn Navigator) -> Result<f64, FunctionCallException> {
      // Java: double sum = (double)0.0F;
      let mut sum: f64 = 0.0;
      // Java: if (!(obj instanceof List))
      if obj.downcast_ref::<List>().is_none() {
         // Java: throw new FunctionCallException("The argument to the sum function must be a node-set");
         return Err(FunctionCallException::new("The argument to the sum function must be a node-set".to_string()));
      }
      // Java: else
      {
         let list = obj.downcast_ref::<List>().unwrap();
         // Java: double term;
         // Java: for(Iterator nodeIter = ((List)obj).iterator(); nodeIter.hasNext(); sum += term)
         for item in list.iter() {
            // Java: term = NumberFunction.evaluate(nodeIter.next(), nav);
            let term = NumberFunction::evaluate(item, Some(nav));
            sum += term;
         }

         // Java: return new Double(sum);
         Ok(sum)
      }
    }
}

impl Function for SumFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
