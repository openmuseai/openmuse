#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Iterator
// Java: import java.util.Set

use super::InkMLSerializer::InkMLWriter;

// Java: public class AnnotationXML implements AnnotationElement, Cloneable
pub struct AnnotationXML {
    // Java field: String type
    pub(crate) r#type: String,
    // Java field: String encoding
    pub(crate) encoding: String,
    // Java field: HashMap<String, String> otherAttributesMap
    pub(crate) otherAttributesMap: HashMap<String, String>,
    // Java field: HashMap<String, String> propertyElementsMap
    pub(crate) propertyElementsMap: HashMap<String, String>,
}

// Java: public class AnnotationXML implements AnnotationElement, Cloneable
impl AnnotationXML {
    // Java: AnnotationXML()
    pub fn AnnotationXML() -> Self {
        // Java: super();
        Self {
            r#type: String::new(),
            encoding: String::new(),
            otherAttributesMap: HashMap::new(),
            propertyElementsMap: HashMap::new(),
        }
    }

    // Java: private HashMap<String, String> getPropertyElementsMapFromHrefElement(String href)
    fn getPropertyElementsMapFromHrefElement(&self, href: &str) -> Result<HashMap<String, String>, String> {
        // Java: throw new InkMLException("Feature (AnnotationXML:getPropertyElementsMapFromHrefElement) is not implimented.");
        Err("Feature (AnnotationXML:getPropertyElementsMapFromHrefElement) is not implimented.".to_string())
    }

    // Java: String getValue(String property, String attribute)
    pub fn getValue(&self, property: &str, attribute: &str) -> Result<String, String> {
        // Java: throw new InkMLException("Feature not yet implemented.");
        Err("Feature not yet implemented.".to_string())
    }

    // Java: String getProperty(String property)
    pub fn getProperty(&self, property: &str) -> String {
        // Java: return (String)this.propertyElementsMap.get(property);
        self.propertyElementsMap.get(property).cloned().unwrap_or_default()
    }

    // Java: void setProperty(String property, String value)
    pub fn setProperty(&mut self, property: &str, value: &str) {
        // Java: this.propertyElementsMap.put(property, value);
        self.propertyElementsMap.insert(property.to_string(), value.to_string());
    }

    // Java: String getType()
    pub fn getType(&self) -> &str {
        // Java: return this.type;
        &self.r#type
    }

    // Java: String getEncoding()
    pub fn getEncoding(&self) -> &str {
        // Java: return this.encoding;
        &self.encoding
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "AnnotationXML";
        "AnnotationXML".to_string()
    }

