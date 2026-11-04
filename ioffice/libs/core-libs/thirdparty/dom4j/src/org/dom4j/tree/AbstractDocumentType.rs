// Java: import java.io.IOException
// Java: import java.io.Writer
// Java: import java.util.List
// Java: import org.dom4j.DocumentType
// Java: import org.dom4j.Element
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.Visitor
// Java: import org.dom4j.dtd.InternalDeclaration

// Java: public class AbstractDocumentType extends AbstractNode implements DocumentType
pub struct AbstractDocumentType {
}

// Java: public class AbstractDocumentType extends AbstractNode implements DocumentType
impl AbstractDocumentType {
    // Java: AbstractDocumentType()
    pub fn new() -> Self {
        // Java: super();
        AbstractDocumentType {}
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) -> super::super::NodeType::NodeType {
        // Java: return NodeType.DOCUMENT_TYPE_NODE;
        super::super::NodeType::NodeType::DocumentTypeNode
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: return this.getElementName();
        self.getElementName()
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: String) {
        // Java: this.setElementName(name);
        self.setElementName(name);
    }

    // Java: String getPath(Element context)
    pub fn getPath(&self) -> String {
        // Java: return "";
        String::new()
    }

    // Java: String getUniquePath(Element context)
    pub fn getUniquePath(&self) -> String {
        // Java: return "";
        String::new()
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
        // Java: List<InternalDeclaration> list = this.getInternalDeclarations();
        let list = self.getInternalDeclarations();
        // Java: if (list != null && list.size() > 0)
        if !list.is_empty() {
            // Java: StringBuilder builder = new StringBuilder();
            let mut builder = String::new();
            // Java: for(InternalDeclaration decl : list)
            for decl in &list {
                // Java: builder.append(decl.toString());
                builder.push_str(decl);
                // Java: builder.append('\n');
                builder.push('\n');
            }
            // Java: if (builder.length() > 0)
            if !builder.is_empty() {
                // Java: builder.setLength(builder.length() - 1);
                builder.pop();
            }
            // Java: return builder.toString();
            builder
        }
        // Java: else
        else {
            // Java: return "";
            String::new()
        }
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString_builder(&self, builder: &mut String) {
        // Java: super.toString(builder);
        // Java: builder.append(" [DocumentType: ");
        builder.push_str(" [DocumentType: ");
        // Java: this.asXML(builder);
        builder.push_str(&self.asXML());
        // Java: builder.append(']');
        builder.push(']');
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
        // Java: StringBuilder builder = new StringBuilder();
        let mut builder = String::new();
        // Java: this.asXML(builder);
        self.asXML_builder(&mut builder);
        // Java: return builder.toString();
        builder
    }

    // Java: void asXML(StringBuilder builder)
    pub fn asXML_builder(&self, builder: &mut String) {
        // Java: builder.append("<!DOCTYPE ");
        builder.push_str("<!DOCTYPE ");
        // Java: builder.append(this.getElementName());
        builder.push_str(&self.getElementName());
        // Java: boolean hasPublicID = false;
        let mut has_public_id = false;
        // Java: String publicID = this.getPublicID();
        let public_id = self.getPublicID();
        // Java: if (publicID != null && publicID.length() > 0)
        if !public_id.is_empty() {
            // Java: builder.append(" PUBLIC ");
            builder.push_str(" PUBLIC ");
            // Java: builder.append('"');
            builder.push('"');
            // Java: builder.append(publicID);
            builder.push_str(&public_id);
            // Java: builder.append('"');
            builder.push('"');
            // Java: hasPublicID = true;
            has_public_id = true;
        }
        // Java: String systemID = this.getSystemID();
        let system_id = self.getSystemID();
        // Java: if (systemID != null && systemID.length() > 0)
        if !system_id.is_empty() {
            // Java: if (!hasPublicID)
            if !has_public_id {
                // Java: builder.append(" SYSTEM");
                builder.push_str(" SYSTEM");
            }
            // Java: builder.append(" \"");
            builder.push_str(" \"");
            // Java: builder.append(systemID);
            builder.push_str(&system_id);
            // Java: builder.append('"');
            builder.push('"');
        }
        // Java: builder.append('>');
        builder.push('>');
    }

    // Java: void write(Writer writer)
    pub fn write(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: writer.write("<!DOCTYPE ");
        write!(writer, "<!DOCTYPE ")?;
        // Java: writer.write(this.getElementName());
        write!(writer, "{}", self.getElementName())?;
        // Java: boolean hasPublicID = false;
        let mut has_public_id = false;
        // Java: String publicID = this.getPublicID();
        let public_id = self.getPublicID();
        // Java: if (publicID != null && publicID.length() > 0)
        if !public_id.is_empty() {
            // Java: writer.write(" PUBLIC \"");
            write!(writer, " PUBLIC \"")?;
            // Java: writer.write(publicID);
            write!(writer, "{}", public_id)?;
            // Java: writer.write(34);
            write!(writer, "\"")?;
            // Java: hasPublicID = true;
            has_public_id = true;
        }
        // Java: String systemID = this.getSystemID();
        let system_id = self.getSystemID();
        // Java: if (systemID != null && systemID.length() > 0)
        if !system_id.is_empty() {
            // Java: if (!hasPublicID)
            if !has_public_id {
                // Java: writer.write(" SYSTEM");
                write!(writer, " SYSTEM")?;
            }
            // Java: writer.write(" \"");
            write!(writer, " \"")?;
            // Java: writer.write(systemID);
            write!(writer, "{}", system_id)?;
            // Java: writer.write(34);
            write!(writer, "\"")?;
        }
        // Java: List<InternalDeclaration> list = this.getInternalDeclarations();
        let list = self.getInternalDeclarations();
        // Java: if (list != null && list.size() > 0)
        if !list.is_empty() {
            // Java: writer.write(" [");
            write!(writer, " [")?;
            // Java: for(InternalDeclaration decl : list)
            for decl in &list {
                // Java: writer.write("\n  ");
                write!(writer, "\n  ")?;
                // Java: writer.write(decl.toString());
                write!(writer, "{}", decl)?;
            }
            // Java: writer.write("\n]");
            write!(writer, "\n]")?;
        }
        // Java: writer.write(62);
        write!(writer, ">")?;
        Ok(())
    }

    // Java: void accept(Visitor visitor)
    pub fn accept(&self) {
        // Java: visitor.visit((DocumentType)this);
    }

    // Java: String getElementName()
    pub fn getElementName(&self) -> String {
        String::new()
    }

    // Java: void setElementName(String name)
    pub fn setElementName(&mut self, _name: String) {
    }

    // Java: String getPublicID()
    pub fn getPublicID(&self) -> String {
        String::new()
    }

    // Java: String getSystemID()
    pub fn getSystemID(&self) -> String {
        String::new()
    }

    // Java: List getInternalDeclarations()
    pub fn getInternalDeclarations(&self) -> Vec<String> {
        Vec::new()
    }
}
