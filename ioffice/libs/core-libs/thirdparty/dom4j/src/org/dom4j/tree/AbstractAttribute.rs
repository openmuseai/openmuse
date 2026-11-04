// Java: import java.io.IOException
// Java: import java.io.Writer
// Java: import org.dom4j.Attribute
// Java: import org.dom4j.Element
// Java: import org.dom4j.Namespace
// Java: import org.dom4j.Node
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.Visitor

// Java: public class AbstractAttribute extends AbstractNode implements Attribute
pub struct AbstractAttribute {
}

// Java: public class AbstractAttribute extends AbstractNode implements Attribute
impl AbstractAttribute {
    // Java: AbstractAttribute()
    pub fn new() -> Self {
        // Java: super();
        AbstractAttribute {}
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) -> super::super::NodeType::NodeType {
        // Java: return NodeType.ATTRIBUTE_NODE;
        super::super::NodeType::NodeType::AttributeNode
    }

    // Java: void setNamespace(Namespace namespace)
    pub fn setNamespace(&self) {
        // Java: String msg = "This Attribute is read only and cannot be changed";
        // Java: throw new UnsupportedOperationException(msg);
        panic!("This Attribute is read only and cannot be changed");
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
        // Java: return this.getValue();
        self.getValue()
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: String) {
        // Java: this.setValue(text);
        self.setValue(text);
    }

    // Java: void setValue(String value)
    pub fn setValue(&mut self, _value: String) {
        // Java: String msg = "This Attribute is read only and cannot be changed";
        // Java: throw new UnsupportedOperationException(msg);
        panic!("This Attribute is read only and cannot be changed");
    }

    // Java: Object getData()
    pub fn getData(&self) -> String {
        // Java: return this.getValue();
        self.getValue()
    }

    // Java: void setData(Object data)
    pub fn setData(&mut self, data: Option<String>) {
        // Java: this.setValue(data == null ? null : data.toString());
        self.setValue(data.unwrap_or_default());
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString_builder(&self, builder: &mut String) {
        // Java: super.toString(builder);
        // Java: builder.append(" [Attribute: name ");
        builder.push_str(" [Attribute: name ");
        // Java: builder.append(this.getQualifiedName());
        builder.push_str(&self.getQualifiedName());
        // Java: builder.append(" value \"");
        builder.push_str(" value \"");
        // Java: builder.append(this.getValue());
        builder.push_str(&self.getValue());
        // Java: builder.append("\"]");
        builder.push_str("\"]");
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
        // Java: return this.getQualifiedName() + "=\"" + this.getValue() + "\"";
        format!("{}=\"{}\"", self.getQualifiedName(), self.getValue())
    }

    // Java: void write(Writer writer)
    pub fn write(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: writer.write(this.getQualifiedName());
        write!(writer, "{}", self.getQualifiedName())?;
        // Java: writer.write("=\"");
        write!(writer, "=\"")?;
        // Java: writer.write(this.getValue());
        write!(writer, "{}", self.getValue())?;
        // Java: writer.write("\"");
        write!(writer, "\"")?;
        Ok(())
    }

    // Java: void accept(Visitor visitor)
    pub fn accept(&self) {
        // Java: visitor.visit((Attribute)this);
    }

    // Java: Namespace getNamespace()
    pub fn getNamespace(&self) -> String {
        // Java: return this.getQName().getNamespace();
        String::new()
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: return this.getQName().getName();
        String::new()
    }

    // Java: String getNamespacePrefix()
    pub fn getNamespacePrefix(&self) -> String {
        // Java: return this.getQName().getNamespacePrefix();
        String::new()
    }

    // Java: String getNamespaceURI()
    pub fn getNamespaceURI(&self) -> String {
        // Java: return this.getQName().getNamespaceURI();
        String::new()
    }

    // Java: String getQualifiedName()
    pub fn getQualifiedName(&self) -> String {
        // Java: return this.getQName().getQualifiedName();
        String::new()
    }

    // Java: String getValue()
    pub fn getValue(&self) -> String {
        String::new()
    }

    // Java: String getPath(Element context)
    pub fn getPath(&self) -> String {
        // Java: StringBuffer result = new StringBuffer();
        let mut result = String::new();
        // Java: Element parent = this.getParent();
        // Java: if (parent != null && parent != context)
        {
            // Java: result.append(parent.getPath(context));
            // Java: result.append("/");
        }
        // Java: result.append("@");
        result.push('@');
        // Java: String uri = this.getNamespaceURI();
        let uri = self.getNamespaceURI();
        // Java: String prefix = this.getNamespacePrefix();
        let prefix = self.getNamespacePrefix();
        // Java: if (uri != null && uri.length() != 0 && prefix != null && prefix.length() != 0)
        if !uri.is_empty() && !prefix.is_empty() {
            // Java: result.append(this.getQualifiedName());
            result.push_str(&self.getQualifiedName());
        }
        // Java: else
        else {
            // Java: result.append(this.getName());
            result.push_str(&self.getName());
        }
        // Java: return result.toString();
        result
    }

    // Java: String getUniquePath(Element context)
    pub fn getUniquePath(&self) -> String {
        // Java: StringBuffer result = new StringBuffer();
        let mut result = String::new();
        // Java: Element parent = this.getParent();
        // Java: if (parent != null && parent != context)
        {
            // Java: result.append(parent.getUniquePath(context));
            // Java: result.append("/");
        }
        // Java: result.append("@");
        result.push('@');
        // Java: String uri = this.getNamespaceURI();
        let uri = self.getNamespaceURI();
        // Java: String prefix = this.getNamespacePrefix();
        let prefix = self.getNamespacePrefix();
        // Java: if (uri != null && uri.length() != 0 && prefix != null && prefix.length() != 0)
        if !uri.is_empty() && !prefix.is_empty() {
            // Java: result.append(this.getQualifiedName());
            result.push_str(&self.getQualifiedName());
        }
        // Java: else
        else {
            // Java: result.append(this.getName());
            result.push_str(&self.getName());
        }
        // Java: return result.toString();
        result
    }

    // Java: Node createXPathResult(Element parent)
    pub fn createXPathResult(&self) {
        // Java: return new DefaultAttribute(parent, this.getQName(), this.getValue());
    }
}
