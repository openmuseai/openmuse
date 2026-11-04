// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.List
// Java: import java.util.Map
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

// Java: public class TranslateFunction implements Function
pub struct TranslateFunction {
}

// Java: public class TranslateFunction implements Function
impl TranslateFunction {
    // Java: TranslateFunction()
    pub fn new() -> Self {
      // Java: super();
      TranslateFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() == 3)
      if args.len() == 3 {
         // Java: return evaluate(args.get(0), args.get(1), args.get(2), context.getNavigator());
         let result = Self::evaluate(&args[0], &args[1], &args[2], context.getNavigator())?;
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("translate() requires three arguments.");
         Err(FunctionCallException::new("translate() requires three arguments.".to_string()))
      }
    }

    // Java: static String evaluate(Object strArg, Object fromArg, Object toArg, Navigator nav)
    pub fn evaluate(strArg: &Object, fromArg: &Object, toArg: &Object, nav: Option<&dyn Navigator>) -> Result<String, FunctionCallException> {
      // Java: String inStr = StringFunction.evaluate(strArg, nav);
      let inStr = StringFunction::evaluate(strArg, nav);
      // Java: String fromStr = StringFunction.evaluate(fromArg, nav);
      let fromStr = StringFunction::evaluate(fromArg, nav);
      // Java: String toStr = StringFunction.evaluate(toArg, nav);
      let toStr = StringFunction::evaluate(toArg, nav);
      // Java: Map characterMap = new HashMap();
      let mut characterMap: HashMap<String, Option<String>> = HashMap::new();
      // Java: String[] fromCharacters = toUnicodeCharacters(fromStr);
      let fromCharacters = Self::toUnicodeCharacters(&fromStr)?;
      // Java: String[] toCharacters = toUnicodeCharacters(toStr);
      let toCharacters = Self::toUnicodeCharacters(&toStr)?;
      // Java: int fromLen = fromCharacters.length;
      let fromLen = fromCharacters.len();
      // Java: int toLen = toCharacters.length;
      let toLen = toCharacters.len();

      // Java: for(int i = 0; i < fromLen; ++i)
      for i in 0..fromLen {
         // Java: String cFrom = fromCharacters[i];
         let cFrom = &fromCharacters[i];
         // Java: if (!characterMap.containsKey(cFrom))
         if !characterMap.contains_key(cFrom) {
            // Java: if (i < toLen)
            if i < toLen {
               // Java: characterMap.put(cFrom, toCharacters[i]);
               characterMap.insert(cFrom.clone(), Some(toCharacters[i].clone()));
            }
            // Java: else
            else {
               // Java: characterMap.put(cFrom, null);
               characterMap.insert(cFrom.clone(), None);
            }
         }
      }
      // Java: StringBuffer outStr = new StringBuffer(inStr.length());
      let mut outStr = String::with_capacity(inStr.len());
      // Java: for(String cIn : toUnicodeCharacters(inStr))
      for cIn in Self::toUnicodeCharacters(&inStr)? {
         // Java: if (characterMap.containsKey(cIn))
         if let Some(mapping) = characterMap.get(&cIn) {
            // Java: String cTo = (String)characterMap.get(cIn);
            // Java: if (cTo != null)
            if let Some(cTo) = mapping {
               // Java: outStr.append(cTo);
               outStr.push_str(cTo);
            }
         }
         // Java: else
         else {
            // Java: outStr.append(cIn);
            outStr.push_str(&cIn);
         }
      }
      // Java: return outStr.toString();
      Ok(outStr)
    }

    // Java: static String[] toUnicodeCharacters(String s)
    fn toUnicodeCharacters(s: &str) -> Result<Vec<String>, FunctionCallException> {
      // Java: String[] result = new String[s.length()];
      let mut result: Vec<String> = Vec::with_capacity(s.len());
      // Java: int stringLength = 0;
      let chars: Vec<char> = s.chars().collect();
      let mut i = 0;
      // Java: for(int i = 0; i < s.length(); ++i)
      while i < chars.len() {
         // Java: char c1 = s.charAt(i);
         let c1 = chars[i];
         // Java: if (isHighSurrogate(c1))
         if Self::isHighSurrogate(c1) {
            // Java: try
            if i + 1 < chars.len() {
               // Java: char c2 = s.charAt(i + 1);
               let c2 = chars[i + 1];
               // Java: if (!isLowSurrogate(c2))
               if !Self::isLowSurrogate(c2) {
                  // Java: throw new FunctionCallException("Mismatched surrogate pair in translate function");
                  return Err(FunctionCallException::new("Mismatched surrogate pair in translate function".to_string()));
               }
               // Java: result[stringLength] = (c1 + "" + c2).intern();
               let mut s2 = String::new();
               s2.push(c1);
               s2.push(c2);
               result.push(s2);
               // Java: ++i;
               i += 1;
            }
            // Java: catch (StringIndexOutOfBoundsException var6)
            else {
               // Java: throw new FunctionCallException("High surrogate without low surrogate at end of string passed to translate function");
               return Err(FunctionCallException::new("High surrogate without low surrogate at end of string passed to translate function".to_string()));
            }
         }
         // Java: else
         else {
            // Java: result[stringLength] = String.valueOf(c1).intern();
            result.push(c1.to_string());
         }
         // Java: ++stringLength;
         i += 1;
      }
      // Java: if (stringLength == result.length)
      // Java: return result;
      // Java: else
      // Java: String[] trimmed = new String[stringLength];
      // Java: System.arraycopy(result, 0, trimmed, 0, stringLength);
      // Java: return trimmed;
      Ok(result)
    }

    // Java: static boolean isHighSurrogate(char c)
    fn isHighSurrogate(c: char) -> bool {
      // Java: return c >= '\ud800' && c <= '\udbff';
      let v = c as u32;
      v >= 0xD800 && v <= 0xDBFF
    }

    // Java: static boolean isLowSurrogate(char c)
    fn isLowSurrogate(c: char) -> bool {
      // Java: return c >= '\udc00' && c <= '\udfff';
      let v = c as u32;
      v >= 0xDC00 && v <= 0xDFFF
    }
}

impl Function for TranslateFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
