// Java: import cn.wps.io.dom.io.DocumentEndOfParseException
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

/// Shared mutable handler reference (Java interface object identity).
pub type ElementHandlerRef = Rc<RefCell<dyn ElementHandlerSimple>>;

// Java: public interface ElementHandlerSimple
pub trait ElementHandlerSimple {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // Java: abstract int filterIndex()
    fn filterIndex(&self) -> i32;

    // Java: abstract void onStart(String var1, String var2, String var3, Attributes var4)
    fn onStart(&mut self, var1: &str, var2: &str, var3: &str, var4: &dyn std::any::Any) -> Result<(), Box<dyn std::error::Error>>;

    // Java: abstract void onEnd(String var1, String var2, String var3)
    fn onEnd(&mut self, var1: &str, var2: &str, var3: &str) -> Result<(), Box<dyn std::error::Error>>;

    // Java: abstract void onStart(int var1, String var2, String var3, String var4, Attributes var5)
    fn onStart_with_id(&mut self, var1: i32, var2: &str, var3: &str, var4: &str, var5: &dyn std::any::Any) -> Result<(), Box<dyn std::error::Error>>;

    // Java: abstract void onEnd(int var1, String var2)
    fn onEnd_with_id(&mut self, var1: i32, var2: &str) -> Result<(), Box<dyn std::error::Error>>;

    // Java: abstract void onCharacters(char[] var1, int var2, int var3)
    fn onCharacters(&mut self, var1: &[char], var2: i32, var3: i32) -> Result<(), Box<dyn std::error::Error>>;

    // Java: abstract void onCharacters(String var1)
    fn onCharacters_str(&mut self, var1: &str) -> Result<(), Box<dyn std::error::Error>>;

    // Java: abstract ElementHandlerSimple getElementHandler(int var1, String var2)
    fn getElementHandler(&self, var1: i32, var2: &str) -> Option<ElementHandlerRef>;

}
