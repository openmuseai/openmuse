// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import org.xml.sax.Attributes

use std::collections::HashMap;
use super::Attributes1::Attributes1;
use super::XmlStringBuilder::XmlStringBuilder;

// Java field: int MAX_NUM
const MAX_NUM: usize = 100;

// Java: public class AttributesImpl implements Attributes1
pub struct AttributesImpl {
    // Java field: int length
    pub(crate) length: usize,
    // Java field: Attribute[] data
    data: Vec<Option<Attribute>>,
    // Java field: HashMap<Integer, String> nameSpacingMap
    pub(crate) nameSpacingMap: HashMap<i32, String>,
    // Java field: HashMap<Integer, String> mHash2Prefix
    pub(crate) mHash2Prefix: Option<HashMap<i32, String>>,
}

// Java: public class Attribute
pub struct Attribute {
    // Java field: XmlStringBuilder mNameBuilder
    pub(crate) mNameBuilder: XmlStringBuilder,
    // Java field: XmlStringBuilder mValueBuilder
    pub(crate) mValueBuilder: XmlStringBuilder,
    // Java field: int mPrefixHashCode
    mPrefixHashCode: i32,
    // Java field: String mName
    pub(crate) mName: Option<String>,
    // Java field: String mQName
    mQName: Option<String>,
    // Java field: String mValue
    pub(crate) mValue: Option<String>,
}

// Java: public class AttributesImpl implements Attributes1
impl AttributesImpl {
    // Java: AttributesImpl(Attributes attrs)
    pub fn new_from_attrs(attrs: &AttributesImpl) -> Self {
        // Java: super();
        // Java: if (attrs instanceof AttributesImpl)
        {
            // Java: this.cloneNameSpacingMap((AttributesImpl)attrs);
            let (ns_map, h2p) = Self::clone_ns_map(attrs);
            // Java: this.cloneData((AttributesImpl)attrs);
            let data = Self::clone_data(attrs);
            // Java: this.length = ((AttributesImpl)attrs).length;
            Self {
                length: attrs.length,
                data,
                nameSpacingMap: ns_map,
                mHash2Prefix: h2p,
            }
        }
        // Java: else
        // Java: IOAssert.fail("TODO");
    }

    // Java: void cloneData(AttributesImpl attrs)
    fn clone_data(attrs: &AttributesImpl) -> Vec<Option<Attribute>> {
        // Java: int size = attrs.length;
        let size = attrs.length;
        // Java: this.data = new Attribute[size];
        let mut data = Vec::with_capacity(size);
        // Java: for(int i = 0; i < size && null != attrs.data[i]; ++i)
        for i in 0..size {
            if let Some(ref attr) = attrs.data[i] {
                // Java: this.data[i] = new Attribute(attrs.data[i]);
                data.push(Some(Attribute::new_copy(attr)));
            } else {
                break;
            }
        }
        while data.len() < size {
            data.push(None);
        }
        data
    }

    // Java: void cloneNameSpacingMap(AttributesImpl attrs)
    fn clone_ns_map(attrs: &AttributesImpl) -> (HashMap<i32, String>, Option<HashMap<i32, String>>) {
        // Java: this.nameSpacingMap = new HashMap();
        let mut ns_map = HashMap::new();
        // Java: for(Integer k : attrs.nameSpacingMap.keySet())
        for (k, v) in &attrs.nameSpacingMap {
            // Java: this.nameSpacingMap.put(k, (String)attrs.nameSpacingMap.get(k));
            ns_map.insert(*k, v.clone());
        }
        // Java: if (null != attrs.mHash2Prefix)
        let h2p = if let Some(ref src) = attrs.mHash2Prefix {
            // Java: this.mHash2Prefix = new HashMap();
            let mut m = HashMap::new();
            // Java: for(Integer k : attrs.mHash2Prefix.keySet())
            for (k, v) in src {
                // Java: this.mHash2Prefix.put(k, (String)attrs.mHash2Prefix.get(k));
                m.insert(*k, v.clone());
            }
            Some(m)
        } else {
            None
        };
        (ns_map, h2p)
    }

