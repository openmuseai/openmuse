// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.io.TypeConversion
// Java: import org.xml.sax.Attributes

pub trait Attributes {
    fn getLength(&self) -> i32;
    fn getValue_name(&self, name: &str) -> Option<String>;
    fn getValue_ns(&self, namespace: &str, name: &str) -> Option<String>;
}

// Java field: String EMPTY_NAMESPACE
pub const EMPTY_NAMESPACE: &str = "";

// Java: public class ConvertHelper
pub struct ConvertHelper {
}

// Java: public class ConvertHelper
impl ConvertHelper {
    // Java: ConvertHelper()
    pub fn ConvertHelper() -> Self {
      // Java: super();
      ConvertHelper {}
    }

    // Java: static String getString4EmptyNamespace(Attributes attributes, String name)
    pub fn getString4EmptyNamespace(attributes: &dyn Attributes, name: &str) -> Option<String> {
      // Java: return getString(attributes, name, "");
      Self::getString_ns(attributes, name, "")
    }

    // Java: static String getString(Attributes attributes, String name)
    pub fn getString(attributes: &dyn Attributes, name: &str) -> Option<String> {
      // Java: IOAssert.assertFalse(name.contains(":"));
      assert!(!name.contains(":"));
      // Java: return attributes.getLength() == 0 ? null : attributes.getValue(name);
      if attributes.getLength() == 0 { None } else { attributes.getValue_name(name) }
    }

    // Java: static String getString(Attributes attributes, String name, String namespace)
    pub fn getString_ns(attributes: &dyn Attributes, name: &str, namespace: &str) -> Option<String> {
      // Java: IOAssert.assertNotNull("attributes should not be null", attributes);
      // Java: IOAssert.assertNotNull("name should not be null", name);
      // Java: IOAssert.assertNotNull("namespace should not be null", namespace);
      // Java: return attributes.getLength() == 0 ? null : attributes.getValue(namespace, name);
      if attributes.getLength() == 0 { None } else { attributes.getValue_ns(namespace, name) }
    }

    // Java: static Boolean toBoolean(Attributes attributes, String name)
    pub fn toBoolean(attributes: &dyn Attributes, name: &str) -> Option<bool> {
      // Java: return toBoolean(attributes, name, "");
      Self::toBoolean_ns(attributes, name, "")
    }

    // Java: static Boolean toBoolean(Attributes attributes, String name, String namespace)
    fn toBoolean_ns(attributes: &dyn Attributes, name: &str, namespace: &str) -> Option<bool> {
      // Java: if (attributes.getLength() == 0)
      if attributes.getLength() == 0
      {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: String boldStr = attributes.getValue(namespace, name);
         let boldStr = attributes.getValue_ns(namespace, name);
         // Java: if (null == boldStr)
         if boldStr.is_none()
         {
            // Java: return null;
            return None;
         }
         // Java: else
         {
            // Java: return isFalse(boldStr) ? Boolean.FALSE : Boolean.TRUE;
            let s = boldStr.unwrap();
            return Some(!Self::isFalse(&s));
         }
      }
    }

    // Java: static boolean isFalse(String str)
    fn isFalse(str_val: &str) -> bool {
      // Java: IOAssert.assertNotNull("str should not be null", str);
      // Java: return "off".equalsIgnoreCase(str) || "0".equals(str) || "false".equalsIgnoreCase(str) || "f".equalsIgnoreCase(str);
      str_val.eq_ignore_ascii_case("off") || str_val == "0" || str_val.eq_ignore_ascii_case("false") || str_val.eq_ignore_ascii_case("f")
    }

    // Java: static Boolean toBoolean4STTrueFalse(Attributes attributes, String name)
    pub fn toBoolean4STTrueFalse(attributes: &dyn Attributes, name: &str) -> Option<bool> {
      // Java: return toBoolean4STTrueFalse(attributes, name, "");
      Self::toBoolean4STTrueFalse_ns(attributes, name, "")
    }

    // Java: static Boolean toBoolean4STTrueFalse(Attributes attributes, String name, String namespace)
    pub fn toBoolean4STTrueFalse_ns(attributes: &dyn Attributes, name: &str, namespace: &str) -> Option<bool> {
      // Java: if (attributes.getLength() == 0)
      if attributes.getLength() == 0
      {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: String boldStr = attributes.getValue(namespace, name);
         let boldStr = attributes.getValue_ns(namespace, name);
         // Java: if (null == boldStr)
         if boldStr.is_none()
         {
            // Java: return null;
            return None;
         }
         // Java: else
         {
            // Java: return !"true".equals(boldStr) && !"t".equals(boldStr) ? Boolean.FALSE : Boolean.TRUE;
            let s = boldStr.unwrap();
            return Some(s == "true" || s == "t");
         }
      }
    }

    // Java: static Float getFloat4EmptyNamespace(Attributes attributes, String name)
    pub fn getFloat4EmptyNamespace(attributes: &dyn Attributes, name: &str) -> Option<f32> {
      // Java: return getFloat(attributes, name, "");
      Self::getFloat(attributes, name, "")
    }

    // Java: static Float getFloat(Attributes attributes, String name, String namespace)
    pub fn getFloat(attributes: &dyn Attributes, name: &str, namespace: &str) -> Option<f32> {
      // Java: IOAssert.assertNotNull("attributes should not be null", attributes);
      // Java: IOAssert.assertNotNull("name should not be null", name);
      // Java: IOAssert.assertNotNull("namespace should not be null", namespace);
      // Java: String str = attributes.getValue(namespace, name);
      let str_val = attributes.getValue_ns(namespace, name);
      // Java: return null == str ? null : TypeConversion.string2Float(str);
      str_val.and_then(|s| s.parse::<f32>().ok())
    }

    // Java: static Integer getInt4EmptyNamespace(Attributes attributes, String name)
    pub fn getInt4EmptyNamespace(attributes: &dyn Attributes, name: &str) -> Option<i32> {
      // Java: String str = attributes.getValue("", name);
      let str_val = attributes.getValue_ns("", name);
      // Java: return null == str ? null : TypeConversion.string2Integer(str);
      str_val.and_then(|s| s.parse::<i32>().ok())
    }

    // Java: static Long getLong4EmptyNamespace(Attributes attributes, String name)
    pub fn getLong4EmptyNamespace(attributes: &dyn Attributes, name: &str) -> Option<i64> {
      // Java: String str = attributes.getValue("", name);
      let str_val = attributes.getValue_ns("", name);
      // Java: return null == str ? null : TypeConversion.string2Long(str);
      str_val.and_then(|s| s.parse::<i64>().ok())
    }

    // Java: static Integer getHex4EmptyNamespace(Attributes attributes, String name)
    pub fn getHex4EmptyNamespace(attributes: &dyn Attributes, name: &str) -> Option<i32> {
      // Java: String str = attributes.getValue("", name);
      let str_val = attributes.getValue_ns("", name);
      // Java: return null == str ? null : TypeConversion.hexString2Integer(str);
      str_val.and_then(|s| i32::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16).ok())
    }

}
