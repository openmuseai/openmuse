// Java: public class ExternalEntityDecl implements Declaration
pub struct ExternalEntityDecl {
    // Java field: String name
    pub(crate) name: String,
    // Java field: String publicID
    pub(crate) publicID: String,
    // Java field: String systemID
    pub(crate) systemID: String,
}

// Java: public class ExternalEntityDecl implements Declaration
impl ExternalEntityDecl {
    // Java: ExternalEntityDecl()
    pub fn ExternalEntityDecl(&self) {
      // Java: super();
      todo!();
    }

    // Java: ExternalEntityDecl(String name, String publicID, String systemID)
    pub fn ExternalEntityDecl_2(&self, name: &str, publicID: &str, systemID: &str) {
      // Java: super();
      todo!();
      // Java: this.name = name;
      todo!();
      // Java: this.publicID = publicID;
      todo!();
      // Java: this.systemID = systemID;
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

    // Java: String getPublicID()
    pub fn getPublicID(&self) -> String {
      // Java: return this.publicID;
      todo!();
    }

    // Java: void setPublicID(String publicID)
    pub fn setPublicID(&self, publicID: &str) {
      // Java: this.publicID = publicID;
      todo!();
    }

    // Java: String getSystemID()
    pub fn getSystemID(&self) -> String {
      // Java: return this.systemID;
      todo!();
    }

    // Java: void setSystemID(String systemID)
    pub fn setSystemID(&self, systemID: &str) {
      // Java: this.systemID = systemID;
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
      // Java: if (this.publicID != null)
        todo!();
      {
         // Java: buffer.append(" PUBLIC \"");
         todo!();
         // Java: buffer.append(this.publicID);
         todo!();
         // Java: buffer.append("\" ");
         todo!();
         // Java: if (this.systemID != null)
            todo!();
         {
            // Java: buffer.append("\"");
            todo!();
            // Java: buffer.append(this.systemID);
            todo!();
            // Java: buffer.append("\" ");
            todo!();
         }
      }
      // Java: else if (this.systemID != null)
        todo!();
      {
         // Java: buffer.append(" SYSTEM \"");
         todo!();
         // Java: buffer.append(this.systemID);
         todo!();
         // Java: buffer.append("\" ");
         todo!();
      }
      // Java: buffer.append(">");
      todo!();
      // Java: return buffer.toString();
      todo!();
    }

}

// impl Declaration for ExternalEntityDecl {
// }
