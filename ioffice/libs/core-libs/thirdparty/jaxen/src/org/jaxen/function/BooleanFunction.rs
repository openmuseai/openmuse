use std::sync::Arc;
use crate::org::jaxen::{Object, List};
use crate::org::jaxen::Context::Context;
use super::Function;
use crate::org::jaxen::FunctionCallException::FunctionCallException;
use crate::org::jaxen::Navigator::Navigator;

// Java: public class BooleanFunction implements Function
pub struct BooleanFunction;

// Java: public class BooleanFunction implements Function
impl BooleanFunction {
    // Java: BooleanFunction()
    pub fn new() -> Self {
        // Java: super();
        BooleanFunction
    }

    // Java: Object call(Context context, List args)
    pub fn call_fn(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
        // Java: if (args.size() == 1)
        if args.len() == 1 {
            // Java: return evaluate(args.get(0), context.getNavigator());
            let result = Self::evaluate(&args[0], context.getNavigator());
            Ok(Arc::new(result))
        }
        // Java: else
        else {
            // Java: throw new FunctionCallException("boolean() requires one argument");
            Err(FunctionCallException::new("boolean() requires one argument".to_string()))
        }
    }

    // Java: static Boolean evaluate(Object obj, Navigator nav)
    pub fn evaluate(obj: &Object, _nav: Option<&dyn Navigator>) -> bool {
        // Java: if (obj instanceof List)
        if let Some(list) = obj.downcast_ref::<Vec<Object>>() {
            // Java: List list = (List)obj;
            // Java: if (list.size() == 0)
            if list.is_empty() {
                // Java: return Boolean.FALSE;
                return false;
            }
            // Java: obj = list.get(0);
            return Self::evaluate(&list[0], _nav);
        }
        // Java: if (obj instanceof Boolean)
        if let Some(b) = obj.downcast_ref::<bool>() {
            // Java: return (Boolean)obj;
            return *b;
        }
        // Java: else if (obj instanceof Number)
        else if let Some(d) = obj.downcast_ref::<f64>() {
            // Java: double d = ((Number)obj).doubleValue();
            let d = *d;
            // Java: return d != (double)0.0F && !Double.isNaN(d) ? Boolean.TRUE : Boolean.FALSE;
            return d != 0.0 && !d.is_nan();
        }
        // Java: else if (obj instanceof String)
        else if let Some(s) = obj.downcast_ref::<String>() {
            // Java: return ((String)obj).length() > 0 ? Boolean.TRUE : Boolean.FALSE;
            return !s.is_empty();
        }
        // Java: else
        else {
            // Java: return obj != null ? Boolean.TRUE : Boolean.FALSE;
            true
        }
    }
}

impl Function for BooleanFunction {
    fn call(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
        self.call_fn(context, args)
    }
}
