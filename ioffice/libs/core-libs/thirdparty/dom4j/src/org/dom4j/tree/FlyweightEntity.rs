// Java: import org.dom4j.Element
// Java: import org.dom4j.Node

// Java: public class FlyweightEntity extends AbstractEntity
pub struct FlyweightEntity {
    // Java field: String name
    pub(crate) name: String,
    // Java field: String text
    pub(crate) text: String,
}

// Java: public class FlyweightEntity extends AbstractEntity
impl FlyweightEntity {
    // Java: FlyweightEntity()
    pub fn new() -> Self {
        // Java: super();
        FlyweightEntity { name: String::new(), text: String::new() }
    }

    // Java: FlyweightEntity(String name)
    pub fn new_name(name: String) -> Self {
        // Java: super();
        // Java: this.name = name;
        FlyweightEntity { name, text: String::new() }
    }

    // Java: FlyweightEntity(String name, String text)
    pub fn new_name_text(name: String, text: String) -> Self {
        // Java: super();
        // Java: this.name = name;
        // Java: this.text = text;
        FlyweightEntity { name, text }
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: return this.name;
        self.name.clone()
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
        // Java: return this.text;
        self.text.clone()
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: String) {
        // Java: if (this.text != null)
        if !self.text.is_empty() {
            // Java: this.text = text;
            self.text = text;
        }
        // Java: else
        else {
            // Java: throw new UnsupportedOperationException("This Entity is read-only. It cannot be modified");
            panic!("This Entity is read-only. It cannot be modified");
        }
    }

    // Java: Node createXPathResult(Element parent)
    pub fn createXPathResult(&self) {
        // Java: return new DefaultEntity(parent, this.getName(), this.getText());
    }
}
