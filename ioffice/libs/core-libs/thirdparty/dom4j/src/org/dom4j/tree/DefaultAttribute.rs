// Java: import org.dom4j.Element
// Java: import org.dom4j.Namespace
// Java: import org.dom4j.QName

// Java: public class DefaultAttribute extends FlyweightAttribute
pub struct DefaultAttribute {
    // Java field: Element parent
    pub(crate) qname: String,
    pub(crate) value: String,
}

// Java: public class DefaultAttribute extends FlyweightAttribute
impl DefaultAttribute {
    // Java: DefaultAttribute(QName qname)
    pub fn new_qname(qname: String) -> Self {
        // Java: super(qname);
        DefaultAttribute { qname, value: String::new() }
    }

    // Java: DefaultAttribute(QName qname, String value)
    pub fn new_qname_value(qname: String, value: String) -> Self {
        // Java: super(qname, value);
        DefaultAttribute { qname, value }
    }

    // Java: DefaultAttribute(Element parent, QName qname, String value)
    pub fn new_parent_qname_value(qname: String, value: String) -> Self {
        // Java: super(qname, value);
        // Java: this.parent = parent;
        DefaultAttribute { qname, value }
    }

    // Java: DefaultAttribute(String name, String value)
    pub fn new_name_value(name: String, value: String) -> Self {
        // Java: super(name, value);
        DefaultAttribute { qname: name, value }
    }

    // Java: DefaultAttribute(String name, String value, Namespace namespace)
    pub fn new_name_value_ns(name: String, value: String) -> Self {
        // Java: super(name, value, namespace);
        DefaultAttribute { qname: name, value }
    }

    // Java: DefaultAttribute(Element parent, String name, String value, Namespace namespace)
    pub fn new_parent_name_value_ns(name: String, value: String) -> Self {
        // Java: super(name, value, namespace);
        // Java: this.parent = parent;
        DefaultAttribute { qname: name, value }
    }

    // Java: void setValue(String value)
    pub fn setValue(&mut self, value: String) {
        // Java: this.value = value;
        self.value = value;
    }

    // Java: Element getParent()
    pub fn getParent(&self) {
        // Java: return this.parent;
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&self) {
        // Java: this.parent = parent;
    }

    // Java: boolean supportsParent()
    pub fn supportsParent(&self) -> bool {
        // Java: return true;
        true
    }

    // Java: boolean isReadOnly()
    pub fn isReadOnly(&self) -> bool {
        // Java: return false;
        false
    }
}
