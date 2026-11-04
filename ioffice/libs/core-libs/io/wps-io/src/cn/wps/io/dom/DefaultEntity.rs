// Java: import cn.wps.io.dom.tree.FlyweightEntity
use super::tree::FlyweightEntity::FlyweightEntity;

// Java: public class DefaultEntity extends FlyweightEntity
pub struct DefaultEntity {
    pub(crate) base: FlyweightEntity,
    // Java field: Element parent
    pub(crate) parent: Option<Box<dyn std::any::Any>>,
}

// Java: public class DefaultEntity extends FlyweightEntity
impl DefaultEntity {
    // Java: DefaultEntity(String name)
    pub fn new_with_name(name: String) -> Self {
        // Java: super(name);
        DefaultEntity {
            base: FlyweightEntity::new_with_name(name),
            parent: None,
        }
    }

    // Java: DefaultEntity(String name, String text)
    pub fn new_with_name_text(name: String, text: String) -> Self {
        // Java: super(name, text);
        DefaultEntity {
            base: FlyweightEntity::new_with_name_text(name, text),
            parent: None,
        }
    }

    // Java: DefaultEntity(Element parent, String name, String text)
    pub fn new_with_parent(parent: Box<dyn std::any::Any>, name: String, text: String) -> Self {
        // Java: super(name, text);
        DefaultEntity {
            base: FlyweightEntity::new_with_name_text(name, text),
            // Java: this.parent = parent;
            parent: Some(parent),
        }
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: String) {
        // Java: this.name = name;
        self.base.name = Some(name);
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: String) {
        // Java: this.text = text;
        self.base.text = Some(text);
    }

    // Java: Element getParent()
    pub fn getParent(&self) -> Option<&dyn std::any::Any> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&mut self, parent: Box<dyn std::any::Any>) {
        // Java: this.parent = parent;
        self.parent = Some(parent);
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
