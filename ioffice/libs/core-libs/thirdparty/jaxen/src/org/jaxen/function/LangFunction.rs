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
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, List};
use super::StringFunction::StringFunction;
use std::sync::Arc;

// Java: public class LangFunction implements Function
pub struct LangFunction {
}

// Java field: String LANG_LOCALNAME
const LANG_LOCALNAME: &str = "lang";
// Java field: String XMLNS_URI
const XMLNS_URI: &str = "http://www.w3.org/XML/1998/namespace";

// Java: public class LangFunction implements Function
impl LangFunction {
    // Java: LangFunction()
    pub fn new() -> Self {
      // Java: super();
      LangFunction {}
    }

    // Java: Object call(Context context, List args)
    pub fn call_impl(&self, context: &Context, args: &List) -> Result<Object, FunctionCallException> {
      // Java: if (args.size() != 1)
      if args.len() != 1 {
         // Java: throw new FunctionCallException("lang() requires exactly one argument.");
         return Err(FunctionCallException::new("lang() requires exactly one argument.".to_string()));
      }
      // Java: else
      {
         // Java: Object arg = args.get(0);
         let arg = &args[0];
         // Java: try
         // Java: return evaluate(context.getNodeSet(), arg, context.getNavigator());
         let nav = context.getNavigator().ok_or_else(|| FunctionCallException::new("No navigator".to_string()))?;
         match Self::evaluate_list(context.getNodeSet(), arg, nav) {
            Ok(result) => Ok(Arc::new(result)),
            // Java: catch (UnsupportedAxisException e)
            Err(_e) => {
               // Java: throw new FunctionCallException("Can't evaluate lang()", e);
               Err(FunctionCallException::new("Can't evaluate lang()".to_string()))
            }
         }
      }
    }

    // Java: private static Boolean evaluate(List contextNodes, Object lang, Navigator nav)
    fn evaluate_list(contextNodes: &List, lang: &Object, nav: &dyn Navigator) -> Result<bool, UnsupportedAxisException> {
      // Java: return evaluate(contextNodes.get(0), StringFunction.evaluate(lang, nav), nav) ? Boolean.TRUE : Boolean.FALSE;
      let lang_str = StringFunction::evaluate(lang, Some(nav));
      Self::evaluate_node(&contextNodes[0], &lang_str, nav)
    }

    // Java: private static boolean evaluate(Object node, String lang, Navigator nav)
    fn evaluate_node(node: &Object, lang: &str, nav: &dyn Navigator) -> Result<bool, UnsupportedAxisException> {
      // Java: Object element = node;
      let mut element: Object = Arc::clone(node);
      // Java: if (!nav.isElement(node))
      if !nav.isElement(node) {
         // Java: element = nav.getParentNode(node);
         element = nav.getParentNode(node)?;
      }

      // Java: while(element != null && nav.isElement(element))
      loop {
         if !nav.isElement(&element) {
            break;
         }
         // Java: Iterator attrs = nav.getAttributeAxisIterator(element);
         let attrs = nav.getAttributeAxisIterator(&element)?;
         // Java: while(attrs.hasNext())
         for attr in attrs {
            // Java: Object attr = attrs.next();
            // Java: if ("lang".equals(nav.getAttributeName(attr)) && "http://www.w3.org/XML/1998/namespace".equals(nav.getAttributeNamespaceUri(attr)))
            if nav.getAttributeName(&attr) == LANG_LOCALNAME && nav.getAttributeNamespaceUri(&attr) == XMLNS_URI {
               // Java: return isSublang(nav.getAttributeStringValue(attr), lang);
               return Ok(Self::isSublang(&nav.getAttributeStringValue(&attr), lang));
            }
         }
         // Java: element = nav.getParentNode(element);
         element = nav.getParentNode(&element)?;
      }

      // Java: return false;
      Ok(false)
    }

    // Java: private static boolean isSublang(String sublang, String lang)
    fn isSublang(sublang: &str, lang: &str) -> bool {
      // Java: if (sublang.equalsIgnoreCase(lang))
      if sublang.eq_ignore_ascii_case(lang) {
         // Java: return true;
         return true;
      }
      // Java: else
      {
         // Java: int ll = lang.length();
         let ll = lang.len();
         // Java: return sublang.length() > ll && sublang.charAt(ll) == '-' && sublang.substring(0, ll).equalsIgnoreCase(lang);
         sublang.len() > ll && sublang.as_bytes()[ll] == b'-' && sublang[..ll].eq_ignore_ascii_case(lang)
      }
    }
}

impl Function for LangFunction {
    fn call(&self, var1: &Context, var2: &List) -> Result<Object, FunctionCallException> {
        self.call_impl(var1, var2)
    }
}
