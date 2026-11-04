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

// Java: public class StringLengthFunction implements Function
pub struct StringLengthFunction {
}

// Java: public class StringLengthFunction implements Function
impl StringLengthFunction {
    // Java: StringLengthFunction()
    pub fn new() -> Self {
      // Java: super();
      StringLengthFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 0)
      if args.is_empty() {
         // Java: return evaluate(context.getNodeSet(), context.getNavigator());
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         let nodeset: Object = Arc::new(context.getNodeSet().clone());
         let result = Self::evaluate(&nodeset, nav)?;
         Ok(Arc::new(result))
      }
      // Java: else if (args.size() == 1)
      else if args.len() == 1 {
         // Java: return evaluate(args.get(0), context.getNavigator());
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         let result = Self::evaluate(&args[0], nav)?;
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("string-length() requires one argument.");
         Err(FunctionCallException::new("string-length() requires one argument.".to_string()))
      }
    }

    // Java: static Double evaluate(Object obj, Navigator nav)
    pub fn evaluate(obj: &Object, nav: &dyn Navigator) -> Result<f64, FunctionCallException> {
      // Java: String str = StringFunction.evaluate(obj, nav);
      let str_val = StringFunction::evaluate(obj, Some(nav));
      // Java: char[] data = str.toCharArray();
      let data: Vec<u16> = str_val.encode_utf16().collect();
      // Java: int length = 0;
      let mut length = 0i32;

      // Java: for(int i = 0; i < data.length; ++i)
      let mut i = 0;
      while i < data.len() {
         // Java: char c = data[i];
         let c = data[i];
         // Java: ++length;
         length += 1;
         // Java: if (c >= '\ud800' && c <= '\udfff')
         if c >= 0xD800 && c <= 0xDFFF {
            // Java: try
            if i + 1 < data.len() {
               // Java: char low = data[i + 1];
               let low = data[i + 1];
               // Java: if (low < '\udc00' || low > '\udfff')
               if low < 0xDC00 || low > 0xDFFF {
                  // Java: throw new FunctionCallException("Bad surrogate pair in string " + str);
                  return Err(FunctionCallException::new(format!("Bad surrogate pair in string {}", str_val)));
               }
               // Java: ++i;
               i += 1;
            } else {
               // Java: catch (ArrayIndexOutOfBoundsException var8)
               // Java: throw new FunctionCallException("Bad surrogate pair in string " + str);
               return Err(FunctionCallException::new(format!("Bad surrogate pair in string {}", str_val)));
            }
         }
         i += 1;
      }

      // Java: return new Double((double)length);
      Ok(length as f64)
    }
}

impl Function for StringLengthFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
