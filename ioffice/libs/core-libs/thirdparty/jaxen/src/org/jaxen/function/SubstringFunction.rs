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
use super::StringLengthFunction::StringLengthFunction;
use super::NumberFunction::NumberFunction;
use super::RoundFunction::RoundFunction;
use std::sync::Arc;

// Java: public class SubstringFunction implements Function
pub struct SubstringFunction {
}

// Java: public class SubstringFunction implements Function
impl SubstringFunction {
    // Java: SubstringFunction()
    pub fn new() -> Self {
      // Java: super();
      SubstringFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: int argc = args.size();
      let argc = args.len();
      // Java: if (argc >= 2 && argc <= 3)
      if argc >= 2 && argc <= 3 {
         // Java: Navigator nav = context.getNavigator();
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         // Java: String str = StringFunction.evaluate(args.get(0), nav);
         let str_val = StringFunction::evaluate(&args[0], Some(nav));
         // Java: if (str == null)
         // Java: return "";
         // Java: else
         {
            // Java: int stringLength = StringLengthFunction.evaluate(args.get(0), nav).intValue();
            let stringLength = StringLengthFunction::evaluate(&args[0], nav)? as i32;
            // Java: if (stringLength == 0)
            if stringLength == 0 {
               // Java: return "";
               return Ok(Arc::new(String::new()));
            }
            // Java: else
            {
               // Java: Double d1 = NumberFunction.evaluate(args.get(1), nav);
               let d1 = NumberFunction::evaluate(&args[1], Some(nav));
               // Java: if (d1.isNaN())
               if d1.is_nan() {
                  // Java: return "";
                  return Ok(Arc::new(String::new()));
               }
               // Java: else
               {
                  // Java: int start = RoundFunction.evaluate(d1, nav).intValue() - 1;
                  let d1_obj: Object = Arc::new(d1);
                  let mut start = RoundFunction::evaluate(&d1_obj, nav) as i32 - 1;
                  // Java: int substringLength = stringLength;
                  let mut substringLength = stringLength;
                  // Java: if (argc == 3)
                  if argc == 3 {
                     // Java: Double d2 = NumberFunction.evaluate(args.get(2), nav);
                     let d2 = NumberFunction::evaluate(&args[2], Some(nav));
                     // Java: if (!d2.isNaN())
                     if !d2.is_nan() {
                        // Java: substringLength = RoundFunction.evaluate(d2, nav).intValue();
                        let d2_obj: Object = Arc::new(d2);
                        substringLength = RoundFunction::evaluate(&d2_obj, nav) as i32;
                     }
                     // Java: else
                     else {
                        // Java: substringLength = 0;
                        substringLength = 0;
                     }
                  }

                  // Java: if (substringLength < 0)
                  if substringLength < 0 {
                     // Java: return "";
                     return Ok(Arc::new(String::new()));
                  }
                  // Java: else
                  {
                     // Java: int end = start + substringLength;
                     let mut end = start + substringLength;
                     // Java: if (argc == 2)
                     if argc == 2 {
                        // Java: end = stringLength;
                        end = stringLength;
                     }

                     // Java: if (start < 0)
                     if start < 0 {
                        // Java: start = 0;
                        start = 0;
                     }
                     // Java: else if (start > stringLength)
                     else if start > stringLength {
                        // Java: return "";
                        return Ok(Arc::new(String::new()));
                     }

                     // Java: if (end > stringLength)
                     if end > stringLength {
                        // Java: end = stringLength;
                        end = stringLength;
                     }
                     // Java: else if (end < start)
                     else if end < start {
                        // Java: return "";
                        return Ok(Arc::new(String::new()));
                     }

                     // Java: return stringLength == str.length() ? str.substring(start, end) : unicodeSubstring(str, start, end);
                     let result = if stringLength as usize == str_val.len() {
                        str_val[start as usize..end as usize].to_string()
                     } else {
                        Self::unicodeSubstring(&str_val, start as usize, end as usize)
                     };
                     Ok(Arc::new(result))
                  }
               }
            }
         }
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("substring() requires two or three arguments.");
         Err(FunctionCallException::new("substring() requires two or three arguments.".to_string()))
      }
    }

    // Java: static String unicodeSubstring(String s, int start, int end)
    fn unicodeSubstring(s: &str, start: usize, end: usize) -> String {
      // Java: StringBuffer result = new StringBuffer(s.length());
      let mut result = String::with_capacity(s.len());
      // Java: int jChar = 0;
      let mut jChar = 0usize;
      let chars: Vec<char> = s.chars().collect();

      // Java: for(int uChar = 0; uChar < end; ++uChar)
      let mut uChar = 0usize;
      while uChar < end && jChar < chars.len() {
         // Java: char c = s.charAt(jChar);
         let c = chars[jChar];
         // Java: if (uChar >= start)
         if uChar >= start {
            // Java: result.append(c);
            result.push(c);
         }

         // Java: if (c >= '\ud800')
         if c as u32 >= 0xD800 {
            // Java: ++jChar;
            jChar += 1;
            // Java: if (uChar >= start)
            if uChar >= start && jChar < chars.len() {
               // Java: result.append(s.charAt(jChar));
               result.push(chars[jChar]);
            }
         }

         // Java: ++jChar;
         jChar += 1;
         uChar += 1;
      }

      // Java: return result.toString();
      result
    }
}

impl Function for SubstringFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
