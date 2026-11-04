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
    pub(crate) valueDefault: String,
}

// Java: public class AttributeDecl implements Declaration
impl AttributeDecl {
    // Java: AttributeDecl()
    pub fn AttributeDecl(&self) {
      // Java: super();
      todo!();
    }

    // Java: AttributeDecl(String elementName, String attributeName, String type, String valueDefault, String value)
    pub fn AttributeDecl_2(&self, elementName: &str, attributeName: &str, r#type: &str, valueDefault: &str, value: &str) {
      // Java: super();
      todo!();
      // Java: this.elementName = elementName;
      todo!();
      // Java: this.attributeName = attributeName;
      todo!();
      // Java: this.type = type;
      todo!();
      // Java: this.value = value;
      todo!();
      // Java: this.valueDefault = valueDefault;
      todo!();
    }

    // Java: String getElementName()
    pub fn getElementName(&self) -> String {
      // Java: return this.elementName;
      todo!();
    }

    // Java: void setElementName(String elementName)
    pub fn setElementName(&self, elementName: &str) {
      // Java: this.elementName = elementName;
      todo!();
    }

    // Java: String getAttributeName()
    pub fn getAttributeName(&self) -> String {
      // Java: return this.attributeName;
      todo!();
    }

    // Java: void setAttributeName(String attributeName)
    pub fn setAttributeName(&self, attributeName: &str) {
      // Java: this.attributeName = attributeName;
      todo!();
    }

    // Java: String getType()
    pub fn getType(&self) -> String {
      // Java: return this.type;
      todo!();
    }

    // Java: void setType(String type)
    pub fn setType(&self, r#type: &str) {
      // Java: this.type = type;
      todo!();
    }

    // Java: String getValue()
    pub fn getValue(&self) -> String {
      // Java: return this.value;
      todo!();
    }

    // Java: void setValue(String value)
    pub fn setValue(&self, value: &str) {
      // Java: this.value = value;
      todo!();
    }

    // Java: String getValueDefault()
    pub fn getValueDefault(&self) -> String {
      // Java: return this.valueDefault;
      todo!();
    }

    // Java: void setValueDefault(String valueDefault)
    pub fn setValueDefault(&self, valueDefault: &str) {
      // Java: this.valueDefault = valueDefault;
      todo!();
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuffer buffer = new StringBuffer("<!ATTLIST ");
      todo!();
      // Java: buffer.append(this.elementName);
      todo!();
      // Java: buffer.append(" ");
      todo!();
      // Java: buffer.append(this.attributeName);
      todo!();
      // Java: buffer.append(" ");
      todo!();
      // Java: buffer.append(this.type);
      todo!();
      // Java: buffer.append(" ");
      todo!();
      // Java: if (this.valueDefault != null)
        todo!();
      {
         // Java: buffer.append(this.valueDefault);
         todo!();
         // Java: if (this.valueDefault.equals("#FIXED"))
            todo!();
         {
            // Java: buffer.append(" \"");
            todo!();
            // Java: buffer.append(this.value);
            todo!();
            // Java: buffer.append("\"");
            todo!();
         }
      }
      // Java: else
        todo!();
      {
         // Java: buffer.append("\"");
         todo!();
         // Java: buffer.append(this.value);
         todo!();
         // Java: buffer.append("\"");
         todo!();
      }
      // Java: buffer.append(">");
      todo!();
      // Java: return buffer.toString();
      todo!();
    }

}

// impl Declaration for AttributeDecl {
// }
