// Java: import java.io.IOException
// Java: import java.io.Writer
// Java: import org.dom4j.Element
// Java: import org.dom4j.Entity
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.Visitor

// Java: public class AbstractEntity extends AbstractNode implements Entity
pub struct AbstractEntity {
}

// Java: public class AbstractEntity extends AbstractNode implements Entity
impl AbstractEntity {
    // Java: AbstractEntity()
    pub fn new() -> Self {
        // Java: super();
        AbstractEntity {}
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) -> super::super::NodeType::NodeType {
        // Java: return NodeType.ENTITY_REFERENCE_NODE;
        super::super::NodeType::NodeType::EntityReferenceNode
    }

    // Java: String getPath(Element context)
    pub fn getPath(&self) -> String {
        // Java: Element parent = this.getParent();
        // Java: return parent != null && parent != context ? parent.getPath(context) + "/text()" : "text()";
        "text()".to_string()
    }

    // Java: String getUniquePath(Element context)
    pub fn getUniquePath(&self) -> String {
        // Java: Element parent = this.getParent();
        // Java: return parent != null && parent != context ? parent.getUniquePath(context) + "/text()" : "text()";
        "text()".to_string()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + " [Entity: &" + this.getName() + ";]";
        format!(" [Entity: &{};]", self.getName())
    }

    // Java: String getStringValue()
    pub fn getStringValue(&self) -> String {
        // Java: return "&" + this.getName() + ";";
        format!("&{};", self.getName())
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
        // Java: return "&" + this.getName() + ";";
        format!("&{};", self.getName())
    }

    // Java: void write(Writer writer)
    pub fn write(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: writer.write("&");
        write!(writer, "&")?;
        // Java: writer.write(this.getName());
        write!(writer, "{}", self.getName())?;
        // Java: writer.write(";");
        write!(writer, ";")?;
        Ok(())
    }

    // Java: void accept(Visitor visitor)
    pub fn accept(&self) {
        // Java: visitor.visit((Entity)this);
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        String::new()
    }
}
