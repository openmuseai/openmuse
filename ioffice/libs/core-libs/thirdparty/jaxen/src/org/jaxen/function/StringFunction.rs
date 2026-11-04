use std::sync::Arc;
use crate::org::jaxen::{Object, List};
use crate::org::jaxen::Context::Context;
use super::Function;
use crate::org::jaxen::FunctionCallException::FunctionCallException;
use crate::org::jaxen::Navigator::Navigator;

// Java field: DecimalFormat format
// (not needed in Rust - using native formatting)

// Java: public class StringFunction implements Function
pub struct StringFunction;

// Java: public class StringFunction implements Function
impl StringFunction {
    // Java: StringFunction()
    pub fn new() -> Self {
        // Java: super();
        StringFunction
    }

    // Java: Object call(Context context, List args)
    pub fn call_fn(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
        // Java: int size = args.size();
        let size = args.len();
        // Java: if (size == 0)
        if size == 0 {
            // Java: return evaluate(context.getNodeSet(), context.getNavigator());
            let node_set: Object = Arc::new(context.getNodeSet().clone());
            let result = Self::evaluate(&node_set, context.getNavigator());
            Ok(Arc::new(result))
        }
        // Java: else if (size == 1)
        else if size == 1 {
            // Java: return evaluate(args.get(0), context.getNavigator());
            let result = Self::evaluate(&args[0], context.getNavigator());
            Ok(Arc::new(result))
        }
        // Java: else
        else {
            // Java: throw new FunctionCallException("string() takes at most argument.");
            Err(FunctionCallException::new("string() takes at most argument.".to_string()))
        }
    }

    // Java: static String evaluate(Object obj, Navigator nav)
    pub fn evaluate(obj: &Object, nav: Option<&dyn Navigator>) -> String {
        // Java: try
        // Java: if (nav != null && nav.isText(obj))
        if let Some(n) = nav {
            if n.isText(obj) {
                // Java: return nav.getTextStringValue(obj);
                return n.getTextStringValue(obj);
            }
        }
        // Java: else
        // Java: if (obj instanceof List)
        if let Some(list) = obj.downcast_ref::<Vec<Object>>() {
            // Java: List list = (List)obj;
            // Java: if (list.isEmpty())
            if list.is_empty() {
                // Java: return "";
                return String::new();
            }
            // Java: obj = list.get(0);
            return Self::evaluate(&list[0], nav);
        }
        // Java: if (nav != null)
        if let Some(n) = nav {
            // Java: if (nav.isElement(obj))
            if n.isElement(obj) {
                // Java: return nav.getElementStringValue(obj);
                return n.getElementStringValue(obj);
            }
            // Java: if (nav.isAttribute(obj))
            if n.isAttribute(obj) {
                // Java: return nav.getAttributeStringValue(obj);
                return n.getAttributeStringValue(obj);
            }
            // Java: if (nav.isDocument(obj))
            if n.isDocument(obj) {
                // Java: Iterator childAxisIterator = nav.getChildAxisIterator(obj);
                if let Ok(mut child_iter) = n.getChildAxisIterator(obj) {
                    // Java: while(childAxisIterator.hasNext())
                    while let Some(descendant) = child_iter.next() {
                        // Java: Object descendant = childAxisIterator.next();
                        // Java: if (nav.isElement(descendant))
                        if n.isElement(&descendant) {
                            // Java: return nav.getElementStringValue(descendant);
                            return n.getElementStringValue(&descendant);
                        }
                    }
                }
            }
            // Java: else
            else {
                // Java: if (nav.isProcessingInstruction(obj))
                if n.isProcessingInstruction(obj) {
                    // Java: return nav.getProcessingInstructionData(obj);
                    return n.getProcessingInstructionData(obj);
                }
                // Java: if (nav.isComment(obj))
                if n.isComment(obj) {
                    // Java: return nav.getCommentStringValue(obj);
                    return n.getCommentStringValue(obj);
                }
                // Java: if (nav.isText(obj))
                if n.isText(obj) {
                    // Java: return nav.getTextStringValue(obj);
                    return n.getTextStringValue(obj);
                }
                // Java: if (nav.isNamespace(obj))
                if n.isNamespace(obj) {
                    // Java: return nav.getNamespaceStringValue(obj);
                    return n.getNamespaceStringValue(obj);
                }
            }
        }
        // Java: if (obj instanceof String)
        if let Some(s) = obj.downcast_ref::<String>() {
            // Java: return (String)obj;
            return s.clone();
        }
        // Java: else if (obj instanceof Boolean)
        else if let Some(b) = obj.downcast_ref::<bool>() {
            // Java: return stringValue((Boolean)obj);
            return Self::stringValue_bool(*b);
        }
        // Java: else
        else {
            // Java: return obj instanceof Number ? stringValue(((Number)obj).doubleValue()) : "";
            if let Some(d) = obj.downcast_ref::<f64>() {
                return Self::stringValue_double(*d);
            }
            return String::new();
        }
        // Java: catch (UnsupportedAxisException e)
        // Java: throw new JaxenRuntimeException(e);
    }

    // Java: static String stringValue(double value)
    fn stringValue_double(value: f64) -> String {
        // Java: if (value == (double)0.0F)
        if value == 0.0 {
            // Java: return "0";
            return "0".to_string();
        }
        // Java: else
        else {
            // Java: String result = null;
            // Java: synchronized(format)
            // Java: result = format.format(value);
            // Java: return result;
            if value.is_nan() {
                "NaN".to_string()
            } else if value.is_infinite() {
                if value > 0.0 { "Infinity".to_string() } else { "-Infinity".to_string() }
            } else if value == value.trunc() && value.abs() < 1e15 {
                format!("{}", value as i64)
            } else {
                format!("{}", value)
            }
        }
    }

    // Java: static String stringValue(boolean value)
    fn stringValue_bool(value: bool) -> String {
        // Java: return value ? "true" : "false";
        if value { "true".to_string() } else { "false".to_string() }
    }
}

impl Function for StringFunction {
    fn call(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
        self.call_fn(context, args)
    }
}
