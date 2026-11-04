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
    pub fn InternalEntityDecl(&self) {
      // Java: super();
      todo!();
    }

    // Java: InternalEntityDecl(String name, String value)
    pub fn InternalEntityDecl_2(&self, name: &str, value: &str) {
      // Java: super();
      todo!();
      // Java: this.name = name;
      todo!();
      // Java: this.value = value;
      todo!();
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
      // Java: return this.name;
      todo!();
    }

    // Java: void setName(String name)
    pub fn setName(&self, name: &str) {
      // Java: this.name = name;
      todo!();
    }

    // Java: String getValue()
    pub fn getValue(&self) -> String {
      // Java: return this.value;
      todo!();
    }

    // Java: void setValue(String value)
    pub fn setValue(&self, value: &str) {
      // Java: this.value = value;
      todo!();
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuffer buffer = new StringBuffer("<!ENTITY ");
      todo!();
      // Java: if (this.name.startsWith("%"))
        todo!();
      {
         // Java: buffer.append("% ");
         todo!();
         // Java: buffer.append(this.name.substring(1));
         todo!();
      }
      // Java: else
        todo!();
      {
         // Java: buffer.append(this.name);
         todo!();
      }
      // Java: buffer.append(" \"");
      todo!();
      // Java: buffer.append(this.escapeEntityValue(this.value));
      todo!();
      // Java: buffer.append("\">");
      todo!();
      // Java: return buffer.toString();
      todo!();
    }

    // Java: String escapeEntityValue(String text)
    pub fn escapeEntityValue(&self, text: &str) -> String {
      // Java: StringBuffer result = new StringBuffer();
      todo!();
      // Java: for(int i = 0; i < text.length(); ++i)
        todo!();
      {
         // Java: char c = text.charAt(i);
         todo!();
         // Java: switch (c)
            todo!();
         {
            // Java: case '"':
            todo!();
               // Java: result.append("&#34;");
               todo!();
               // Java: break;
               todo!();
            // Java: case '&':
            todo!();
               // Java: result.append("&#38;#38;");
               todo!();
               // Java: break;
               todo!();
            // Java: case '\'':
            todo!();
               // Java: result.append("&#39;");
               todo!();
               // Java: break;
               todo!();
            // Java: case '<':
            todo!();
               // Java: result.append("&#38;#60;");
               todo!();
               // Java: break;
               todo!();
            // Java: case '>':
            todo!();
               // Java: result.append("&#62;");
               todo!();
               // Java: break;
               todo!();
            // Java: default:
            todo!();
               // Java: if (c < ' ')
                todo!();
               {
                  // Java: result.append("&#" + c + ";");
                  todo!();
               }
               // Java: else
                todo!();
               {
                  // Java: result.append(c);
                  todo!();
               }
         }
      }
      // Java: return result.toString();
      todo!();
    }

}

// impl InternalDeclaration for InternalEntityDecl {
// }
