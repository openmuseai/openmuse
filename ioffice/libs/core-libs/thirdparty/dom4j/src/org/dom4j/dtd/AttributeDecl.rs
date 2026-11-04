use crate::org::dom4j::dtd::Declaration::Declaration;

// Java: public class AttributeDecl implements Declaration
pub struct AttributeDecl {
    // Java field: String elementName
    pub(crate) elementName: String,
    // Java field: String attributeName
    pub(crate) attributeName: String,
    // Java field: String type
    pub(crate) r#type: String,
    // Java field: String value
    pub(crate) value: String,
    // Java field: String valueDefault
    pub(crate) valueDefault: Option<String>,
}

// Java: public class AttributeDecl implements Declaration
impl AttributeDecl {
    // Java: AttributeDecl()
    pub fn new() -> Self {
        // Java: super();
        AttributeDecl {
            elementName: String::new(),
            attributeName: String::new(),
            r#type: String::new(),
            value: String::new(),
            valueDefault: None,
        }
    }

    // Java: AttributeDecl(String elementName, String attributeName, String type, String valueDefault, String value)
    pub fn new_with_args(element_name: String, attribute_name: String, type_val: String, value_default: Option<String>, value: String) -> Self {
        // Java: super();
        // Java: this.elementName = elementName;
        // Java: this.attributeName = attributeName;
        // Java: this.type = type;
        // Java: this.value = value;
        // Java: this.valueDefault = valueDefault;
        AttributeDecl {
            elementName: element_name,
            attributeName: attribute_name,
            r#type: type_val,
            value,
            valueDefault: value_default,
        }
    }

    // Java: String getElementName()
    pub fn getElementName(&self) -> &str {
        // Java: return this.elementName;
        &self.elementName
    }

    // Java: void setElementName(String elementName)
    pub fn setElementName(&mut self, element_name: String) {
        // Java: this.elementName = elementName;
        self.elementName = element_name;
    }

    // Java: String getAttributeName()
    pub fn getAttributeName(&self) -> &str {
        // Java: return this.attributeName;
        &self.attributeName
    }

    // Java: void setAttributeName(String attributeName)
    pub fn setAttributeName(&mut self, attribute_name: String) {
        // Java: this.attributeName = attributeName;
        self.attributeName = attribute_name;
    }

    // Java: String getType()
    pub fn getType(&self) -> &str {
        // Java: return this.type;
        &self.r#type
    }

    // Java: void setType(String type)
    pub fn setType(&mut self, type_val: String) {
        // Java: this.type = type;
        self.r#type = type_val;
    }

    // Java: String getValue()
    pub fn getValue(&self) -> &str {
        // Java: return this.value;
        &self.value
    }

    // Java: void setValue(String value)
    pub fn setValue(&mut self, value: String) {
        // Java: this.value = value;
        self.value = value;
    }

    // Java: String getValueDefault()
    pub fn getValueDefault(&self) -> Option<&str> {
        // Java: return this.valueDefault;
        self.valueDefault.as_deref()
    }

    // Java: void setValueDefault(String valueDefault)
    pub fn setValueDefault(&mut self, value_default: Option<String>) {
        // Java: this.valueDefault = valueDefault;
        self.valueDefault = value_default;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuffer buffer = new StringBuffer("<!ATTLIST ");
        let mut buffer = String::from("<!ATTLIST ");
        // Java: buffer.append(this.elementName);
        buffer.push_str(&self.elementName);
        // Java: buffer.append(" ");
        buffer.push(' ');
        // Java: buffer.append(this.attributeName);
        buffer.push_str(&self.attributeName);
        // Java: buffer.append(" ");
        buffer.push(' ');
        // Java: buffer.append(this.type);
        buffer.push_str(&self.r#type);
        // Java: buffer.append(" ");
        buffer.push(' ');
        // Java: if (this.valueDefault != null)
        if let Some(ref vd) = self.valueDefault
        {
            // Java: buffer.append(this.valueDefault);
            buffer.push_str(vd);
            // Java: if (this.valueDefault.equals("#FIXED"))
            if vd == "#FIXED"
            {
                // Java: buffer.append(" \"");
                buffer.push_str(" \"");
                // Java: buffer.append(this.value);
                buffer.push_str(&self.value);
                // Java: buffer.append("\"");
                buffer.push('"');
            }
        }
        // Java: else
        else
        {
            // Java: buffer.append("\"");
            buffer.push('"');
            // Java: buffer.append(this.value);
            buffer.push_str(&self.value);
            // Java: buffer.append("\"");
            buffer.push('"');
        }
        // Java: buffer.append(">");
        buffer.push('>');
        // Java: return buffer.toString();
        buffer
    }
}

impl Declaration for AttributeDecl {
}
