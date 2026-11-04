// Java: import org.dom4j.Namespace
// Java: import org.dom4j.QName

// Java: public class FlyweightAttribute extends AbstractAttribute
pub struct FlyweightAttribute {
    // Java field: QName qname
    pub(crate) qname: String,
    // Java field: String value
    pub(crate) value: String,
}

// Java: public class FlyweightAttribute extends AbstractAttribute
impl FlyweightAttribute {
    // Java: FlyweightAttribute(QName qname)
    pub fn new_qname(qname: String) -> Self {
        // Java: super();
        // Java: this.qname = qname;
        FlyweightAttribute {
            qname,
            value: String::new(),
        }
    }

    // Java: FlyweightAttribute(QName qname, String value)
    pub fn new_qname_value(qname: String, value: String) -> Self {
        // Java: super();
        // Java: this.qname = qname;
        // Java: this.value = value;
        FlyweightAttribute {
            qname,
            value,
        }
    }

    // Java: FlyweightAttribute(String name, String value)
    pub fn new_name_value(name: String, value: String) -> Self {
        // Java: super();
        // Java: this.qname = this.getDocumentFactory().createQName(name);
        // Java: this.value = value;
        FlyweightAttribute {
            qname: name,
            value,
        }
    }

    // Java: FlyweightAttribute(String name, String value, Namespace namespace)
    pub fn new_name_value_ns(name: String, value: String, _namespace: String) -> Self {
        // Java: super();
        // Java: this.qname = this.getDocumentFactory().createQName(name, namespace);
        // Java: this.value = value;
        FlyweightAttribute {
            qname: name,
            value,
        }
    }

    // Java: String getValue()
    pub fn getValue(&self) -> String {
        // Java: return this.value;
        self.value.clone()
    }

    // Java: QName getQName()
    pub fn getQName(&self) -> String {
        // Java: return this.qname;
        self.qname.clone()
    }
}
