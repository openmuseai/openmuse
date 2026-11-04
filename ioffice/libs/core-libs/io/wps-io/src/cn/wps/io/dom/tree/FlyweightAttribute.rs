// Java: import cn.wps.io.dom.Namespace

// Java: public class FlyweightAttribute extends AbstractAttribute
pub struct FlyweightAttribute {
    // Java field: String mNamespaceURI
    pub(crate) mNamespaceURI: Option<String>,
    // Java field: String mPrefix
    pub(crate) mPrefix: Option<String>,
    // Java field: String mLocalName
    pub(crate) mLocalName: Option<String>,
    // Java field: String mValue
    pub(crate) mValue: Option<String>,
}

// Java: public class FlyweightAttribute extends AbstractAttribute
impl FlyweightAttribute {
    // Java: FlyweightAttribute(String namespaceURI, String prefix, String localName, String value)
    pub(crate) fn create(ns: &str, prefix: &str, local: &str, value: &str) -> Self {
        FlyweightAttribute {
            mNamespaceURI: Some(ns.to_string()),
            mPrefix: Some(prefix.to_string()),
            mLocalName: Some(local.to_string()),
            mValue: Some(value.to_string()),
        }
    }


    // Java: FlyweightAttribute()
    pub fn new() -> Self {
      // Java: super();
      todo!();
    }

    // Java: FlyweightAttribute(String namespaceURI, String prefix, String localName, String qualifiedName)
    pub fn new_1() -> Self {
      // Java: super();
      todo!();
      // Java: this.mNamespaceURI = namespaceURI;
      todo!();
      // Java: this.mPrefix = prefix;
      todo!();
      // Java: this.mLocalName = localName;
      todo!();
    }

    // Java: FlyweightAttribute(String namespaceURI, String prefix, String localName, String qualifiedName, String value)
    pub fn new_2() -> Self {
      // Java: super();
      todo!();
      // Java: this.mNamespaceURI = namespaceURI;
      todo!();
      // Java: this.mPrefix = prefix;
      todo!();
      // Java: this.mLocalName = localName;
      todo!();
      // Java: this.mValue = value;
      todo!();
    }

    // Java: FlyweightAttribute(String name, String value)
    pub fn new_3() -> Self {
      // Java: super();
      todo!();
      // Java: this.mLocalName = name;
      todo!();
      // Java: this.mValue = value;
      todo!();
    }

    // Java: FlyweightAttribute(String name, String value, Namespace namespace)
    pub fn new_4() -> Self {
      // Java: super();
      todo!();
      // Java: this.mLocalName = name;
      todo!();
      // Java: this.mNamespaceURI = namespace.getURI();
      todo!();
      // Java: this.mValue = value;
      todo!();
    }

    // Java: String getValue()
    pub fn getValue(&self) -> String {
      // Java: return this.mValue;
      self.mValue.clone().unwrap_or_default()
    }

    // Java: void setValue(String value)
    pub fn setValue(&mut self, value: &str) {
      // Java: this.mValue = value;
      self.mValue = Some(value.to_string());
    }

    // Java: String getNamespaceURI()
    pub fn getNamespaceURI(&self) -> String {
      // Java: return this.mNamespaceURI;
      self.mNamespaceURI.clone().unwrap_or_default()
    }

    // Java: void setNamespaceURI(String namespaceURI)
    pub fn setNamespaceURI(&mut self, namespaceURI: &str) {
      // Java: this.mNamespaceURI = namespaceURI;
      self.mNamespaceURI = Some(namespaceURI.to_string());
    }

    // Java: String getNamespacePrefix()
    pub fn getNamespacePrefix(&self) -> String {
      // Java: return this.mPrefix;
      self.mPrefix.clone().unwrap_or_default()
    }

    // Java: String getLocalName()
    pub fn getLocalName(&self) -> String {
      // Java: return this.mLocalName;
      self.mLocalName.clone().unwrap_or_default()
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
      // Java: return this.mLocalName;
      self.mLocalName.clone().unwrap_or_default()
    }

    // Java: void setName(String name)
    pub fn setName(&mut self) {
      // Java: this.mLocalName = name;
      self.mPrefix.clone().unwrap_or_default();
    }

    // Java: String getQualifiedName()
    pub fn getQualifiedName(&mut self) -> String {
      // Java: String qualifiedName = null;
      todo!();
      // Java: if (this.mPrefix != null && this.mPrefix.length() > 0)
      todo!();
      {
         // Java: qualifiedName = this.mPrefix + ":" + this.mLocalName;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: qualifiedName = this.mLocalName;
      todo!();
      }
      // Java: return qualifiedName;
      todo!();
    }

}
