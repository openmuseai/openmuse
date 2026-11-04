#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import org.dom4j.Element

use std::collections::HashMap;

pub struct Element {
    attributes: HashMap<String, String>,
    children: Vec<Element>,
    name: String,
    text: String,
}

impl Element {
    pub fn new(name: String) -> Self {
        Element {
            attributes: HashMap::new(),
            children: Vec::new(),
            name,
            text: String::new(),
        }
    }

    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    pub fn getName(&self) -> &str {
        &self.name
    }
}

// Java: public class XmlObject
pub struct XmlObject {
    // Java field: Element mElement
    pub(crate) mElement: Option<Element>,
}

// Java: public class XmlObject
impl XmlObject {
    // Java: XmlObject()
    pub fn new() -> Self {
      // Java: super();
      XmlObject {
          mElement: None,
      }
    }

    // Java: void setElement(Element e)
    pub fn setElement(&mut self, e: Element) {
      // Java: Assert.assertNotNull("e should not be null", e);
      // Java: this.mElement = e;
      self.mElement = Some(e);
    }

    // Java: void resetElement()
    pub fn resetElement(&mut self) {
      // Java: this.mElement = null;
      self.mElement = None;
    }
}
