// Java: import java.util.List
// Java: import java.util.Locale
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.FunctionCallException
use crate::org::jaxen::FunctionCallException::FunctionCallException;
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.function.StringFunction
use crate::org::jaxen::function::StringFunction::StringFunction;
use crate::org::jaxen::{Object, List};
use crate::org::jaxen::Function::Function;
use super::LocaleFunctionSupport::LocaleFunctionSupport;
use std::sync::Arc;

// Java: public class UpperFunction extends LocaleFunctionSupport
pub struct UpperFunction {
    pub(crate) base: LocaleFunctionSupport,
}

// Java: public class UpperFunction extends LocaleFunctionSupport
impl UpperFunction {
    // Java: UpperFunction()
    pub fn new() -> Self {
      // Java: super();
      UpperFunction { base: LocaleFunctionSupport::new() }
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: Navigator navigator = context.getNavigator();
      let navigator = context.getNavigator();
      // Java: int size = args.size();
      let size = args.len();
      // Java: if (size > 0)
      if size > 0 {
         // Java: Object text = args.get(0);
         let text = &args[0];
         // Java: Locale locale = null;
         // Java: if (size > 1)
         // Java: locale = this.getLocale(args.get(1), navigator);
         // Java: return evaluate(text, locale, navigator);
         let result = Self::evaluate(text, navigator);
         Ok(Arc::new(result))
      }
      // Java: else
      else {
         // Java: throw new FunctionCallException("upper-case() requires at least one argument.");
         Err(FunctionCallException::new("upper-case() requires at least one argument.".to_string()))
      }
    }

    // Java: static String evaluate(Object strArg, Locale locale, Navigator nav)
    pub fn evaluate(strArg: &Object, nav: Option<&dyn Navigator>) -> String {
      // Java: String str = StringFunction.evaluate(strArg, nav);
      let str_val = StringFunction::evaluate(strArg, nav);
      // Java: if (locale == null)
      // Java: locale = Locale.ENGLISH;
      // Java: return str.toUpperCase(locale);
      str_val.to_uppercase()
    }
}

impl Function for UpperFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
