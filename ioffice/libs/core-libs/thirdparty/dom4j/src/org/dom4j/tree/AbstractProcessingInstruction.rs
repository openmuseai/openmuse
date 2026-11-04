// Java: import java.io.IOException
// Java: import java.io.Writer
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import java.util.StringTokenizer
// Java: import org.dom4j.Element
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.ProcessingInstruction
// Java: import org.dom4j.Visitor

use std::collections::HashMap;

// Java: public class AbstractProcessingInstruction extends AbstractNode implements ProcessingInstruction
pub struct AbstractProcessingInstruction {
}

// Java: public class AbstractProcessingInstruction extends AbstractNode implements ProcessingInstruction
impl AbstractProcessingInstruction {
    // Java: AbstractProcessingInstruction()
    pub fn new() -> Self {
        // Java: super();
        AbstractProcessingInstruction {}
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) -> super::super::NodeType::NodeType {
        // Java: return NodeType.PROCESSING_INSTRUCTION_NODE;
        super::super::NodeType::NodeType::ProcessingInstructionNode
    }

    // Java: String getPath(Element context)
    pub fn getPath(&self) -> String {
        // Java: Element parent = this.getParent();
        // Java: return parent != null && parent != context ? parent.getPath(context) + "/processing-instruction()" : "processing-instruction()";
        "processing-instruction()".to_string()
    }

    // Java: String getUniquePath(Element context)
    pub fn getUniquePath(&self) -> String {
        // Java: Element parent = this.getParent();
        // Java: return parent != null && parent != context ? parent.getUniquePath(context) + "/processing-instruction()" : "processing-instruction()";
        "processing-instruction()".to_string()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + " [ProcessingInstruction: &" + this.getName() + ";]";
        format!(" [ProcessingInstruction: &{};]", self.getName())
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
        // Java: return "<?" + this.getName() + " " + this.getText() + "?>";
        format!("<?{} {}?>", self.getName(), self.getText())
    }

    // Java: void write(Writer writer)
    pub fn write(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: writer.write("<?");
        write!(writer, "<?")?;
        // Java: writer.write(this.getName());
        write!(writer, "{}", self.getName())?;
        // Java: writer.write(" ");
        write!(writer, " ")?;
        // Java: writer.write(this.getText());
        write!(writer, "{}", self.getText())?;
        // Java: writer.write("?>");
        write!(writer, "?>")?;
        Ok(())
    }

    // Java: void accept(Visitor visitor)
    pub fn accept(&self) {
        // Java: visitor.visit((ProcessingInstruction)this);
    }

    // Java: void setValue(String name, String value)
    pub fn setValue(&self, _name: &str, _value: &str) {
        // Java: throw new UnsupportedOperationException("This PI is read-only and cannot be modified");
        panic!("This PI is read-only and cannot be modified");
    }

    // Java: void setValues(Map<String, String> data)
    pub fn setValues(&self, _data: &HashMap<String, String>) {
        // Java: throw new UnsupportedOperationException("This PI is read-only and cannot be modified");
        panic!("This PI is read-only and cannot be modified");
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: return this.getTarget();
        self.getTarget()
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: String) {
        // Java: this.setTarget(name);
        self.setTarget(name);
    }

    // Java: boolean removeValue(String name)
    pub fn removeValue(&self, _name: &str) -> bool {
        // Java: return false;
        false
    }

    // Java: String getTarget()
    pub fn getTarget(&self) -> String {
        String::new()
    }

    // Java: void setTarget(String target)
    pub fn setTarget(&mut self, _target: String) {
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
        String::new()
    }

    // Java: String toString(Map<String, String> values)
    pub fn toString_map(&self, values: &HashMap<String, String>) -> String {
        // Java: StringBuilder builder = new StringBuilder();
        let mut builder = String::new();
        // Java: for(Map.Entry<String, String> entry : values.entrySet())
        for (name, value) in values {
            // Java: String name = (String)entry.getKey();
            // Java: String value = (String)entry.getValue();
            // Java: builder.append(name);
            builder.push_str(name);
            // Java: builder.append("=\"");
            builder.push_str("=\"");
            // Java: builder.append(value);
            builder.push_str(value);
            // Java: builder.append("\" ");
            builder.push_str("\" ");
        }
        // Java: builder.setLength(builder.length() - 1);
        if !builder.is_empty() {
            builder.pop();
        }
        // Java: return builder.toString();
        builder
    }

    // Java: protected Map<String, String> parseValues(String text)
    pub fn parseValues(&self, text: &str) -> HashMap<String, String> {
        // Java: Map<String, String> data = new HashMap();
        let mut data = HashMap::new();
        // Java: StringTokenizer s = new StringTokenizer(text, " ='\"", true);
        let parts: Vec<&str> = text.split(|c: char| c == ' ' || c == '=' || c == '\'' || c == '"')
            .filter(|s| !s.is_empty())
            .collect();
        // Java: while(s.hasMoreTokens())
        let mut i = 0;
        while i + 1 < parts.len() {
            // Java: String name = this.getName(s);
            let name = parts[i].to_string();
            // Java: if (s.hasMoreTokens())
            // Java: String value = this.getValue(s);
            let value = parts[i + 1].to_string();
            // Java: data.put(name, value);
            data.insert(name, value);
            i += 2;
        }
        // Java: return data;
        data
    }

    // Java: String getName(StringTokenizer tokenizer)
    pub fn getName_tokenizer(&self, tokens: &[&str], pos: &mut usize) -> String {
        // Java: String token = tokenizer.nextToken();
        // Java: StringBuilder name = new StringBuilder(token);
        let mut name = String::new();
        if *pos < tokens.len() {
            name.push_str(tokens[*pos]);
            *pos += 1;
        }
        // Java: while(tokenizer.hasMoreTokens())
        while *pos < tokens.len() {
            // Java: token = tokenizer.nextToken();
            let token = tokens[*pos];
            // Java: if (token.equals("="))
            if token == "=" {
                // Java: break;
                *pos += 1;
                break;
            }
            // Java: name.append(token);
            name.push_str(token);
            *pos += 1;
        }
        // Java: return name.toString().trim();
        name.trim().to_string()
    }

    // Java: String getValue(StringTokenizer tokenizer)
    pub fn getValue_tokenizer(&self, tokens: &[&str], pos: &mut usize) -> String {
        // Java: String token = tokenizer.nextToken();
        // Java: StringBuilder value;
        let mut value = String::new();
        // Java: for(value = new StringBuilder(); tokenizer.hasMoreTokens() && !token.equals("'") && !token.equals("\""); token = tokenizer.nextToken())
        // Java: String quote = token;
        let mut quote = "";
        while *pos < tokens.len() {
            let token = tokens[*pos];
            *pos += 1;
            if token == "'" || token == "\"" {
                quote = token;
                break;
            }
        }
        // Java: while(tokenizer.hasMoreTokens())
        while *pos < tokens.len() {
            // Java: token = tokenizer.nextToken();
            let token = tokens[*pos];
            *pos += 1;
            // Java: if (quote.equals(token))
            if token == quote {
                // Java: break;
                break;
            }
            // Java: value.append(token);
            value.push_str(token);
        }
        // Java: return value.toString();
        value
    }
}
