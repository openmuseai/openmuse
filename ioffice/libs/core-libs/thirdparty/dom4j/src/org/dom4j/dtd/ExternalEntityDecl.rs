use crate::org::dom4j::dtd::Declaration::Declaration;

// Java: public class ExternalEntityDecl implements Declaration
pub struct ExternalEntityDecl {
    // Java field: String name
    pub(crate) name: String,
    // Java field: String publicID
    pub(crate) publicID: Option<String>,
    // Java field: String systemID
    pub(crate) systemID: Option<String>,
}

// Java: public class ExternalEntityDecl implements Declaration
impl ExternalEntityDecl {
    // Java: ExternalEntityDecl()
    pub fn new() -> Self {
        // Java: super();
        ExternalEntityDecl {
            name: String::new(),
            publicID: None,
            systemID: None,
        }
    }

    // Java: ExternalEntityDecl(String name, String publicID, String systemID)
    pub fn new_with_args(name: String, public_id: Option<String>, system_id: Option<String>) -> Self {
        // Java: super();
        // Java: this.name = name;
        // Java: this.publicID = publicID;
        // Java: this.systemID = systemID;
        ExternalEntityDecl {
            name,
            publicID: public_id,
            systemID: system_id,
        }
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

    // Java: String getPublicID()
    pub fn getPublicID(&self) -> Option<&str> {
        // Java: return this.publicID;
        self.publicID.as_deref()
    }

    // Java: void setPublicID(String publicID)
    pub fn setPublicID(&mut self, public_id: Option<String>) {
        // Java: this.publicID = publicID;
        self.publicID = public_id;
    }

    // Java: String getSystemID()
    pub fn getSystemID(&self) -> Option<&str> {
        // Java: return this.systemID;
        self.systemID.as_deref()
    }

    // Java: void setSystemID(String systemID)
    pub fn setSystemID(&mut self, system_id: Option<String>) {
        // Java: this.systemID = systemID;
        self.systemID = system_id;
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
        // Java: if (this.publicID != null)
        if let Some(ref pid) = self.publicID
        {
            // Java: buffer.append(" PUBLIC \"");
            buffer.push_str(" PUBLIC \"");
            // Java: buffer.append(this.publicID);
            buffer.push_str(pid);
            // Java: buffer.append("\" ");
            buffer.push_str("\" ");
            // Java: if (this.systemID != null)
            if let Some(ref sid) = self.systemID
            {
                // Java: buffer.append("\"");
                buffer.push('"');
                // Java: buffer.append(this.systemID);
                buffer.push_str(sid);
                // Java: buffer.append("\" ");
                buffer.push_str("\" ");
            }
        }
        // Java: else if (this.systemID != null)
        else if let Some(ref sid) = self.systemID
        {
            // Java: buffer.append(" SYSTEM \"");
            buffer.push_str(" SYSTEM \"");
            // Java: buffer.append(this.systemID);
            buffer.push_str(sid);
            // Java: buffer.append("\" ");
            buffer.push_str("\" ");
        }
        // Java: buffer.append(">");
        buffer.push('>');
        // Java: return buffer.toString();
        buffer
    }
}

impl Declaration for ExternalEntityDecl {
}
