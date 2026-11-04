// Java: import cn.wps.io.dom.tree.AbstractNode
// Java: import cn.wps.io.dom.tree.NamespaceCache

// Java: public class Namespace extends AbstractNode
pub struct Namespace {
    // Java field: String prefix
    prefix: String,
    // Java field: String uri
    uri: String,
    // Java field: int hashCode
    hashCode: i32,
}

// Java: public class Namespace extends AbstractNode
impl Namespace {
    // Java: Namespace(String prefix, String uri)
    pub fn new(prefix: &str, uri: &str) -> Self {
        Namespace {
            prefix: if prefix.is_empty() { String::new() } else { prefix.to_string() },
            uri: if uri.is_empty() { String::new() } else { uri.to_string() },
            hashCode: 0,
        }
    }

    // Java: static Namespace get(String prefix, String uri)
    pub fn get(prefix: &str, uri: &str) -> Self {
        Namespace::new(prefix, uri)
    }

    // Java: static Namespace get(String uri)
    pub fn get_by_uri(uri: &str) -> Self {
        Namespace::new("", uri)
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) -> &str {
        "NAMESPACE_NODE"
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
        if self.hashCode == 0 {
            self.hashCode = self.createHashCode();
        }
        self.hashCode
    }

    // Java: int createHashCode()
    fn createHashCode(&self) -> i32 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.uri.hash(&mut hasher);
        self.prefix.hash(&mut hasher);
        let result = hasher.finish() as i32;
        if result == 0 { 47806 } else { result }
    }

    // Java: boolean equals(Object object)
    pub fn equals(&self, other: &Namespace) -> bool {
        std::ptr::eq(self, other) || (self.uri == other.uri && self.prefix == other.prefix)
    }

    // Java: String getText()
    pub fn getText(&self) -> &str {
        &self.uri
    }

    // Java: String getStringValue()
    pub fn getStringValue(&self) -> &str {
        &self.uri
    }

    // Java: String getPrefix()
    pub fn getPrefix(&self) -> &str {
        &self.prefix
    }

    // Java: String getURI()
    pub fn getURI(&self) -> &str {
        &self.uri
    }

    // Java: String getXPathNameStep()
    pub fn getXPathNameStep(&self) -> String {
        if !self.prefix.is_empty() {
            format!("namespace::{}", self.prefix)
        } else {
            "namespace::*[name()='']".to_string()
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        format!("Namespace [Namespace: prefix {} mapped to URI \"{}\"]", self.getPrefix(), self.getURI())
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
        let pref = self.getPrefix();
        let mut asxml = String::with_capacity(10);
        if !pref.is_empty() {
            asxml.push_str("xmlns:");
            asxml.push_str(pref);
            asxml.push_str("=\"");
        } else {
            asxml.push_str("xmlns=\"");
        }
        asxml.push_str(self.getURI());
        asxml.push('"');
        asxml
    }
}
