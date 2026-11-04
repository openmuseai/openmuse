#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.api.iterator.ILongIterator
// Java: import cn.wps.kfc.api.map.ILongObjectMap
// Java: import cn.wps.kfc.impl.map.WLongObjectHashMap
// Java: import java.util.Date
// Java: import java.util.HashMap
// Java: import java.util.Iterator
// Java: import java.util.Map
// Java: import java.util.Set
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum PropertyValue {
    Str(String),
    Long(i64),
    Double(f64),
    Int(i32),
    Bool(bool),
    None,
}

#[derive(Clone, Debug)]
pub struct Property {
    pub(crate) id: i64,
    pub(crate) type_: i64,
    pub(crate) value: PropertyValue,
}

impl Property {
    pub fn new() -> Self {
        Self { id: 0, type_: 0, value: PropertyValue::None }
    }
    pub fn getID(&self) -> i64 { self.id }
    pub fn getType(&self) -> i64 { self.type_ }
    pub fn getValue(&self) -> &PropertyValue { &self.value }
}

#[derive(Clone, Debug)]
pub struct MutableProperty {
    pub(crate) prop: Property,
}

impl MutableProperty {
    pub fn new() -> Self { Self { prop: Property::new() } }
    pub fn setID(&mut self, id: i64) { self.prop.id = id; }
    pub fn setType(&mut self, t: i64) { self.prop.type_ = t; }
    pub fn setValue(&mut self, v: PropertyValue) { self.prop.value = v; }
    pub fn getID(&self) -> i64 { self.prop.id }
    pub fn getType(&self) -> i64 { self.prop.type_ }
    pub fn getValue(&self) -> &PropertyValue { &self.prop.value }
}

#[derive(Clone, Debug)]
pub struct CustomProperty {
    pub(crate) inner: MutableProperty,
    pub(crate) name: Option<String>,
    pub(crate) linkTarget: Option<String>,
}

impl CustomProperty {
    pub fn new_from_property(p: MutableProperty) -> Self {
        Self { inner: p, name: None, linkTarget: None }
    }
    pub fn new_from_property_with_name(p: MutableProperty, name: &str) -> Self {
        Self { inner: p, name: Some(name.to_string()), linkTarget: None }
    }
    pub fn getName(&self) -> Option<&str> { self.name.as_deref() }
    pub fn setName(&mut self, name: &str) { self.name = Some(name.to_string()); }
    pub fn getID(&self) -> i64 { self.inner.getID() }
    pub fn setID(&mut self, id: i64) { self.inner.setID(id); }
    pub fn getType(&self) -> i64 { self.inner.getType() }
    pub fn getValue(&self) -> &PropertyValue { self.inner.getValue() }
    pub fn hashCode(&self) -> i32 { self.getID() as i32 }
}

// Java: public class CustomProperties extends HashMap
pub struct CustomProperties {
    // Java field: ILongObjectMap<String> dictionaryIDToName
    dictionaryIDToName: HashMap<i64, String>,
    // Java field: Map<String, Long> dictionaryNameToID
    pub(crate) dictionaryNameToID: HashMap<String, i64>,
    // Java field: boolean isPure
    pub(crate) isPure: bool,
    entries: HashMap<i64, CustomProperty>,
}

// Java: public class CustomProperties extends HashMap
impl CustomProperties {
    // Java: CustomProperties()
    pub fn new() -> Self {
        // Java: super();
        Self {
            dictionaryIDToName: HashMap::new(),
            dictionaryNameToID: HashMap::new(),
            isPure: true,
            entries: HashMap::new(),
        }
    }

    // Java: CustomProperty put(String name, CustomProperty cp)
    pub fn put_named(&mut self, name: &str, cp: CustomProperty) -> Option<CustomProperty> {
        // Java: if (name == null)
        // (name is &str, always non-null in Rust)
        {
            // Java: this.isPure = false;
            // Java: return null;
        }
        // Java: else if (!name.equals(cp.getName()))
        if cp.getName() != Some(name)
        {
            // Java: throw new IllegalArgumentException("Parameter \"name\" (" + name + ") and custom property's name (" + cp.getName() + ") do not match.");
            panic!("Parameter \"name\" ({}) and custom property's name ({:?}) do not match.", name, cp.getName());
        }
        // Java: else
        {
            // Java: long idKey = cp.getID();
            let idKey = cp.getID();
            // Java: Long oldID = (Long)this.dictionaryNameToID.get(name);
            let oldID = self.dictionaryNameToID.get(name).copied();
            // Java: if (oldID != null)
            if let Some(old_id) = oldID
            {
                // Java: this.dictionaryIDToName.remove(oldID);
                self.dictionaryIDToName.remove(&old_id);
            }
            // Java: this.dictionaryNameToID.put(name, idKey);
            self.dictionaryNameToID.insert(name.to_string(), idKey);
            // Java: this.dictionaryIDToName.put(idKey, name);
            self.dictionaryIDToName.insert(idKey, name.to_string());
            // Java: CustomProperty oldCp = (CustomProperty)super.remove(oldID);
            let oldCp = oldID.and_then(|id| self.entries.remove(&id));
            // Java: super.put(idKey, cp);
            self.entries.insert(idKey, cp);
            // Java: return oldCp;
            oldCp
        }
    }

