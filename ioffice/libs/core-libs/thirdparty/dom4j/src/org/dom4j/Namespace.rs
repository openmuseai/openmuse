// Java: public class Namespace extends AbstractNode
#[derive(Clone)]
pub struct Namespace {
    // Java field: NamespaceCache CACHE
    // Java field: Namespace XML_NAMESPACE
    // Java field: Namespace NO_NAMESPACE
    // Java field: String prefix
    prefix: String,
    // Java field: String uri
    uri: String,
    // Java field: int hashCode
    pub(crate) hashCode: i32,
}

// Java: public class Namespace extends AbstractNode
impl Namespace {
    // Java: Namespace(String prefix, String uri)
    pub fn new(prefix: &str, uri: &str) -> Self {
        // Java: super();
        // Java: this.prefix = prefix != null ? prefix : "";
        // Java: this.uri = uri != null ? uri : "";
        Namespace {
            prefix: prefix.to_string(),
            uri: uri.to_string(),
            hashCode: 0,
        }
    }

    // Java: static Namespace get(String prefix, String uri)
    pub fn get(_prefix: &str, _uri: &str) -> Namespace {
        // Java: return CACHE.get(prefix, uri);
        Namespace::new(_prefix, _uri)
    }

    // Java: static Namespace get(String uri)
    pub fn get_uri(_uri: &str) -> Namespace {
        // Java: return CACHE.get(uri);
        Namespace::new("", _uri)
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) -> i32 {
        // Java: return NodeType.NAMESPACE_NODE;
        13
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
        // Java: if (this.hashCode == 0)
        if self.hashCode == 0 {
            // Java: this.hashCode = this.createHashCode();
            self.hashCode = self.createHashCode();
        }
        // Java: return this.hashCode;
        self.hashCode
    }

    // Java: int createHashCode()
    fn createHashCode(&self) -> i32 {
        // Java: int result = this.uri.hashCode() ^ this.prefix.hashCode();
        use std::hash::{Hash, Hasher};
        let mut h1 = std::collections::hash_map::DefaultHasher::new();
        self.uri.hash(&mut h1);
        let uri_hash = h1.finish() as i32;
        let mut h2 = std::collections::hash_map::DefaultHasher::new();
        self.prefix.hash(&mut h2);
        let prefix_hash = h2.finish() as i32;
        let mut result = uri_hash ^ prefix_hash;
        // Java: if (result == 0)
        if result == 0 {
            // Java: result = 47806;
            result = 47806;
        }
        // Java: return result;
        result
    }

    // Java: boolean equals(Object object)
    pub fn equals(&mut self, other: &mut Namespace) -> bool {
        // Java: if (this == object)
        if std::ptr::eq(self, other) {
            // Java: return true;
            return true;
        }
        // Java: else
        {
            // Java: if (object instanceof Namespace)
            // Java: Namespace that = (Namespace)object;
            // Java: if (this.hashCode() == that.hashCode())
            if self.hashCode() == other.hashCode() {
                // Java: return this.uri.equals(that.getURI()) && this.prefix.equals(that.getPrefix());
                return self.uri == other.uri && self.prefix == other.prefix;
            }
        }
        // Java: return false;
        false
    }

    // Java: String getText()
    pub fn getText(&self) -> &str {
        // Java: return this.uri;
        &self.uri
    }

    // Java: String getStringValue()
    pub fn getStringValue(&self) -> &str {
        // Java: return this.uri;
        &self.uri
    }

    // Java: String getPrefix()
    pub fn getPrefix(&self) -> &str {
        // Java: return this.prefix;
        &self.prefix
    }

    // Java: String getURI()
    pub fn getURI(&self) -> &str {
        // Java: return this.uri;
        &self.uri
    }

    // Java: String getXPathNameStep()
    pub fn getXPathNameStep(&self) -> String {
        // Java: return this.prefix != null && !"".equals(this.prefix) ? "namespace::" + this.prefix : "namespace::*[name()='']";
        if !self.prefix.is_empty() {
            format!("namespace::{}", self.prefix)
        } else {
            "namespace::*[name()='']".to_string()
        }
    }

    // Java: String getPath(Element context)
    pub fn getPath(&self) -> String {
        // Java: StringBuffer path = new StringBuffer(10);
        let mut path = String::with_capacity(10);
        // Java: Element parent = this.getParent();
        // Java: if (parent != null && parent != context)
        {
            // Java: path.append(parent.getPath(context));
            // Java: path.append('/');
        }
        // Java: path.append(this.getXPathNameStep());
        path.push_str(&self.getXPathNameStep());
        // Java: return path.toString();
        path
    }

    // Java: String getUniquePath(Element context)
    pub fn getUniquePath(&self) -> String {
        // Java: StringBuffer path = new StringBuffer(10);
        let mut path = String::with_capacity(10);
        // Java: Element parent = this.getParent();
        // Java: if (parent != null && parent != context)
        {
            // Java: path.append(parent.getUniquePath(context));
            // Java: path.append('/');
        }
        // Java: path.append(this.getXPathNameStep());
        path.push_str(&self.getXPathNameStep());
        // Java: return path.toString();
        path
    }

    // Java: String toString()
    pub fn to_string_display(&self) -> String {
        // Java: return super.toString() + " [Namespace: prefix " + this.getPrefix() + " mapped to URI \"" + this.getURI() + "\"]";
        format!("Namespace [Namespace: prefix {} mapped to URI \"{}\"]", self.getPrefix(), self.getURI())
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
        // Java: StringBuffer asxml = new StringBuffer(10);
        let mut asxml = String::with_capacity(10);
        // Java: String pref = this.getPrefix();
        let pref = self.getPrefix();
        // Java: if (pref != null && pref.length() > 0)
        if !pref.is_empty() {
            // Java: asxml.append("xmlns:");
            asxml.push_str("xmlns:");
            // Java: asxml.append(pref);
            asxml.push_str(pref);
            // Java: asxml.append("=\"");
            asxml.push_str("=\"");
        }
        // Java: else
        else {
            // Java: asxml.append("xmlns=\"");
            asxml.push_str("xmlns=\"");
        }
        // Java: asxml.append(this.getURI());
        asxml.push_str(self.getURI());
        // Java: asxml.append("\"");
        asxml.push('"');
        // Java: return asxml.toString();
        asxml
    }

    // Java: void accept(Visitor visitor)
    pub fn accept(&self) {
        // Java: visitor.visit(this);
    }

    // Java: Node createXPathResult(Element parent)
    pub fn createXPathResult(&self) {
        // Java: return new DefaultNamespace(parent, this.getPrefix(), this.getURI());
    }
}
