// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.dom4j.QName
use std::collections::HashMap;

// Java: public class DoubleNameMap
pub struct DoubleNameMap<T: Clone> {
    // Java field: Map<String, T> namedMap
    pub(crate) namedMap: HashMap<String, T>,
    // Java field: Map<QName, T> qNamedMap
    pub(crate) qNamedMap: HashMap<String, T>,
}

// Java: public class DoubleNameMap
impl<T: Clone> DoubleNameMap<T> {
    // Java: DoubleNameMap()
    pub fn new() -> Self {
        // Java: super();
        DoubleNameMap {
            namedMap: HashMap::new(),
            qNamedMap: HashMap::new(),
        }
    }

    // Java: void put(QName qName, T value)
    pub fn put(&mut self, qname_key: &str, name_key: &str, value: T) {
        // Java: this.qNamedMap.put(qName, value);
        self.qNamedMap.insert(qname_key.to_string(), value.clone());
        // Java: this.namedMap.put(qName.getName(), value);
        self.namedMap.insert(name_key.to_string(), value);
    }

    // Java: T get(String name)
    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        // Java: return (T)this.namedMap.get(name);
        self.namedMap.get(name)
    }

    // Java: T get(QName qName)
    pub fn get_by_qname(&self, qname_key: &str) -> Option<&T> {
        // Java: return (T)this.qNamedMap.get(qName);
        self.qNamedMap.get(qname_key)
    }

    // Java: void remove(QName qName)
    pub fn remove(&mut self, qname_key: &str, name_key: &str) {
        // Java: this.qNamedMap.remove(qName);
        self.qNamedMap.remove(qname_key);
        // Java: this.namedMap.remove(qName.getName());
        self.namedMap.remove(name_key);
    }
}
