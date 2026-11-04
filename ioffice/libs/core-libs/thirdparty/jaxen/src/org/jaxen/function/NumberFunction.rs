use std::sync::Arc;
use crate::org::jaxen::{Object, List};
use crate::org::jaxen::Context::Context;
use super::Function;
use crate::org::jaxen::FunctionCallException::FunctionCallException;
use crate::org::jaxen::Navigator::Navigator;
use super::StringFunction::StringFunction;

// Java field: Double NaN
pub(crate) const NAN: f64 = f64::NAN;

// Java: public class NumberFunction implements Function
pub struct NumberFunction;

// Java: public class NumberFunction implements Function
impl NumberFunction {
    // Java: NumberFunction()
    pub fn new() -> Self {
        // Java: super();
        NumberFunction
    }

    // Java: Object call(Context context, List args)
    pub fn call_fn(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
        // Java: if (args.size() == 1)
        if args.len() == 1 {
            // Java: return evaluate(args.get(0), context.getNavigator());
            let result = Self::evaluate(&args[0], context.getNavigator());
            Ok(Arc::new(result))
        }
        // Java: else if (args.size() == 0)
        else if args.is_empty() {
            // Java: return evaluate(context.getNodeSet(), context.getNavigator());
            let node_set: Object = Arc::new(context.getNodeSet().clone());
            let result = Self::evaluate(&node_set, context.getNavigator());
            Ok(Arc::new(result))
        }
        // Java: else
        else {
            // Java: throw new FunctionCallException("number() takes at most one argument.");
            Err(FunctionCallException::new("number() takes at most one argument.".to_string()))
        }
    }

    // Java: static Double evaluate(Object obj, Navigator nav)
    pub fn evaluate(obj: &Object, nav: Option<&dyn Navigator>) -> f64 {
        // Java: if (obj instanceof Double)
        if let Some(d) = obj.downcast_ref::<f64>() {
            // Java: return (Double)obj;
            return *d;
        }
        // Java: else if (obj instanceof String)
        else if let Some(str_val) = obj.downcast_ref::<String>() {
            // Java: String str = (String)obj;
            // Java: try
            // Java: Double doubleValue = new Double(str);
            // Java: return doubleValue;
            if let Ok(double_value) = str_val.trim().parse::<f64>() {
                return double_value;
            }
            // Java: catch (NumberFormatException var4)
            else {
                // Java: return NaN;
                return f64::NAN;
            }
        }
        // Java: else if (!(obj instanceof List) && !(obj instanceof Iterator))
        else if obj.downcast_ref::<Vec<Object>>().is_none() {
            // Java: if (!nav.isElement(obj) && !nav.isAttribute(obj) && !nav.isText(obj) && !nav.isComment(obj) && !nav.isProcessingInstruction(obj) && !nav.isDocument(obj) && !nav.isNamespace(obj))
            // Java: if (obj instanceof Boolean)
            if let Some(b) = obj.downcast_ref::<bool>() {
                // Java: return obj == Boolean.TRUE ? new Double((double)1.0F) : new Double((double)0.0F);
                return if *b { 1.0 } else { 0.0 };
            }
            // Java: else
            else {
                // Java: return NaN;
                return f64::NAN;
            }
            // Java: else
            // Java: return evaluate(StringFunction.evaluate(obj, nav), nav);
        }
        // Java: else
        else {
            // Java: return evaluate(StringFunction.evaluate(obj, nav), nav);
            let s = StringFunction::evaluate(obj, nav);
            let str_obj: Object = Arc::new(s);
            return Self::evaluate(&str_obj, nav);
        }
    }

    // Java: static boolean isNaN(double val)
    pub fn isNaN_double(val: f64) -> bool {
        // Java: return Double.isNaN(val);
        val.is_nan()
    }

    // Java: static boolean isNaN(Double val)
    pub fn isNaN(val: f64) -> bool {
        // Java: return val.equals(NaN);
        val.is_nan()
    }
}

impl Function for NumberFunction {
    fn call(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
        self.call_fn(context, args)
    }
}