    // Java: boolean equals2(AnnotationXML annotationXML)
    pub fn equals2(&self, annotationXML: &AnnotationXML) -> bool {
        // Java: boolean isEqual = true;
        let is_equal = true;
        // Java: if (this.otherAttributesMap.size() != annotationXML.otherAttributesMap.size())
        if self.otherAttributesMap.len() != annotationXML.otherAttributesMap.len() {
            // Java: return false;
            return false;
        // Java: else if (!this.otherAttributesMap.keySet().containsAll(annotationXML.otherAttributesMap.keySet()))
        } else if !annotationXML.otherAttributesMap.keys().all(|k| self.otherAttributesMap.contains_key(k)) {
            // Java: return false;
            return false;
        // Java: else if (this.propertyElementsMap.size() != annotationXML.propertyElementsMap.size())
        } else if self.propertyElementsMap.len() != annotationXML.propertyElementsMap.len() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: Set<String> propertyKeys = this.propertyElementsMap.keySet();
            // Java: if (!propertyKeys.containsAll(annotationXML.propertyElementsMap.keySet()))
            if !annotationXML.propertyElementsMap.keys().all(|k| self.propertyElementsMap.contains_key(k)) {
                // Java: return false;
                return false;
            // Java: else
            } else {
                // Java: for(String key : propertyKeys)
                for key in self.propertyElementsMap.keys() {
                    // Java: String propertyValue1 = null;
                    // Java: String propertyValue2 = null;
                    // Java: propertyValue1 = this.getProperty(key);
                    let pv1 = self.getProperty(key);
                    // Java: propertyValue2 = annotationXML.getProperty(key);
                    let pv2 = annotationXML.getProperty(key);
                    // Java: if (!propertyValue1.equalsIgnoreCase(propertyValue2))
                    if !pv1.eq_ignore_ascii_case(&pv2) {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: return isEqual;
                is_equal
            }
        }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: StringBuffer annotXMLData = new StringBuffer("<annotationXML>");
        let mut annotXMLData = String::from("<annotationXML>");
        // Java: if (this.propertyElementsMap.size() != 0)
        if !self.propertyElementsMap.is_empty() {
            // Java: for(String key : this.propertyElementsMap.keySet())
            for (key, value) in &self.propertyElementsMap {
                // Java: String value = (String)this.propertyElementsMap.get(key);
                // Java: annotXMLData.append("<" + key + ">" + value + "</" + key + ">");
                annotXMLData += &format!("<{}>{}</{}>", key, value, key);
            }
        }
        // Java: annotXMLData.append("</annotationXML>");
        annotXMLData += "</annotationXML>";
        // Java: return annotXMLData.toString();
        annotXMLData
    }

    // Java: String[] getPropertyNames()
    pub fn getPropertyNames(&self) -> Vec<String> {
        // Java: int nProperties = this.propertyElementsMap.size();
        // Java: String[] propertyNames = new String[nProperties];
        // Java: Set<String> keys = this.propertyElementsMap.keySet();
        // Java: Iterator<String> itr = keys.iterator();
        // Java: for(int index = 0; itr.hasNext(); propertyNames[index++] = (String)itr.next())
        // Java: return propertyNames;
        self.propertyElementsMap.keys().cloned().collect()
    }

    // Java: boolean isPropertyExist(String propertyName)
    pub fn isPropertyExist(&self, propertyName: &str) -> bool {
        // Java: return this.propertyElementsMap.containsKey(propertyName);
        self.propertyElementsMap.contains_key(propertyName)
    }

    // Java: void override(AnnotationXML referredAxmlObject)
    pub fn override_axml(&mut self, referredAxmlObject: Option<&AnnotationXML>) {
        // Java: if (null != referredAxmlObject)
        if let Some(referred) = referredAxmlObject {
            // Java: String[] propertyNames = referredAxmlObject.getPropertyNames();
            let propertyNames = referred.getPropertyNames();
            // Java: for(int i = 0; i < propertyNames.length; ++i)
            for name in &propertyNames {
                // Java: this.propertyElementsMap.put(propertyNames[i], referredAxmlObject.getProperty(propertyNames[i]));
                let val = referred.getProperty(name);
                self.propertyElementsMap.insert(name.clone(), val);
            }
        }
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

    // Java: void setHref(URI value)
    pub fn setHref(&mut self, value: &str) -> Result<(), String> {
        // Java: throw new InkMLException("Feature (AnnotationXML:setHref) is not implimented.");
        Err("Feature (AnnotationXML:setHref) is not implimented.".to_string())
    }

    // Java: URI getHref()
    pub fn getHref(&self) -> Result<String, String> {
        // Java: throw new InkMLException("Feature (AnnotationXML:getHref) is not implimented.");
        Err("Feature (AnnotationXML:getHref) is not implimented.".to_string())
    }

    // Java: void addAllToPropertyMap(String ref)
    pub fn addAllToPropertyMap(&mut self, r#ref: &str) {
        // Java: try
        {
            // Java: this.propertyElementsMap.putAll(this.getPropertyElementsMapFromHrefElement(ref));
            if let Ok(map) = self.getPropertyElementsMapFromHrefElement(r#ref) {
                self.propertyElementsMap.extend(map);
            }
        }
        // Java: catch (InkMLException e)
        {
            // Java: KSLog.e("Problem in binding 'href' attribute of AnnotationXML data.\nReason: ", e.getMessage());
        }
    }

    // Java: void addToOtherAttributesMap(String attributeName, String value)
    pub fn addToOtherAttributesMap(&mut self, attributeName: &str, value: &str) {
        // Java: this.otherAttributesMap.put(attributeName, value);
        self.otherAttributesMap.insert(attributeName.to_string(), value.to_string());
    }

    // Java: void addToPropertyElementsMap(String name, String value)
    pub fn addToPropertyElementsMap(&mut self, name: &str, value: &str) {
        // Java: this.propertyElementsMap.put(name, value);
        self.propertyElementsMap.insert(name.to_string(), value.to_string());
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: if (null != writer)
        {
            // Java: writer.writeXML(this.toInkML());
        }
    }

    // Java: AnnotationXML clone()
    pub fn clone_axml(&self) -> AnnotationXML {
        // Java: AnnotationXML ret = new AnnotationXML();
        let mut ret = AnnotationXML::AnnotationXML();
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
        // Java: ret.otherAttributesMap = this.cloneOtherAttributesMap();
        ret.otherAttributesMap = self.cloneOtherAttributesMap();
        // Java: ret.propertyElementsMap = this.clonePropertyElementsMap();
        ret.propertyElementsMap = self.clonePropertyElementsMap();
        // Java: return ret;
        ret
    }

    // Java: private HashMap<String, String> clonePropertyElementsMap()
    fn clonePropertyElementsMap(&self) -> HashMap<String, String> {
        // Java: if (null == this.propertyElementsMap)
        if self.propertyElementsMap.is_empty() {
            // Java: return null;
            return HashMap::new();
        // Java: else
        } else {
            // Java: HashMap<String, String> ret = new HashMap();
            let mut ret = HashMap::new();
            // Java: for(String k : this.propertyElementsMap.keySet())
            for (k, v) in &self.propertyElementsMap {
                // Java: String e = (String)this.propertyElementsMap.get(k);
                // Java: ret.put(new String(k), new String(e));
                ret.insert(k.clone(), v.clone());
            }
            // Java: return ret;
            ret
        }
    }

    // Java: private HashMap<String, String> cloneOtherAttributesMap()
    fn cloneOtherAttributesMap(&self) -> HashMap<String, String> {
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
