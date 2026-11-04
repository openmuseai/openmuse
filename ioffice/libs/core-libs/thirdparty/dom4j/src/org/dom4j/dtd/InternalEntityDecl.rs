use crate::org::dom4j::dtd::Declaration::Declaration;
use crate::org::dom4j::dtd::InternalDeclaration::InternalDeclaration;

// Java: public class InternalEntityDecl implements InternalDeclaration
pub struct InternalEntityDecl {
    // Java field: String name
    pub(crate) name: String,
    // Java field: String value
    pub(crate) value: String,
}

// Java: public class InternalEntityDecl implements InternalDeclaration
impl InternalEntityDecl {
    // Java: InternalEntityDecl()
    pub fn new() -> Self {
        // Java: super();
        InternalEntityDecl {
            name: String::new(),
            value: String::new(),
        }
    }

    // Java: InternalEntityDecl(String name, String value)
    pub fn new_with_args(name: String, value: String) -> Self {
        // Java: super();
        // Java: this.name = name;
        // Java: this.value = value;
        InternalEntityDecl { name, value }
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
        // Java: return this.name;
        &self.name
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: String) {
        // Java: this.name = name;
        self.name = name;
    }

    // Java: String getValue()
    pub fn getValue(&self) -> &str {
        // Java: return this.value;
        &self.value
    }

    // Java: void setValue(String value)
    pub fn setValue(&mut self, value: String) {
        // Java: this.value = value;
        self.value = value;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuffer buffer = new StringBuffer("<!ENTITY ");
        let mut buffer = String::from("<!ENTITY ");
        // Java: if (this.name.startsWith("%"))
        if self.name.starts_with('%')
        {
            // Java: buffer.append("% ");
            buffer.push_str("% ");
            // Java: buffer.append(this.name.substring(1));
            buffer.push_str(&self.name[1..]);
        }
        // Java: else
        else
        {
            // Java: buffer.append(this.name);
            buffer.push_str(&self.name);
        }
        // Java: buffer.append(" \"");
        buffer.push_str(" \"");
        // Java: buffer.append(this.escapeEntityValue(this.value));
        buffer.push_str(&self.escapeEntityValue(&self.value));
        // Java: buffer.append("\">");
        buffer.push_str("\">");
        // Java: return buffer.toString();
        buffer
    }

    // Java: String escapeEntityValue(String text)
    fn escapeEntityValue(&self, text: &str) -> String {
        // Java: StringBuffer result = new StringBuffer();
        let mut result = String::new();
        // Java: for(int i = 0; i < text.length(); ++i)
        for c in text.chars()
        {
            // Java: char c = text.charAt(i);
            // Java: switch (c)
            match c
            {
                // Java: case '"':
                '"' =>
                {
                    // Java: result.append("&#34;");
                    result.push_str("&#34;");
                    // Java: break;
                }
                // Java: case '&':
                '&' =>
                {
                    // Java: result.append("&#38;#38;");
                    result.push_str("&#38;#38;");
                    // Java: break;
                }
                // Java: case '\'':
                '\'' =>
                {
                    // Java: result.append("&#39;");
                    result.push_str("&#39;");
                    // Java: break;
                }
                // Java: case '<':
                '<' =>
                {
                    // Java: result.append("&#38;#60;");
                    result.push_str("&#38;#60;");
                    // Java: break;
                }
                // Java: case '>':
                '>' =>
                {
                    // Java: result.append("&#62;");
                    result.push_str("&#62;");
                    // Java: break;
                }
                // Java: default:
                _ =>
                {
                    // Java: if (c < ' ')
                    if c < ' '
                    {
                        // Java: result.append("&#" + c + ";");
                        result.push_str(&format!("&#{};", c as u32));
                    }
                    // Java: else
                    else
                    {
                        // Java: result.append(c);
                        result.push(c);
                    }
                }
            }
        }
        // Java: return result.toString();
        result
    }
}

impl Declaration for InternalEntityDecl {
}

impl InternalDeclaration for InternalEntityDecl {
}
