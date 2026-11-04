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

// Java: public class NormalizeSpaceFunction implements Function
pub struct NormalizeSpaceFunction {
}

// Java: public class NormalizeSpaceFunction implements Function
impl NormalizeSpaceFunction {
    // Java: NormalizeSpaceFunction()
    pub fn new() -> Self {
      // Java: super();
      NormalizeSpaceFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 0)
      if args.is_empty() {
         // Java: return evaluate(context.getNodeSet(), context.getNavigator());
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         let nodeset: Object = Arc::new(context.getNodeSet().clone());
         let result = Self::evaluate(&nodeset, Some(nav));
         Ok(Arc::new(result))
      }
      // Java: else if (args.size() == 1)
      else if args.len() == 1 {
         // Java: return evaluate(args.get(0), context.getNavigator());
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         let result = Self::evaluate(&args[0], Some(nav));
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("normalize-space() cannot have more than one argument");
         Err(FunctionCallException::new("normalize-space() cannot have more than one argument".to_string()))
      }
    }

    // Java: static String evaluate(Object strArg, Navigator nav)
    pub fn evaluate(strArg: &Object, nav: Option<&dyn Navigator>) -> String {
      // Java: String str = StringFunction.evaluate(strArg, nav);
      let str_val = StringFunction::evaluate(strArg, nav);
      // Java: char[] buffer = str.toCharArray();
      let buffer: Vec<char> = str_val.chars().collect();
      // Java: int write = 0;
      let mut write = 0usize;
      // Java: int lastWrite = 0;
      let mut lastWrite = 0usize;
      // Java: boolean wroteOne = false;
      let mut wroteOne = false;
      // Java: int read = 0;
      let mut read = 0usize;
      let mut out: Vec<char> = vec!['\0'; buffer.len()];

      // Java: while(read < buffer.length)
      while read < buffer.len() {
         // Java: if (isXMLSpace(buffer[read]))
         if Self::isXMLSpace(buffer[read]) {
            // Java: if (wroteOne)
            if wroteOne {
               // Java: buffer[write++] = ' ';
               out[write] = ' ';
               write += 1;
            }
            // Java: while(true)
            loop {
               // Java: ++read;
               read += 1;
               // Java: if (read >= buffer.length || !isXMLSpace(buffer[read]))
               if read >= buffer.len() || !Self::isXMLSpace(buffer[read]) {
                  // Java: break;
                  break;
               }
            }
         }
         // Java: else
         else {
            // Java: buffer[write++] = buffer[read++];
            out[write] = buffer[read];
            write += 1;
            read += 1;
            // Java: wroteOne = true;
            wroteOne = true;
            // Java: lastWrite = write;
            lastWrite = write;
         }
      }

      // Java: return new String(buffer, 0, lastWrite);
      out[..lastWrite].iter().collect()
    }

    // Java: static boolean isXMLSpace(char c)
    fn isXMLSpace(c: char) -> bool {
      // Java: return c == ' ' || c == '\n' || c == '\r' || c == '\t';
      c == ' ' || c == '\n' || c == '\r' || c == '\t'
    }
}

impl Function for NormalizeSpaceFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
