use crate::org::dom4j::Element::Element;
use super::FlyweightText::FlyweightText;

// Java: public class DefaultText extends FlyweightText
pub struct DefaultText {
    // Java field: FlyweightText base
    pub(crate) base: FlyweightText,
    // Java field: Element parent
    pub(crate) parent: Option<Box<dyn Element>>,
}

// Java: public class DefaultText extends FlyweightText
impl DefaultText {
    // Java: DefaultText(String text)
    pub fn new(text: String) -> Self {
        // Java: super(text);
        DefaultText {
            base: FlyweightText::new(text),
            parent: None,
        }
    }

    // Java: DefaultText(Element parent, String text)
    pub fn new_with_parent(parent: Box<dyn Element>, text: String) -> Self {
        // Java: super(text);
        // Java: this.parent = parent;
        DefaultText {
            base: FlyweightText::new(text),
            parent: Some(parent),
        }
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: String) {
        // Java: this.text = text;
        self.base.text = text;
    }

    // Java: Element getParent()
    pub fn getParent(&self) -> Option<&dyn Element> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&mut self, parent: Option<Box<dyn Element>>) {
        // Java: this.parent = parent;
        self.parent = parent;
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