    // Java: AttributesImpl(HashMap<Integer, String> nameSpacingMap, HashMap<Integer, String> hash2Prefix)
    pub fn new_with_maps(nameSpacingMap: HashMap<i32, String>, hash2Prefix: Option<HashMap<i32, String>>) -> Self {
        // Java: super();
        // Java: this.nameSpacingMap = nameSpacingMap;
        // Java: this.mHash2Prefix = hash2Prefix;
        Self {
            length: 0,
            data: Vec::new(),
            nameSpacingMap,
            mHash2Prefix: hash2Prefix,
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.length = 0;
        self.length = 0;
    }

    // Java: void addAttrite(int nameSpacing, String qualifiedName, String name, XmlStringBuilder value)
    pub fn addAttrite(&mut self, _nameSpacing: i32, _qualifiedName: &str, _name: &str, value: &XmlStringBuilder) {
        // Java: this.ensureCapacity(this.length + 1);
        self.ensureCapacity(self.length + 1);
        // Java: Attribute attribute = this.data[this.length];
        // Java: IOAssert.assertNotNull(attribute);
        if let Some(ref mut attribute) = self.data[self.length] {
            // Java: attribute.mValueBuilder.restore(value);
            attribute.mValueBuilder.restore(value);
        }
        // Java: ++this.length;
        self.length += 1;
    }

    // Java: void addAttriteName(XmlStringBuilder xmlStringBuilder)
    pub fn addAttriteName(&mut self, xmlStringBuilder: &XmlStringBuilder) {
        // Java: this.ensureCapacity(this.length + 1);
        self.ensureCapacity(self.length + 1);
        // Java: Attribute attribute = this.data[this.length];
        // Java: if (null == attribute)
        if self.data[self.length].is_none() {
            // Java: attribute = new Attribute();
            // Java: this.data[this.length] = attribute;
            self.data[self.length] = Some(Attribute::new());
        }
        // Java: attribute.mNameBuilder.restore(xmlStringBuilder);
        if let Some(ref mut attribute) = self.data[self.length] {
            attribute.mNameBuilder.restore(xmlStringBuilder);
        }
    }

    // Java: void addAttriteValue(XmlStringBuilder value)
    pub fn addAttriteValue(&mut self, value: &XmlStringBuilder) {
        // Java: this.ensureCapacity(this.length + 1);
        self.ensureCapacity(self.length + 1);
        // Java: Attribute attribute = this.data[this.length];
        // Java: IOAssert.assertNotNull(attribute);
        if let Some(ref mut attribute) = self.data[self.length] {
            // Java: attribute.mValueBuilder.restore(value);
            attribute.mValueBuilder.restore(value);
        }
        // Java: ++this.length;
        self.length += 1;
    }

    // Java: void ensureCapacity(int n)
    fn ensureCapacity(&mut self, n: usize) {
        // Java: if (n > 0)
        if n > 0 {
            // Java: int max;
            let max;
            // Java: if (this.data != null && this.data.length != 0)
            if !self.data.is_empty() {
                // Java: if (this.data.length > n)
                if self.data.len() > n {
                    // Java: return;
                    return;
                }
                // Java: max = this.data.length * 2;
                max = self.data.len() * 2;
            }
            // Java: else
            else {
                // Java: max = 100;
                max = MAX_NUM;
            }
            // Java: Attribute[] newData = new Attribute[max];
            // Java: if (this.data != null)
            // Java: System.arraycopy(this.data, 0, newData, 0, this.data.length);
            self.data.resize_with(max, || None);
            // Java: this.data = newData;
        }
    }

    // Java: String getByName(String name)
    fn getByName(&self, name: &str) -> Option<String> {
        // Java: for(int i = 0; i < this.length; ++i)
        for i in 0..self.length {
            // Java: Attribute attribute = this.data[i];
            if let Some(ref attribute) = self.data[i] {
                // Java: if (attribute.mNameBuilder.isNameEqual(name))
                if attribute.mNameBuilder.isNameEqual(name) {
                    // Java: return attribute.mValueBuilder.toString();
                    return Some(attribute.mValueBuilder.to_string());
                }
            }
        }
        // Java: return null;
        None
    }

    // Java: Attribute getArributeByName(String name)
    fn getArributeByName_single(&self, name: &str) -> Option<&Attribute> {
        // Java: for(int i = 0; i < this.length; ++i)
        for i in 0..self.length {
            // Java: Attribute attribute = this.data[i];
            if let Some(ref attribute) = self.data[i] {
                // Java: if (attribute.mNameBuilder.isNameEqual(name))
                if attribute.mNameBuilder.isNameEqual(name) {
                    // Java: return attribute;
                    return Some(attribute);
                }
            }
        }
        // Java: return null;
        None
    }

    // Java: Attribute getArributeByName(String uri, String name)
    fn getArributeByName(&self, uri: &str, name: &str) -> Option<&Attribute> {
        // Java: for(int i = 0; i < this.length; ++i)
        for i in 0..self.length {
            // Java: Attribute attribute = this.data[i];
            if let Some(ref attribute) = self.data[i] {
                // Java: String nameSpacing = (String)this.nameSpacingMap.get(attribute.mNameBuilder.getPrefixHashCode());
                let prefix_hash = attribute.mNameBuilder.getPrefixHashCode();
                if let Some(name_spacing) = self.nameSpacingMap.get(&prefix_hash) {
                    // Java: if (null != nameSpacing && attribute.mNameBuilder.isNameEqual(name) && nameSpacing.equals(uri))
                    if attribute.mNameBuilder.isNameEqual(name) && name_spacing == uri {
                        // Java: return attribute;
                        return Some(attribute);
                    }
                }
            }
        }
        // Java: return null;
        None
    }

    // Java: Attribute getArribute(int index)
    fn getArribute(&self, index: usize) -> Option<&Attribute> {
        // Java: return index < this.length && index >= 0 ? this.data[index] : null;
        if index < self.length { self.data[index].as_ref() } else { None }
    }

    // Java: int getLength()
    pub fn getLength(&self) -> usize {
        // Java: return this.length;
        self.length
    }

    // Java: String getURI(int index)
    pub fn getURI(&self, index: usize) -> Option<String> {
        // Java: Attribute attribute = this.getArribute(index);
        let attribute = self.getArribute(index);
        // Java: return null == attribute ? null : (String)this.nameSpacingMap.get(attribute.mNameBuilder.getPrefixHashCode());
        attribute.and_then(|a| self.nameSpacingMap.get(&a.mNameBuilder.getPrefixHashCode()).cloned())
    }

    // Java: String getLocalName(int index)
    pub fn getLocalName(&self, index: usize) -> Option<String> {
        // Java: Attribute attribute = this.getArribute(index);
        let attribute = self.getArribute(index);
        // Java: return null == attribute ? null : attribute.mNameBuilder.getName();
        attribute.map(|a| a.mNameBuilder.getName())
    }

    // Java: String getQName(int index)
    pub fn getQName(&self, index: usize) -> Option<String> {
        // Java: Attribute attribute = this.getArribute(index);
        let attribute = self.getArribute(index);
        // Java: return null == attribute ? null : attribute.mNameBuilder.toString();
        attribute.map(|a| a.mNameBuilder.to_string())
    }

    // Java: String getType(int index)
    pub fn getType_by_index(&self, _index: usize) -> Option<String> {
        // Java: return null;
        None
    }

    // Java: String getValue(int index)
    pub fn getValue_by_index(&self, index: usize) -> Option<String> {
        // Java: Attribute attribute = this.getArribute(index);
        let attribute = self.getArribute(index);
        // Java: return null == attribute ? null : attribute.mValueBuilder.toString();
        attribute.map(|a| a.mValueBuilder.to_string())
    }

    // Java: int getIndex(String uri, String localName)
    pub fn getIndex_by_uri(&self, _uri: &str, _localName: &str) -> i32 {
        // Java: return 0;
        0
    }

    // Java: int getIndex(String qName)
    pub fn getIndex_by_qname(&self, _qName: &str) -> i32 {
        // Java: return 0;
        0
    }

    // Java: String getType(String uri, String localName)
    pub fn getType_by_uri(&self, _uri: &str, _localName: &str) -> Option<String> {
        // Java: return null;
        None
    }

    // Java: String getType(String qName)
    pub fn getType_by_qname(&self, _qName: &str) -> Option<String> {
        // Java: return null;
        None
    }

    // Java: String getValue(String uri, String localName)
    pub fn getValue_by_uri(&self, _uri: &str, localName: &str) -> Option<String> {
        // Java: return this.getByName(localName);
        self.getByName(localName)
    }

    // Java: String getValue(String qName)
    pub fn getValue_by_qname(&self, qName: &str) -> Option<String> {
        // Java: return this.getByName(qName);
        self.getByName(qName)
    }

    // Java: String toString()
    pub fn to_string_value(&self) -> String {
        // Java: StringBuilder stringBuilder = new StringBuilder();
        let mut s = String::new();
        // Java: for(int i = 0; i < this.length; ++i)
        for i in 0..self.length {
            // Java: Attribute attribute = this.data[i];
            if let Some(ref attribute) = self.data[i] {
                // Java: stringBuilder.append(attribute.toString());
                s.push_str(&attribute.to_string_value());
                // Java: stringBuilder.append(", ");
                s.push_str(", ");
            }
        }
        // Java: return stringBuilder.toString();
        s
    }

    // Java: boolean isPreserveSpace()
    pub fn isPreserveSpace(&self) -> bool {
        // Java: for(int i = 0; i < this.length; ++i)
        for i in 0..self.length {
            // Java: Attribute attribute = this.data[i];
            if let Some(ref attribute) = self.data[i] {
                // Java: if (attribute.mNameBuilder.isPrefixEqual("xml") && attribute.mNameBuilder.isNameEqual("space"))
                if attribute.mNameBuilder.isPrefixEqual("xml") && attribute.mNameBuilder.isNameEqual("space") {
                    // Java: return attribute.mValueBuilder.getHashCode() == "preserve".hashCode();
                    return attribute.mValueBuilder.getHashCode() == XmlStringBuilder::hashcode_char(&"preserve".chars().collect::<Vec<_>>(), 0, 8);
                }
            }
        }
        // Java: return false;
        false
    }

    // Java: ArrayList<String> getNamespaces()
    pub fn getNamespaces(&self) -> Option<Vec<String>> {
        // Java: if (null != this.mHash2Prefix && null != this.nameSpacingMap)
        if let Some(ref h2p) = self.mHash2Prefix {
            // Java: ArrayList<String> ret = new ArrayList();
            let mut ret = Vec::new();
            // Java: for(Integer k : this.nameSpacingMap.keySet())
            for (k, val) in &self.nameSpacingMap {
                // Java: String val = (String)this.nameSpacingMap.get(k);
                // Java: if (null != val)
                {
                    // Java: String prefix = (String)this.mHash2Prefix.get(k);
                    if let Some(prefix) = h2p.get(k) {
                        // Java: if (null != prefix)
                        {
                            // Java: ret.add(prefix);
                            ret.push(prefix.clone());
                            // Java: ret.add(val);
                            ret.push(val.clone());
                        }
                    }
                }
            }
            // Java: return ret;
            Some(ret)
        }
        // Java: else
        else {
            // Java: return null;
            None
        }
    }

    // Java: Integer getIntValue(String uri, String localName)
    pub fn getIntValue(&self, uri: &str, localName: &str) -> Option<i32> {
        // Java: Attribute attribute = this.getArributeByName(uri, localName);
        let attribute = self.getArributeByName(uri, localName);
        // Java: return null == attribute ? null : attribute.mValueBuilder.toInteger();
        attribute.and_then(|a| a.mValueBuilder.toInteger())
    }

    // Java: boolean getBooleanValue(String uri, String localName, boolean defaultValue)
    pub fn getBooleanValue(&self, uri: &str, localName: &str, defaultValue: bool) -> bool {
        // Java: Attribute attribute = this.getArributeByName(uri, localName);
        let attribute = self.getArributeByName(uri, localName);
        // Java: return null == attribute ? defaultValue : attribute.mValueBuilder.toBoolean();
        attribute.map_or(defaultValue, |a| a.mValueBuilder.toBoolean())
    }

    // Java: int getStringHashCode(String uri, String localName)
    pub fn getStringHashCode(&self, uri: &str, localName: &str) -> i32 {
        // Java: Attribute attribute = this.getArributeByName(uri, localName);
        let attribute = self.getArributeByName(uri, localName);
        // Java: return null == attribute ? -1 : attribute.mValueBuilder.getHashCode();
        attribute.map_or(-1, |a| a.mValueBuilder.getHashCode())
    }

}

// Java: public class Attribute
impl Attribute {
    // Java: Attribute()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mNameBuilder = new XmlStringBuilder();
        // Java: this.mValueBuilder = new XmlStringBuilder();
        Self {
            mNameBuilder: XmlStringBuilder::new(),
            mValueBuilder: XmlStringBuilder::new(),
            mPrefixHashCode: 0,
            mName: None,
            mQName: None,
            mValue: None,
        }
    }

