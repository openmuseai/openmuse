// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// Java: import java.io.IOException
// Java: import java.util.Stack

// Java: public class UofHandlerBase implements ElementHandler
pub struct UofHandlerBase {
}

// Java: public class TAG
pub struct TAG {
    // Java field: int name
    name: i32,
    // Java field: boolean hasContent
    hasContent: bool,
    // Java field: boolean closed
    closed: bool,
}

// Java field: Stack<TAG> mStack
static M_STACK: std::sync::OnceLock<std::sync::Mutex<Vec<TAG>>> = std::sync::OnceLock::new();

fn get_mStack() -> &'static std::sync::Mutex<Vec<TAG>> {
    M_STACK.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

// Java: public class UofHandlerBase implements ElementHandler
impl UofHandlerBase {
    // Java: UofHandlerBase()
    pub fn UofHandlerBase() -> Self {
      // Java: super();
      UofHandlerBase {}
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {
    }

    // Java: String buildAttributesString(XmlRoAttr attributes)
    fn buildAttributesString(&self, attributes: &dyn XmlRoAttr) -> String {
      // Java: StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // Java: int length = attributes.count();
      let length = attributes.count();
      // Java: for(int i = 0; i < length; ++i)
      for _i in 0..length
      {
         // Java: int qname = attributes.getNameAt(0);
         let qname = attributes.get_name_at(0);
         // Java: String value = attributes.getByName(qname).toString();
         let value = attributes.get_by_name(qname).map(|a| a.to_string()).unwrap_or_default();
         // Java: sb.append(String.format(" %s=\"%s\"", qname, value));
         sb.push_str(&format!(" {}=\"{}\"", qname, value));
      }
      // Java: return sb.toString();
      sb
    }

    // Java: String tabs()
    fn tabs(&self) -> String {
      // Java: StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // Java: for(int i = 0; i < mStack.size() - 1; ++i)
      let stack = get_mStack().lock().unwrap();
      let size = stack.len() as i32;
      for _i in 0..(size - 1).max(0)
      {
         // Java: sb.append('\t');
         sb.push('\t');
      }
      // Java: return sb.toString();
      sb
    }

    // Java: void endElement(int elementId)
    pub fn endElement(&self, _elementId: i32) {
    }

    // Java: void characters(String chars)
    pub fn characters(&self, _chars: &str) {
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // Java: return null;
      None
    }

}

// Java: public class TAG
impl TAG {
    fn TAG(tag: i32) -> Self {
        TAG {
            name: tag,
            hasContent: false,
            closed: false,
        }
    }
}

impl ElementHandler for UofHandlerBase {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) {
        self.startElement(var1, var2);
    }
    fn endElement(&self, var1: i32) {
        self.endElement(var1);
    }
    fn characters(&self, var1: &str) {
        self.characters(var1);
    }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> {
        self.enterSubElement(var1)
    }
}
