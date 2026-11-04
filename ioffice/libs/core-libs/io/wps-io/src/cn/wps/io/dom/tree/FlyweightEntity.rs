// Java: public class FlyweightEntity extends AbstractEntity
pub struct FlyweightEntity {
    // Java field: String name
    pub(crate) name: Option<String>,
    // Java field: String text
    pub(crate) text: Option<String>,
}

// Java: public class FlyweightEntity extends AbstractEntity
impl FlyweightEntity {
    // Java: FlyweightEntity()
    pub fn new() -> Self {
        // Java: super();
        FlyweightEntity { name: None, text: None }
    }

    // Java: FlyweightEntity(String name)
    pub fn new_with_name(name: String) -> Self {
        // Java: super();
        FlyweightEntity {
            // Java: this.name = name;
            name: Some(name),
            text: None,
        }
    }

    // Java: FlyweightEntity(String name, String text)
    pub fn new_with_name_text(name: String, text: String) -> Self {
        // Java: super();
        FlyweightEntity {
            // Java: this.name = name;
            name: Some(name),
            // Java: this.text = text;
            text: Some(text),
        }
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
        // Java: return this.name;
        self.name.as_deref().unwrap_or("")
    }

    // Java: String getText()
    pub fn getText(&self) -> &str {
        // Java: return this.text;
        self.text.as_deref().unwrap_or("")
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: String) {
        // Java: if (this.text != null)
        if self.text.is_some() {
            // Java: this.text = text;
            self.text = Some(text);
        // Java: else
        } else {
            // Java: throw new UnsupportedOperationException("This Entity is read-only. It cannot be modified");
            panic!("This Entity is read-only. It cannot be modified");
        }
    }
}
