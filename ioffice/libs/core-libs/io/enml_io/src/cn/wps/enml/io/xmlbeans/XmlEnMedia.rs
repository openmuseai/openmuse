#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::KSAssert;
// Java: import cn.wps.enml.io.ENMLMediaType
use adapter::cn::wps::enml::io::ENMLMediaType::ENMLMediaType;
// Java: import cn.wps.enml.io.IENMLMedia
use adapter::cn::wps::enml::io::IENMLMedia::IENMLMedia;
// Java: import org.dom4j.Attribute
// Java: import org.dom4j.Element

// Java field: String CHILD_ATTRIBUTE_TYPE
pub(crate) const CHILD_ATTRIBUTE_TYPE: &str = "type";
// Java field: String CHILD_ATTRIBUTE_HASH
pub(crate) const CHILD_ATTRIBUTE_HASH: &str = "hash";

pub struct Element {
    attributes: std::collections::HashMap<String, String>,
}

impl Element {
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }
}

// Java: public class XmlEnMedia implements IENMLMedia
pub struct XmlEnMedia {
    // Java field: Element mElement
    mElement: Element,
}

// Java: public class XmlEnMedia implements IENMLMedia
impl XmlEnMedia {
    // Java: XmlEnMedia(Element element)
    pub fn new(element: Element) -> Self {
      // Java: super();
      // Java: KSAssert.assertNotNull("element should not be null", element);
      // Java: this.mElement = element;
      XmlEnMedia {
          mElement: element,
      }
    }

    // Java: ENMLMediaType getType()
    pub fn getType(&self) -> ENMLMediaType {
      // Java: Attribute attr = this.mElement.attribute("type");
      let attr = self.mElement.attribute(CHILD_ATTRIBUTE_TYPE);
      // Java: if (null != attr)
      if let Some(attr_val) = attr {
         // Java: String val = attr.getValue();
         let val = attr_val;
         // Java: if (null != val)
         {
            // Java: return ENMLMediaType.fromString(val.trim());
            return ENMLMediaType::fromString(Some(val.trim()));
         }
      }
      // Java: return ENMLMediaType.UNKNOWN;
      ENMLMediaType::fromString(None)
    }

    // Java: String getHash()
    pub fn getHash(&self) -> Option<String> {
      // Java: Attribute attr = this.mElement.attribute("hash");
      let attr = self.mElement.attribute(CHILD_ATTRIBUTE_HASH);
      // Java: return null != attr ? attr.getValue() : null;
      attr.map(|v| v.to_string())
    }
}

impl IENMLMedia for XmlEnMedia {
    fn getType(&self) -> ENMLMediaType {
        self.getType()
    }

    fn getHash(&self) -> String {
        self.getHash().unwrap_or_default()
    }
}
