// Java: import java.io.IOException
// Java: import java.io.Writer
// Java: import org.dom4j.Comment
// Java: import org.dom4j.Element
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.Visitor

// Java: public class AbstractComment extends AbstractCharacterData implements Comment
pub struct AbstractComment {
}

// Java: public class AbstractComment extends AbstractCharacterData implements Comment
impl AbstractComment {
    // Java: AbstractComment()
    pub fn new() -> Self {
        // Java: super();
        AbstractComment {}
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) -> super::super::NodeType::NodeType {
        // Java: return NodeType.COMMENT_NODE;
        super::super::NodeType::NodeType::CommentNode
    }

    // Java: String getPath(Element context)
    pub fn getPath(&self) -> String {
        // Java: Element parent = this.getParent();
        // Java: return parent != null && parent != context ? parent.getPath(context) + "/comment()" : "comment()";
        "comment()".to_string()
    }

    // Java: String getUniquePath(Element context)
    pub fn getUniquePath(&self) -> String {
        // Java: Element parent = this.getParent();
        // Java: return parent != null && parent != context ? parent.getUniquePath(context) + "/comment()" : "comment()";
        "comment()".to_string()
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString_builder(&self, builder: &mut String) {
        // Java: super.toString(builder);
        // Java: builder.append(" [Comment: \"");
        builder.push_str(" [Comment: \"");
        // Java: builder.append(this.getText());
        builder.push_str(&self.getText());
        // Java: builder.append("\"]");
        builder.push_str("\"]");
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
        // Java: return "<!--" + this.getText() + "-->";
        format!("<!--{}-->", self.getText())
    }

    // Java: void write(Writer writer)
    pub fn write(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: writer.write("<!--");
        write!(writer, "<!--")?;
        // Java: writer.write(this.getText());
        write!(writer, "{}", self.getText())?;
        // Java: writer.write("-->");
        write!(writer, "-->")?;
        Ok(())
    }

    // Java: void accept(Visitor visitor)
    pub fn accept(&self) {
        // Java: visitor.visit((Comment)this);
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
        String::new()
    }
}