    // Java: Object put(CustomProperty customProperty)
    fn put_custom(&mut self, mut customProperty: CustomProperty) -> Option<CustomProperty> {
        // Java: String name = customProperty.getName();
        let name = customProperty.getName().unwrap_or("").to_string();
        // Java: Long oldId = (Long)this.dictionaryNameToID.get(name);
        let oldId = self.dictionaryNameToID.get(&name).copied();
        // Java: if (oldId != null)
        if let Some(old_id) = oldId
        {
            // Java: customProperty.setID(oldId);
            customProperty.setID(old_id);
        }
        // Java: else
        else
        {
            // Java: long max = 1L;
            let mut max: i64 = 1;
            // Java: ILongIterator i = this.dictionaryIDToName.keySet().iterator();
            // Java: while(i.hasNext())
            for &id in self.dictionaryIDToName.keys()
            {
                // Java: long id = i.next();
                // Java: if (id > max)
                if id > max
                {
                    // Java: max = id;
                    max = id;
                }
            }
            // Java: customProperty.setID(max + 1L);
            customProperty.setID(max + 1);
        }
        // Java: return this.put(name, customProperty);
        self.put_named(&name, customProperty)
    }

    // Java: Object remove(String name)
    pub fn remove_by_name(&mut self, name: &str) -> Option<CustomProperty> {
        // Java: Long id = (Long)this.dictionaryNameToID.get(name);
        let id = self.dictionaryNameToID.get(name).copied();
        // Java: if (id == null)
        if id.is_none()
        {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            let id = id.unwrap();
            // Java: this.dictionaryIDToName.remove(id);
            self.dictionaryIDToName.remove(&id);
            // Java: this.dictionaryNameToID.remove(name);
            self.dictionaryNameToID.remove(name);
            // Java: return super.remove(id);
            self.entries.remove(&id)
        }
    }

    // Java: Object put(String name, String value)
    pub fn put_string(&mut self, name: &str, value: &str) -> Option<CustomProperty> {
        // Java: MutableProperty p = new MutableProperty();
        let mut p = MutableProperty::new();
        // Java: p.setID(-1L);
        p.setID(-1);
        // Java: p.setType(31L);
        p.setType(31);
        // Java: p.setValue(value);
        p.setValue(PropertyValue::Str(value.to_string()));
        // Java: CustomProperty cp = new CustomProperty(p, name);
        let cp = CustomProperty::new_from_property_with_name(p, name);
        // Java: return this.put(cp);
        self.put_custom(cp)
    }

    // Java: Object put(String name, Long value)
    pub fn put_long(&mut self, name: &str, value: i64) -> Option<CustomProperty> {
        // Java: MutableProperty p = new MutableProperty();
        let mut p = MutableProperty::new();
        // Java: p.setID(-1L);
        p.setID(-1);
        // Java: p.setType(20L);
        p.setType(20);
        // Java: p.setValue(value);
        p.setValue(PropertyValue::Long(value));
        // Java: CustomProperty cp = new CustomProperty(p, name);
        let cp = CustomProperty::new_from_property_with_name(p, name);
        // Java: return this.put(cp);
        self.put_custom(cp)
    }

    // Java: Object put(String name, Double value)
    pub fn put_double(&mut self, name: &str, value: f64) -> Option<CustomProperty> {
        // Java: MutableProperty p = new MutableProperty();
        let mut p = MutableProperty::new();
        // Java: p.setID(-1L);
        p.setID(-1);
        // Java: p.setType(5L);
        p.setType(5);
        // Java: p.setValue(value);
        p.setValue(PropertyValue::Double(value));
        // Java: CustomProperty cp = new CustomProperty(p, name);
        let cp = CustomProperty::new_from_property_with_name(p, name);
        // Java: return this.put(cp);
        self.put_custom(cp)
    }

    // Java: Object put(String name, Integer value)
    pub fn put_integer(&mut self, name: &str, value: i32) -> Option<CustomProperty> {
        // Java: MutableProperty p = new MutableProperty();
        let mut p = MutableProperty::new();
        // Java: p.setID(-1L);
        p.setID(-1);
        // Java: p.setType(3L);
        p.setType(3);
        // Java: p.setValue(value);
        p.setValue(PropertyValue::Int(value));
        // Java: CustomProperty cp = new CustomProperty(p, name);
        let cp = CustomProperty::new_from_property_with_name(p, name);
        // Java: return this.put(cp);
        self.put_custom(cp)
    }

    // Java: Object put(String name, Boolean value)
    pub fn put_boolean(&mut self, name: &str, value: bool) -> Option<CustomProperty> {
        // Java: MutableProperty p = new MutableProperty();
        let mut p = MutableProperty::new();
        // Java: p.setID(-1L);
        p.setID(-1);
        // Java: p.setType(11L);
        p.setType(11);
        // Java: p.setValue(value);
        p.setValue(PropertyValue::Bool(value));
        // Java: CustomProperty cp = new CustomProperty(p, name);
        let cp = CustomProperty::new_from_property_with_name(p, name);
        // Java: return this.put(cp);
        self.put_custom(cp)
    }

