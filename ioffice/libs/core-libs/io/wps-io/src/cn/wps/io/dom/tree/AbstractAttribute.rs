// Java: import cn.wps.io.dom.Attribute
// Java: import cn.wps.io.dom.Namespace
// Java: import cn.wps.io.dom.NodeType

// Java: public class AbstractAttribute extends AbstractNode implements Attribute
pub struct AbstractAttribute {
}

// Java: public class AbstractAttribute extends AbstractNode implements Attribute
impl AbstractAttribute {
    // Java: AbstractAttribute()
    pub fn new() -> Self {
      // Java: super();
      todo!();
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&mut self) -> Option<super::super::NodeType::NodeType> {
      // Java: return NodeType.ATTRIBUTE_NODE;
      todo!();
    }

    // Java: void setNamespace(Namespace namespace)
    pub fn setNamespace(&mut self) {
      // Java: String msg = "This Attribute is read only and cannot be changed";
      todo!();
      // Java: throw new UnsupportedOperationException(msg);
      todo!();
    }

    // Java: String getText()
    pub fn getText(&mut self) -> String {
      // Java: return this.getValue();
      todo!();
    }

    // Java: void setText(String text)
    pub fn setText(&mut self) {
      // Java: this.setValue(text);
      todo!();
    }

    // Java: void setValue(String value)
    pub fn setValue(&mut self) {
      // Java: String msg = "This Attribute is read only and cannot be changed";
      todo!();
      // Java: throw new UnsupportedOperationException(msg);
      todo!();
    }

    // Java: Object getData()
    pub fn getData(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: return this.getValue();
      todo!();
    }

    // Java: void setData(Object data)
    pub fn setData(&mut self) {
      // Java: this.setValue(data == null ? null : data.toString());
      todo!();
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString(&mut self) {
      // Java: super.toString(builder);
      todo!();
      // Java: builder.append(" [Attribute: name ");
      todo!();
      // Java: builder.append(this.getName());
      todo!();
      // Java: builder.append(" value \"");
      todo!();
      // Java: builder.append(this.getValue());
      todo!();
      // Java: builder.append("\"]");
      todo!();
    }

    // Java: String asXML()
    pub fn asXML(&mut self) -> String {
      // Java: return this.getQualifiedName() + "=\"" + this.getValue() + "\"";
      todo!();
    }

}
