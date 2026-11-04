#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import java.util.HashMap
use std::collections::HashMap;

use super::InkMLSerializer::InkMLWriter;

// Java: public class Annotation implements AnnotationElement, Cloneable
pub struct Annotation {
    // Java field: String type
    pub(crate) r#type: String,
    // Java field: String encoding
    pub(crate) encoding: String,
    // Java field: HashMap<String, String> otherAttributesMap
    pub(crate) otherAttributesMap: HashMap<String, String>,
    // Java field: String annotationTextValue
    pub(crate) annotationTextValue: String,
}

// Java: public class Annotation implements AnnotationElement, Cloneable
impl Annotation {
    // Java: Annotation()
    pub fn Annotation() -> Self {
        // Java: super();
        Self {
            r#type: String::new(),
            encoding: String::new(),
            otherAttributesMap: HashMap::new(),
            annotationTextValue: String::new(),
        }
    }

    // Java: String getAttributeValue(String attributeName)
    pub fn getAttributeValue(&self, attributeName: Option<&str>) -> Option<String> {
        // Java: if (attributeName == null)
        let attr_name = match attributeName {
            None => {
                // Java: return null;
                return None;
            }
            Some(n) => n,
        };
        // Java: else
        {
            // Java: String value = null;
            let value;
            // Java: if ("type".equals(attributeName))
            if "type" == attr_name {
                // Java: value = this.type;
                value = Some(self.r#type.clone());
            // Java: else if ("encoding".equals(attributeName))
            } else if "encoding" == attr_name {
                // Java: value = this.encoding;
                value = Some(self.encoding.clone());
            // Java: else
            } else {
                // Java: value = (String)this.otherAttributesMap.get(attributeName);
                value = self.otherAttributesMap.get(attr_name).cloned();
            }
            // Java: return value;
            value
        }
    }

    // Java: String getEncoding()
    pub fn getEncoding(&self) -> &str {
        // Java: return this.encoding;
        &self.encoding
    }

    // Java: String getType()
    pub fn getType(&self) -> &str {
        // Java: return this.type;
        &self.r#type
    }

    // Java: String getAnnotationTextValue()
    pub fn getAnnotationTextValue(&self) -> &str {
        // Java: return this.annotationTextValue;
        &self.annotationTextValue
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "Annotation";
        "Annotation".to_string()
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: String xml = "<annotation";
        let mut xml = String::from("<annotation");
        // Java: if (!"".equals(this.type))
        if !self.r#type.is_empty() {
            // Java: xml = xml + " type='" + this.type + "'";
            xml += &format!(" type='{}'", self.r#type);
        }
        // Java: if (!"".equals(this.encoding))
        if !self.encoding.is_empty() {
            // Java: xml = xml + " encoding='" + this.encoding + "'";
            xml += &format!(" encoding='{}'", self.encoding);
        }
        // Java: if (0 != this.otherAttributesMap.size())
        if !self.otherAttributesMap.is_empty() {
            // Java: for(Object attr : this.otherAttributesMap.keySet())
            for (attr, value) in &self.otherAttributesMap {
                // Java: Object value = this.otherAttributesMap.get(attr);
                // Java: xml = xml + " " + attr + "='" + value + "'";
                xml += &format!(" {}='{}'", attr, value);
            }
        }
        // Java: if ("".equals(this.annotationTextValue))
        if self.annotationTextValue.is_empty() {
            // Java: xml = xml + "/>";
            xml += "/>";
        // Java: else
        } else {
            // Java: xml = xml + "> " + this.annotationTextValue + " </annotation>";
            xml += &format!("> {} </annotation>", self.annotationTextValue);
        }
        // Java: return xml;
        xml
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: writer.writeXML(this.toInkML());
    }

    // Java: void setType(String value)
    pub fn setType(&mut self, value: &str) {
        // Java: this.type = value;
        self.r#type = value.to_string();
    }

    // Java: void setEncoding(String value)
    pub fn setEncoding(&mut self, value: &str) {
        // Java: this.encoding = value;
        self.encoding = value.to_string();
    }

    // Java: void addToOtherAttributesMap(String attributeName, String value)
    pub fn addToOtherAttributesMap(&mut self, attributeName: &str, value: &str) {
        // Java: this.otherAttributesMap.put(attributeName, value);
        self.otherAttributesMap.insert(attributeName.to_string(), value.to_string());
    }

    // Java: void setAnnotationTextValue(String annotationText)
    pub fn setAnnotationTextValue(&mut self, annotationText: &str) {
        // Java: this.annotationTextValue = annotationText;
        self.annotationTextValue = annotationText.to_string();
    }

    // Java: Annotation clone()
    pub fn clone_annotation(&self) -> Annotation {
        // Java: Annotation ret = new Annotation();
        let mut ret = Annotation::Annotation();
        // Java: if (null != this.annotationTextValue)
        {
            // Java: ret.annotationTextValue = new String(this.annotationTextValue);
            ret.annotationTextValue = self.annotationTextValue.clone();
        }
        // Java: if (null != this.encoding)
        {
            // Java: ret.encoding = new String(this.encoding);
            ret.encoding = self.encoding.clone();
        }
        // Java: if (null != this.type)
        {
            // Java: ret.type = new String(this.type);
            ret.r#type = self.r#type.clone();
        }
        // Java: ret.otherAttributesMap = this.cloneOhterAttributesMap();
        ret.otherAttributesMap = self.cloneOhterAttributesMap();
        // Java: return ret;
        ret
    }

    // Java: private HashMap<String, String> cloneOhterAttributesMap()
    fn cloneOhterAttributesMap(&self) -> HashMap<String, String> {
        // Java: if (null == this.otherAttributesMap)
        if self.otherAttributesMap.is_empty() {
            // Java: return null;
            return HashMap::new();
        // Java: else
        } else {
            // Java: HashMap<String, String> ret = new HashMap();
            let mut ret = HashMap::new();
            // Java: for(String k : this.otherAttributesMap.keySet())
            for (k, v) in &self.otherAttributesMap {
                // Java: String e = (String)this.otherAttributesMap.get(k);
                // Java: ret.put(new String(k), new String(e));
                ret.insert(k.clone(), v.clone());
            }
            // Java: return ret;
            ret
        }
    }
}