    // Java: Attribute(Attribute attribute)
    pub fn new_copy(attribute: &Attribute) -> Self {
        // Java: super();
        // Java: this.mNameBuilder = new XmlStringBuilder(attribute.mNameBuilder);
        // Java: this.mValueBuilder = new XmlStringBuilder(attribute.mValueBuilder);
        Self {
            mNameBuilder: XmlStringBuilder::new_from(&attribute.mNameBuilder),
            mValueBuilder: XmlStringBuilder::new_from(&attribute.mValueBuilder),
            mPrefixHashCode: attribute.mPrefixHashCode,
            mName: attribute.mName.clone(),
            mQName: attribute.mQName.clone(),
            mValue: attribute.mValue.clone(),
        }
    }

    // Java: String toString()
    pub fn to_string_value(&self) -> String {
        // Java: StringBuilder stringBuilder = new StringBuilder();
        let mut s = String::new();
        // Java: stringBuilder.append(this.mNameBuilder.toString());
        s.push_str(&self.mNameBuilder.to_string());
        // Java: stringBuilder.append(':');
        s.push(':');
        // Java: stringBuilder.append('"');
        s.push('"');
        // Java: stringBuilder.append(this.mValueBuilder);
        s.push_str(&self.mValueBuilder.to_string());
        // Java: stringBuilder.append('"');
        s.push('"');
        // Java: return stringBuilder.toString();
        s
    }

}

impl Attributes1 for AttributesImpl {
    fn addAttrite(&mut self, nameSpacing: i32, qualifiedName: &str, name: &str, _var4: Box<dyn std::any::Any + Send + Sync>) {
        // stub: forward to struct method is not directly compatible
    }

    fn addAttriteName(&mut self, _var1: Box<dyn std::any::Any + Send + Sync>) {
    }

    fn addAttriteValue(&mut self, _var1: Box<dyn std::any::Any + Send + Sync>) {
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn getIntValue(&self, var1: &str, var2: &str) -> Option<i32> {
        self.getIntValue(var1, var2)
    }

    fn getStringHashCode(&self, var1: &str, var2: &str) -> i32 {
        self.getStringHashCode(var1, var2)
    }

    fn getBooleanValue(&self, var1: &str, var2: &str, var3: bool) -> bool {
        self.getBooleanValue(var1, var2, var3)
    }

    fn isPreserveSpace(&self) -> bool {
        self.isPreserveSpace()
    }

    fn getNamespaces(&self) -> Vec<String> {
        self.getNamespaces().unwrap_or_default()
    }
}
