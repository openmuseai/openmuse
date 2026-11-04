// Java: import org.dom4j.Element

// Java: public class DefaultEntity extends FlyweightEntity
pub struct DefaultEntity {
    // Java field: Element parent
    pub(crate) name: String,
    pub(crate) text: String,
}

// Java: public class DefaultEntity extends FlyweightEntity
impl DefaultEntity {
    // Java: DefaultEntity(String name)
    pub fn new_name(name: String) -> Self {
        // Java: super(name);
        DefaultEntity { name, text: String::new() }
    }

    // Java: DefaultEntity(String name, String text)
    pub fn new_name_text(name: String, text: String) -> Self {
        // Java: super(name, text);
        DefaultEntity { name, text }
    }

    // Java: DefaultEntity(Element parent, String name, String text)
    pub fn new_parent_name_text(name: String, text: String) -> Self {
        // Java: super(name, text);
        // Java: this.parent = parent;
        DefaultEntity { name, text }
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: String) {
        // Java: this.name = name;
        self.name = name;
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: String) {
        // Java: this.text = text;
        self.text = text;
    }

    // Java: Element getParent()
    pub fn getParent(&self) {
        // Java: return this.parent;
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&self) {
        // Java: this.parent = parent;
    }

    // Java: boolean supportsParent()
    pub fn supportsParent(&self) -> bool {
        // Java: return true;
        true
    }

    // Java: boolean isReadOnly()
    pub fn isReadOnly(&self) -> bool {
        // Java: return false;
        false
    }
}