    // Java: Object get(String name)
    pub fn get_by_name(&self, name: &str) -> Option<&PropertyValue> {
        // Java: Long id = (Long)this.dictionaryNameToID.get(name);
        let id = self.dictionaryNameToID.get(name)?;
        // Java: CustomProperty cp = (CustomProperty)super.get(id);
        let cp = self.entries.get(id)?;
        // Java: return cp != null ? cp.getValue() : null;
        Some(cp.getValue())
    }

    // Java: Object put(String name, Date value)
    pub fn put_date(&mut self, name: &str, value: i64) -> Option<CustomProperty> {
        // Java: MutableProperty p = new MutableProperty();
        let mut p = MutableProperty::new();
        // Java: p.setID(-1L);
        p.setID(-1);
        // Java: p.setType(64L);
        p.setType(64);
        // Java: p.setValue(value);
        p.setValue(PropertyValue::Long(value));
        // Java: CustomProperty cp = new CustomProperty(p, name);
        let cp = CustomProperty::new_from_property_with_name(p, name);
        // Java: return this.put(cp);
        self.put_custom(cp)
    }

    // Java: Set keySet()
    pub fn keySet(&self) -> HashSet<String> {
        // Java: return this.dictionaryNameToID.keySet();
        self.dictionaryNameToID.keys().cloned().collect()
    }

    // Java: Set<String> nameSet()
    pub fn nameSet(&self) -> HashSet<String> {
        // Java: return this.dictionaryNameToID.keySet();
        self.dictionaryNameToID.keys().cloned().collect()
    }

    // Java: Set<String> idSet()
    pub fn idSet(&self) -> HashSet<String> {
        // Java: return this.dictionaryNameToID.keySet();
        self.dictionaryNameToID.keys().cloned().collect()
    }

    // Java: void setCodepage(int codepage)
    pub fn setCodepage(&mut self, codepage: i32) {
        // Java: MutableProperty p = new MutableProperty();
        let mut p = MutableProperty::new();
        // Java: p.setID(1L);
        p.setID(1);
        // Java: p.setType(2L);
        p.setType(2);
        // Java: p.setValue(codepage);
        p.setValue(PropertyValue::Int(codepage));
        // Java: this.put(new CustomProperty(p));
        let cp = CustomProperty::new_from_property(p);
        self.put_custom(cp);
    }

    // Java: ILongObjectMap<String> getDictionary()
    pub fn getDictionary(&self) -> &HashMap<i64, String> {
        // Java: return this.dictionaryIDToName;
        &self.dictionaryIDToName
    }

    // Java: boolean containsKey(Object key)
    pub fn containsKey_long(&self, key: i64) -> bool {
        // Java: if (key instanceof Long)
        // Java: return super.containsKey((Long)key);
        self.entries.contains_key(&key)
        // Java: else
        // Java: return key instanceof String ? super.containsKey((Long)this.dictionaryNameToID.get(key)) : false;
    }

    pub fn containsKey_name(&self, key: &str) -> bool {
        if let Some(&id) = self.dictionaryNameToID.get(key) {
            self.entries.contains_key(&id)
        } else {
            false
        }
    }

    // Java: boolean containsValue(Object value)
    pub fn containsValue(&self, value: &CustomProperty) -> bool {
        // Java: if (value instanceof CustomProperty)
        // Java: return super.containsValue((CustomProperty)value);
        // Java: else
        // Java: for(CustomProperty cp : super.values())
        // Java: if (cp.getValue() == value)
        // Java: return true;
        for cp in self.entries.values() {
            if cp.getID() == value.getID() {
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: int getCodepage()
    pub fn getCodepage(&self) -> i32 {
        // Java: int codepage = -1;
        let mut codepage: i32 = -1;
        // Java: Iterator<CustomProperty> i = this.values().iterator();
        // Java: while(codepage == -1 && i.hasNext())
        for cp in self.entries.values()
        {
            if codepage != -1 { break; }
            // Java: CustomProperty cp = (CustomProperty)i.next();
            // Java: if (cp.getID() == 1L)
            if cp.getID() == 1
            {
                // Java: codepage = (Integer)cp.getValue();
                if let PropertyValue::Int(v) = cp.getValue() {
                    codepage = *v;
                }
            }
        }
        // Java: return codepage;
        codepage
    }

    // Java: boolean isPure()
    pub fn isPure(&self) -> bool {
        // Java: return this.isPure;
        self.isPure
    }

    // Java: void setPure(boolean isPure)
    pub fn setPure(&mut self, isPure: bool) {
        // Java: this.isPure = isPure;
        self.isPure = isPure;
    }

    pub fn values(&self) -> impl Iterator<Item = &CustomProperty> {
        self.entries.values()
    }
}
