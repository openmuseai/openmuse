// Java: import org.dom4j.Attribute
// Java: import org.dom4j.Element
// Java: import org.dom4j.QName

// Java: public class AttributeHelper
pub struct AttributeHelper {
}

// Java: public class AttributeHelper
impl AttributeHelper {
    // Java: AttributeHelper()
    pub fn new() -> Self {
        // Java: super();
        AttributeHelper {}
    }

    // Java: static boolean booleanValue(Element element, String attributeName)
    pub fn booleanValue_by_name(_attribute_value: Option<&str>) -> bool {
        // Java: return booleanValue(element.attribute(attributeName));
        Self::booleanValue_str(_attribute_value)
    }

    // Java: static boolean booleanValue(Element element, QName attributeQName)
    pub fn booleanValue_by_qname(_attribute_value: Option<&str>) -> bool {
        // Java: return booleanValue(element.attribute(attributeQName));
        Self::booleanValue_str(_attribute_value)
    }

    // Java: static boolean booleanValue(Attribute attribute)
    pub fn booleanValue_str(value: Option<&str>) -> bool {
        // Java: if (attribute == null)
        if value.is_none()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: Object value = attribute.getData();
            let val = value.unwrap();
            // Java: if (value == null)
            if val.is_empty()
            {
                // Java: return false;
                return false;
            }
            // Java: else if (value instanceof Boolean)
            {
                // Java: Boolean b = (Boolean)value;
                // Java: return b;
            }
            // Java: else
            {
                // Java: return "true".equalsIgnoreCase(value.toString());
                return val.eq_ignore_ascii_case("true");
            }
        }
    }
}
