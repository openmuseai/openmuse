// Java: import java.io.IOException
// Java: import java.io.StringWriter
// Java: import java.io.Writer
// Java: import org.dom4j.CDATA
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.Visitor

// Java: public class AbstractCDATA extends AbstractCharacterData implements CDATA
pub struct AbstractCDATA {
}

// Java: public class AbstractCDATA extends AbstractCharacterData implements CDATA
impl AbstractCDATA {
    // Java: AbstractCDATA()
    pub fn new() -> Self {
        // Java: super();
        AbstractCDATA {}
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) -> super::super::NodeType::NodeType {
        // Java: return NodeType.CDATA_SECTION_NODE;
        super::super::NodeType::NodeType::CdataSectionNode
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString_builder(&self, builder: &mut String) {
        // Java: super.toString(builder);
        // Java: builder.append(" [CDATA: \"");
        builder.push_str(" [CDATA: \"");
        // Java: builder.append(this.getText());
        builder.push_str(&self.getText());
        // Java: builder.append("\"]");
        builder.push_str("\"]");
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
        // Java: StringWriter writer = new StringWriter();
        let mut writer = Vec::new();
        // Java: try
        // Java: this.write(writer);
        let _ = self.write(&mut writer);
        // Java: catch (IOException var3)
        // Java: return writer.toString();
        String::from_utf8(writer).unwrap_or_default()
    }

    // Java: void write(Writer writer)
    pub fn write(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: writer.write("<![CDATA[");
        write!(writer, "<![CDATA[")?;
        // Java: if (this.getText() != null)
        {
            // Java: writer.write(this.getText());
            write!(writer, "{}", self.getText())?;
        }
        // Java: writer.write("]]>");
        write!(writer, "]]>")?;
        Ok(())
    }

    // Java: void accept(Visitor visitor)
    pub fn accept(&self) {
        // Java: visitor.visit((CDATA)this);
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
        String::new()
    }
}
