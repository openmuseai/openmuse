#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import org.dom4j.ElementHandler
// Java: import org.dom4j.ElementPath

pub trait ElementPath {
    fn getOnlyUseName(&self) -> bool;
    fn size(&self) -> i32;
    fn getPath(&self) -> String;
    fn getCurrentName(&self) -> String;
}

pub trait ElementHandler {
    fn onStart(&mut self, var1: &dyn ElementPath);
    fn onCharacters(&mut self, var1: &str);
    fn onEnd(&mut self, var1: &dyn ElementPath);
    fn shouldNotifyChild(&self) -> bool;
    fn onStartChild(&mut self, var1: &dyn ElementPath);
    fn onEndChild(&mut self, var1: &dyn ElementPath);
}

// Java: public class XmlDefaultHandler implements ElementHandler
pub struct XmlDefaultHandler {
}

// Java: public class XmlDefaultHandler implements ElementHandler
impl XmlDefaultHandler {
    // Java: XmlDefaultHandler()
    pub fn new() -> Self {
      // Java: super();
      XmlDefaultHandler {}
    }

    // Java: abstract String filterName()
    pub fn filterName(&self) -> String {
      todo!()
    }

    // Java: boolean shouldNotifyChild()
    pub fn shouldNotifyChild_val(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: void onStartChild(ElementPath elementPath)
    pub fn onStartChild_val(&mut self, _elementPath: &dyn ElementPath) {
    }

    // Java: void onEndChild(ElementPath elementPath)
    pub fn onEndChild_val(&mut self, _elementPath: &dyn ElementPath) {
    }
}

impl ElementHandler for XmlDefaultHandler {
    fn onStart(&mut self, _var1: &dyn ElementPath) {
    }

    fn onCharacters(&mut self, _var1: &str) {
    }

    fn onEnd(&mut self, _var1: &dyn ElementPath) {
    }

    fn shouldNotifyChild(&self) -> bool {
      self.shouldNotifyChild_val()
    }

    fn onStartChild(&mut self, _var1: &dyn ElementPath) {
      self.onStartChild_val(_var1)
    }

    fn onEndChild(&mut self, _var1: &dyn ElementPath) {
      self.onEndChild_val(_var1)
    }
}
