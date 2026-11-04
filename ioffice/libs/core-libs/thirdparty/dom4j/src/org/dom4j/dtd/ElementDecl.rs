use crate::org::dom4j::dtd::Declaration::Declaration;

// Java: public class ElementDecl implements Declaration
pub struct ElementDecl {
    // Java field: String name
    pub(crate) name: String,
    // Java field: String model
    pub(crate) model: String,
}

// Java: public class ElementDecl implements Declaration
impl ElementDecl {
    // Java: ElementDecl()
    pub fn new() -> Self {
        // Java: super();
        ElementDecl {
            name: String::new(),
            model: String::new(),
        }
    }

    // Java: ElementDecl(String name, String model)
    pub fn new_with_args(name: String, model: String) -> Self {
        // Java: super();
        // Java: this.name = name;
        // Java: this.model = model;
        ElementDecl { name, model }
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

    // Java: String getModel()
    pub fn getModel(&self) -> &str {
        // Java: return this.model;
        &self.model
    }

    // Java: void setModel(String model)
    pub fn setModel(&mut self, model: String) {
        // Java: this.model = model;
        self.model = model;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "<!ELEMENT " + this.name + " " + this.model + ">";
        format!("<!ELEMENT {} {}>", self.name, self.model)
    }
}

impl Declaration for ElementDecl {
}
